use async_trait::async_trait;
use ic_cdk::export::Principal;
use mockall::mock;

use ego_lib::ego_canister::TEgoCanister;
use ego_store_mod::app::EgoStoreApp;
use ego_store_mod::c2c::ego_tenant::TEgoTenant;
use ego_store_mod::service::EgoStoreService;
use ego_store_mod::state::EGO_STORE;
use ego_store_mod::tenant::Tenant;
use ego_store_mod::wallet::Wallet;
use ego_types::app::{App, AppId, Canister, CanisterType, Category, UserApp, Wasm};
use ego_types::app::CanisterType::BACKEND;
use ego_types::app::EgoError;
use ego_types::app::Version;
use ego_types::app_info::AppInfo;

static FILE_CANISTER_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";

static EXISTS_WALLET_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";
static EXISTS_USER_ID: &str = "225da-yaaaa-aaaah-qahrq-cai";
static EXISTS_USER_APP_BACKEND: &str = "224jh-lqaaa-aaaad-qaxda-cai";
static FAKE_USER_APP_BACKEND: &str = "223vg-sqaaa-aaaak-abtmq-cai";

static EXISTS_TENANT_ID: &str = "22ayq-aiaaa-aaaai-qgmma-cai";

static EXISTS_APP_ID: &str = "app_exists";
static APP_NAME: &str = "app1";
static APP_LOGO: &str = "logo";
static APP_DESCRIPTION: &str = "test is app description";

static TEST_APP_ID: &str = "app_test";

static TEST_WALLET_ID: &str = "227wz-liaaa-aaaaa-qaara-cai";
static TEST_USER_ID: &str = "2265i-mqaaa-aaaad-qbsga-cai";

static TEST_USER_APP_BACKEND: &str = "225cg-4iaaa-aaaaj-adouq-cai";

mock! {
  Tenant {}

  #[async_trait]
  impl TEgoTenant for Tenant {
    async fn app_main_install(
        &self,
        ego_tenant_id: Principal,
        wallet_id: Principal,
        user_id: Principal,
        wasm: &Wasm,
    ) -> Result<Principal, EgoError>;
    async fn app_main_upgrade(
        &self,
        ego_tenant_id: Principal,
        canister_id: Principal,
        wasm: &Wasm,
    ) -> Result<bool, EgoError>;
    fn canister_main_track(
        &self,
        ego_tenant_id: Principal,
        wallet_id: &Principal,
        canister_id: &Principal,
    );
    fn canister_main_untrack(
        &self,
        ego_tenant_id: Principal,
        wallet_id: &Principal,
        canister_id: &Principal,
    );
  }
}

mock! {
  Canister {}

  #[async_trait]
  impl TEgoCanister for Canister {
    fn ego_owner_set(&self, target_canister_id: Principal, principals: Vec<Principal>);
    fn ego_owner_add(&self, target_canister_id: Principal, principal: Principal);
    fn ego_owner_remove(&self, target_canister_id: Principal, principal: Principal);

    fn ego_user_set(&self, target_canister_id: Principal, user_ids: Vec<Principal>);
    fn ego_user_add(&self, target_canister_id: Principal, principal: Principal);
    fn ego_user_remove(&self, target_canister_id: Principal, principal: Principal);

    fn ego_op_add(&self, target_canister_id: Principal, user_id: Principal);

    fn ego_canister_add(&self, target_canister_id: Principal, name: String, principal: Principal);

    fn ego_controller_set(&self, target_canister_id: Principal, principals: Vec<Principal>);
    async fn ego_controller_add(&self, target_canister_id: Principal, principal: Principal) -> Result<(), String>;
    fn ego_controller_remove(&self, target_canister_id: Principal, principal: Principal);

    async fn balance_get(&self, target_canister_id: Principal) -> Result<u128, String>;

    // app info
    async fn app_info_update(&self, target_canister_id: Principal, wallet_id: Principal, app_id: AppId, version: Version) -> Result<(), String>;
    async fn app_info_get(&self, target_canister_id: Principal) -> Result<AppInfo, String>;
    async fn app_version_check(&self, target_canister_id: Principal) -> Result<App, String>;

    // canister upgrade
    fn ego_canister_upgrade(&self, target_canister_id: Principal);
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
    ego_store
      .borrow_mut()
      .tenants
      .insert(tenant_principal, Tenant::new(tenant_principal));

    // add exists app
    let wasm = Wasm::new(EXISTS_APP_ID.to_string(), version, BACKEND, file_canister);

    let app = App {
      app_id: EXISTS_APP_ID.to_string(),
      name: APP_NAME.to_string(),
      category: Category::Vault,
      logo: APP_LOGO.to_string(),
      description: APP_DESCRIPTION.to_string(),
      current_version: version,
      price: 0.0,
    };

    let ego_store_app = EgoStoreApp::new(
      app.clone(),
      wasm,
    );

    ego_store
      .borrow_mut()
      .apps
      .insert(EXISTS_APP_ID.to_string(), ego_store_app);


    // add test app
    let test_wasm = Wasm::new(TEST_APP_ID.to_string(), version, BACKEND, file_canister);

    let test_app = App {
      app_id: TEST_APP_ID.to_string(),
      name: APP_NAME.to_string(),
      category: Category::Vault,
      logo: APP_LOGO.to_string(),
      description: APP_DESCRIPTION.to_string(),
      current_version: version,
      price: 0.0,
    };

    let test_ego_store_app = EgoStoreApp::new(
      test_app.clone(), test_wasm,
    );
    ego_store
      .borrow_mut()
      .apps
      .insert(TEST_APP_ID.to_string(), test_ego_store_app);

    // add wallet
    let mut wallet = Wallet::new(tenant_principal, wallet_principal, user_principal);


    let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();
    wallet.apps.insert(
      backend_principal,
      UserApp::new(
        &app,
        Canister::new(backend_principal, CanisterType::BACKEND),
      ),
    );
    ego_store
      .borrow_mut()
      .wallets
      .insert(wallet_principal, wallet);
  });
}

