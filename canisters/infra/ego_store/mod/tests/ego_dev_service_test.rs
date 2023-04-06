use ic_cdk::export::Principal;

use ego_store_mod::app::EgoStoreApp;
use ego_store_mod::service::EgoStoreService;
use ego_store_mod::state::EGO_STORE;
use ego_types::app::{App, Category, Wasm};
use ego_types::app::CanisterType::BACKEND;
use ego_types::app::Version;

static FILE_CANISTER_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";

static EXISTS_APP_ID: &str = "app_test";
static EXISTS_APP_NAME: &str = "test app";
static EXISTS_APP_LOGO: &str = "logo";
static EXISTS_APP_DESCRIPTION: &str = "test is app description";

static NEW_APP_ID: &str = "new_app";
static NEW_APP_NAME: &str = "new_app";
static NEW_APP_LOGO: &str = "new_app logo";
static NEW_APP_DESCRIPTION: &str = "test is new app description";

pub fn set_up() {
  let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();

  let version = Version::new(1, 0, 0);

  EGO_STORE.with(|ego_store| {
    // add app
    let wasm = Wasm::new(EXISTS_APP_ID.to_string(), version, BACKEND, file_canister);
    let app = App {
      app_id: EXISTS_APP_ID.to_string(),
      name: EXISTS_APP_NAME.to_string(),
      category: Category::Vault,
      logo: EXISTS_APP_LOGO.to_string(),
      description: EXISTS_APP_DESCRIPTION.to_string(),
      current_version: version,
      price: 0.0,
      app_hash: "".to_string(),
    };

    let ego_store_app = EgoStoreApp::new(
      app,
      wasm,
    );

    ego_store
      .borrow_mut()
      .apps
      .insert(EXISTS_APP_ID.to_string(), ego_store_app);
  });
}

#[test]
fn app_main_release_new_app() {
  set_up();

  // before new app release
  let apps = EgoStoreService::app_main_list()
    .unwrap();
  assert_eq!(1, apps.len());

  // add app
  let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();
  let version = Version::new(1, 0, 0);

  let wasm = Wasm::new(NEW_APP_ID.to_string(), version, BACKEND, file_canister);

  let app = App {
    app_id: NEW_APP_ID.to_string(),
    name: NEW_APP_NAME.to_string(),
    category: Category::Vault,
    logo: NEW_APP_LOGO.to_string(),
    description: NEW_APP_DESCRIPTION.to_string(),
    current_version: version,
    price: 0.0,
    app_hash: "".to_string(),
  };

  let ego_store_app = EgoStoreApp::new(
    app,
    wasm,
  );

  let result = EgoStoreService::app_main_release(ego_store_app);
  assert!(result.is_ok());

  let apps = EgoStoreService::app_main_list()
    .unwrap();
  assert_eq!(2, apps.len());

  let ego_store_app = EgoStoreService::app_main_get(&NEW_APP_ID.to_string()).unwrap();
  assert_eq!(NEW_APP_NAME.to_string(), ego_store_app.app.name);
}

#[test]
fn app_main_release_new_app_version() {
  set_up();

  // before new app release
  let apps = EgoStoreService::app_main_list()
    .unwrap();
  assert_eq!(1, apps.len());

  // add app
  let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();
  let version = Version::new(1, 0, 1);

  let wasm = Wasm::new(EXISTS_APP_ID.to_string(), version, BACKEND, file_canister);
  let app = App {
    app_id: EXISTS_APP_ID.to_string(),
    name: EXISTS_APP_NAME.to_string(),
    category: Category::Vault,
    logo: EXISTS_APP_LOGO.to_string(),
    description: EXISTS_APP_DESCRIPTION.to_string(),
    current_version: version,
    price: 0.0,
    app_hash: "".to_string(),
  };

  let app = EgoStoreApp::new(app, wasm);

  let result = EgoStoreService::app_main_release(app);
  assert!(result.is_ok());

  let apps = EgoStoreService::app_main_list()
    .unwrap();
  assert_eq!(1, apps.len());

  let app = EgoStoreService::app_main_get(&EXISTS_APP_ID.to_string()).unwrap();
  assert_eq!(EXISTS_APP_NAME.to_string(), app.app.name);
  assert_eq!(version, app.app.current_version);
}
