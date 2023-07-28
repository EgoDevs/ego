use async_trait::async_trait;
use candid::Principal;
use mockall::mock;

use ego_lib::ego_canister::TEgoCanister;
use ego_lib::inject_mock_ego_canister;
use ego_store_mod::c2c::ego_tenant::TEgoTenant;
use ego_store_mod::service::EgoStoreService;
use ego_store_mod::types::ego_store_app::EgoStoreApp;
use ego_store_mod::types::tenant::Tenant;
use ego_store_mod::types::user_app::UserApp;
use ego_store_mod::types::wallet::Wallet;
use ego_types::app::{App, AppId, Canister, CanisterType, Category, Wasm};
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

static NEW_WALLET_ID: &str = "2222s-4iaaa-aaaaf-ax2uq-cai";

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
    async fn app_main_reinstall(
        &self,
        ego_tenant_id: Principal,
        canister_id: Principal,
        wasm: &Wasm,
    ) -> Result<bool, EgoError>;
    fn canister_main_track(
        &self,
        ego_tenant_id: Principal,
        canister_id: &Principal,
    );
    fn canister_main_untrack(&self, ego_tenant_id: Principal, canister_id: &Principal);
    fn app_main_delete(&self, ego_tenant_id: Principal, canister_id: &Principal);
  }
}

inject_mock_ego_canister!();

pub fn set_up() {
  let tenant_principal = Principal::from_text(EXISTS_TENANT_ID.to_string()).unwrap();
  let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();
  let user_principal = Principal::from_text(EXISTS_USER_ID.to_string()).unwrap();
  let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();

  let version = Version::new(1, 0, 1);

  // add tenant
  let tenant = Tenant::new(&tenant_principal);
  tenant.save();

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
    app_hash: "".to_string(),
  };

  let mut ego_store_app = EgoStoreApp::new(&app, &wasm);
  ego_store_app.save();

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
    app_hash: "".to_string(),
  };

  let mut test_ego_store_app = EgoStoreApp::new(&test_app, &test_wasm);
  test_ego_store_app.save();

  // add wallet
  let mut wallet = Wallet::new(&tenant_principal, &wallet_principal, &user_principal);
  wallet.save();

  let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();
  let mut user_app = UserApp::new(&app,
                                  Canister::new(backend_principal, CanisterType::BACKEND), Some(wallet.wallet_id));
  user_app.save();


  let new_wallet_principal = Principal::from_text(NEW_WALLET_ID.to_string()).unwrap();
  let mut new_wallet = Wallet::new(&tenant_principal, &new_wallet_principal, &user_principal);
  new_wallet.save();
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
  let result = EgoStoreService::wallet_main_register(&wallet_principal, &user_principal);
  assert!(result.is_ok());

  let tenant_id = result.unwrap();
  assert_eq!(EXISTS_TENANT_ID, tenant_id.to_string())
}

#[test]
fn wallet_app_list() {
  set_up();

  let wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();

  let apps = EgoStoreService::wallet_app_list(&wallet_id);
  assert_eq!(1, apps.len());
  let user_app = apps.first().unwrap();
  assert_eq!(EXISTS_APP_ID, user_app.app.app_id)
}

#[test]
fn wallet_app_list_failed_wallet_not_exists() {
  set_up();
  let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();

  let apps = EgoStoreService::wallet_app_list(&wallet_id);
  assert_eq!(0, apps.len());
}

#[tokio::test]
async fn wallet_app_install_success() {
  set_up();

  let wallet_principal = Principal::from_text(TEST_WALLET_ID).unwrap();
  let user_principal = Principal::from_text(TEST_USER_ID).unwrap();

  let backend_principal = Principal::from_text(TEST_USER_APP_BACKEND).unwrap();

  // register wallet
  let result = EgoStoreService::wallet_main_register(&wallet_principal, &user_principal);
  assert!(result.is_ok());

  // get app list before app install
  let apps = EgoStoreService::wallet_app_list(&wallet_principal);
  assert_eq!(0, apps.len());

  // install app
  let mut ego_tenant = MockTenant::new();

  ego_tenant
    .expect_app_main_install()
    .returning(move |_, _, _, _| Ok(backend_principal));

  ego_tenant
    .expect_canister_main_track()
    .returning(|_, _| ());

  let mut ego_canister = MockCanister::new();
  ego_canister
    .expect_ego_app_info_update()
    .returning(|_, _, _, _| ());

  let ego_store_app = EgoStoreService::app_main_get(&TEST_APP_ID.to_string()).unwrap();
  let result = EgoStoreService::wallet_app_install(
    ego_tenant,
    ego_canister,
    &wallet_principal,
    &ego_store_app,
  )
    .await;
  assert!(result.is_ok());

  // get app list after app install
  let apps = EgoStoreService::wallet_app_list(&wallet_principal);
  assert_eq!(1, apps.len());

  let app_installed = apps.get(0).unwrap();
  assert_eq!(backend_principal, app_installed.canister.canister_id);
}

