use candid::Principal;

use ego_dev_mod::types::file::File;

static FILE_ID1: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";
static FILE_ID2: &str = "223xb-saaaa-aaaaf-arlqa-cai";

pub fn set_up() {
  let ego_file_id = Principal::from_text(FILE_ID1.to_string()).unwrap();
  let file = File::new(&ego_file_id);
  file.save();
}

#[test]
pub fn new() {
  set_up();

  assert_eq!(1, File::len());

  let ego_file_id = Principal::from_text(FILE_ID2.to_string()).unwrap();
  let file = File::new(&ego_file_id);
  file.save();

  assert_eq!(2, File::len());
}

#[test]
pub fn len() {
  set_up();
  assert_eq!(1, File::len());
}

#[test]
pub fn list() {
  set_up();

  let ego_file_id = Principal::from_text(FILE_ID1.to_string()).unwrap();

  let files = File::list(0, 100);

  assert_eq!(1, files.len());
  assert_eq!(ego_file_id, files.get(0).unwrap().canister_id);
}

#[test]
pub fn get() {
  set_up();

  let ego_file_id = Principal::from_text(FILE_ID1.to_string()).unwrap();
  let ego_file = File::get(&ego_file_id);
  assert!(ego_file.is_some());

  let ego_file_id = Principal::from_text(FILE_ID2.to_string()).unwrap();
  let ego_file = File::get(&ego_file_id);
  assert!(ego_file.is_none());
}