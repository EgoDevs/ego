use async_trait::async_trait;
use candid::Principal;
use mockall::mock;

use ego_dev_mod::c2c::ego_file::TEgoFile;
use ego_dev_mod::c2c::ego_store::TEgoStore;
use ego_dev_mod::service::EgoDevService;
use ego_dev_mod::types::app_version::{AppVersion, AppVersionStatus};
use ego_dev_mod::types::developer::Developer;
use ego_dev_mod::types::ego_dev_app::EgoDevApp;
use ego_dev_mod::types::EgoDevErr;
use ego_dev_mod::types::file::File;
use ego_types::app::{App, Wasm};
use ego_types::app::{CanisterType, Category};
use ego_types::app::EgoError;
use ego_types::app::Version;
use ego_utils::util::get_md5;

mock! {
  File {}

  #[async_trait]
  impl TEgoFile for File {
    async fn file_main_write(&self, canister_id: Principal, fid: String, hash: String, data: Vec<u8>) -> Result<bool, EgoError>;
  }
}

mock! {
  Store {}

  #[async_trait]
  impl TEgoStore for Store {
    fn app_main_release(
        &self,
        app: App,
        wasm: Wasm
    );
  }
}

static FILE_CANISTER_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";

static AUDITER_PRINCIPAL_ID: &str = "22ayq-aiaaa-aaaai-qgmma-cai";

static DEVELOPER_PRINCIPAL_ID: &str =
  "5oynr-yl472-mav57-c2oxo-g7woc-yytib-mp5bo-kzg3b-622pu-uatef-uqe";
static DEVELOPER_NAME: &str = "dev_1";
static EXIST_APP_ID: &str = "app_1";
static EXIST_APP_NAME: &str = "app 1";
static APP_LOGO: &str = "logo";
static APP_DESCRIPTION: &str = "description";

static RELEASED_APP_ID: &str = "app_2";
static RELEASED_APP_NAME: &str = "app 2";

static TEST_PRINCIPAL_ID: &str = "d2qpe-l63sh-47jxj-2764e-pa6i7-qocm4-icuie-nt2lb-yiwwk-bmq7z-pqe";
static TEST_CANISTER_ID: &str = "22w4s-syaaa-aaaai-acjkq-cai";
static TEST_APP_ID: &str = "test_app";
static TEST_APP_NAME: &str = "test app";

pub fn set_up() {
  let developer_principal = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let auditer_principal = Principal::from_text(AUDITER_PRINCIPAL_ID.to_string()).unwrap();
  let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();

  let version = Version::new(1, 0, 1);

  // add file canister
  let ego_file = File::new(&file_canister);
  ego_file.save();

  // registered developer
  let mut developer = Developer::new(&developer_principal, DEVELOPER_NAME);
  developer.save();

  // registered auditer
  let mut auditer = Developer::new(&auditer_principal, "audit 1");
  auditer.is_app_auditor = true;
  auditer.save();

  // submitted app
  let wasm = Wasm {
    app_id: "".to_string(),
    version: Default::default(),
    canister_type: CanisterType::BACKEND,
    canister_id: file_canister,
  };

  let mut app_version = AppVersion::new(&EXIST_APP_ID.to_string(), &file_canister, &version);
  app_version.status = AppVersionStatus::SUBMITTED;
  app_version.wasm = Some(wasm);
  app_version.save();

  let mut app = EgoDevApp::new(
    &developer_principal,
    &EXIST_APP_ID.to_string(),
    EXIST_APP_NAME,
    APP_LOGO,
    APP_DESCRIPTION,
    &Category::Vault,
    0f32,
  );
  app.audit_version = Some(version);
  app.save();

  // released app
  let mut app_version = AppVersion::new(&RELEASED_APP_ID.to_string(), &file_canister, &version);
  app_version.status = AppVersionStatus::RELEASED;
  app_version.save();

  let mut ego_dev_app = EgoDevApp::new(
    &developer_principal,
    &RELEASED_APP_ID.to_string(),
    RELEASED_APP_NAME,
    APP_LOGO,
    APP_DESCRIPTION,
    &Category::Vault,
    0f32,
  );

  ego_dev_app.app.current_version = version;
  ego_dev_app.save();
}

#[test]
fn admin_file_add() {
  let len_before = File::list(0, 100).len();

  let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();
  EgoDevService::admin_ego_file_add(&file_canister);

  let len_after = File::list(0, 100).len();

  assert!(len_after == len_before + 1);
}

