use candid::Principal;

use ego_store_mod::types::ego_store_app::EgoStoreApp;
use ego_types::app::{App, Category, Version, Wasm};
use ego_types::app::CanisterType::BACKEND;
use ego_utils::util::time;

static FILE_CANISTER_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";
static EXISTS_APP_ID: &str = "app_exists";
static APP_NAME: &str = "app1";
static APP_LOGO: &str = "logo";
static APP_DESCRIPTION: &str = "test is app description";

static TEST_APP_ID: &str = "app_test";


pub fn set_up() {
  // add exists app
  let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();
  let version = Version::new(1, 0, 1);

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
}

#[test]
pub fn new() {
  set_up();

  assert_eq!(1, EgoStoreApp::len());

  // add test app
  let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();
  let version = Version::new(1, 0, 1);

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

  assert_eq!(2, EgoStoreApp::len());
}

#[test]
pub fn len() {
  set_up();
  assert_eq!(1, EgoStoreApp::len());
}

#[test]
pub fn by_last_update() {
  set_up();

  let now = time();

  assert_eq!(1, EgoStoreApp::by_last_update(0, 100, now).len());
}

#[test]
pub fn list() {
  set_up();

  let apps = EgoStoreApp::list(0, 100);

  assert_eq!(1, apps.len());
  assert_eq!(EXISTS_APP_ID, apps.get(0).unwrap().app.app_id);
}

#[test]
pub fn get() {
  set_up();

  let app = EgoStoreApp::get(&EXISTS_APP_ID.to_string());
  assert!(app.is_some());

  let app = EgoStoreApp::get(&TEST_APP_ID.to_string());
  assert!(app.is_none());
}

