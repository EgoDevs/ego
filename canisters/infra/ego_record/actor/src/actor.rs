use std::collections::BTreeMap;
use ic_cdk::export::candid::{CandidType, Deserialize};
use candid::candid_method;
use ic_cdk::{caller, storage};
use ic_cdk::api::time;
use ic_cdk::export::Principal;
use ic_cdk_macros::*;
use ego_record_mod::record::{EgoRecord, Record};
use ego_record_mod::service::RecordService;
use ego_record_mod::state::*;
use ego_types::registry::Registry;
use ego_types::user::User;
use serde::Serialize;
use ego_macros::{inject_cycle_info_api, inject_ego_api};

inject_ego_api!();
inject_cycle_info_api!();

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct InitArg {
    init_caller: Option<Principal>,
}

#[init]
#[candid_method(init)]
pub fn init(arg: InitArg) {
    let caller = arg.init_caller.unwrap_or(caller());
    info_log_add(format!("ego_record: init, caller is {}", caller.clone()).as_str());

    info_log_add("==> add caller as the owner");
    owner_add(caller.clone());
}

#[derive(CandidType, Deserialize, Serialize)]
struct PersistState {
    ego_record: EgoRecord,
    users: Option<User>,
    registry: Option<Registry>,
    cycle_info: Option<CycleInfo>,
}

#[pre_upgrade]
fn pre_upgrade() {
    info_log_add("ego_record: pre_upgrade");

    let ego_record = EGO_RECORD.with(|ego_record| ego_record.borrow().clone());

    let state = PersistState {
        ego_record,
        users: Some(users_pre_upgrade()),
        registry: Some(registry_pre_upgrade()),
        cycle_info: Some(cycle_info_pre_upgrade()),
    };
    storage::stable_save((state, )).unwrap();
}


#[post_upgrade]
fn post_upgrade() {
    info_log_add("ego_record: post_upgrade");

    let (state, ): (PersistState, ) = storage::stable_restore().unwrap();
    EGO_RECORD.with(|ego_record| *ego_record.borrow_mut() = state.ego_record);

    match state.users {
        None => {}
        Some(users) => {
            users_post_upgrade(users);
        }
    }

    match state.registry {
        None => {}
        Some(registry) => {
            registry_post_upgrade(registry);
        }
    }

    match state.cycle_info {
        None => {}
        Some(cycle_info) => {
            cycle_info_post_upgrade(cycle_info);
        }
    }
}

#[update(name = "record_add", guard = "user_guard")]
#[candid_method(update, rename = "record_add")]
fn record_add(scope: String, event: String, message: String, created_at: Option<u64>) {
    info_log_add("ego_record: record_add");
    RecordService::record_add(scope.as_str(), event.as_str(), message.as_str(), created_at.unwrap_or(time()));
}

#[query(name = "record_amount", guard = "user_guard")]
#[candid_method(update, rename = "record_amount")]
fn record_amount() -> usize {
    info_log_add("ego_record: record_amount");
    RecordService::record_amount()
}

#[query(name = "record_list", guard = "user_guard")]
#[candid_method(update, rename = "record_list")]
fn record_list(amount: usize) -> Vec<Record> {
    info_log_add(format!("ego_record: record_list {}", amount).as_str());
    RecordService::record_list(amount)
}

#[update(name = "record_retain", guard = "user_guard")]
#[candid_method(update, rename = "record_retain")]
fn record_retain(amount: usize) {
    info_log_add(format!("ego_record: record_retain {}", amount).as_str());
    RecordService::record_retain(amount);
}

#[update(name = "record_retain_after", guard = "user_guard")]
#[candid_method(update, rename = "record_retain_after")]
fn record_retain_after(end_time: u64) {
    info_log_add(format!("ego_record: record_retain_after {}", end_time).as_str());
    RecordService::record_retain_after(end_time);
}


/********************  methods for ego_cycle_threshold_get   ********************/
pub fn cycle_threshold_get() -> u128 {
    1_000_000_000_000
}

pub fn runtime_cycle_threshold_get() -> u128 {
    1_000_000_000_000
}