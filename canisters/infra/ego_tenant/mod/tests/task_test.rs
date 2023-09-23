use candid::Principal;
use ego_tenant_mod::service::EgoTenantService;
use ego_tenant_mod::types::task::Task;
use ego_utils::util::time;

static CANISTER_ID1: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";
static CANISTER_ID2: &str = "223xb-saaaa-aaaaf-arlqa-cai";
static CANISTER_ID3: &str = "wtb37-uyaaa-aaaai-qa3zq-cai";

pub fn set_up() {
  let canister1 = Principal::from_text(CANISTER_ID1.to_string()).unwrap();
  let _ = EgoTenantService::canister_main_track(&canister1, 0);
}

#[test]
pub fn new() {
  set_up();

  let canister2 = Principal::from_text(CANISTER_ID2.to_string()).unwrap();
  let mut task = Task::new(&canister2, 10, None);
  task.save();
  assert_eq!(2, Task::len());
}

#[test]
pub fn len() {
  set_up();
  assert_eq!(1, Task::len());
}

#[test]
pub fn by_last_update() {
  set_up();

  let now = time();

  let canister2 = Principal::from_text(CANISTER_ID2.to_string()).unwrap();
  let mut task2 = Task::new(&canister2, 10, None);
  task2.save();

  // task1 and task2
  assert_eq!(2, Task::by_last_update(0, 100, now).len());
}

#[test]
pub fn by_sentinel() {
  set_up();

  let canister2 = Principal::from_text(CANISTER_ID2.to_string()).unwrap();
  let mut task2 = Task::new(&canister2, 1000, None);
  task2.save();

  let canister3 = Principal::from_text(CANISTER_ID3.to_string()).unwrap();
  let mut task3 = Task::new(&canister3, 1500, None);
  task3.try_count = 5;
  task3.save();

  // task1
  assert_eq!(1, Task::by_sentinel(500).len());

  // task3 will not be return, cause the try_count is over the limit
  assert_eq!(2, Task::by_sentinel(1500).len());
}

#[test]
pub fn list() {
  set_up();

  let canister1 = Principal::from_text(CANISTER_ID1.to_string()).unwrap();

  let tasks = Task::list(0, 100);

  assert_eq!(1, tasks.len());
  assert_eq!(canister1, tasks.get(0).unwrap().canister_id);
}

#[test]
pub fn get() {
  set_up();

  let canister1 = Principal::from_text(CANISTER_ID1.to_string()).unwrap();
  let canister2 = Principal::from_text(CANISTER_ID2.to_string()).unwrap();

  let result = Task::get(&canister1);
  assert!(result.is_some());

  let result = Task::get(&canister2);
  assert!(result.is_none());
}

#[test]
pub fn remove() {
  set_up();

  let canister1 = Principal::from_text(CANISTER_ID1.to_string()).unwrap();
  let canister2 = Principal::from_text(CANISTER_ID2.to_string()).unwrap();
  let result = Task::get(&canister1);
  assert!(result.is_some());

  Task::remove(&canister1);

  let result = Task::get(&canister2);
  assert!(result.is_none());
}