#[test]
fn developer_main_register_success() {
  set_up();

  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let developer = EgoDevService::developer_main_register(&caller, "user_1").unwrap();

  assert_eq!(caller, developer.developer_id);
  assert_eq!("user_1", developer.name);

  // register with the same principal id will not change the previous user name
  let developer = EgoDevService::developer_main_register(&caller, "user_2").unwrap();
  assert_eq!(caller, developer.developer_id);
  assert_eq!("user_1", developer.name);
}

#[test]
fn developer_main_register_fail_name_existed() {
  set_up();

  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();

  let result = EgoDevService::developer_main_register(&caller, DEVELOPER_NAME);
  assert!(result.is_err());
  assert_eq!(1010, result.unwrap_err().code);
}

#[test]
#[should_panic]
fn developer_main_get_fail_with_not_exists_id() {
  set_up();

  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  Developer::get(&caller).unwrap();
}

#[test]
fn developer_main_get_success() {
  set_up();

  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();

  let result = EgoDevService::developer_main_register(&caller, "user_1");
  assert!(result.is_ok());

  let developer = Developer::get(&caller).unwrap();
  assert_eq!(caller, developer.developer_id);
}

#[test]
fn developer_app_new_fail_with_exists_app_id() {
  set_up();

  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let result = EgoDevService::developer_app_new(
    &caller,
    &EXIST_APP_ID.to_string(),
    EXIST_APP_NAME,
    APP_LOGO,
    APP_DESCRIPTION,
    &Category::Vault,
    0f32,
  );

  assert_eq!(1001, result.unwrap_err().code);
}

#[test]
fn developer_app_transfer() {
  set_up();

  let developer = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();

  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let result = EgoDevService::developer_app_new(
    &caller,
    &EXIST_APP_ID.to_string(),
    EXIST_APP_NAME,
    APP_LOGO,
    APP_DESCRIPTION,
    &Category::Vault,
    0f32,
  );

  assert_eq!(1001, result.unwrap_err().code);

  let ego_dev_app = EgoDevApp::get(&EXIST_APP_ID.to_string()).unwrap();
  assert_eq!(developer, ego_dev_app.developer_id);

  let result = EgoDevService::developer_app_transfer(&caller, &EXIST_APP_ID.to_string());
  assert!(result.is_ok());

  let ego_dev_app = EgoDevApp::get(&EXIST_APP_ID.to_string()).unwrap();
  assert_eq!(caller, ego_dev_app.developer_id);

  let result = EgoDevService::developer_app_new(
    &caller,
    &EXIST_APP_ID.to_string(),
    EXIST_APP_NAME,
    APP_LOGO,
    APP_DESCRIPTION,
    &Category::Vault,
    0f32,
  );
  assert!(result.is_ok());
}

#[test]
#[should_panic]
fn developer_app_new_fail_with_none_register_developer() {
  set_up();

  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let _ = EgoDevService::developer_app_new(
    &caller,
    &TEST_APP_ID.to_string(),
    TEST_APP_NAME,
    APP_LOGO,
    APP_DESCRIPTION,
    &Category::Vault,
    0f32,
  );
}

#[test]
fn developer_app_new_success() {
  set_up();

  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let result = EgoDevService::developer_main_register(&caller, "user_1");
  assert!(result.is_ok());

  let list_result = EgoDevApp::by_developer_id(&caller);
  assert_eq!(0, list_result.len());

  // app new success with developer
  let result = EgoDevService::developer_app_new(
    &caller,
    &TEST_APP_ID.to_string(),
    TEST_APP_NAME,
    APP_LOGO,
    APP_DESCRIPTION,
    &Category::Vault,
    0f32,
  );

  assert!(result.is_ok());

  let ego_dev_app = result.unwrap();
  assert_eq!(TEST_APP_ID, ego_dev_app.app.app_id);

  // test_app has been added successfully by developer
  let list_result = EgoDevApp::by_developer_id(&caller);
  let app = &list_result[0];
  assert_eq!(TEST_APP_ID, app.app.app_id);
}

