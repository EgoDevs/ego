use ic_types::Principal;

use ego_dev_mod::app::{App, Category};
use ego_dev_mod::developer::Developer;
use ego_dev_mod::service::EgoDevService;
use ego_dev_mod::state::APP_STORE;

static FILE_CANISTER_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";

static EXIST_PRINCIPAL_ID: &str = "5oynr-yl472-mav57-c2oxo-g7woc-yytib-mp5bo-kzg3b-622pu-uatef-uqe";
static EXIST_APP_ID: &str = "app_1";
static EXIST_APP_NAME: &str = "app 1";

static TEST_PRINCIPAL_ID: &str = "d2qpe-l63sh-47jxj-2764e-pa6i7-qocm4-icuie-nt2lb-yiwwk-bmq7z-pqe";
static TEST_APP_ID: &str = "test_app";
static TEST_APP_NAME: &str = "test app";

pub fn set_up() {
  let caller = Principal::from_text(EXIST_PRINCIPAL_ID.to_string()).unwrap();
  let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();

  APP_STORE.with(|app_store| {
    // register developer
    app_store.borrow_mut().developers.insert(caller, Developer::new(caller, "dev 1".to_string()));

    // create app
    app_store.borrow_mut().apps.insert(EXIST_APP_ID.to_string(), App::new(caller, EXIST_APP_ID.to_string(), EXIST_APP_NAME.to_string(), Category::Vault, file_canister, 0f32));
  });
}


#[test]
fn developer_main_register() {
  set_up();

  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let developer = EgoDevService::developer_main_register(caller, "user_1".to_string()).unwrap();

  assert_eq!(caller, developer.user_id);
  assert_eq!("user_1", developer.name);

  // register with the same principal id will not change the previous user name
  let developer = EgoDevService::developer_main_register(caller, "user_2".to_string()).unwrap();
  assert_eq!(caller, developer.user_id);
  assert_eq!("user_1", developer.name);
}

#[test]
fn developer_main_get_fail() {
  set_up();

  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let result = EgoDevService::developer_main_get(caller);

  assert!(result.is_err());

  let error = result.unwrap_err();
  assert_eq!(1011, error.code);
}

#[test]
fn developer_main_get_success() {
  set_up();

  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();

  let result = EgoDevService::developer_main_register(caller, "user_1".to_string());
  assert!(result.is_ok());

  let result = EgoDevService::developer_main_get(caller);
  let developer = result.unwrap();
  assert_eq!(caller, developer.user_id);
}

#[test]
fn developer_app_new_fail_with_exists_app_id() {
  set_up();

  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let result = EgoDevService::developer_app_new(caller, EXIST_APP_ID.to_string(), EXIST_APP_NAME.to_string(), Category::Vault, 0f32);

  assert_eq!(1001, result.unwrap_err().code);
}

#[test]
fn developer_app_new_fail_with_none_register_developer() {
  set_up();

  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let result = EgoDevService::developer_app_new(caller, TEST_APP_ID.to_string(), TEST_APP_NAME.to_string(), Category::Vault, 0f32);
  println!("{:?}", result);

  assert_eq!(1011, result.unwrap_err().code);
}

// fn developer_app_new_success(){
//   // case 1: not registered developer, can not new app

//
//   let caller = Principal::from_text(PRINCIPAL_ID_A.to_string()).unwrap();
//   let result = EgoDevService::developer_app_new(caller, APP_1_ID.to_string(), APP_1_NAME.to_string(), Category::Vault, 0f32);
//
//   assert!(result.is_err());
//   assert_eq!(1012, result.unwrap_err().code);
//
//   let app = result.unwrap();
//   assert_eq!(APP_1_ID, app.app_id);
// }