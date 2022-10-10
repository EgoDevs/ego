use async_trait::async_trait;
use ic_cdk::export::Principal;
use mockall::mock;

use ego_store_mod::c2c::ego_tenant::TEgoTenant;
use ego_store_mod::service::EgoStoreService;
use ego_store_mod::state::EGO_STORE;
use ego_store_mod::tenant::Tenant;
use ego_store_mod::types::QueryParam;
use ego_store_mod::user_app::{UserApp};
use ego_store_mod::wallet::Wallet;
use ego_types::app::{App, Canister, CanisterType, Category, Wasm};
use ego_types::app::CanisterType::{ASSET, BACKEND};
use ego_types::ego_error::EgoError;
use ego_types::version::Version;

static FILE_CANISTER_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";

static EXISTS_WALLET_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";
static EXISTS_USER_ID: &str = "225da-yaaaa-aaaah-qahrq-cai";
static EXISTS_USER_APP_FRONTEND: &str = "223xb-saaaa-aaaaf-arlqa-cai";
static EXISTS_USER_APP_BACKEND: &str = "224jh-lqaaa-aaaad-qaxda-cai";

static EXISTS_TENANT_ID: &str = "22ayq-aiaaa-aaaai-qgmma-cai";

static EXISTS_APP_ID: &str = "app_exists";
static APP_NAME: &str = "app1";
static APP_LOGO: &str = "logo";
static APP_DESCRIPTION: &str = "test is app description";

static TEST_APP_ID: &str = "app_test";

static TEST_WALLET_ID: &str = "227wz-liaaa-aaaaa-qaara-cai";
static TEST_USER_ID: &str = "2265i-mqaaa-aaaad-qbsga-cai";

static TEST_USER_APP_FRONTEND: &str = "224lq-3aaaa-aaaaf-ase7a-cai";
static TEST_USER_APP_BACKEND: &str = "225cg-4iaaa-aaaaj-adouq-cai";


mock! {
  Tenant {}

  #[async_trait]
  impl TEgoTenant for Tenant {
    async fn app_main_install(&self, ego_tenant_id: Principal, wallet_id: Principal, user_id: Principal, wasm: Wasm) -> Result<Principal, EgoError>;
    async fn app_main_upgrade(&self, ego_tenant_id: Principal, canister_id: Principal, wasm: Wasm) -> Result<bool, EgoError>;
    async fn canister_main_track(&self, ego_tenant_id: Principal, wallet_id: Principal, canister_id: Principal) -> Result<bool, EgoError>;
    async fn canister_main_untrack(&self, ego_tenant_id: Principal, wallet_id: Principal, canister_id: Principal) -> Result<bool, EgoError>;
  }
}


pub fn set_up() {
  let tenant_principal = Principal::from_text(EXISTS_TENANT_ID.to_string()).unwrap();
  let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();
  let user_principal = Principal::from_text(EXISTS_USER_ID.to_string()).unwrap();
  let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();

  let version = Version::new(1, 0, 1);

  EGO_STORE.with(|ego_store| {
    // add tenant
    ego_store.borrow_mut().tenants.insert(tenant_principal, Tenant::new(tenant_principal));

    // add exists app
    let frontend = Wasm::new(EXISTS_APP_ID.to_string(), version, ASSET, None);
    let backend = Wasm::new(EXISTS_APP_ID.to_string(), version, BACKEND, Some(file_canister));
    let app = App::new(EXISTS_APP_ID.to_string(), APP_NAME.to_string(), Category::Vault, APP_LOGO.to_string(), APP_DESCRIPTION.to_string(), version, frontend, backend, 1.2f32);
    ego_store.borrow_mut().apps.insert(EXISTS_APP_ID.to_string(), app);

    let frontend = Wasm::new(TEST_APP_ID.to_string(), version, ASSET, Some(file_canister));
    let backend = Wasm::new(TEST_APP_ID.to_string(), version, BACKEND, Some(file_canister));
    let app = App::new(TEST_APP_ID.to_string(), APP_NAME.to_string(), Category::Vault, APP_LOGO.to_string(), APP_DESCRIPTION.to_string(), version, frontend, backend, 1.2f32);
    ego_store.borrow_mut().apps.insert(TEST_APP_ID.to_string(), app);

    // add wallet
    let mut wallet = Wallet::new(tenant_principal, wallet_principal, user_principal);

    let frontend_principal = Principal::from_text(EXISTS_USER_APP_FRONTEND.to_string()).unwrap();
    let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();
    wallet.apps.insert(EXISTS_APP_ID.to_string(), UserApp::new(&EXISTS_APP_ID.to_string(), &version, Some(Canister::new(frontend_principal, CanisterType::ASSET)), Some(Canister::new(backend_principal, CanisterType::BACKEND))));
    ego_store.borrow_mut().wallets.insert(wallet_principal, wallet);
  });
}