#[test]
fn app_version_new_success() {
  set_up();

  let version = Version::new(1, 0, 1);

  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let result = EgoDevService::developer_main_register(&caller, "user_1");
  assert!(result.is_ok());

  let result = EgoDevService::developer_app_new(
    &caller,
    &TEST_APP_ID.to_string(),
    TEST_APP_NAME,
    APP_LOGO,
    APP_DESCRIPTION,
    &Category::Vault,
    0f32,
  );
  assert!(result.is_ok());

  let result = EgoDevService::app_version_new(&caller, &TEST_APP_ID.to_string(), &version);
  assert!(result.is_ok());

  let app_version = result.unwrap();
  assert_eq!(version, app_version.version);
  assert_eq!(AppVersionStatus::NEW, app_version.status);
}

#[test]
fn app_version_submit_process() {
  set_up();

  let version = Version::new(1, 0, 1);

  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let result = EgoDevService::developer_main_register(&caller, "user_1");
  assert!(result.is_ok());

  let result = EgoDevService::developer_app_new(
    &caller,
    &TEST_APP_ID.to_string(),
    TEST_APP_NAME,
    APP_LOGO,
    APP_DESCRIPTION,
    &Category::Vault,
    0f32,
  );
  assert!(result.is_ok());

  let result = EgoDevService::app_version_new(&caller, &TEST_APP_ID.to_string(), &version);
  assert!(result.is_ok());

  // test submit
  let result = EgoDevService::app_version_submit(&caller, &TEST_APP_ID.to_string(), &version);
  assert!(result.is_ok());

  let app_version = result.unwrap();
  assert_eq!(version, app_version.version);
  assert_eq!(AppVersionStatus::SUBMITTED, app_version.status);

  let result = EgoDevApp::by_developer_id_and_id(&caller, &TEST_APP_ID.to_string());
  assert!(result.is_some());

  let app = result.unwrap();
  assert_eq!(version, app.audit_version.unwrap());

  // test revoke
  let result = EgoDevService::app_version_revoke(&caller, &TEST_APP_ID.to_string(), &version);
  assert!(result.is_ok());

  let app_version = result.unwrap();
  assert_eq!(version, app_version.version);
  assert_eq!(AppVersionStatus::REVOKED, app_version.status);
}

#[test]
fn app_version_approve() {
  set_up();

  let developer = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let version = Version::new(1, 0, 1);

  // check before audit
  let result = EgoDevApp::by_developer_id_and_id(&developer, &EXIST_APP_ID.to_string());

  assert!(result.is_some());
  let app = result.unwrap();
  let app_version = app.version_get(&version).unwrap();
  assert_eq!(version, app_version.version);
  assert_eq!(AppVersionStatus::SUBMITTED, app_version.status);
  assert_eq!(version, app.audit_version.unwrap());

  let result = EgoDevService::app_version_approve(&EXIST_APP_ID.to_string());
  assert!(result.is_ok());

  // check after audit
  let result = EgoDevApp::by_developer_id_and_id(&developer, &EXIST_APP_ID.to_string());
  assert!(result.is_some());
  let app = result.unwrap();
  let app_version = app.version_get(&version).unwrap();
  assert_eq!(version, app_version.version);
  assert_eq!(AppVersionStatus::APPROVED, app_version.status);
  assert!(app.audit_version.is_none());
}

#[test]
fn app_version_wait_for_audit() {
  set_up();
  let result = EgoDevApp::version_wait_for_audit();
  assert_eq!(1, result.len());
}

#[test]
fn app_version_reject() {
  set_up();

  let developer = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let version = Version::new(1, 0, 1);

  // check before audit
  let result = EgoDevApp::by_developer_id_and_id(&developer, &EXIST_APP_ID.to_string());

  assert!(result.is_some());
  let app = result.unwrap();
  let app_version = app.version_get(&version).unwrap();
  assert_eq!(version, app_version.version);
  assert_eq!(AppVersionStatus::SUBMITTED, app_version.status);
  assert_eq!(version, app.audit_version.unwrap());

  let result = EgoDevService::app_version_reject(&EXIST_APP_ID.to_string());
  assert!(result.is_ok());

  // check after audit
  let result = EgoDevApp::by_developer_id_and_id(&developer, &EXIST_APP_ID.to_string());
  assert!(result.is_some());
  let app = result.unwrap();
  let app_version = app.version_get(&version).unwrap();
  assert_eq!(version, app_version.version);
  assert_eq!(AppVersionStatus::REJECTED, app_version.status);
  assert!(app.audit_version.is_none());
}