#[tokio::test]
async fn wallet_app_upgrade_not_exists_wallet() {
  set_up();

  let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();

  // install with not exists wallet
  let ego_tenant = MockTenant::new();
  let ego_canister = MockCanister::new();
  let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();

  let result = EgoStoreService::wallet_app_upgrade(
    ego_tenant,
    ego_canister,
    &wallet_id,
    &backend_principal,
  )
    .await;
  assert!(result.is_err());
  assert_eq!(3002, result.unwrap_err().code);
}

#[tokio::test]
async fn wallet_app_upgrade_not_installed_app() {
  set_up();

  let wallet_principal = Principal::from_text(TEST_WALLET_ID).unwrap();
  let user_principal = Principal::from_text(TEST_USER_ID).unwrap();
  let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();

  // register wallet
  let result = EgoStoreService::wallet_main_register(&wallet_principal, &user_principal);
  assert!(result.is_ok());

  // upgrade not installed app
  let ego_tenant = MockTenant::new();
  let ego_canister = MockCanister::new();
  let result = EgoStoreService::wallet_app_upgrade(
    ego_tenant,
    ego_canister,
    &wallet_principal,
    &backend_principal,
  )
    .await;
  assert!(result.is_err());
  assert_eq!(3002, result.unwrap_err().code);
}

#[tokio::test]
async fn wallet_app_upgrade_success() {
  set_up();

  let current_version = Version::new(1, 0, 1);
  let latest_version = Version::new(1, 0, 2);

  let exist_wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
  let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();

  // get app list before upgrade
  let user_apps = EgoStoreService::wallet_app_list(&exist_wallet_id);
  let user_app = user_apps.get(0).unwrap();
  assert_eq!(current_version, user_app.app.current_version);

  // upgrade app version
  let mut ego_store_app = EgoStoreApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  ego_store_app.app.current_version = latest_version;
  ego_store_app.save();

  // upgrade installed app
  let mut ego_tenant = MockTenant::new();
  ego_tenant
    .expect_app_main_upgrade()
    .returning(|_, _, _| Ok(true));
  let mut ego_canister = MockCanister::new();
  ego_canister
    .expect_ego_app_info_update()
    .returning(|_, _, _, _| ());

  let result = EgoStoreService::wallet_app_upgrade(
    ego_tenant,
    ego_canister,
    &exist_wallet_id,
    &backend_principal,
  )
    .await;
  assert!(result.is_ok());

  // get app list after upgrade
  let user_apps = EgoStoreService::wallet_app_list(&exist_wallet_id);
  let user_app = user_apps.get(0).unwrap();
  assert_eq!(latest_version, user_app.app.current_version);
}

#[test]
#[should_panic]
fn wallet_app_remove_not_exists_wallet() {
  set_up();

  let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();
  let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();
  let ego_tenant = MockTenant::new();

  // remove not exists wallet
  let _ = EgoStoreService::wallet_app_remove(ego_tenant, &wallet_id, &backend_principal);
}

#[test]
#[should_panic]
fn wallet_app_remove_not_exists_app() {
  set_up();

  let exist_wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
  let fake_principal = Principal::from_text(FAKE_USER_APP_BACKEND.to_string()).unwrap();
  let ego_tenant = MockTenant::new();

  // install with not exists wallet
  let _ = EgoStoreService::wallet_app_remove(ego_tenant, &exist_wallet_id, &fake_principal);
}

#[test]
#[should_panic]
fn wallet_app_remove_not_installed_app() {
  set_up();

  let wallet_principal = Principal::from_text(TEST_WALLET_ID).unwrap();
  let user_principal = Principal::from_text(TEST_USER_ID).unwrap();
  let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();
  let ego_tenant = MockTenant::new();

  // register wallet
  let result = EgoStoreService::wallet_main_register(&wallet_principal, &user_principal);
  assert!(result.is_ok());

  // remove not exists wallet
  let _ =
    EgoStoreService::wallet_app_remove(ego_tenant, &wallet_principal, &backend_principal);
}

