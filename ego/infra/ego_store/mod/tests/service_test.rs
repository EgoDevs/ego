use ic_cdk::export::Principal;
use ego_store_mod::service::EgoStoreService;
use ego_store_mod::state::EGO_STORE;
use ego_store_mod::tenant::Tenant;
use ego_store_mod::types::QueryParam;
use ego_store_mod::wallet::Wallet;
use ego_types::app::{App, Category, Wasm};
use ego_types::app::CanisterType::{ASSET, BACKEND};
use ego_types::version::Version;

static STORE_ID: &str = "22cl3-kqaaa-aaaaf-add7q-cai";

static FILE_CANISTER_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";

static EXISTS_WALLET_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";

static EXISTS_TENANT_ID: &str = "22ayq-aiaaa-aaaai-qgmma-cai";

static EXISTS_APP_ID: &str = "app_test";
static EXISTS_APP_NAME: &str = "test app";
static EXISTS_APP_LOGO: &str = "logo";
static EXISTS_APP_DESCRIPTION: &str = "test is app description";

static TEST_WALLET_ID: &str = "227wz-liaaa-aaaaa-qaara-cai";


pub fn set_up() {
  let tenant_principal = Principal::from_text(EXISTS_TENANT_ID.to_string()).unwrap();
  let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();
  let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();

  let version = Version::new(1, 0, 1);

  EGO_STORE.with(|ego_store| {
    // add tenant
    ego_store.borrow_mut().tenants.insert(tenant_principal, Tenant::new(tenant_principal));

    // add app
    let frontend = Wasm::new(EXISTS_APP_ID.to_string(), version, ASSET, None);
    let backend = Wasm::new(EXISTS_APP_ID.to_string(), version, BACKEND, Some(file_canister));
    let app = App::new(EXISTS_APP_ID.to_string(), EXISTS_APP_NAME.to_string(), Category::Vault, EXISTS_APP_LOGO.to_string(), EXISTS_APP_DESCRIPTION.to_string(), version, frontend, backend, 1.2f32);
    ego_store.borrow_mut().apps.insert(EXISTS_APP_ID.to_string(), app);

    // add wallet
    let mut wallet = Wallet::new(tenant_principal, wallet_principal);
    wallet.apps.push(EXISTS_APP_ID.to_string());
    ego_store.borrow_mut().wallets.insert(wallet_principal, wallet);
  });
}