#[tokio::test]
async fn app_version_release() {
  set_up();

  let developer = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let version = Version::new(1, 0, 1);

  let mut ego_store = MockStore::new();
  ego_store.expect_app_main_release().returning(|app, _wasm| {
    assert_eq!("e2a24d7f694107d056b967aace21349b", app.app_hash);
    ()
  });

  // approve version
  let result = EgoDevService::app_version_approve(&EXIST_APP_ID.to_string());
  assert!(result.is_ok());

  // check after audit
  let result =
    EgoDevService::app_version_release(&developer, &EXIST_APP_ID.to_string(), &version, ego_store);
  assert!(result.is_ok());

  let result = EgoDevApp::by_developer_id_and_id(&developer, &EXIST_APP_ID.to_string());
  assert!(result.is_some());

  let app = result.unwrap();
  let app_version = app.version_get(&version).unwrap();
  assert_eq!(version, app_version.version);
  assert_eq!(AppVersionStatus::RELEASED, app_version.status);
}

#[tokio::test]
async fn app_version_upload_wasm() {
  set_up();

  let developer_principal = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let version = Version::new(1, 0, 1);
  let data = vec![1, 0, 1, 0, 1, 0, 0, 0, 1];

  let mut service = MockFile::new();

  service
    .expect_file_main_write()
    .returning(|_, _, _, _| Ok(true));
  match EgoDevService::app_version_upload_wasm(
    service,
    &developer_principal,
    &EXIST_APP_ID.to_string(),
    &version,
    data.clone(),
    get_md5(&data),
  )
    .await
  {
    Ok(ret) => {
      assert!(ret)
    }
    Err(_) => {
      panic!("should not go here")
    }
  }
}

#[tokio::test]
async fn app_version_upload_wasm_not_exist_app() {
  let developer_principal = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let version = Version::new(1, 0, 1);
  let data = vec![1, 0, 1, 0, 1, 0, 0, 0, 1];

  let mut service = MockFile::new();

  service
    .expect_file_main_write()
    .returning(|_, _, _, _| Ok(true));
  match EgoDevService::app_version_upload_wasm(
    service,
    &developer_principal,
    &EXIST_APP_ID.to_string(),
    &version,
    data.clone(),
    get_md5(&data),
  )
    .await
  {
    Ok(_) => {
      panic!("should not go here")
    }
    Err(e) => {
      assert_eq!(1002, e.code)
    }
  }
}

#[tokio::test]
async fn app_version_upload_wasm_released_app() {
  set_up();

  let developer_principal = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let version = Version::new(1, 0, 1);
  let data = vec![1, 0, 1, 0, 1, 0, 0, 0, 1];

  let mut service = MockFile::new();

  service
    .expect_file_main_write()
    .returning(|_, _, _, _| Ok(true));
  match EgoDevService::app_version_upload_wasm(
    service,
    &developer_principal,
    &RELEASED_APP_ID.to_string(),
    &version,
    data.clone(),
    get_md5(&data),
  )
    .await
  {
    Ok(_) => {
      panic!("should not go here")
    }
    Err(e) => {
      assert_eq!(1013, e.code)
    }
  }
}

#[test]
fn user_main_list_success() {
  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let developer = EgoDevService::developer_main_register(&caller, "user_1").unwrap();
  let user_list = Developer::list_by_name("user_1");
  assert_eq!("user_1", user_list[0].name);
  assert_eq!("user_1", developer.name);
}

#[test]
fn user_main_list_not_exist() {
  let user_list = Developer::list_by_name("user_1");
  assert!(user_list.is_empty());
}

#[test]
fn user_role_set_success() {
  set_up();
  let caller = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let user_role = EgoDevService::user_role_set(&caller, true, true).unwrap();
  assert_eq!(user_role, true);
  let developer_get = Developer::get(&caller).unwrap();
  assert_eq!(developer_get.is_app_auditor, true);
  assert_eq!(developer_get.is_manager, true);
}

#[test]
fn user_role_set_fail_with_not_developer() {
  set_up();
  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  match EgoDevService::user_role_set(&caller, true, true) {
    Ok(_) => {
      panic!("should not go here")
    }
    Err(e) => {
      assert_eq!(1011, e.code)
    }
  }
}

#[test]
fn app_get_fail_app_not_exists() {
  set_up();
  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let get_app = EgoDevApp::by_developer_id_and_id(&caller, &"not_exists_app".to_string());
  assert!(get_app.is_none());
}

