use candid::Principal;

use ego_dev_mod::types::app_version::AppVersion;
use ego_dev_mod::types::app_version::AppVersionStatus::{APPROVED, NEW, REJECTED, RELEASED, REVOKED, SUBMITTED};
use ego_dev_mod::types::ego_dev_app::EgoDevApp;
use ego_dev_mod::types::EgoDevErr;
use ego_types::app::{Category, EgoError, Version};
use ego_utils::util::time;

static DEVELOPER_ID1: &str = "23vqh-waaaa-aaaai-qhcya-cai";
static DEVELOPER_ID2: &str = "2avdy-paaaa-aaaaf-abcga-cai";

static EXISTS_APP_ID: &str = "app_exists";
static APP_NAME: &str = "app1";
static APP_LOGO: &str = "logo";
static APP_DESCRIPTION: &str = "test is app description";

static TEST_APP_ID: &str = "app_test";

static FILE_ID1: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";


pub fn set_up() {
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

  // add version 1
  let file_id = Principal::from_text(FILE_ID1.to_string()).unwrap();
  let version = Version {
    major: 1,
    minor: 0,
    patch: 1,
  };
  ego_dev_app.version_new(&file_id, &version).unwrap();

  // add version 2
  let file_id = Principal::from_text(FILE_ID1.to_string()).unwrap();
  let version = Version {
    major: 1,
    minor: 0,
    patch: 2,
  };
  ego_dev_app.version_new(&file_id, &version).unwrap();
}

#[test]
pub fn new() {
  set_up();

  assert_eq!(1, EgoDevApp::len());

  // add new one
  let developer_id = Principal::from_text(DEVELOPER_ID1.to_string()).unwrap();
  let mut ego_dev_app = EgoDevApp::new(&developer_id,
                                       &TEST_APP_ID.to_string(),
                                       APP_NAME,
                                       APP_LOGO,
                                       APP_DESCRIPTION,
                                       &Category::Vault,
                                       0.0,
  );

  ego_dev_app.save();

  assert_eq!(2, EgoDevApp::len());
}

#[test]
pub fn new_with_exist_one() {
  set_up();

  assert_eq!(1, EgoDevApp::len());

  // add exists one
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

  assert_eq!(1, EgoDevApp::len());
}

#[test]
pub fn len() {
  set_up();
  assert_eq!(1, EgoDevApp::len());
}

#[test]
pub fn by_last_update() {
  set_up();

  let now = time() - 10;

  assert_eq!(1, EgoDevApp::by_last_update(0, 100, now).len());
}

#[test]
pub fn list() {
  set_up();

  let ego_dev_apps = EgoDevApp::list(0, 100);

  assert_eq!(1, ego_dev_apps.len());
  assert_eq!(EXISTS_APP_ID, ego_dev_apps.get(0).unwrap().app.app_id);
}

#[test]
pub fn get() {
  set_up();

  let ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string());
  assert!(ego_dev_app.is_some());

  let ego_dev_app = EgoDevApp::get(&TEST_APP_ID.to_string());
  assert!(ego_dev_app.is_none());
}


#[test]
pub fn version_new() {
  set_up();

  assert_eq!(2, AppVersion::len());
  let file_id = Principal::from_text(FILE_ID1.to_string()).unwrap();

  // add new version
  let mut ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  let version = Version {
    major: 1,
    minor: 1,
    patch: 1,
  };
  let app_version = ego_dev_app.version_new(&file_id, &version);
  assert!(app_version.is_ok());

  assert_eq!(3, AppVersion::len());

  // add same version
  let mut ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  let app_version = ego_dev_app.version_new(&file_id, &version);
  assert!(app_version.is_err());
  assert_eq!(EgoError::from(EgoDevErr::VersionExists), app_version.unwrap_err());

  assert_eq!(3, AppVersion::len());
}


#[test]
pub fn version_get() {
  set_up();

  // exists version
  let version = Version {
    major: 1,
    minor: 0,
    patch: 1,
  };
  let ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  let app_version = ego_dev_app.version_get(&version);
  assert!(app_version.is_some());

  // not exists version
  let version = Version {
    major: 1,
    minor: 1,
    patch: 1,
  };
  let ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  let app_version = ego_dev_app.version_get(&version);
  assert!(app_version.is_none());
}


#[test]
pub fn version_submit() {
  set_up();

  // exists version
  let version = Version {
    major: 1,
    minor: 0,
    patch: 1,
  };

  // before test
  let ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  assert_eq!(None, ego_dev_app.audit_version);

  let app_version = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version).unwrap();
  assert_eq!(NEW, app_version.status);

  // submit
  let mut ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  let result = ego_dev_app.version_submit(&version);
  assert!(result.is_ok());

  // check data
  let ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  assert_eq!(version, ego_dev_app.audit_version.unwrap());

  let app_version = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version).unwrap();
  assert_eq!(SUBMITTED, app_version.status);
}

