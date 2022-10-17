use ic_cdk::export::Principal;
use rand::Rng;
use ego_dev_mod::app::{App, AppVersion, AppVersionStatus};
use ego_dev_mod::developer::Developer;
use ego_dev_mod::file::File;
use ego_dev_mod::service::EgoDevService;
use ego_dev_mod::state::{EGO_DEV, EGO_STORE_CANISTER_ID};
use ego_dev_mod::c2c::ego_file::TEgoFile;
use ego_dev_mod::c2c::ego_store::TEgoStore;
use async_trait::async_trait;
use mockall::mock;
use ego_types::app::Category;
use ego_types::version::Version;
use ego_types::ego_error::EgoError;

mock! {
  Dump {}

  #[async_trait]
  impl TEgoFile for Dump {
    async fn file_main_write(&self, canister_id: Principal, fid: String, hash: String, data: Vec<u8>) -> Result<bool, EgoError>;
  }
}

mock! {
  Store {}

  #[async_trait]
  impl TEgoStore for Store {
    async fn app_main_release(&self, canister_id: Principal, app: App) -> Result<bool, EgoError>;
  }
}

static FILE_CANISTER_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";
static STORE_CANISTER_ID: &str = "2265i-mqaaa-aaaad-qbsga-cai";

static AUDITER_PRINCIPAL_ID: &str = "22ayq-aiaaa-aaaai-qgmma-cai";

static DEVELOPER_PRINCIPAL_ID: &str = "5oynr-yl472-mav57-c2oxo-g7woc-yytib-mp5bo-kzg3b-622pu-uatef-uqe";
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
  let store_canister = Principal::from_text(STORE_CANISTER_ID.to_string()).unwrap();

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
    let mut app = App::new(developer_principal, EXIST_APP_ID.to_string(), EXIST_APP_NAME.to_string(), APP_LOGO.to_string(), APP_DESCRIPTION.to_string(), Category::Vault, 0f32);
    let mut app_version = AppVersion::new(EXIST_APP_ID.to_string(), file_canister, version);
    app_version.status = AppVersionStatus::SUBMITTED;
    app.versions.push(app_version);

    app.audit_version = Some(version);
    ego_dev.borrow_mut().apps.insert(EXIST_APP_ID.to_string(), app);

    // relesed app
    let mut app = App::new(developer_principal, RELEASED_APP_ID.to_string(), RELEASED_APP_NAME.to_string(), APP_LOGO.to_string(), APP_DESCRIPTION.to_string(), Category::Vault, 0f32);
    let mut app_version = AppVersion::new(RELEASED_APP_ID.to_string(), file_canister, version);
    app_version.status = AppVersionStatus::RELEASED;
    app.versions.push(app_version);

    app.release_version = Some(version);
    ego_dev.borrow_mut().apps.insert(RELEASED_APP_ID.to_string(), app);
  });

  EGO_STORE_CANISTER_ID.with(|s| {
    *s.borrow_mut() = Some(store_canister);
  });
}