#[test]
fn app_main_list() {
  set_up();

  let result = EgoStoreService::app_main_list(QueryParam::ByCategory { category: Category::Vault });
  assert!(result.is_ok());

  let apps = result.unwrap();
  assert_eq!(2, apps.len());

  let app = apps.first().unwrap();
  assert_eq!(EXISTS_APP_ID, app.app_id);
}

#[test]
fn app_main_get() {
  set_up();

  let app = EgoStoreService::app_main_get(EXISTS_APP_ID.to_string()).unwrap();

  assert_eq!(EXISTS_APP_ID, app.app_id);
}

#[test]
fn app_main_get_failed_with_not_exists_wallet() {
  set_up();

  let result = EgoStoreService::app_main_get("not_exists".to_string());
  assert!(result.is_err());
}

#[test]
fn wallet_main_new() {
  set_up();

  let wallet_principal = Principal::from_text(TEST_WALLET_ID).unwrap();
  let user_principal = Principal::from_text(TEST_USER_ID).unwrap();
  let result = EgoStoreService::wallet_main_new(wallet_principal, user_principal);
  assert!(result.is_ok());

  let tenant_id = result.unwrap();
  assert_eq!(EXISTS_TENANT_ID, tenant_id.to_string())
}

#[test]
fn wallet_app_list() {
  set_up();

  let wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();

  let result = EgoStoreService::wallet_app_list(wallet_id);
  assert!(result.is_ok());

  let apps = result.unwrap();
  assert_eq!(1, apps.len());
  let app = apps.first().unwrap();
  assert_eq!(EXISTS_APP_ID, app.app_id)
}

#[tokio::test]
async fn wallet_app_install_not_exists_wallet() {
  set_up();

  let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();

  // install with not exists wallet
  let ego_tenant = MockTenant::new();

  let result = EgoStoreService::wallet_app_install(ego_tenant, wallet_id, EXISTS_APP_ID.to_string()).await;
  assert!(result.is_err());
  assert_eq!(3006, result.unwrap_err().code);
}

#[tokio::test]
async fn wallet_app_install_not_exists_app() {
  set_up();

  let wallet_principal = Principal::from_text(TEST_WALLET_ID).unwrap();
  let user_principal = Principal::from_text(TEST_USER_ID).unwrap();

  // register wallet
  let result = EgoStoreService::wallet_main_new(wallet_principal, user_principal);
  assert!(result.is_ok());

  // install app
  let ego_tenant = MockTenant::new();

  let result = EgoStoreService::wallet_app_install(ego_tenant, wallet_principal, "not_exists_app".to_string()).await;
  assert!(result.is_err());
  assert_eq!(3002, result.unwrap_err().code);
}

#[tokio::test]
async fn wallet_app_install_installed_app() {
  set_up();

  let exist_wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();

  // install app
  let ego_tenant = MockTenant::new();

  let result = EgoStoreService::wallet_app_install(ego_tenant, exist_wallet_id, EXISTS_APP_ID.to_string()).await;
  assert!(result.is_err());
  assert_eq!(3009, result.unwrap_err().code);
}


#[tokio::test]
async fn wallet_app_install_success() {
  set_up();

  let wallet_principal = Principal::from_text(TEST_WALLET_ID).unwrap();
  let user_principal = Principal::from_text(TEST_USER_ID).unwrap();

  let frontend_principal = Principal::from_text(TEST_USER_APP_FRONTEND).unwrap();
  let backend_principal = Principal::from_text(TEST_USER_APP_BACKEND).unwrap();

  // register wallet
  let result = EgoStoreService::wallet_main_new(wallet_principal, user_principal);
  assert!(result.is_ok());

  // get app list before app install
  let result = EgoStoreService::wallet_app_list(wallet_principal);
  assert!(result.is_ok());
  assert_eq!(0, result.unwrap().len());

  // install app
  let mut ego_tenant = MockTenant::new();
  ego_tenant.expect_app_main_install().times(1).returning(move |_, _, _, _| {
    Ok(frontend_principal)
  });
  ego_tenant.expect_app_main_install().returning(move |_, _, _, _| {
    Ok(backend_principal)
  });

  let result = EgoStoreService::wallet_app_install(ego_tenant, wallet_principal, TEST_APP_ID.to_string()).await;
  assert!(result.is_ok());

  // get app list after app install
  let result = EgoStoreService::wallet_app_list(wallet_principal);
  assert!(result.is_ok());
  assert_eq!(1, result.as_ref().unwrap().len());

  match result {
    Ok(app_installeds) => {
      let app_installed = app_installeds.get(0).unwrap();
      assert_eq!(frontend_principal, app_installed.frontend.as_ref().unwrap().canister_id);
      assert_eq!(backend_principal, app_installed.backend.as_ref().unwrap().canister_id);
    }
    Err(_) => {
      panic!("should not go here")
    }
  }
}

