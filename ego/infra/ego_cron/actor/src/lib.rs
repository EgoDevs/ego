use candid::{candid_method};
use ic_cdk::{notify, storage};
use ic_cdk::api::time;
use ic_cdk_macros::*;
use ic_cron::implement_cron;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cron::types::{Iterations, SchedulingOptions};
use serde::Serialize;
use ego_cron_mod::ego_cron::EgoCron;

use ego_cron_mod::task::Task;
use ego_cron_mod::service::{EgoCronService};
use ego_cron_mod::state::EGO_CRON;
use ego_cron_mod::types::{cron_interval, TaskMainAddRequest, TaskMainCancelRequest};
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

#[derive(CandidType, Deserialize)]
enum TaskKind {
    DoSomethingElse,
}

#[update(name = "task_main_add")]
#[candid_method(update, rename = "task_main_add")]
fn task_main_add(req: TaskMainAddRequest) -> Result<(), EgoError> {
    ic_cdk::println!("ego-cron: task_main_add {} / {} / {:?}", req.canister_id, req.method, req.interval);

    match EgoCronService::task_main_add(req.canister_id, req.method) {
        Some(task) => {
            let duration_nano = cron_interval(req.interval);
            let _res = cron_enqueue(
                task.clone(),
                SchedulingOptions {
                    delay_nano: 0,
                    interval_nano: duration_nano,
                    iterations: Iterations::Infinite,
                },
            );
        },
        _ => {}
    };

    Ok(())
}

#[update(name = "task_main_cancel")]
#[candid_method(update, rename = "task_main_cancel")]
fn task_main_cancel(req: TaskMainCancelRequest) -> Result<(), EgoError> {
    ic_cdk::println!("ego-cron: task_main_cancel {} / {}", req.canister_id, req.method);

    match EgoCronService::task_main_cancel(req.canister_id, req.method) {
        Some(task_id) => {
            cron_dequeue(task_id);
        },
        _ => {}
    };

    Ok(())
}


/********************   heartbeat   ********************/
implement_cron!();

#[heartbeat]
async fn tick() {
    let ready_tasks = cron_ready_tasks();
    ic_cdk::println!("TICK {} / {:?}", time(), ready_tasks.len());

    for tasks in ready_tasks {

        let task = tasks
            .get_payload::<Task>()
            .expect("Unable to deserialize cron task kind");



        cron_call(task).await;
    }
}

pub async fn cron_call(task: Task) {
    ic_cdk::println!("ego-cron: notify task: {:?}", task);
    let notify_result = notify(
        task.canister_id,
        task.method.as_str(),
        (),
    );

    match notify_result {
        Err(code) => {
            let code = code as u16;
            Err(EgoError { code, msg: "cron_call failed".to_string() })
        },
        _ => Ok(true)
    }.expect("notify success");
}
