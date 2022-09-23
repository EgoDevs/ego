use candid::{candid_method};
use ic_cdk::{call, storage};
use ic_cdk_macros::*;
use ic_cron::implement_cron;
use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;
use ego_cron_mod::ego_cron::EgoCron;

use ego_cron_mod::task::Task;
use ego_cron_mod::service::{EgoCronService};
use ego_cron_mod::state::EGO_CRON;
use ego_cron_mod::types::{TaskMainAddRequest, TaskMainCancelRequest};
use ego_types::ego_error::EgoError;

use ego_users::inject_ego_users;

inject_ego_users!();

#[init]
#[candid_method(init)]
fn init() {
    ic_cdk::println!("ego-cron: init, caller is {}", caller());

    ic_cdk::println!("==> add caller as the owner");
    users_init();
}

#[derive(CandidType, Deserialize, Serialize)]
struct PersistState{
    pub ego_cron: EgoCron,
    pub user: User
}

#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("ego-cron: pre_upgrade");
    let ego_cron = EGO_CRON.with(|ego_cron| ego_cron.borrow().clone());
    let user = users_pre_upgrade();

    let state = PersistState{ego_cron, user};
    storage::stable_save((state, )).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("ego-cron: post_upgrade");
    let (state, ): (PersistState, ) = storage::stable_restore().unwrap();
    EGO_CRON.with(|ego_cron|
      *ego_cron.borrow_mut() = state.ego_cron
    );

    users_post_upgrade(state.user);
}

/********************   cron method   ********************/
#[update(name = "task_main_add", guard = "owner_guard")]
#[candid_method(update, rename = "task_main_add")]
fn task_main_add(req: TaskMainAddRequest) -> Result<(), EgoError> {
    ic_cdk::println!("ego-cron: task_main_add {} / {} / {:?}", req.canister_id, req.method, req.interval);

    EgoCronService::task_main_add(req.canister_id, req.method, req.interval)
}

#[update(name = "task_main_cancel", guard = "owner_guard")]
#[candid_method(update, rename = "task_main_cancel")]
fn task_main_cancel(req: TaskMainCancelRequest) -> Result<(), EgoError> {
    ic_cdk::println!("ego-cron: task_main_cancel {} / {}", req.canister_id, req.method);

    EgoCronService::task_main_cancel(req.canister_id, req.method)
}


/********************   heartbeat   ********************/
implement_cron!();

#[heartbeat]
async fn tick() {
    let ready_tasks = cron_ready_tasks();
    ic_cdk::println!("TICK {:?}", ready_tasks.len());

    for tasks in ready_tasks {

        let task = tasks
            .get_payload::<Task>()
            .expect("Unable to deserialize cron task kind");

        ic_cdk::println!("call task: {:?}", task);

        cron_call(task).await;
    }
}

pub async fn cron_call(task: Task) {
    ic_cdk::print("call canister");
    let cb = call(
        task.canister_id,
        task.method.as_str(),
        ()
    ).await as Result<((),), _>;
    match cb {
        Ok(_) => {},
        Err(e) => trap(format!("{:?}", e).as_str())
    }
}