#[test]
fn app_main_list() {
  set_up();

  let result = EgoStoreService::app_main_list();
  assert!(result.is_ok());

  let apps = result.unwrap();
  assert_eq!(2, apps.len());

  let app = apps.first().unwrap();
  assert_eq!(EXISTS_APP_ID, app.app_id);
}

#[test]
fn app_main_get() {
  set_up();

  let ego_store_app = EgoStoreService::app_main_get(&EXISTS_APP_ID.to_string()).unwrap();

  assert_eq!(EXISTS_APP_ID, ego_store_app.app.app_id);
}

#[test]
fn app_main_get_failed_with_not_exists_wallet() {
  set_up();

  let result = EgoStoreService::app_main_get(&"not_exists".to_string());
  assert!(result.is_err());
}

#[test]
fn wallet_main_register() {
  set_up();

  let wallet_principal = Principal::from_text(TEST_WALLET_ID).unwrap();
  let user_principal = Principal::from_text(TEST_USER_ID).unwrap();
  let result = EgoStoreService::wallet_main_register(wallet_principal, user_principal);
  assert!(result.is_ok());

  let tenant_id = result.unwrap();
  assert_eq!(EXISTS_TENANT_ID, tenant_id.to_string())
}

#[test]
fn wallet_app_list() {
  set_up();

  let wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();

  let result = EgoStoreService::wallet_app_list(&wallet_id);
  assert!(result.is_ok());

  let apps = result.unwrap();
  assert_eq!(1, apps.len());
  let user_app = apps.first().unwrap();
  assert_eq!(EXISTS_APP_ID, user_app.app.app_id)
}

#[test]
fn wallet_app_list_failed_wallet_not_exists() {
  set_up();
  let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();
  // wallet not exists
  let result = EgoStoreService::wallet_app_list(&wallet_id);
  assert!(result.is_err());
  let wallet_not_exists = result.unwrap_err();
  assert_eq!(3006, wallet_not_exists.code);
  assert_eq!("ego-store: wallet not exists", wallet_not_exists.msg);
}

