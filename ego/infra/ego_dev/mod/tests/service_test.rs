use ic_types::Principal;

use ego_dev_mod::app::{App, AppVersion, AppVersionStatus};
use ego_dev_mod::developer::Developer;
use ego_dev_mod::file::File;
use ego_dev_mod::service::EgoDevService;
use ego_dev_mod::state::EGO_DEV;
use ego_utils::types::{Category, Version};
use ego_dev_mod::c2c::ego_file::TEgoFile;
use ego_utils::types::EgoError;
use async_trait::async_trait;
use mockall::mock;

mock! {
  Dump {}

  #[async_trait]
  impl TEgoFile for Dump {
    async fn file_main_write(&self, canister_id: Principal, fid: String, hash: String, data: Vec<u8>) -> Result<bool, EgoError>;
  }
}

static FILE_CANISTER_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";

static AUDITER_PRINCIPAL_ID: &str = "22ayq-aiaaa-aaaai-qgmma-cai";

static DEVELOPER_PRINCIPAL_ID: &str = "5oynr-yl472-mav57-c2oxo-g7woc-yytib-mp5bo-kzg3b-622pu-uatef-uqe";
static EXIST_APP_ID: &str = "app_1";
static EXIST_APP_NAME: &str = "app 1";

static RELEASED_APP_ID: &str = "app_2";
static RELEASED_APP_NAME: &str = "app 2";

static TEST_PRINCIPAL_ID: &str = "d2qpe-l63sh-47jxj-2764e-pa6i7-qocm4-icuie-nt2lb-yiwwk-bmq7z-pqe";
static TEST_APP_ID: &str = "test_app";
static TEST_APP_NAME: &str = "test app";


pub fn set_up() {
  let developer_principal = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let auditer_principal = Principal::from_text(AUDITER_PRINCIPAL_ID.to_string()).unwrap();
  let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();

  let version = Version::new(1, 0, 1);

  EGO_DEV.with(|ego_dev| {
    // add file canister
    ego_dev.borrow_mut().ego_files.push( File::new(file_canister));

    // registered developer
    let developer = Developer::new(developer_principal, "dev 1".to_string());
    ego_dev.borrow_mut().developers.insert(developer_principal, developer);

    // registered auditer
    let mut auditer = Developer::new(auditer_principal, "audit 1".to_string());
    auditer.is_app_auditor = true;
    ego_dev.borrow_mut().developers.insert(auditer_principal, auditer);

    // submitted app
    let mut app = App::new(developer_principal, EXIST_APP_ID.to_string(), EXIST_APP_NAME.to_string(), Category::Vault,  0f32);
    let mut app_version = AppVersion::new(EXIST_APP_ID.to_string(), file_canister, version);
    app_version.status = AppVersionStatus::SUBMITTED;
    app.versions.push(app_version);

    app.audit_version = Some(version);
    ego_dev.borrow_mut().apps.insert(EXIST_APP_ID.to_string(), app);

    // relesed app
    let mut app = App::new(developer_principal, RELEASED_APP_ID.to_string(), RELEASED_APP_NAME.to_string(), Category::Vault,  0f32);
    let mut app_version = AppVersion::new(RELEASED_APP_ID.to_string(), file_canister, version);
    app_version.status = AppVersionStatus::RELEASED;
    app.versions.push(app_version);

    app.release_version = Some(version);
    ego_dev.borrow_mut().apps.insert(RELEASED_APP_ID.to_string(), app);
  });
}

#[test]
fn admin_file_add() {
  let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();
  let resp = EgoDevService::admin_ego_file_add(file_canister);
  assert!(resp.is_ok());

  EGO_DEV.with(|ego_dev| {
    assert_eq!(1, ego_dev.borrow().ego_files.len());
  })
}