#[test]
fn wallet_app_remove_success() {
  set_up();

  let exist_wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
  let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();
  let mut ego_tenant = MockTenant::new();
  ego_tenant.expect_app_main_delete().returning(|_, _| ());

  // get app list before upgrade
  let apps = EgoStoreService::wallet_app_list(&exist_wallet_id);
  assert_eq!(1, apps.len());

  // upgrade installed app
  let result =
    EgoStoreService::wallet_app_remove(ego_tenant, &exist_wallet_id, &backend_principal);
  assert!(result.is_ok());

  // get app list after upgrade
  let apps = EgoStoreService::wallet_app_list(&exist_wallet_id);
  assert_eq!(0, apps.len());
}

#[test]
fn wallet_tenant_get() {
  set_up();

  let exist_wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();

  let wallet = Wallet::get(&exist_wallet_id).unwrap();
  assert_eq!(EXISTS_TENANT_ID, wallet.tenant_id.to_string())
}

#[test]
fn wallet_tenant_get_failed_wallet_not_exists() {
  set_up();
  let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();
  let exist_wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();

  let result = Wallet::get(&wallet_id);
  assert!(result.is_none());

  let result = Wallet::get(&exist_wallet_id);
  assert!(result.is_some());
  assert_eq!(EXISTS_TENANT_ID, result.unwrap().tenant_id.to_string())
}

#[tokio::test]
async fn wallet_canister_track_wallet_not_exists() {
  set_up();

  let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();
  let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();

  let mut ego_tenant = MockTenant::new();
  ego_tenant
    .expect_canister_main_track()
    .returning(|_, _| ());
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
    .returning(|_, _| ());
  //ego-store app not install
  let canister_track =
    EgoStoreService::wallet_canister_track(ego_tenant, &wallet_id, &fake_principal);
  assert!(canister_track.is_err());
  assert_eq!(3002, canister_track.as_ref().unwrap_err().code);
  assert_eq!(
    "ego-store: app not exists",
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
    .returning(|_, _| ());
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
    .returning(|_, _| ());
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
    .returning(|_, _| ());
  let wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
  let fake_principal = Principal::from_text(FAKE_USER_APP_BACKEND.to_string()).unwrap();

  let canister_untrack =
    EgoStoreService::wallet_canister_untrack(ego_tenant, &wallet_id, &fake_principal);
  assert!(canister_untrack.is_err());
  assert_eq!(3002, canister_untrack.as_ref().unwrap_err().code);
  assert_eq!(
    "ego-store: app not exists",
    canister_untrack.as_ref().unwrap_err().msg
  );
}

#[tokio::test]
async fn wallet_canister_untrack() {
  set_up();
  let mut ego_tenant = MockTenant::new();
  ego_tenant
    .expect_canister_main_untrack()
    .returning(|_, _| ());
  let wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
  let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();

  let canister_untrack =
    EgoStoreService::wallet_canister_untrack(ego_tenant, &wallet_id, &backend_principal);
  assert!(canister_untrack.is_ok());
}

#[test]
fn wallet_user_apps_track() {
  set_up();
  let wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
  let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();

  let mut ego_tenant = MockTenant::new();
  ego_tenant
    .expect_canister_main_track()
    .returning(move |_, &canister_id| {
      assert_eq!(backend_principal, canister_id);
      ()
    });
  // wallet canister track success
  let canister_track = EgoStoreService::wallet_user_apps_track(ego_tenant, &wallet_id);
  println!("{:?}", canister_track);
  assert!(canister_track.is_ok());
}

#[test]
fn admin_wallet_app_transfer() {
  set_up();

  let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();
  let new_wallet_principal = Principal::from_text(NEW_WALLET_ID.to_string()).unwrap();
  let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();

  assert_eq!(1, UserApp::by_wallet_id(&wallet_principal).len());
  assert_eq!(0, UserApp::by_wallet_id(&new_wallet_principal).len());

  let result = EgoStoreService::admin_wallet_app_transfer(
    &new_wallet_principal,
    &backend_principal,
  );
  assert!(result.is_ok());

  assert_eq!(0, UserApp::by_wallet_id(&wallet_principal).len());
  assert_eq!(1, UserApp::by_wallet_id(&new_wallet_principal).len());
}

#[test]
fn admin_wallet_app_transfer_wrong_canister() {
  set_up();

  let new_wallet_principal = Principal::from_text(NEW_WALLET_ID.to_string()).unwrap();
  let backend_principal = Principal::from_text(TEST_USER_APP_BACKEND.to_string()).unwrap();

  let result = EgoStoreService::admin_wallet_app_transfer(
    &new_wallet_principal,
    &backend_principal,
  );
  assert!(result.is_err());
  assert_eq!(
    format!("user app {} not exists", backend_principal),
    result.err().unwrap().msg
  );
}