#[test]
fn admin_file_add() {
  let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();
  let resp = EgoDevService::admin_ego_file_add(file_canister);
  assert!(resp.is_ok());
  println!("resp message is: {:?}",resp);

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
fn developer_main_register_fail_name_existed(){
  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let num = rand::thread_rng().gen_range(1..=10001).to_string();
  let name = "user";
  let users = format!("{}_{}", name, num);
  println!("names is {}", users);
  let developer = EgoDevService::developer_main_register(caller, users.to_string()).unwrap();
  println!("register name is: {}, user id is: {}", developer.name, developer.user_id);
  assert_eq!(caller, developer.user_id);
  assert_eq!(users, developer.name);
  println!("created_apps is {:?}", developer);
  // The user name and principal have been existed 
  let developer = EgoDevService::developer_main_register(caller, users.to_string()).unwrap();
  // assert!(false);
  let result = format!("{:?}",developer);
  assert_eq!("the user name and principal are already exists", result);
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
  println!("{:?}",developer);
  assert_eq!(caller, developer.user_id);
}

#[test]
fn developer_app_new_fail_with_exists_app_id() {
  set_up();

  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let result = EgoDevService::developer_app_new(caller, EXIST_APP_ID.to_string(), EXIST_APP_NAME.to_string(), APP_LOGO.to_string(), APP_DESCRIPTION.to_string(), Category::Vault, 0f32);

  assert_eq!(1001, result.unwrap_err().code);
}

#[test]
fn developer_app_new_fail_with_none_register_developer() {
  set_up();

  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let result = EgoDevService::developer_app_new(caller, TEST_APP_ID.to_string(), TEST_APP_NAME.to_string(), APP_LOGO.to_string(), APP_DESCRIPTION.to_string(), Category::Vault, 0f32);
  assert!(result.is_err());

  assert_eq!(1011, result.unwrap_err().code);
}

#[test]
fn developer_app_new_success(){
  set_up();

  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let result = EgoDevService::developer_main_register(caller, "user_1".to_string());
  assert!(result.is_ok());

  let get_app_list = EgoDevService::developer_app_list(caller);
  // println!("app list is {:?}", get_app_list);
  let list_result = format!("{:?}", get_app_list);
 
  assert_eq!("Ok([])", list_result);
  // app new success with developer
  let result = EgoDevService::developer_app_new(caller, TEST_APP_ID.to_string(), TEST_APP_NAME.to_string(), APP_LOGO.to_string(), APP_DESCRIPTION.to_string(), Category::Vault, 0f32);

  assert!(result.is_ok());

  let app = result.unwrap();
  assert_eq!(TEST_APP_ID, app.app_id);
  // test_app has been added successfully by developer
  let get_new_app_list = EgoDevService::developer_app_list(caller);
  let list_result = get_new_app_list.unwrap();
  let app = &list_result[0];
  assert_eq!(TEST_APP_ID, app.app_id);
}

#[test]
fn app_version_new_success(){
  set_up();

  let version = Version::new(1, 0, 1);

  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let result = EgoDevService::developer_main_register(caller, "user_1".to_string());
  assert!(result.is_ok());

  let result = EgoDevService::developer_app_new(caller, TEST_APP_ID.to_string(), TEST_APP_NAME.to_string(), APP_LOGO.to_string(), APP_DESCRIPTION.to_string(), Category::Vault, 0f32);
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

  let result = EgoDevService::developer_app_new(caller, TEST_APP_ID.to_string(), TEST_APP_NAME.to_string(), APP_LOGO.to_string(), APP_DESCRIPTION.to_string(), Category::Vault, 0f32);
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
  println!("{:?}", result);
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

#[tokio::test]
async fn app_version_release(){
  set_up();

  let developer = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let version = Version::new(1, 0, 1);

  let mut ego_store = MockStore::new();
  ego_store.expect_app_main_release().returning(|_, _| {
    Ok(true)
  });

  // approve version
  let result = EgoDevService::app_version_approve(EXIST_APP_ID.to_string(), version);
  assert!(result.is_ok());

  // check after audit
  let result = EgoDevService::app_version_release(developer, EXIST_APP_ID.to_string(), version, ego_store).await;
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

#[test]
fn user_main_list_success(){
  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let developer = EgoDevService::developer_main_register(caller, "user_1".to_string()).unwrap();
  let user_list = EgoDevService::user_main_list("user_1".to_string());
  assert_eq!("user_1", user_list[0].name);
  assert_eq!("user_1", developer.name);
}

#[test]
fn user_main_list_not_exist(){
  let user_list = EgoDevService::user_main_list("user_1".to_string());
  assert_eq!(user_list.is_empty(),true);
}

#[test]
fn user_role_set_success(){
  set_up();
  let caller = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let user_role = EgoDevService::user_role_set(caller, true, true).unwrap();
  assert_eq!(user_role, true);
  let developer_get = EgoDevService::developer_main_get(caller).unwrap();
  assert_eq!(developer_get.is_app_auditor, true);
  assert_eq!(developer_get.is_manager, true);
}

#[test]
fn user_role_set_fail_with_not_developer(){
  set_up();
  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  match EgoDevService::user_role_set(caller, true, true){
    Ok(_) => {
      panic!("should not go here")
    },
    Err(e) => {
      assert_eq!(1011, e.code)
    },
  }
}

#[test]
fn app_get_fail_app_not_exists(){
  set_up();
  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let get_app = EgoDevService::developer_app_get(caller, "test_app".to_string());
  assert!(get_app.is_err());
  let get_app = get_app.unwrap_err();
  assert_eq!("ego-dev: app not exists", get_app.msg);
  assert_eq!(1002, get_app.code);
}

#[test]
fn app_get_fail_unauthorized(){
  set_up();
  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let get_app = EgoDevService::developer_app_get(caller, EXIST_APP_ID.to_string());
  println!("{:?}", get_app);
  assert!(get_app.is_err());
  let get_app = get_app.unwrap_err();
  assert_eq!("ego-dev: unauthorized", get_app.msg);
  assert_eq!(1007, get_app.code);
}

#[test]
fn app_get_success(){
  set_up();
  let caller = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let get_app = EgoDevService::developer_app_get(caller, EXIST_APP_ID.to_string());
  println!("{:?}", get_app);
  assert!(get_app.is_ok());
  let get_app = get_app.unwrap();
  assert_eq!(EXIST_APP_ID, get_app.app_id);
  assert_eq!(caller, get_app.developer_id);
}

#[test]
fn app_version_new_fail_with_unauthorized(){
  set_up();
  let version = Version::new(1, 0, 1);
  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let new_version = EgoDevService::app_version_new(caller, EXIST_APP_ID.to_string(), version);
  println!("{:?}", new_version);
  assert!(new_version.is_err());
  let new_version = new_version.unwrap_err();
  assert_eq!("ego-dev: unauthorized", new_version.msg);
  assert_eq!(1007, new_version.code);
}

#[test]
fn app_version_new_fail_with_version_exists(){
  set_up();
  let version = Version::new(1, 0, 1);
  let caller = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let new_version = EgoDevService::app_version_new(caller, EXIST_APP_ID.to_string(), version);
  println!("{:?}", new_version);
  assert!(new_version.is_err());
  let new_version = new_version.unwrap_err();
  assert_eq!("ego-dev: version exists", new_version.msg);
  assert_eq!(1003, new_version.code);
}

#[test]
fn app_version_set_frontend_address_fail(){
  set_up();
  let version = Version::new(1, 0, 1);
  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let canister_id = Principal::from_text(TEST_CANISTER_ID.to_string()).unwrap();
  // app not exists
  let set_frontend = EgoDevService::app_version_set_frontend_address(caller, "test_app_id".to_string(), version, canister_id);
  assert!(set_frontend.is_err());
  let set_frontend = set_frontend.unwrap_err();
  assert_eq!(1002, set_frontend.code);
  assert_eq!("ego-dev: app not exists", set_frontend.msg);

  // unauthorized
  let frontend_unauthorized = EgoDevService::app_version_set_frontend_address(caller,EXIST_APP_ID.to_string(), version, canister_id);
  assert!(frontend_unauthorized.is_err());
  let frontend_unauthorized = frontend_unauthorized.unwrap_err();
  assert_eq!(1007, frontend_unauthorized.code);
  assert_eq!("ego-dev: unauthorized", frontend_unauthorized.msg);

  // version not exists
  let new_version = Version::new(1, 0, 0);
  let caller_dev = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let version_not_exists = EgoDevService::app_version_set_frontend_address(caller_dev,EXIST_APP_ID.to_string(), new_version, canister_id);
  assert!(version_not_exists.is_err());
  let version_not_exists = version_not_exists.unwrap_err();
  assert_eq!(1004, version_not_exists.code);
  assert_eq!("ego-dev: version not exists", version_not_exists.msg);
}

#[test]
fn app_version_set_frontend_address_success(){
  set_up();
  let version = Version::new(1, 0, 1);
  let caller = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let canister_id = Principal::from_text(TEST_CANISTER_ID.to_string()).unwrap();
  let set_frontend = EgoDevService::app_version_set_frontend_address(caller,EXIST_APP_ID.to_string(), version, canister_id);
  assert!(set_frontend.is_ok());
  // println!("{:?}", set_frontend);
}

#[test]
fn app_version_submit_fail(){
  set_up();
  let version = Version::new(1, 0, 1);
  let new_version = Version::new(1, 0,0);
  let caller = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let caller_dev = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  // app not exists
  let not_exists = EgoDevService::app_version_submit(caller, "app_id".to_string(), version);
  assert!(not_exists.is_err());
  // println!("{:?}", not_exists);
  let not_exists = not_exists.unwrap_err();
  assert_eq!("ego-dev: app not exists", not_exists.msg);
  assert_eq!(1002, not_exists.code);

  // unauthorized
  let submit_unauthorized = EgoDevService::app_version_submit(caller, EXIST_APP_ID.to_string(), version);
  assert!(submit_unauthorized.is_err());
  let submit_unauthorized = submit_unauthorized.unwrap_err();
  assert_eq!("ego-dev: unauthorized", submit_unauthorized.msg);
  assert_eq!(1007, submit_unauthorized.code);

  // version not exists
  let version_not_exists = EgoDevService::app_version_submit(caller_dev, EXIST_APP_ID.to_string(), new_version);
  assert!(version_not_exists.is_err());
  let version_not_exists = version_not_exists.unwrap_err();
  assert_eq!(1004, version_not_exists.code);
  assert_eq!("ego-dev: version not exists", version_not_exists.msg);
  // println!("{:?}", version_not_exists);
}

#[test]
fn app_version_revoke_fail(){
  set_up();
  let caller_test = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  let caller_dev = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let version = Version::new(1, 0, 1);
  let new_version = Version::new(1, 0, 0);

  // app not exists
  let version_not_exists = EgoDevService::app_version_revoke(caller_test, "app_test".to_string(), version);
  assert!(version_not_exists.is_err());
  let version_not_exists = version_not_exists.unwrap_err();
  assert_eq!(1002, version_not_exists.code);
  assert_eq!("ego-dev: app not exists", version_not_exists.msg);

  // test caller unauthorized
  let version_submit = EgoDevService::app_version_submit(caller_dev, EXIST_APP_ID.to_string(), version);
  assert!(version_submit.is_ok());
  let caller_unauthorized = EgoDevService::app_version_revoke(caller_test, EXIST_APP_ID.to_string(), version);
  assert!(caller_unauthorized.is_err());
  let caller_unauthorized = caller_unauthorized.unwrap_err();
  assert_eq!(1007, caller_unauthorized.code);
  assert_eq!("ego-dev: unauthorized", caller_unauthorized.msg);

  // version not exists
  let version_not_exists = EgoDevService::app_version_revoke(caller_dev, EXIST_APP_ID.to_string(), new_version);
  assert!(version_not_exists.is_err());
  let version_not_exists = version_not_exists.unwrap_err();
  assert_eq!(1004, version_not_exists.code);
  assert_eq!("ego-dev: version not exists", version_not_exists.msg);

  // app version revoke success
  let version_revoke = EgoDevService::app_version_revoke(caller_dev, EXIST_APP_ID.to_string(), version);
  assert!(version_revoke.is_ok());
  let version_revoke = version_revoke.unwrap();
  assert_eq!(EXIST_APP_ID, version_revoke.app_id);
  assert_eq!(AppVersionStatus::REVOKED, version_revoke.status);
}

#[tokio::test]
async fn app_version_release_fail (){
  set_up();
  let caller_test = Principal::from_text(TEST_PRINCIPAL_ID.to_string()).unwrap();
  // let caller_dev = Principal::from_text(DEVELOPER_PRINCIPAL_ID.to_string()).unwrap();
  let version = Version::new(1, 0, 1);
  // let new_version = Version::new(1, 0, 0);
  let mut ego_store = MockStore::new();
  ego_store.expect_app_main_release().returning(|_, _| {
    Ok(true)
  });

  // approve version
  let result = EgoDevService::app_version_approve(EXIST_APP_ID.to_string(), version);
  assert!(result.is_ok());

  // app version release
  let version_release = EgoDevService::app_version_release(caller_test, TEST_APP_ID.to_string(), version, ego_store).await;
  assert!(version_release.is_err());
  let version_release = version_release.unwrap_err();
  assert_eq!(version_release.code, 1002);
  // println!("{:#?}", version_release);
}