#[test]
fn app_get_fail_unauthorized() {
  set_up();
  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let get_app = EgoDevApp::by_developer_id_and_id(&caller, &EXIST_APP_ID.to_string());

  assert!(get_app.is_none());
}

#[test]
fn app_get_success() {
  set_up();
  let caller = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let get_app = EgoDevApp::by_developer_id_and_id(&caller, &EXIST_APP_ID.to_string());

  assert!(get_app.is_some());
  let get_app = get_app.unwrap();
  assert_eq!(EXIST_APP_ID, get_app.app.app_id);
  assert_eq!(caller, get_app.developer_id);
}

#[test]
fn app_version_new_fail_with_unauthorized() {
  set_up();
  let version = Version::new(1, 0, 1);
  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let new_version = EgoDevService::app_version_new(&caller, &EXIST_APP_ID.to_string(), &version);

  assert!(new_version.is_err());
  let new_version = new_version.unwrap_err();
  assert_eq!("ego-dev: app not exists", new_version.msg);
  assert_eq!(1002, new_version.code);
}

#[test]
fn app_version_new_fail_with_version_exists() {
  set_up();
  let version = Version::new(1, 0, 1);
  let caller = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let new_version = EgoDevService::app_version_new(&caller, &EXIST_APP_ID.to_string(), &version);

  assert!(new_version.is_err());
  let new_version = new_version.unwrap_err();
  assert_eq!("ego-dev: version exists", new_version.msg);
  assert_eq!(1003, new_version.code);
}

#[test]
fn app_version_set_frontend_address_fail() {
  set_up();
  let version = Version::new(1, 0, 1);
  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let canister_id = Principal::from_text(TEST_CANISTER_ID.to_string()).unwrap();
  // app not exists
  let set_frontend = EgoDevService::app_version_set_frontend_address(
    &caller,
    &"test_app_id".to_string(),
    &version,
    &canister_id,
  );
  assert!(set_frontend.is_err());
  let set_frontend = set_frontend.unwrap_err();
  assert_eq!(1002, set_frontend.code);

  // unauthorized
  let frontend_unauthorized = EgoDevService::app_version_set_frontend_address(
    &caller,
    &EXIST_APP_ID.to_string(),
    &version,
    &canister_id,
  );
  assert!(frontend_unauthorized.is_err());
  let frontend_unauthorized = frontend_unauthorized.unwrap_err();
  assert_eq!(1002, frontend_unauthorized.code);

  // version not exists
  let new_version = Version::new(1, 0, 0);
  let caller_dev = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let version_not_exists = EgoDevService::app_version_set_frontend_address(
    &caller_dev,
    &EXIST_APP_ID.to_string(),
    &new_version,
    &canister_id,
  );
  assert!(version_not_exists.is_err());
  let version_not_exists = version_not_exists.unwrap_err();
  assert_eq!(1004, version_not_exists.code);
  assert_eq!("ego-dev: version not exists", version_not_exists.msg);
}

#[test]
fn app_version_set_frontend_address_success() {
  set_up();
  let version = Version::new(1, 0, 1);
  let caller = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let canister_id = Principal::from_text(TEST_CANISTER_ID.to_string()).unwrap();
  let set_frontend = EgoDevService::app_version_set_frontend_address(
    &caller,
    &EXIST_APP_ID.to_string(),
    &version,
    &canister_id,
  );
  assert!(set_frontend.is_ok());
}

#[test]
fn app_version_submit_fail() {
  set_up();
  let version = Version::new(1, 0, 1);
  let new_version = Version::new(1, 0, 0);
  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let caller_dev = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  // app not exists
  let not_exists = EgoDevService::app_version_submit(&caller, &"app_id".to_string(), &version);
  assert!(not_exists.is_err());

  let not_exists = not_exists.unwrap_err();
  assert_eq!("ego-dev: app not exists", not_exists.msg);
  assert_eq!(1002, not_exists.code);

  // unauthorized
  let submit_unauthorized =
    EgoDevService::app_version_submit(&caller, &EXIST_APP_ID.to_string(), &version);
  assert!(submit_unauthorized.is_err());
  let submit_unauthorized = submit_unauthorized.unwrap_err();
  assert_eq!(1002, submit_unauthorized.code);

  // version not exists
  let version_not_exists =
    EgoDevService::app_version_submit(&caller_dev, &EXIST_APP_ID.to_string(), &new_version);
  assert!(version_not_exists.is_err());
  let version_not_exists = version_not_exists.unwrap_err();
  assert_eq!(EgoError::from(EgoDevErr::OperationNotPermitted), version_not_exists);
}