#[tokio::test]
async fn wallet_app_install_success() {
  set_up();

  let wallet_principal = Principal::from_text(TEST_WALLET_ID).unwrap();
  let user_principal = Principal::from_text(TEST_USER_ID).unwrap();

  let backend_principal = Principal::from_text(TEST_USER_APP_BACKEND).unwrap();

  // register wallet
  let result = EgoStoreService::wallet_main_register(wallet_principal, user_principal);
  assert!(result.is_ok());

  // get app list before app install
  let result = EgoStoreService::wallet_app_list(&wallet_principal);
  assert!(result.is_ok());
  assert_eq!(0, result.unwrap().len());

  // install app
  let mut ego_tenant = MockTenant::new();

  ego_tenant
    .expect_app_main_install()
    .returning(move |_, _, _, _| Ok(backend_principal));

  let mut ego_canister = MockCanister::new();
  ego_canister.expect_app_info_update().returning(|_, _, _, _| {
    Ok(())
  });

  let ego_store_app = EgoStoreService::app_main_get(&TEST_APP_ID.to_string()).unwrap();
  let result = EgoStoreService::wallet_app_install(ego_tenant, ego_canister, wallet_principal, ego_store_app).await;
  assert!(result.is_ok());

  // get app list after app install
  let result = EgoStoreService::wallet_app_list(
    &wallet_principal);
  assert!(result.is_ok());
  assert_eq!(1, result.as_ref().unwrap().len());

  match result {
    Ok(app_installeds) => {
      let app_installed = app_installeds.get(0).unwrap();
      assert_eq!(
        backend_principal,
        app_installed.canister.canister_id
      );
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
  let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();

  let result = EgoStoreService::wallet_app_upgrade(ego_tenant, &wallet_id, &backend_principal).await;
  assert!(result.is_err());
  assert_eq!(3006, result.unwrap_err().code);
}

#[tokio::test]
async fn wallet_app_upgrade_not_installed_app() {
  set_up();

  let wallet_principal = Principal::from_text(TEST_WALLET_ID).unwrap();
  let user_principal = Principal::from_text(TEST_USER_ID).unwrap();
  let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();

  // register wallet
  let result = EgoStoreService::wallet_main_register(wallet_principal, user_principal);
  assert!(result.is_ok());

  // upgrade not installed app
  let ego_tenant = MockTenant::new();
  let result = EgoStoreService::wallet_app_upgrade(ego_tenant, &wallet_principal, &backend_principal).await;
  assert!(result.is_err());
  assert_eq!(3010, result.unwrap_err().code);
}

#[tokio::test]
async fn wallet_app_upgrade_success() {
  set_up();

  let exist_wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
  let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();


  // get app list before upgrade
  let result = EgoStoreService::wallet_app_list(&exist_wallet_id);
  assert!(result.is_ok());
  assert_eq!(1, result.unwrap().len());

  // upgrade installed app
  let mut ego_tenant = MockTenant::new();
  ego_tenant
    .expect_app_main_upgrade()
    .returning(|_, _, _| Ok(true));
  let result = EgoStoreService::wallet_app_upgrade(ego_tenant, &exist_wallet_id, &backend_principal).await;
  assert!(result.is_ok());

  // get app list after upgrade
  let result = EgoStoreService::wallet_app_list(&exist_wallet_id);
  assert!(result.is_ok());
  assert_eq!(1, result.unwrap().len());
}

#[test]
fn wallet_app_remove_not_exists_wallet() {
  set_up();

  let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();
  let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();

  // remove not exists wallet
  let result = EgoStoreService::wallet_app_remove(&wallet_id, &backend_principal);
  assert!(result.is_err());
  assert_eq!(3006, result.unwrap_err().code);
}

#[test]
fn wallet_app_remove_not_exists_app() {
  set_up();

  let exist_wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
  let fake_principal = Principal::from_text(FAKE_USER_APP_BACKEND.to_string()).unwrap();

  // install with not exists wallet
  let result = EgoStoreService::wallet_app_remove(&exist_wallet_id, &fake_principal);
  println!("{:?}", result);
  assert!(result.is_err());
  assert_eq!(3010, result.unwrap_err().code);
}

#[test]
fn wallet_app_remove_not_installed_app() {
  set_up();

  let wallet_principal = Principal::from_text(TEST_WALLET_ID).unwrap();
  let user_principal = Principal::from_text(TEST_USER_ID).unwrap();
  let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();

  // register wallet
  let result = EgoStoreService::wallet_main_register(wallet_principal, user_principal);
  assert!(result.is_ok());

  // remove not exists wallet
  let result = EgoStoreService::wallet_app_remove(&wallet_principal, &backend_principal);
  assert!(result.is_err());
  assert_eq!(3010, result.unwrap_err().code);
}

#[test]
fn wallet_app_remove_success() {
  set_up();

  let exist_wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
  let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();

  // get app list before upgrade
  let result = EgoStoreService::wallet_app_list(&exist_wallet_id);
  assert!(result.is_ok());
  assert_eq!(1, result.unwrap().len());

  // upgrade installed app
  let result = EgoStoreService::wallet_app_remove(&exist_wallet_id, &backend_principal);
  assert!(result.is_ok());

  // get app list after upgrade
  let result = EgoStoreService::wallet_app_list(&exist_wallet_id);
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

#[test]
fn wallet_tenant_get_failed_wallet_not_exists() {
  set_up();
  let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();
  let exist_wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();

  let result = EgoStoreService::wallet_tenant_get(wallet_id);
  assert!(result.is_err());
  let wallet_not_exists = result.unwrap_err();
  assert_eq!(3006, wallet_not_exists.code);
  assert_eq!("ego-store: wallet not exists", wallet_not_exists.msg);

  let result = EgoStoreService::wallet_tenant_get(exist_wallet_id);
  assert!(result.is_ok());
  assert_eq!(EXISTS_TENANT_ID, result.unwrap().to_string())
}

#[tokio::test]
async fn wallet_canister_track_wallet_not_exists() {
  set_up();

  let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();
  let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();

  let mut ego_tenant = MockTenant::new();
  ego_tenant
    .expect_canister_main_track()
    .returning(|_, _, _| ());
  // ego-store wallet not exists
  let canister_track =
    EgoStoreService::wallet_canister_track(ego_tenant, &wallet_id, &backend_principal);
  assert!(canister_track.is_err());
  assert_eq!(3006, canister_track.as_ref().unwrap_err().code);
  assert_eq!(
    "ego-store: wallet not exists",
    canister_track.as_ref().unwrap_err().msg
  );
}

#[tokio::test]
async fn wallet_canister_track_wallet_app_not_install() {
  set_up();
  let wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
  let fake_principal = Principal::from_text(FAKE_USER_APP_BACKEND.to_string()).unwrap();

  let mut ego_tenant = MockTenant::new();
  ego_tenant
    .expect_canister_main_track()
    .returning(|_, _, _| ());
  //ego-store app not install
  let canister_track =
    EgoStoreService::wallet_canister_track(ego_tenant, &wallet_id, &fake_principal);
  assert!(canister_track.is_err());
  assert_eq!(3010, canister_track.as_ref().unwrap_err().code);
  assert_eq!(
    "ego-store: app not install",
    canister_track.as_ref().unwrap_err().msg
  );
}

#[tokio::test]
async fn wallet_canister_track_success() {
  set_up();
  let wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
  let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();
  let mut ego_tenant = MockTenant::new();
  ego_tenant
    .expect_canister_main_track()
    .returning(|_, _, _| ());
  // wallet canister track success
  let canister_track =
    EgoStoreService::wallet_canister_track(ego_tenant, &wallet_id, &backend_principal);
  assert!(canister_track.is_ok());
}

#[tokio::test]
async fn wallet_canister_untrack_wallet_not_exists() {
  set_up();
  let mut ego_tenant = MockTenant::new();
  ego_tenant
    .expect_canister_main_untrack()
    .returning(|_, _, _| ());
  let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();
  let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();

  let canister_untrack =
    EgoStoreService::wallet_canister_untrack(ego_tenant, &wallet_id, &backend_principal);
  assert!(canister_untrack.is_err());
  assert_eq!(3006, canister_untrack.as_ref().unwrap_err().code);
  assert_eq!(
    "ego-store: wallet not exists",
    canister_untrack.as_ref().unwrap_err().msg
  );
}

#[tokio::test]
async fn wallet_canister_untrack_app_not_install() {
  set_up();
  let mut ego_tenant = MockTenant::new();
  ego_tenant
    .expect_canister_main_untrack()
    .returning(|_, _, _| ());
  let wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
  let fake_principal = Principal::from_text(FAKE_USER_APP_BACKEND.to_string()).unwrap();

  let canister_untrack =
    EgoStoreService::wallet_canister_untrack(ego_tenant, &wallet_id, &fake_principal);
  assert!(canister_untrack.is_err());
  assert_eq!(3010, canister_untrack.as_ref().unwrap_err().code);
  assert_eq!(
    "ego-store: app not install",
    canister_untrack.as_ref().unwrap_err().msg
  );
}

#[tokio::test]
async fn wallet_canister_untrack() {
  set_up();
  let mut ego_tenant = MockTenant::new();
  ego_tenant
    .expect_canister_main_untrack()
    .returning(|_, _, _| ());
  let wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
  let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();

  let canister_untrack =
    EgoStoreService::wallet_canister_untrack(ego_tenant, &wallet_id, &backend_principal);
  assert!(canister_untrack.is_ok());
}