#[test]
fn developer_main_register_success() {
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
fn developer_main_get_fail_with_not_exists_id() {
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
  assert!(result.is_err());

  assert_eq!(1011, result.unwrap_err().code);
}

#[test]
fn developer_app_new_success(){
  set_up();

  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let result = EgoDevService::developer_main_register(caller, "user_1".to_string());
  assert!(result.is_ok());

  let result = EgoDevService::developer_app_new(caller, TEST_APP_ID.to_string(), TEST_APP_NAME.to_string(), Category::Vault, 0f32);

  assert!(result.is_ok());

  let app = result.unwrap();
  assert_eq!(TEST_APP_ID, app.app_id);
}

#[test]
fn app_version_new_success(){
  set_up();

  let version = Version::new(1, 0, 1);

  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let result = EgoDevService::developer_main_register(caller, "user_1".to_string());
  assert!(result.is_ok());

  let result = EgoDevService::developer_app_new(caller, TEST_APP_ID.to_string(), TEST_APP_NAME.to_string(), Category::Vault, 0f32);
  assert!(result.is_ok());

  let result = EgoDevService::app_version_new(caller, TEST_APP_ID.to_string(), version);
  assert!(result.is_ok());

  let app_version = result.unwrap();
  assert_eq!(version, app_version.version);
  assert_eq!(AppVersionStatus::NEW, app_version.status);
}

#[test]
fn app_version_submit_process(){
  set_up();

  let version = Version::new(1, 0, 1);

  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let result = EgoDevService::developer_main_register(caller, "user_1".to_string());
  assert!(result.is_ok());

  let result = EgoDevService::developer_app_new(caller, TEST_APP_ID.to_string(), TEST_APP_NAME.to_string(), Category::Vault, 0f32);
  assert!(result.is_ok());

  let result = EgoDevService::app_version_new(caller, TEST_APP_ID.to_string(), version);
  assert!(result.is_ok());

  // test submit
  let result = EgoDevService::app_version_submit(caller, TEST_APP_ID.to_string(), version);
  assert!(result.is_ok());

  let app_version = result.unwrap();
  assert_eq!(version, app_version.version);
  assert_eq!(AppVersionStatus::SUBMITTED, app_version.status);

  let result = EgoDevService::developer_app_get(caller, TEST_APP_ID.to_string());
  assert!(result.is_ok());

  let app = result.unwrap();
  assert_eq!(version, app.audit_version.unwrap());

  // test revoke
  let result = EgoDevService::app_version_revoke(caller, TEST_APP_ID.to_string(), version);
  assert!(result.is_ok());

  let app_version = result.unwrap();
  assert_eq!(version, app_version.version);
  assert_eq!(AppVersionStatus::REVOKED, app_version.status);

  let result = EgoDevService::developer_app_get(caller, TEST_APP_ID.to_string());
  assert!(result.is_ok());

  let app = result.unwrap();
  assert!(app.audit_version.is_none());
}

#[test]
fn app_version_approve(){
  set_up();

  let developer = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let version = Version::new(1, 0, 1);

  // check before audit
  let result = EgoDevService::developer_app_get(developer, EXIST_APP_ID.to_string());

  assert!(result.is_ok());
  let app = result.unwrap();
  let app_version = app.version_get(version).unwrap();
  assert_eq!(version, app_version.version);
  assert_eq!(AppVersionStatus::SUBMITTED, app_version.status);
  assert_eq!(version, app.audit_version.unwrap());

  let result = EgoDevService::app_version_approve(EXIST_APP_ID.to_string(), version);
  assert!(result.is_ok());

  // check after audit
  let result = EgoDevService::developer_app_get(developer, EXIST_APP_ID.to_string());
  assert!(result.is_ok());
  let app = result.unwrap();
  let app_version = app.version_get(version).unwrap();
  assert_eq!(version, app_version.version);
  assert_eq!(AppVersionStatus::APPROVED, app_version.status);
  assert!(app.audit_version.is_none());
}

#[test]
fn app_version_wait_for_audit(){
  set_up();
  let result = EgoDevService::app_version_wait_for_audit();
  assert_eq!(1, result.len());
}

#[test]
fn app_version_reject(){
  set_up();

  let developer = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let version = Version::new(1, 0, 1);

  // check before audit
  let result = EgoDevService::developer_app_get(developer, EXIST_APP_ID.to_string());

  assert!(result.is_ok());
  let app = result.unwrap();
  let app_version = app.version_get(version).unwrap();
  assert_eq!(version, app_version.version);
  assert_eq!(AppVersionStatus::SUBMITTED, app_version.status);
  assert_eq!(version, app.audit_version.unwrap());

  let result = EgoDevService::app_version_reject(EXIST_APP_ID.to_string(), version);
  assert!(result.is_ok());

  // check after audit
  let result = EgoDevService::developer_app_get(developer, EXIST_APP_ID.to_string());
  assert!(result.is_ok());
  let app = result.unwrap();
  let app_version = app.version_get(version).unwrap();
  assert_eq!(version, app_version.version);
  assert_eq!(AppVersionStatus::REJECTED, app_version.status);
  assert!(app.audit_version.is_none());
}

#[test]
fn app_version_release(){
  set_up();

  let developer = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let version = Version::new(1, 0, 1);

  // approve version
  let result = EgoDevService::app_version_approve(EXIST_APP_ID.to_string(), version);
  assert!(result.is_ok());

  // check after audit
  let result = EgoDevService::app_version_release(developer, EXIST_APP_ID.to_string(), version);
  assert!(result.is_ok());

  // check after audit
  let result = EgoDevService::developer_app_get(developer, EXIST_APP_ID.to_string());
  assert!(result.is_ok());
  let app = result.unwrap();
  let app_version = app.version_get(version).unwrap();
  assert_eq!(version, app_version.version);
  assert_eq!(AppVersionStatus::RELEASED, app_version.status);
}

fn get_md5(data: &Vec<u8>) -> String {
  let digest = md5::compute(data);
  return format!("{:?}", digest);
}


#[tokio::test]
async fn app_version_upload_wasm() {
  set_up();

  let developer_principal = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let version = Version::new(1, 0, 1);
  let data = vec![1,0,1,0,1,0,0,0,1];

  let mut service = MockDump::new();

  service.expect_file_main_write().returning(|_, _, _, _| Ok(true));
  match EgoDevService::app_version_upload_wasm(service, developer_principal, EXIST_APP_ID.to_string(), version, data.clone(), get_md5(&data)).await {
    Ok(ret) => {
      assert!(ret)
    },
    Err(_) => {
      panic!("should not go here")
    },
  }
}

#[tokio::test]
async fn app_version_upload_wasm_not_exist_app() {
  let developer_principal = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let version = Version::new(1, 0, 1);
  let data = vec![1,0,1,0,1,0,0,0,1];

  let mut service = MockDump::new();

  service.expect_file_main_write().returning(|_, _, _, _| Ok(true));
  match EgoDevService::app_version_upload_wasm(service, developer_principal, EXIST_APP_ID.to_string(), version, data.clone(), get_md5(&data)).await {
    Ok(_) => {
      panic!("should not go here")
    },
    Err(e) => {
      assert_eq!(1002, e.code)
    },
  }
}

#[tokio::test]
async fn app_version_upload_wasm_released_app() {
  set_up();

  let developer_principal = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let version = Version::new(1, 0, 1);
  let data = vec![1,0,1,0,1,0,0,0,1];

  let mut service = MockDump::new();

  service.expect_file_main_write().returning(|_, _, _, _| Ok(true));
  match EgoDevService::app_version_upload_wasm(service, developer_principal, RELEASED_APP_ID.to_string(), version, data.clone(), get_md5(&data)).await {
    Ok(_) => {
      panic!("should not go here")
    },
    Err(e) => {
      assert_eq!(1013, e.code)
    },
  }
}