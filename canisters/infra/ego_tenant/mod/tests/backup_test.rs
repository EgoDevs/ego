use candid::Principal;
use ego_tenant_mod::backup::{job_list, record_export};
use ego_tenant_mod::state::{canister_add, owner_add};
use ego_tenant_mod::types::task::Task;
use ego_utils::util::time;

static EXISTS_CANISTER_ID: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";
static EXISTS_USER_ID: &str = "225da-yaaaa-aaaah-qahrq-cai";

static CANISTER_ID1: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";
static CANISTER_ID2: &str = "223xb-saaaa-aaaaf-arlqa-cai";

fn set_up() {
  owner_add(Principal::from_text(EXISTS_USER_ID.to_string()).unwrap());
  canister_add("test".to_string(), Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap());

  let canister1 = Principal::from_text(CANISTER_ID1.to_string()).unwrap();
  let mut task1 = Task::new(&canister1, 100, None);
  task1.save();

  let canister2 = Principal::from_text(CANISTER_ID2.to_string()).unwrap();
  let mut task2 = Task::new(&canister2, 200, None);
  task2.save();
}

#[test]
fn test_job_list(){
  set_up();

  let jobs = job_list();
  assert_eq!(2, jobs.len());

  assert_eq!("config", jobs.get(0).unwrap().name);
  assert_eq!(1, jobs.get(0).unwrap().amount);

  assert_eq!("tasks", jobs.get(1).unwrap().name);
  assert_eq!(2, jobs.get(1).unwrap().amount);
}

#[test]
fn test_export_config(){
  set_up();

  let result = record_export("config".to_string(), 0, 1000, 0).expect("record not founded");

  assert_eq!("config", result.name);
  assert_eq!(r#"{"users":{"owners":{"225da-yaaaa-aaaah-qahrq-cai":"225da-yaaaa-aaaah-qahrq-cai"},"users":{},"ops":{}},"registry":{"canisters":{"test":["22fyd-yaaaa-aaaaf-aml4q-cai"]}},"cycle_info":{"records":[],"estimate_remaining":0},"backup_info":{"state":"RUNNING"}}"#, String::from_utf8(result.data).unwrap())
}

#[test]
fn test_export_tasks(){
  set_up();

  let result = record_export("tasks".to_string(), 0, 1000,0).expect("record not founded");

  assert_eq!("tasks", result.name);
  let tasks: Vec<Task> = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(2, tasks.len());
  assert_eq!(Principal::from_text(CANISTER_ID1.to_string()).unwrap(), tasks.get(0).unwrap().canister_id);
  assert_eq!(Principal::from_text(CANISTER_ID2.to_string()).unwrap(), tasks.get(1).unwrap().canister_id);
}

#[test]
fn test_export_tasks_with_last_update(){
  set_up();

  let result = record_export("tasks".to_string(), 0, 1000,time() + 1).expect("record not founded");

  assert_eq!("tasks", result.name);
  let tasks: Vec<Task> = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(0, tasks.len());
}