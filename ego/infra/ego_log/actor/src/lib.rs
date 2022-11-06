use candid::candid_method;
use ic_cdk::api::time;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::storage;
use serde::Serialize;

use ego_log_mod::ego_log::EgoLog;
use ego_log_mod::log::Log;
use ego_log_mod::state::EGO_LOG;
use ic_cdk_macros::*;

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
    ic_cdk::println!("ego-log: init, caller is {}", caller.clone());

    ic_cdk::println!("==> add caller as the owner");
    users_init(caller.clone());
}

#[derive(CandidType, Deserialize, Serialize)]
struct PersistState {
    pub ego_log: EgoLog,
    pub user: User,
    pub registry: Registry
}

#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("ego-log: pre_upgrade");
    let ego_log = EGO_LOG.with(|ego_log| ego_log.borrow().clone());
    let user = users_pre_upgrade();
    let registry = registry_pre_upgrade();

    let state = PersistState { ego_log, user, registry };
    storage::stable_save((state,)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("ego-log: post_upgrade");
    let (state,): (PersistState,) = storage::stable_restore().unwrap();
    EGO_LOG.with(|ego_log| *ego_log.borrow_mut() = state.ego_log);

    users_post_upgrade(state.user);
    registry_post_upgrade(state.registry);
}

/********************  methods for ego_registry   ********************/
fn on_canister_added(name: &str, canister_id: Principal) {
    let _ = match name {
        _ => role_user_add(canister_id).unwrap()
    };
}


/********************  user  ********************/
#[update(name = "canister_log_add", guard = "user_guard")]
#[candid_method(update, rename = "canister_log_add")]
fn canister_log_add(message: String) {
    let ts = time();
    ic_cdk::println!("ego-log: canister_log_add, message: {}, ts: {}", message, ts);
    EGO_LOG.with(|ego_log| {
        ego_log
            .borrow_mut()
            .canister_log_add(caller(), ts, message)
    });
}

#[query(name = "canister_log_get", guard = "user_guard")]
#[candid_method(query, rename = "canister_log_get")]
fn canister_log_get(from_ts: u64, to_ts: u64) -> Vec<Log> {
    ic_cdk::println!("ego-log: canister_log_get between {} and {}", from_ts, to_ts);
    EGO_LOG.with(|ego_log| ego_log.borrow().canister_log_get(from_ts, to_ts))
}

#[query(name = "canister_log_clear", guard = "owner_guard")]
#[candid_method(query, rename = "canister_log_clear")]
fn canister_log_clear()  {
    ic_cdk::println!("ego-log: canister_log_clear");
    EGO_LOG.with(|ego_log| ego_log.borrow_mut().logs.clear());
}

/********************  notify  ********************/
#[update(name = "message_main_notify", guard = "user_guard")]
#[candid_method(update, rename = "message_main_notify")]
async fn message_main_notify() {
    ic_cdk::println!("ego-log: message_main_notify");
}