#[test]
fn app_version_revoke_fail() {
  set_up();
  let caller_test = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let caller_dev = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let version = Version::new(1, 0, 1);
  let new_version = Version::new(1, 0, 0);

  // app not exists
  let version_not_exists =
    EgoDevService::app_version_revoke(&caller_test, &"app_test".to_string(), &version);
  assert!(version_not_exists.is_err());
  let version_not_exists = version_not_exists.unwrap_err();
  assert_eq!(1002, version_not_exists.code);
  assert_eq!("ego-dev: app not exists", version_not_exists.msg);

  // test caller unauthorized
  let caller_unauthorized =
    EgoDevService::app_version_revoke(&caller_test, &EXIST_APP_ID.to_string(), &version);
  assert!(caller_unauthorized.is_err());
  let caller_unauthorized = caller_unauthorized.unwrap_err();
  assert_eq!(EgoError::from(EgoDevErr::AppNotExists), caller_unauthorized);

  // version not exists
  let version_not_exists =
    EgoDevService::app_version_revoke(&caller_dev, &EXIST_APP_ID.to_string(), &new_version);
  assert!(version_not_exists.is_err());
  let version_not_exists = version_not_exists.unwrap_err();
  assert_eq!(EgoError::from(EgoDevErr::VersionNotExists), version_not_exists);

  // app version revoke success
  let version_revoke =
    EgoDevService::app_version_revoke(&caller_dev, &EXIST_APP_ID.to_string(), &version);
  assert!(version_revoke.is_ok());
  let version_revoke = version_revoke.unwrap();
  assert_eq!(EXIST_APP_ID, version_revoke.app_id);
  assert_eq!(AppVersionStatus::REVOKED, version_revoke.status);
}

#[tokio::test]
async fn app_version_release_fail() {
  set_up();
  let caller_test = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  // let caller_dev = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let version = Version::new(1, 0, 1);
  // let new_version = Version::new(1, 0, 0);

  // approve version
  let result = EgoDevService::app_version_approve(&EXIST_APP_ID.to_string());
  assert!(result.is_ok());

  // app not exists
  let mut ego_store = MockStore::new();
  let _result = ego_store.expect_app_main_release();
  let version_release = EgoDevService::app_version_release(
    &caller_test,
    &TEST_APP_ID.to_string(),
    &version,
    ego_store,
  );
  assert!(version_release.is_err());
  let version_release = version_release.unwrap_err();
  assert_eq!(1002, version_release.code);

  // test caller unauthorized
  let mut ego_store = MockStore::new();
  let _result = ego_store.expect_app_main_release();
  let caller_unauthorized = EgoDevService::app_version_release(
    &caller_test,
    &EXIST_APP_ID.to_string(),
    &version,
    ego_store,
  );
  assert!(caller_unauthorized.is_err());
  assert_eq!(1002, caller_unauthorized.unwrap_err().code);
}

#[test]
fn app_version_approve_fail() {
  set_up();

  let caller = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let get_app = EgoDevApp::by_developer_id_and_id(&caller, &EXIST_APP_ID.to_string());
  assert!(get_app.is_some());

  // app not exists
  let appid_not_exists = EgoDevService::app_version_approve(&"EXIST_APP_ID".to_string());
  let appid_not_exists = appid_not_exists.unwrap_err();
  assert_eq!("ego-dev: app not exists", appid_not_exists.msg);

  // approve success
  let approve_success = EgoDevService::app_version_approve(&EXIST_APP_ID.to_string());
  assert!(approve_success.is_ok());

  let approve_success = approve_success.unwrap();
  assert_eq!(AppVersionStatus::APPROVED, approve_success.status);
}

#[test]
fn app_version_reject_fail() {
  set_up();

  // app not exists
  let app_not_exists = EgoDevService::app_version_reject(&"EXIST_APP_ID".to_string());
  assert!(app_not_exists.is_err());
  let app_not_exists = app_not_exists.unwrap_err();
  assert_eq!("ego-dev: app not exists", app_not_exists.msg);
}