#[test]
pub fn version_submit_failed_with_not_exists_version() {
  set_up();

  // not exists version
  let version = Version {
    major: 1,
    minor: 1,
    patch: 1,
  };

  // before test
  let ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  assert_eq!(None, ego_dev_app.audit_version);

  let app_version = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version);
  assert!(app_version.is_none());

  // submit
  let mut ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  let result = ego_dev_app.version_submit(&version);
  assert!(result.is_err());
  assert_eq!(None, ego_dev_app.audit_version);

  // check data
  let ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  assert_eq!(None, ego_dev_app.audit_version);

  let app_version = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version);
  assert!(app_version.is_none());
}

#[test]
pub fn version_submit_failed_with_submitted_version() {
  set_up();

  // exists version
  let version1 = Version {
    major: 1,
    minor: 0,
    patch: 1,
  };

  // before test
  let mut ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  ego_dev_app.audit_version = Some(version1);
  ego_dev_app.save();

  let mut app_version = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version1).unwrap();
  app_version.status = SUBMITTED;
  app_version.save();

  // submit
  let version2 = Version {
    major: 1,
    minor: 0,
    patch: 2,
  };
  let mut ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  let result = ego_dev_app.version_submit(&version2);
  assert!(result.is_err());

  // check data
  let ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  assert_eq!(version1, ego_dev_app.audit_version.unwrap());

  let app_version = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version1).unwrap();
  assert_eq!(SUBMITTED, app_version.status);

  let app_version = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version2).unwrap();
  assert_eq!(NEW, app_version.status);
}


#[test]
pub fn version_revoke_submitted_version() {
  set_up();

  // exists version
  let version = Version {
    major: 1,
    minor: 0,
    patch: 1,
  };

  // before test
  let mut ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  ego_dev_app.audit_version = Some(version);
  ego_dev_app.save();

  let mut app_version = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version).unwrap();
  app_version.status = SUBMITTED;
  app_version.save();

  // revoke
  let mut ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  let result = ego_dev_app.version_revoke(&version);
  assert!(result.is_ok());

  // check data
  let ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  assert_eq!(None, ego_dev_app.audit_version);

  let app_version = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version).unwrap();
  assert_eq!(REVOKED, app_version.status);
}

#[test]
pub fn version_revoke_released_version() {
  set_up();

  // exists version
  let version1 = Version {
    major: 1,
    minor: 0,
    patch: 1,
  };

  let version2 = Version {
    major: 1,
    minor: 0,
    patch: 2,
  };

  // before test
  let mut ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  ego_dev_app.audit_version = Some(version1);
  ego_dev_app.save();

  let mut app_version = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version1).unwrap();
  app_version.status = SUBMITTED;
  app_version.save();

  let mut app_version = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version2).unwrap();
  app_version.status = RELEASED;
  app_version.save();

  // revoke
  let mut ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  let result = ego_dev_app.version_revoke(&version2);
  assert!(result.is_ok());

  // check data
  let ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  assert_eq!(version1, ego_dev_app.audit_version.unwrap());

  let app_version = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version1).unwrap();
  assert_eq!(SUBMITTED, app_version.status);

  let app_version = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version2).unwrap();
  assert_eq!(REVOKED, app_version.status);
}

#[test]
pub fn version_revoke_failed_not_exists_version() {
  set_up();

  // not exists version
  let version = Version {
    major: 1,
    minor: 1,
    patch: 1,
  };

  // before test
  let ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  assert_eq!(None, ego_dev_app.audit_version);

  let app_version = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version);
  assert_eq!(None, app_version);

  let mut ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  let result = ego_dev_app.version_revoke(&version);
  assert!(result.is_err());
}

#[test]
pub fn version_revoke_failed_wrong_status() {
  set_up();

  // not exists version
  let version = Version {
    major: 1,
    minor: 0,
    patch: 1,
  };

  // before test
  let mut app_version = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version).unwrap();
  app_version.status = APPROVED;
  app_version.save();

  let mut ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  let result = ego_dev_app.version_revoke(&version);
  assert!(result.is_err());
}

#[test]
pub fn version_release() {
  set_up();

  // exists version
  let version = Version {
    major: 1,
    minor: 0,
    patch: 1,
  };

  // before test
  let mut app_version = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version).unwrap();
  app_version.status = APPROVED;
  app_version.save();

  // release
  let mut ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  let result = ego_dev_app.version_release(&version);
  assert!(result.is_ok());

  // check data
  let ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  assert_eq!(version, ego_dev_app.released_version().unwrap().version);

  let app_version = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version).unwrap();
  assert_eq!(RELEASED, app_version.status);
}

