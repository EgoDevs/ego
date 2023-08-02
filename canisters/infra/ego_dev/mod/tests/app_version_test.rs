use candid::Principal;

use ego_dev_mod::types::app_version::AppVersion;
use ego_types::app::CanisterType::{ASSET, BACKEND};
use ego_types::app::Version;
use ego_utils::util::time;

static FILE_ID1: &str = "amybd-zyaaa-aaaah-qc4hq-cai";
static EXISTS_APP_ID: &str = "app_exists";
static TEST_APP_ID: &str = "app_test";
static CANISTER_ID: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";

pub fn set_up() {
  // add app_version
  let ego_file_id = Principal::from_text(FILE_ID1.to_string()).unwrap();
  let version = Version::new(1, 0, 1);

  let mut app_version = AppVersion::new(&EXISTS_APP_ID.to_string(), &ego_file_id, &version);
  app_version.save();
}

#[test]
pub fn new() {
  set_up();

  assert_eq!(1, AppVersion::len());

  // add new version
  let ego_file_id = Principal::from_text(FILE_ID1.to_string()).unwrap();
  let version = Version::new(1, 0, 2);

  let mut app_version = AppVersion::new(&EXISTS_APP_ID.to_string(), &ego_file_id, &version);
  app_version.save();

  assert_eq!(2, AppVersion::len());
}

#[test]
pub fn len() {
  set_up();
  assert_eq!(1, AppVersion::len());
}

#[test]
pub fn by_last_update() {
  set_up();

  let now = time();

  assert_eq!(1, AppVersion::by_last_update(now).len());
}

#[test]
pub fn list() {
  set_up();

  let app_versions = AppVersion::list();

  assert_eq!(1, app_versions.len());
  let app_version = app_versions.get(0).unwrap();
  assert_eq!(EXISTS_APP_ID, app_version.app_id);
  assert_eq!(Version {
    major: 1,
    minor: 0,
    patch: 1,
  }, app_version.version);
}

#[test]
pub fn get() {
  set_up();

  let app_version = AppVersion::get(&1);
  assert!(app_version.is_some());

  let app_version = AppVersion::get(&100);
  assert!(app_version.is_none());
}


#[test]
pub fn frontend_update() {
  set_up();

  let canister_id = Principal::from_text(CANISTER_ID.to_string()).unwrap();

  let mut app_version = AppVersion::get(&1).unwrap();
  assert!(app_version.wasm.is_none());

  app_version.frontend_update(&canister_id);

  let app_version = AppVersion::get(&1).unwrap();
  assert!(app_version.wasm.is_some());
  assert_eq!(canister_id, app_version.wasm.clone().unwrap().canister_id);
  assert_eq!(ASSET, app_version.wasm.clone().unwrap().canister_type);
}

#[test]
pub fn backend_update() {
  set_up();

  let file_id = Principal::from_text(FILE_ID1.to_string()).unwrap();

  let mut app_version = AppVersion::get(&1).unwrap();
  assert!(app_version.wasm.is_none());

  app_version.backend_update();

  let app_version = AppVersion::get(&1).unwrap();
  assert!(app_version.wasm.is_some());
  assert_eq!(file_id, app_version.wasm.clone().unwrap().canister_id);
  assert_eq!(BACKEND, app_version.wasm.clone().unwrap().canister_type);
}

#[test]
pub fn by_app_id() {
  set_up();

  assert_eq!(1, AppVersion::by_app_id(&EXISTS_APP_ID.to_string()).len());

  assert_eq!(0, AppVersion::by_app_id(&TEST_APP_ID.to_string()).len());
}

#[test]
pub fn by_app_id_and_version() {
  set_up();

  let version = Version::new(1, 0, 1);
  let result = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version);
  assert!(result.is_some());

  let version = Version::new(1, 0, 2);
  let result = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version);
  assert!(result.is_none());
}
