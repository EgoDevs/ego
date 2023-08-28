use candid::Principal;

use ego_dev_mod::backup::{job_list, record_export};
use ego_dev_mod::state::{canister_add, owner_add};
use ego_dev_mod::types::app_version::AppVersion;
use ego_dev_mod::types::developer::Developer;
use ego_dev_mod::types::ego_dev_app::EgoDevApp;
use ego_dev_mod::types::file::File;
use ego_dev_mod::types::stable_state::StableState;
use ego_types::app::{Category, Version};
use ego_utils::util::time;

static EXISTS_CANISTER_ID: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";
static EXISTS_USER_ID: &str = "225da-yaaaa-aaaah-qahrq-cai";

static DEVELOPER_ID1: &str = "23vqh-waaaa-aaaai-qhcya-cai";
static DEVELOPER_NAME1: &str = "developer 1";

static EXISTS_APP_ID: &str = "app_exists";
static APP_NAME: &str = "app1";
static APP_LOGO: &str = "logo";
static APP_DESCRIPTION: &str = "test is app description";

static FILE_ID1: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";

fn set_up() {
  owner_add(Principal::from_text(EXISTS_USER_ID.to_string()).unwrap());
  canister_add("test".to_string(), Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap());

  // add ego dev app
  let developer_id = Principal::from_text(DEVELOPER_ID1.to_string()).unwrap();
  let mut ego_dev_app = EgoDevApp::new(&developer_id,
                                       &EXISTS_APP_ID.to_string(),
                                       APP_NAME,
                                       APP_LOGO,
                                       APP_DESCRIPTION,
                                       &Category::Vault,
                                       0.0,
  );

  ego_dev_app.save();

  // add file
  let ego_file_id = Principal::from_text(FILE_ID1.to_string()).unwrap();
  let file = File::new(&ego_file_id);
  file.save();

  // add developer
  let mut developer = Developer::new(&developer_id, DEVELOPER_NAME1);
  developer.save();

  // add app_version
  let ego_file_id = Principal::from_text(FILE_ID1.to_string()).unwrap();
  let version = Version::new(1, 0, 1);

  let mut app_version = AppVersion::new(&EXISTS_APP_ID.to_string(), &ego_file_id, &version);
  app_version.save();
}

#[test]
fn test_job_list() {
  set_up();

  let jobs = job_list();
  assert_eq!(5, jobs.len());

  assert_eq!("config", jobs.get(0).unwrap().name);
  assert_eq!(1, jobs.get(0).unwrap().amount);

  assert_eq!("ego_dev_apps", jobs.get(1).unwrap().name);
  assert_eq!(1, jobs.get(1).unwrap().amount);

  assert_eq!("files", jobs.get(2).unwrap().name);
  assert_eq!(1, jobs.get(2).unwrap().amount);

  assert_eq!("developers", jobs.get(3).unwrap().name);
  assert_eq!(1, jobs.get(3).unwrap().amount);

  assert_eq!("app_versions", jobs.get(4).unwrap().name);
  assert_eq!(1, jobs.get(4).unwrap().amount);
}

#[test]
fn test_export_config() {
  set_up();

  let result = record_export("config".to_string(), 0, 1000, None).expect("record not founded");
  assert_eq!("config", result.name);
  let stable_state: StableState = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(1, stable_state.seq.clone().unwrap().get_number("app_version").unwrap());
}

#[test]
fn test_export() {
  set_up();

  let result = record_export("ego_dev_apps".to_string(), 0, 1000, None).expect("record not founded");
  assert_eq!("ego_dev_apps", result.name);
  let ego_dev_apps: Vec<EgoDevApp> = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(1, ego_dev_apps.len());
  assert_eq!(EXISTS_APP_ID, ego_dev_apps.get(0).unwrap().app.app_id);

  let result = record_export("files".to_string(), 0, 1000, None).expect("record not founded");
  assert_eq!("files", result.name);
  let files: Vec<File> = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(1, files.len());
  assert_eq!(FILE_ID1, files.get(0).unwrap().canister_id.to_string());

  let result = record_export("developers".to_string(), 0, 1000, None).expect("record not founded");
  assert_eq!("developers", result.name);
  let developers: Vec<Developer> = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(1, developers.len());
  assert_eq!(DEVELOPER_ID1, developers.get(0).unwrap().developer_id.to_string());

  let result = record_export("app_versions".to_string(), 0, 1000, None).expect("record not founded");
  assert_eq!("app_versions", result.name);
  let app_versions: Vec<AppVersion> = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(1, app_versions.len());
  assert_eq!(EXISTS_APP_ID, app_versions.get(0).unwrap().app_id);
}

#[test]
fn test_export_with_last_update() {
  set_up();

  let last_update = Some(time() + 100);

  let result = record_export("ego_dev_apps".to_string(), 0, 1000, last_update).expect("record not founded");
  assert_eq!("ego_dev_apps", result.name);
  let ego_dev_apps: Vec<EgoDevApp> = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(0, ego_dev_apps.len());

  let result = record_export("files".to_string(), 0, 1000, last_update).expect("record not founded");
  assert_eq!("files", result.name);
  let files: Vec<File> = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(1, files.len());
  assert_eq!(FILE_ID1, files.get(0).unwrap().canister_id.to_string());

  let result = record_export("developers".to_string(), 0, 1000, last_update).expect("record not founded");
  assert_eq!("developers", result.name);
  let developers: Vec<Developer> = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(0, developers.len());

  let result = record_export("app_versions".to_string(), 0, 1000, last_update).expect("record not founded");
  assert_eq!("app_versions", result.name);
  let app_versions: Vec<AppVersion> = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(0, app_versions.len());
}