#[test]
pub fn version_release_failed_with_wrong_status() {
  set_up();

  // exists version
  let version = Version {
    major: 1,
    minor: 0,
    patch: 1,
  };

  // before test
  let mut app_version = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version).unwrap();
  app_version.status = REJECTED;
  app_version.save();

  // release
  let mut ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  let result = ego_dev_app.version_release(&version);
  assert!(result.is_err());

  // check data
  let ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  assert_eq!(None, ego_dev_app.released_version());

  let app_version = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version).unwrap();
  assert_eq!(REJECTED, app_version.status);
}

#[test]
pub fn version_approve() {
  set_up();

  // exists version
  let version = Version {
    major: 1,
    minor: 0,
    patch: 1,
  };

  // before test
  let mut ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  ego_dev_app.audit_version = Some(version);
  ego_dev_app.save();

  let mut app_version = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version).unwrap();
  app_version.status = SUBMITTED;
  app_version.save();

  // approve
  let mut ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  let result = ego_dev_app.version_approve();
  assert!(result.is_ok());

  // check data
  let ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  assert_eq!(None, ego_dev_app.audit_version);

  let app_version = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version).unwrap();
  assert_eq!(APPROVED, app_version.status);
}

#[test]
pub fn version_approve_failed() {
  set_up();

  let mut ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  assert_eq!(None, ego_dev_app.audit_version);

  // not the audit version
  let result = ego_dev_app.version_approve();
  assert!(result.is_err());
}

#[test]
pub fn version_reject() {
  set_up();

  // exists version
  let version = Version {
    major: 1,
    minor: 0,
    patch: 1,
  };

  // before test
  let mut ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  ego_dev_app.audit_version = Some(version);
  ego_dev_app.save();

  let mut app_version = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version).unwrap();
  app_version.status = SUBMITTED;
  app_version.save();

  // approve
  let mut ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  let result = ego_dev_app.version_reject();
  assert!(result.is_ok());

  // check data
  let ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  assert_eq!(None, ego_dev_app.audit_version);

  let app_version = AppVersion::get_by_app_id_and_version(&EXISTS_APP_ID.to_string(), &version).unwrap();
  assert_eq!(REJECTED, app_version.status);
}

#[test]
pub fn version_reject_failed() {
  set_up();

  let mut ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  assert_eq!(None, ego_dev_app.audit_version);

  // not the audit version
  let result = ego_dev_app.version_reject();
  assert!(result.is_err());
}

#[test]
pub fn released_version() {
  set_up();

  let ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  assert_eq!(None, ego_dev_app.released_version());

  let version = Version {
    major: 1,
    minor: 0,
    patch: 1,
  };
  let mut ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  ego_dev_app.app.current_version = version;
  ego_dev_app.save();

  let ego_dev_app = EgoDevApp::get(&EXISTS_APP_ID.to_string()).unwrap();
  assert_eq!(version, ego_dev_app.released_version().unwrap().version);
}

#[test]
pub fn by_developer_id() {
  set_up();
  let developer_id = Principal::from_text(DEVELOPER_ID1.to_string()).unwrap();
  let ego_dev_apps = EgoDevApp::by_developer_id(&developer_id);

  assert_eq!(1, ego_dev_apps.len());

  let developer_id = Principal::from_text(DEVELOPER_ID2.to_string()).unwrap();
  let ego_dev_apps = EgoDevApp::by_developer_id(&developer_id);

  assert_eq!(0, ego_dev_apps.len());
}

#[test]
pub fn by_developer_id_and_id() {
  set_up();
  let developer_id = Principal::from_text(DEVELOPER_ID1.to_string()).unwrap();
  let ego_dev_app = EgoDevApp::by_developer_id_and_id(&developer_id, &EXISTS_APP_ID.to_string());

  assert!(ego_dev_app.is_some());

  let developer_id = Principal::from_text(DEVELOPER_ID1.to_string()).unwrap();
  let ego_dev_app = EgoDevApp::by_developer_id_and_id(&developer_id, &TEST_APP_ID.to_string());

  assert!(ego_dev_app.is_none());

  let developer_id = Principal::from_text(DEVELOPER_ID2.to_string()).unwrap();
  let ego_dev_app = EgoDevApp::by_developer_id_and_id(&developer_id, &EXISTS_APP_ID.to_string());

  assert!(ego_dev_app.is_none());
}