#[tokio::test]
async fn wallet_app_upgrade_not_exists_wallet() {
  set_up();

  let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();

  // install with not exists wallet
  let ego_tenant = MockTenant::new();

  let result = EgoStoreService::wallet_app_upgrade(ego_tenant, wallet_id, EXISTS_APP_ID.to_string()).await;
  assert!(result.is_err());
  assert_eq!(3006, result.unwrap_err().code);
}

#[tokio::test]
async fn wallet_app_upgrade_not_exists_app() {
  set_up();

  let wallet_principal = Principal::from_text(TEST_WALLET_ID).unwrap();
  let user_principal = Principal::from_text(TEST_USER_ID).unwrap();

  // register wallet
  let result = EgoStoreService::wallet_main_new(wallet_principal, user_principal);
  assert!(result.is_ok());

  // install app
  let ego_tenant = MockTenant::new();

  let result = EgoStoreService::wallet_app_install(ego_tenant, wallet_principal, "not_exists_app".to_string()).await;
  assert!(result.is_err());
  assert_eq!(3002, result.unwrap_err().code);
}

#[tokio::test]
async fn wallet_app_upgrade_not_installed_app() {
  set_up();

  let wallet_principal = Principal::from_text(TEST_WALLET_ID).unwrap();
  let user_principal = Principal::from_text(TEST_USER_ID).unwrap();

  // register wallet
  let result = EgoStoreService::wallet_main_new(wallet_principal, user_principal);
  assert!(result.is_ok());

  // upgrade not installed app
  let ego_tenant = MockTenant::new();

  let result = EgoStoreService::wallet_app_upgrade(ego_tenant, wallet_principal, EXISTS_APP_ID.to_string()).await;
  assert!(result.is_err());
  assert_eq!(3010, result.unwrap_err().code);
}

#[tokio::test]
async fn wallet_app_upgrade_success() {
  set_up();

  let exist_wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();

  // get app list before upgrade
  let result = EgoStoreService::wallet_app_list(exist_wallet_id);
  assert!(result.is_ok());
  assert_eq!(1, result.unwrap().len());

  // upgrade installed app
  let mut ego_tenant = MockTenant::new();
  ego_tenant.expect_app_main_upgrade().returning(|_, _, _| {
    Ok(true)
  });
  let result = EgoStoreService::wallet_app_upgrade(ego_tenant, exist_wallet_id, EXISTS_APP_ID.to_string()).await;
  assert!(result.is_ok());

  // get app list after upgrade
  let result = EgoStoreService::wallet_app_list(exist_wallet_id);
  assert!(result.is_ok());
  assert_eq!(1, result.unwrap().len());
}

#[test]
fn wallet_app_remove_not_exists_wallet() {
  set_up();

  let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();

  // remove not exists wallet
  let result = EgoStoreService::wallet_app_remove(wallet_id, EXISTS_APP_ID.to_string());
  assert!(result.is_err());
  assert_eq!(3006, result.unwrap_err().code);
}

#[test]
fn wallet_app_remove_not_exists_app() {
  set_up();

  let exist_wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();

  // install with not exists wallet
  let result = EgoStoreService::wallet_app_remove(exist_wallet_id, "not_exists".to_string());
  assert!(result.is_err());
  assert_eq!(3010, result.unwrap_err().code);
}

#[test]
fn wallet_app_remove_not_installed_app() {
  set_up();

  let wallet_principal = Principal::from_text(TEST_WALLET_ID).unwrap();
  let user_principal = Principal::from_text(TEST_USER_ID).unwrap();

  // register wallet
  let result = EgoStoreService::wallet_main_new(wallet_principal, user_principal);
  assert!(result.is_ok());

  // remove not exists wallet
  let result = EgoStoreService::wallet_app_remove(wallet_principal, EXISTS_APP_ID.to_string());
  assert!(result.is_err());
  assert_eq!(3010, result.unwrap_err().code);
}

#[test]
fn wallet_app_remove_success() {
  set_up();

  let exist_wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();

  // get app list before upgrade
  let result = EgoStoreService::wallet_app_list(exist_wallet_id);
  assert!(result.is_ok());
  assert_eq!(1, result.unwrap().len());

  // upgrade installed app
  let result = EgoStoreService::wallet_app_remove(exist_wallet_id, EXISTS_APP_ID.to_string());
  assert!(result.is_ok());

  // get app list after upgrade
  let result = EgoStoreService::wallet_app_list(exist_wallet_id);
  assert!(result.is_ok());
  assert_eq!(0, result.unwrap().len());
}

#[test]
fn wallet_tenant_get() {
  set_up();

  let exist_wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();

  let result = EgoStoreService::wallet_tenant_get(exist_wallet_id);
  assert!(result.is_ok());
  assert_eq!(EXISTS_TENANT_ID, result.unwrap().to_string())
}