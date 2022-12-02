use ego_cron_mod::service::EgoCronService;
use ego_cron_mod::state::EGO_CRON;
use ego_cron_mod::task::Task;
use ic_cdk::export::Principal;

static EXISTS_CANISTER_ID: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";
static TEST_CANISTER_ID: &str = "224jh-lqaaa-aaaad-qaxda-cai";
static EXISTS_TASK_ID: u64 = 64;
static TEST_TASK_ID: u64 = 128;

pub fn set_up() {
    let canister_principal = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();
    let task_id = EXISTS_TASK_ID;

    EGO_CRON.with(|ego_corn| {
        ego_corn.borrow_mut().cron_tasks.insert(
            task_id,
            Task::new(canister_principal, "canister principal".to_string()),
        );
    });
}

#[test]
fn ego_corn_test_success() {
    set_up();
    let task_id = TEST_TASK_ID;
    let canister_id = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();

    let task = EGO_CRON.with(|ego_cron| {
        ego_cron
            .borrow()
            .cron_tasks
            .get(&EXISTS_TASK_ID)
            .unwrap()
            .clone()
    });

    let ret = EgoCronService::task_main_add(task_id, task);
    assert!(true);
    println!("{:?}", ret);

    let result = EgoCronService::task_main_get(canister_id, "canister principal".to_string());
    assert!(true);
    assert_eq!(EXISTS_TASK_ID, result.unwrap());
}

#[test]
fn task_main_get_test_fail_method_error() {
    set_up();
    let canister_id = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();

    // The task is None by method error
    let result = EgoCronService::task_main_get(canister_id, "canister".to_string());
    assert!(true);
    assert_eq!(None, result);
}

#[test]
fn task_main_get_task_is_none() {
    let canister_id = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();

    // The task is None
    let result = EgoCronService::task_main_get(canister_id, "canister principal".to_string());
    assert!(true);
    assert_eq!(None, result);
}

#[test]
fn task_main_get_task_by_canister_error() {
    set_up();
    let canister_id = Principal::from_text(TEST_CANISTER_ID.to_string()).unwrap();

    // The task is None by canister error
    let result = EgoCronService::task_main_get(canister_id, "canister principal".to_string());
    assert!(true);
    assert_eq!(None, result);
}

#[test]
fn task_main_get_success() {
    set_up();
    let canister_id = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();

    // task get success
    let result = EgoCronService::task_main_get(canister_id, "canister principal".to_string());
    assert!(true);
    assert_eq!(EXISTS_TASK_ID, result.unwrap());
}

#[test]
fn task_main_cancle_success() {
    set_up();
    let task_id = EXISTS_TASK_ID;
    let ret = EgoCronService::task_main_cancel(task_id);
    println!("{:?}", ret);

    let canister_id = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();

    // task get fail
    let result = EgoCronService::task_main_get(canister_id, "canister principal".to_string());
    assert!(true);
    assert_eq!(None, result);
}
