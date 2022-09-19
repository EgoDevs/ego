use candid::{candid_method};
use ic_cdk::{call, caller, trap};
use ic_cdk_macros::*;
use ic_cron::implement_cron;

use ego_cron_mod::task::CronTask;
use ego_cron_mod::service::{EgoCronService};
use ego_cron_mod::types::{TaskMainCancelResponse, TaskMainAddRequest, TaskMainAddResponse, TaskMainCancelRequest};
use ego_types::ego_error::EgoError;

#[init]
#[candid_method(init)]
fn init() {
    ic_cdk::println!("ego-cron: init, caller is {}", caller());
}

/********************   cron method   ********************/
#[update(name = "task_main_add")]
#[candid_method(update, rename = "task_main_add")]
fn task_main_add(req: TaskMainAddRequest) -> Result<TaskMainAddResponse, EgoError> {
    ic_cdk::println!("ego-cron: task_main_add");

    match EgoCronService::task_main_add(req.canister_id, req.method, req.interval){
        Ok(task_id) => Ok(TaskMainAddResponse{task_id}),
        Err(e) => Err(e)
    }
}

#[update(name = "task_main_cancel")]
#[candid_method(update, rename = "task_main_cancel")]
fn task_main_cancel(req: TaskMainCancelRequest) -> Result<TaskMainCancelResponse, EgoError> {
    ic_cdk::print(format!("ego-cron: task_main_cancel {}", req.task_id));

    match EgoCronService::task_main_cancel(req.task_id){
        Ok(ret) => Ok(TaskMainCancelResponse{ret}),
        Err(e) => Err(e)
    }
}


/********************   heartbeat   ********************/
implement_cron!();

#[heartbeat]
async fn tick() {
    // ic_cdk::print("TICK");
    for tasks in cron_ready_tasks() {
        let task = tasks
            .get_payload::<CronTask>()
            .expect("Unable to deserialize cron task kind");

        ic_cdk::print(format!("{:?}", task));
        cron_call(task).await;
    }
}

pub async fn cron_call(task: CronTask) {
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
