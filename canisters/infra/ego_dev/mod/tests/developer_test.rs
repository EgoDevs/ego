use candid::Principal;

use ego_dev_mod::types::developer::Developer;
use ego_utils::util::time;

static DEVELOPER_ID1: &str = "23vqh-waaaa-aaaai-qhcya-cai";
static DEVELOPER_NAME1: &str = "developer 1";

static DEVELOPER_ID2: &str = "2avdy-paaaa-aaaaf-abcga-cai";
static DEVELOPER_NAME2: &str = "developer 2";


pub fn set_up() {
  // add
  let developer_id = Principal::from_text(DEVELOPER_ID1.to_string()).unwrap();
  let mut developer = Developer::new(&developer_id, DEVELOPER_NAME1);
  developer.save();
}

#[test]
pub fn new() {
  set_up();

  assert_eq!(1, Developer::len());

  // add new one
  let developer_id = Principal::from_text(DEVELOPER_ID2.to_string()).unwrap();
  let mut developer = Developer::new(&developer_id, DEVELOPER_NAME2);
  developer.save();

  assert_eq!(2, Developer::len());
}

#[test]
pub fn new_with_exist_one() {
  set_up();

  assert_eq!(1, Developer::len());

  // add same one
  let developer_id = Principal::from_text(DEVELOPER_ID1.to_string()).unwrap();
  let mut developer = Developer::new(&developer_id, DEVELOPER_NAME1);
  developer.save();

  assert_eq!(1, Developer::len());
}

#[test]
pub fn len() {
  set_up();
  assert_eq!(1, Developer::len());
}

#[test]
pub fn by_last_update() {
  set_up();

  let now = time();

  assert_eq!(1, Developer::by_last_update(now).len());
}

#[test]
pub fn list() {
  set_up();

  let developers = Developer::list();

  assert_eq!(1, developers.len());
  assert_eq!(DEVELOPER_ID1, developers.get(0).unwrap().developer_id.to_string());
}

#[test]
pub fn get() {
  set_up();

  let developer_id = Principal::from_text(DEVELOPER_ID1.to_string()).unwrap();
  let developer = Developer::get(&developer_id);
  assert!(developer.is_some());

  let developer_id = Principal::from_text(DEVELOPER_ID2.to_string()).unwrap();
  let developer = Developer::get(&developer_id);
  assert!(developer.is_none());
}

#[test]
pub fn list_by_name() {
  set_up();

  let developers = Developer::list_by_name(DEVELOPER_NAME1);
  assert_eq!(1, developers.len());
  assert_eq!(DEVELOPER_NAME1, developers.get(0).unwrap().name);

  let developers = Developer::list_by_name(DEVELOPER_NAME2);
  assert_eq!(0, developers.len());
}