#[test]
fn app_main_list() {
  set_up();

  let result = EgoStoreService::app_main_list(QueryParam::ByCategory { category: Category::Vault });
  assert!(result.is_ok());

  let apps = result.unwrap();
  assert_eq!(1, apps.len());

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

  let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();

  let result = EgoStoreService::wallet_main_new(wallet_id);
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

#[test]
fn wallet_app_install_not_exists_wallet() {
  set_up();

  let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();

  // install with not exists wallet
  let result = EgoStoreService::wallet_app_install(wallet_id, EXISTS_APP_ID.to_string());
  assert!(result.is_err());
  assert_eq!(3006, result.unwrap_err().code);
}

#[test]
fn wallet_app_install_not_exists_app() {
  set_up();

  let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();

  // register wallet
  let result = EgoStoreService::wallet_main_new(wallet_id);
  assert!(result.is_ok());

  // install app
  let result = EgoStoreService::wallet_app_install(wallet_id, "not_exists_app".to_string());
  assert!(result.is_err());
  assert_eq!(3002, result.unwrap_err().code);
}

#[test]
fn wallet_app_install_installed_app() {
  set_up();

  let exist_wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();

  // install app
  let result = EgoStoreService::wallet_app_install(exist_wallet_id, EXISTS_APP_ID.to_string());
  assert!(result.is_err());
  assert_eq!(3009, result.unwrap_err().code);
}


#[test]
fn wallet_app_install_success() {
  set_up();

  let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();

  // register wallet
  let result = EgoStoreService::wallet_main_new(wallet_id);
  assert!(result.is_ok());

  // get app list before app install
  let result = EgoStoreService::wallet_app_list(wallet_id);
  assert!(result.is_ok());
  assert_eq!(0, result.unwrap().len());

  // install app
  let result = EgoStoreService::wallet_app_install(wallet_id, EXISTS_APP_ID.to_string());
  assert!(result.is_ok());

  // get app list before app install
  let result = EgoStoreService::wallet_app_list(wallet_id);
  assert!(result.is_ok());
  assert_eq!(1, result.unwrap().len());

  // TODO: canisters test, 需要接入安装之后才能测试

  // install same app multi times
  let result = EgoStoreService::wallet_app_install(wallet_id, EXISTS_APP_ID.to_string());
  assert!(result.is_err());
  assert_eq!(3009, result.unwrap_err().code);
}

#[test]
fn wallet_app_upgrade_not_exists_wallet() {
  set_up();

  let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();

  // install with not exists wallet
  let result = EgoStoreService::wallet_app_upgrade(wallet_id, EXISTS_APP_ID.to_string());
  assert!(result.is_err());
  assert_eq!(3006, result.unwrap_err().code);
}

#[test]
fn wallet_app_upgrade_not_exists_app() {
  set_up();

  let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();

  // register wallet
  let result = EgoStoreService::wallet_main_new(wallet_id);
  assert!(result.is_ok());

  // install app
  let result = EgoStoreService::wallet_app_install(wallet_id, "not_exists_app".to_string());
  assert!(result.is_err());
  assert_eq!(3002, result.unwrap_err().code);
}

#[test]
fn wallet_app_upgrade_not_installed_app() {
  set_up();

  let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();

  // register wallet
  let result = EgoStoreService::wallet_main_new(wallet_id);
  assert!(result.is_ok());

  // upgrade not installed app
  let result = EgoStoreService::wallet_app_upgrade(wallet_id, EXISTS_APP_ID.to_string());
  assert!(result.is_err());
  assert_eq!(3010, result.unwrap_err().code);
}

#[test]
fn wallet_app_upgrade_success() {
  set_up();

  let exist_wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();

  // get app list before upgrade
  let result = EgoStoreService::wallet_app_list(exist_wallet_id);
  assert!(result.is_ok());
  assert_eq!(1, result.unwrap().len());

  // upgrade installed app
  let result = EgoStoreService::wallet_app_upgrade(exist_wallet_id, EXISTS_APP_ID.to_string());
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
  assert_eq!(3002, result.unwrap_err().code);
}

#[test]
fn wallet_app_remove_not_installed_app() {
  set_up();

  let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();

  // register wallet
  let result = EgoStoreService::wallet_main_new(wallet_id);
  assert!(result.is_ok());

  // remove not exists wallet
  let result = EgoStoreService::wallet_app_remove(wallet_id, EXISTS_APP_ID.to_string());
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
fn wallet_order() {
  set_up();

  let exist_wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
  let store_id = Principal::from_text(STORE_ID).unwrap();

  // get order list before make order
  let result = EgoStoreService::wallet_order_list(exist_wallet_id);
  assert!(result.is_ok());
  assert_eq!(0, result.unwrap().len());

  // create order
  let result = EgoStoreService::wallet_order_new(exist_wallet_id, store_id, 1.2f32);
  assert!(result.is_ok());
  assert_eq!(1, result.unwrap().memo.0);

  // get order list after make order
  let result = EgoStoreService::wallet_order_list(exist_wallet_id);
  assert!(result.is_ok());
  assert_eq!(1, result.unwrap().len());
}

#[test]
fn wallet_tenant_get() {
  set_up();

  let exist_wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();

  let result = EgoStoreService::wallet_tenant_get(exist_wallet_id);
  assert!(result.is_ok());
  assert_eq!(EXISTS_TENANT_ID, result.unwrap().to_string())
}