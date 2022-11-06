use candid::candid_method;
use ego_cron_mod::ego_cron::EgoCron;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::{notify, storage};
use ic_cdk_macros::*;
use ic_cron::implement_cron;
use ic_cron::types::{Iterations, SchedulingOptions};
use serde::Serialize;

use ego_cron_mod::service::EgoCronService;
use ego_cron_mod::state::EGO_CRON;
use ego_cron_mod::task::Task;
use ego_cron_mod::types::{cron_interval, TaskMainAddRequest, TaskMainCancelRequest};
use ego_types::ego_error::EgoError;

use ego_macros::inject_balance_get;
use ego_macros::inject_ego_log;
use ego_users::inject_ego_users;
use ego_registry::inject_ego_registry;

inject_balance_get!();
inject_ego_users!();
inject_ego_registry!();
inject_ego_log!();

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct InitArg {
    init_caller: Option<Principal>,
}

#[init]
#[candid_method(init)]
pub fn init(arg: InitArg) {
    let caller = arg.init_caller.unwrap_or(caller());
    ic_cdk::println!("ego-cron: init, caller is {}", caller.clone());

    ic_cdk::println!("==> add caller as the owner");
    users_init(caller.clone());
}

#[derive(CandidType, Deserialize, Serialize)]
struct PersistState {
    pub ego_cron: EgoCron,
    pub user: User,
    pub registry: Registry
}

#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("ego-cron: pre_upgrade");
    let ego_cron = EGO_CRON.with(|ego_cron| ego_cron.borrow().clone());
    let user = users_pre_upgrade();
    let registry = registry_pre_upgrade();

    let state = PersistState { ego_cron, user, registry };
    storage::stable_save((state,)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("ego-cron: post_upgrade");
    let (state,): (PersistState,) = storage::stable_restore().unwrap();
    EGO_CRON.with(|ego_cron| *ego_cron.borrow_mut() = state.ego_cron);

    users_post_upgrade(state.user);
    registry_post_upgrade(state.registry);
}

/********************  methods for ego_registry   ********************/
fn on_canister_added(name: &str, canister_id: Principal) {
    let _ = match name {
        "ego_ledger" => {
            role_user_add(canister_id).unwrap();
        },
        "ego_tenant" => {
            role_user_add(canister_id).unwrap();
        },
        _ => {}
    };
}

/********************   cron method   ********************/

#[derive(CandidType, Deserialize)]
enum TaskKind {
    DoSomethingElse,
}

#[update(name = "task_main_add", guard = "user_guard")]
#[candid_method(update, rename = "task_main_add")]
fn task_main_add(req: TaskMainAddRequest) -> Result<(), EgoError> {
    let canister_id = caller();
    ego_log(&format!(
        "ego-cron: task_main_add {} / {} / {:?}",
        canister_id,
        req.method,
        req.interval)
    );

    match EgoCronService::task_main_get(canister_id, req.method.clone()){
        None => {
            let duration_nano = cron_interval(req.interval);
            let task = Task::new(canister_id, req.method);
             match cron_enqueue(
                task.clone(),
                SchedulingOptions {
                    delay_nano: 0,
                    interval_nano: duration_nano,
                    iterations: Iterations::Infinite,
                },
            ) {
                 Ok(task_id) => {
                     EgoCronService::task_main_add(task_id, task);
                 }
                 Err(_) => {}
             };
        }
        Some(_) => {}
    }

    Ok(())
}

#[update(name = "task_main_cancel", guard = "user_guard")]
#[candid_method(update, rename = "task_main_cancel")]
fn task_main_cancel(req: TaskMainCancelRequest) -> Result<(), EgoError> {
    let canister_id = caller();

    ego_log(&format!(
        "ego-cron: task_main_cancel {} / {}",
        canister_id,
        req.method)
    );

    match EgoCronService::task_main_get(canister_id, req.method.clone()){
        None => {}
        Some(task_id) => {
            EgoCronService::task_main_cancel(task_id);
            cron_dequeue(task_id);
        }
    }

    Ok(())
}

#[update(name = "task_main_check", guard = "owner_guard")]
#[candid_method(update, rename = "task_main_check")]
fn task_main_check() -> Result<(), EgoError> {
    let ready_tasks = cron_ready_tasks();

    for tasks in ready_tasks {
        let task = tasks
          .get_payload::<Task>()
          .expect("Unable to deserialize cron task kind");

        ego_log(&format!("ego-cron: notify task: {:?}", task));
        let _result = notify(task.canister_id, task.method.as_str(), ());
    }

    Ok(())
}

/********************   heartbeat   ********************/
implement_cron!();

// remove heartbeat to save cycle
// #[heartbeat]
async fn tick() {
    let _result = task_main_check();
}
