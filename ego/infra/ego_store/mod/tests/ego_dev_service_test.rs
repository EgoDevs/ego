use ic_cdk::export::Principal;

use ego_store_mod::app::EgoStoreApp;
use ego_store_mod::service::EgoStoreService;
use ego_store_mod::state::EGO_STORE;
use ego_store_mod::types::QueryParam;
use ego_types::app::{Category, DeployMode, Wasm};
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
    let backend = Wasm::new(EXISTS_APP_ID.to_string(), version, BACKEND, file_canister);
    let app = EgoStoreApp::new(
      EXISTS_APP_ID.to_string(),
      EXISTS_APP_NAME.to_string(),
      Category::Vault,
      EXISTS_APP_LOGO.to_string(),
      EXISTS_APP_DESCRIPTION.to_string(),
      version,
      None,
      Some(backend),
      1.2f32,
      DeployMode::DEDICATED,
    );
    ego_store
      .borrow_mut()
      .apps
      .insert(EXISTS_APP_ID.to_string(), app);
  });
}

#[test]
fn app_main_release_new_app() {
  set_up();

  // before new app release
  let apps = EgoStoreService::app_main_list(QueryParam::ByCategory {
    category: Category::Vault,
  })
    .unwrap();
  assert_eq!(1, apps.len());

  // add app
  let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();
  let version = Version::new(1, 0, 0);

  let backend = Wasm::new(NEW_APP_ID.to_string(), version, BACKEND, file_canister);
  let app = EgoStoreApp::new(
    NEW_APP_ID.to_string(),
    NEW_APP_NAME.to_string(),
    Category::Vault,
    NEW_APP_LOGO.to_string(),
    NEW_APP_DESCRIPTION.to_string(),
    version,
    None,
    Some(backend),
    1.2f32,
    DeployMode::DEDICATED,
  );

  let result = EgoStoreService::app_main_release(app);
  assert!(result.is_ok());

  let apps = EgoStoreService::app_main_list(QueryParam::ByCategory {
    category: Category::Vault,
  })
    .unwrap();
  assert_eq!(2, apps.len());

  let app = EgoStoreService::app_main_get(&NEW_APP_ID.to_string()).unwrap();
  assert_eq!(NEW_APP_NAME.to_string(), app.name);
}

#[test]
fn app_main_release_new_app_version() {
  set_up();

  // before new app release
  let apps = EgoStoreService::app_main_list(QueryParam::ByCategory {
    category: Category::Vault,
  })
    .unwrap();
  assert_eq!(1, apps.len());

  // add app
  let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();
  let version = Version::new(1, 0, 1);

  let backend = Wasm::new(EXISTS_APP_ID.to_string(), version, BACKEND, file_canister);
  let app = EgoStoreApp::new(
    EXISTS_APP_ID.to_string(),
    EXISTS_APP_NAME.to_string(),
    Category::Vault,
    EXISTS_APP_LOGO.to_string(),
    EXISTS_APP_DESCRIPTION.to_string(),
    version,
    None,
    Some(backend),
    1.2f32,
    DeployMode::DEDICATED,
  );

  let result = EgoStoreService::app_main_release(app);
  assert!(result.is_ok());

  let apps = EgoStoreService::app_main_list(QueryParam::ByCategory {
    category: Category::Vault,
  })
    .unwrap();
  assert_eq!(1, apps.len());

  let app = EgoStoreService::app_main_get(&EXISTS_APP_ID.to_string()).unwrap();
  assert_eq!(EXISTS_APP_NAME.to_string(), app.name);
  assert_eq!(version, app.current_version);
}
