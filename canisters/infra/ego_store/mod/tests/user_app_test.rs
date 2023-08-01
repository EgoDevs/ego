use candid::Principal;

use ego_store_mod::types::ego_store_app::EgoStoreApp;
use ego_store_mod::types::user_app::UserApp;
use ego_types::app::{App, Canister, Category, Version, Wasm};
use ego_types::app::CanisterType::BACKEND;
use ego_utils::util::time;

static FILE_CANISTER_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";
static EXISTS_APP_ID: &str = "app_exists";
static APP_NAME: &str = "app1";
static APP_LOGO: &str = "logo";
static APP_DESCRIPTION: &str = "test is app description";

static WALLET_ID1: &str = "23vqh-waaaa-aaaai-qhcya-cai";
static USER_APP_CANISTER_ID1: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";

static WALLET_ID2: &str = "2avdy-paaaa-aaaaf-abcga-cai";
static USER_APP_CANISTER_ID2: &str = "225da-yaaaa-aaaah-qahrq-cai";


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

  let wallet_id1 = Principal::from_text(WALLET_ID1.to_string()).unwrap();
  let user_app_canister_id1 = Principal::from_text(USER_APP_CANISTER_ID1.to_string()).unwrap();
  let canister = Canister::new(user_app_canister_id1, BACKEND);
  let mut user_app = UserApp::new(&app, &canister, Some(wallet_id1));
  user_app.save();
}

#[test]
pub fn new() {
  set_up();

  assert_eq!(1, UserApp::len());
  let ego_store_app = EgoStoreApp::get(&EXISTS_APP_ID.to_string()).unwrap();

  // add test app
  let wallet_id2 = Principal::from_text(WALLET_ID2.to_string()).unwrap();
  let user_app_canister_id2 = Principal::from_text(USER_APP_CANISTER_ID2.to_string()).unwrap();
  let canister = Canister::new(user_app_canister_id2, BACKEND);
  let mut user_app = UserApp::new(&ego_store_app.app, &canister, Some(wallet_id2));
  user_app.save();

  assert_eq!(2, UserApp::len());
}

#[test]
pub fn len() {
  set_up();
  assert_eq!(1, UserApp::len());
}

#[test]
pub fn by_last_update() {
  set_up();

  let now = time();

  assert_eq!(1, UserApp::by_last_update(now).len());
}

#[test]
pub fn list() {
  set_up();

  let apps = UserApp::list();

  assert_eq!(1, apps.len());
  assert_eq!(EXISTS_APP_ID, apps.get(0).unwrap().app.app_id);
}

#[test]
pub fn get() {
  set_up();

  let user_app_canister_id1 = Principal::from_text(USER_APP_CANISTER_ID1.to_string()).unwrap();
  let app = UserApp::get(&user_app_canister_id1);
  assert!(app.is_some());

  let user_app_canister_id2 = Principal::from_text(USER_APP_CANISTER_ID2.to_string()).unwrap();
  let app = UserApp::get(&user_app_canister_id2);
  assert!(app.is_none());
}

#[test]
pub fn by_wallet_id() {
  set_up();

  let wallet1 = Principal::from_text(WALLET_ID1.to_string()).unwrap();
  let user_apps = UserApp::by_wallet_id(&wallet1);
  assert_eq!(1, user_apps.len());

  let wallet2 = Principal::from_text(WALLET_ID2.to_string()).unwrap();
  let user_apps = UserApp::by_wallet_id(&wallet2);
  assert_eq!(0, user_apps.len());
}

#[test]
pub fn by_wallet_id_and_id() {
  set_up();

  let wallet1 = Principal::from_text(WALLET_ID1.to_string()).unwrap();
  let user_app_canister_id1 = Principal::from_text(USER_APP_CANISTER_ID1.to_string()).unwrap();

  let user_app = UserApp::by_wallet_id_and_id(&wallet1, &user_app_canister_id1);
  assert!(user_app.is_some());

  let wallet2 = Principal::from_text(WALLET_ID2.to_string()).unwrap();
  let user_app = UserApp::by_wallet_id_and_id(&wallet2, &user_app_canister_id1);
  assert!(user_app.is_none());
}


#[test]
pub fn into() {
  set_up();

  let user_apps = UserApp::list();

  let user_app = user_apps.get(0).unwrap();

  let mut ego_store_app = EgoStoreApp::get(&user_app.app.app_id).unwrap();
  let latest_version = Version::new(10, 10, 10);
  ego_store_app.app.current_version = latest_version;
  ego_store_app.save();

  let u_a: ego_types::app::UserApp = user_app.clone().into();

  assert_eq!(user_app.canister.canister_id, u_a.canister.canister_id);
  assert_eq!(user_app.app.app_id, u_a.app.app_id);
  assert_eq!(ego_store_app.app.current_version, u_a.latest_version);
}
