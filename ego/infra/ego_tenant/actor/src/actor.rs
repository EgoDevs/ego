use std::collections::BTreeMap;
use std::ops::Div;
use std::time::Duration;

use candid::candid_method;
use ic_cdk::{caller, id, storage};
use ic_cdk::api::time;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_cdk::timer::set_timer_interval;
use ic_cdk_macros::*;
use serde::Serialize;

use ego_lib::ego_canister::{EgoCanister, TEgoCanister};
use ego_macros::inject_ego_api;
use ego_tenant_mod::c2c::ego_file::EgoFile;
use ego_tenant_mod::c2c::ego_store::EgoStore;
use ego_tenant_mod::c2c::ic_management::IcManagement;
use ego_tenant_mod::service::EgoTenantService;
use ego_tenant_mod::state::*;
use ego_tenant_mod::state::EGO_TENANT;
use ego_tenant_mod::tenant::Tenant;
use ego_tenant_mod::types::{
  AppMainInstallRequest, AppMainUpgradeRequest,
};
use ego_inner_rpc::ego_record::{EgoEvent, EgoRecord, TEgoRecord};
use ego_inner_rpc::types::{GeneralEnumResponse, SnapshotCycleBalanceRecord};

use ego_tenant_mod::types::EgoTenantErr::CanisterNotFounded;
use ego_types::app::EgoError;
use ego_types::cycle_info::CycleRecord;
use ego_types::registry::Registry;
use ego_types::user::User;

inject_ego_api!();

pub const CHECK_DURATION: u64 = 60 * 2; // 2 minutes

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct InitArg {
  init_caller: Option<Principal>,
}

#[init]
#[candid_method(init)]
pub fn init(arg: InitArg) {
  let caller = arg.init_caller.unwrap_or(caller());
  info_log_add(format!("ego-tenant: init, caller is {}", caller.clone()).as_str());

  info_log_add("==> add caller as the owner");
  owner_add(caller.clone());

  let duration = Duration::new(CHECK_DURATION, 0);
  set_timer_interval(duration, || {
    task_run();
  });
}

#[derive(CandidType, Deserialize, Serialize)]
struct PersistState {
  ego_tenant: Tenant,
  users: Option<User>,
  registry: Option<Registry>,
}

#[pre_upgrade]
fn pre_upgrade() {
  info_log_add("ego-tenant: pre_upgrade");
  let ego_tenant = EGO_TENANT.with(|ego_tenant| ego_tenant.borrow().clone());

  let state = PersistState {
    ego_tenant,
    users: Some(users_pre_upgrade()),
    registry: Some(registry_pre_upgrade()),
  };
  storage::stable_save((state, )).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
  info_log_add("ego-tenant: post_upgrade");
  let (state, ): (PersistState, ) = storage::stable_restore().unwrap();
  EGO_TENANT.with(|ego_tenant| *ego_tenant.borrow_mut() = state.ego_tenant);

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

  let duration = Duration::new(CHECK_DURATION, 0);
  set_timer_interval(duration, || {
    task_run();
  });
}


/********************  methods for ego_store   ********************/
#[update(name = "app_main_install", guard = "user_guard")]
#[candid_method(update, rename = "app_main_install")]
async fn app_main_install(req: AppMainInstallRequest) -> Result<Principal, EgoError> {
  info_log_add("ego_tenant: app_main_install");

  let ego_tenant_id = id();
  let management = IcManagement::new();
  let ego_file = EgoFile::new();
  let ego_canister = EgoCanister::new();

  let canister_id = EgoTenantService::app_main_install(
    ego_tenant_id,
    ego_file,
    management,
    ego_canister,
    req.wallet_id,
    req.user_id,
    req.wasm,
  )
    .await?;
  Ok(canister_id)
}

#[update(name = "app_main_upgrade", guard = "user_guard")]
#[candid_method(update, rename = "app_main_upgrade")]
async fn app_main_upgrade(req: AppMainUpgradeRequest) -> Result<bool, EgoError> {
  info_log_add("ego_tenant: app_main_upgrade");
  let management = IcManagement::new();
  let ego_file = EgoFile::new();

  let ego_canister = EgoCanister::new();

  let ret =
    EgoTenantService::app_main_upgrade(ego_file, management, ego_canister, req.canister_id, req.wasm, id()).await?;
  Ok(ret)
}

#[update(name = "app_main_delete", guard = "user_guard")]
#[candid_method(update, rename = "app_main_delete")]
async fn app_main_delete(canister_id: Principal) -> Result<(), EgoError> {
  info_log_add("ego_tenant: app_main_upgrade");
  let management = IcManagement::new();

  EgoTenantService::app_main_delete(management, &canister_id).await
}

#[update(name = "canister_main_track", guard = "user_guard")]
#[candid_method(update, rename = "canister_main_track")]
fn canister_main_track(wallet_id: Principal, canister_id: Principal) -> Result<(), EgoError> {
  info_log_add("ego_tenant: canister_main_track");

  let next_check_time = time().div(1e9 as u64) + 60;  // convert to second

  EgoTenantService::canister_main_track(wallet_id, canister_id, next_check_time)?;
  Ok(())
}

#[update(name = "canister_main_untrack", guard = "user_guard")]
#[candid_method(update, rename = "canister_main_untrack")]
fn canister_main_untrack(canister_id: Principal) -> Result<(), EgoError> {
  info_log_add("ego_tenant: canister_main_untrack");

  EgoTenantService::canister_main_untrack(canister_id)?;
  Ok(())
}

#[update(name = "ego_cycle_check_cb")]
#[candid_method(update, rename = "ego_cycle_check_cb")]
async fn ego_cycle_check_cb(records: Vec<CycleRecord>, threshold: u128) -> Result<(), EgoError> {
  let canister_id = caller();
  info_log_add(format!("ego_tenant: ego_cycle_check_cb, canister_id: {}", canister_id).as_str());

  let management = IcManagement::new();

  let ego_store_id = canister_get_one("ego_store").unwrap();
  let ego_store = EgoStore::new(ego_store_id);

  let ego_canister = EgoCanister::new();

  info_log_add("1. get task by canister_id");
  let task = EGO_TENANT.with(|ego_tenant| {
    match ego_tenant.borrow().tasks.get(&canister_id) {
      None => {
        info_log_add("ego_tenant error, can not find task");
        Err(EgoError::from(CanisterNotFounded))
      }
      Some(task) => {
        Ok(task.clone())
      }
    }
  })?;

  EgoTenantService::ego_cycle_check_cb(management, ego_store, ego_canister, &task, &canister_id, &records, threshold).await?;

  let ego_record_id = canister_get_one("ego_record").unwrap();
  let ego_record = EgoRecord::new(ego_record_id);
  ego_record.record_add(EgoEvent::SnapshotCycleBalance(SnapshotCycleBalanceRecord {
    cycles: records[0].balance,
    ts: records[0].ts,
    canister_id,
    wallet_id: task.wallet_id,
    response: GeneralEnumResponse::Success,
  }));
  Ok(())
}

#[update(name = "wallet_cycle_recharge")]
#[candid_method(update, rename = "wallet_cycle_recharge")]
async fn wallet_cycle_recharge(cycles: u128) -> Result<(), EgoError> {
  let canister_id = caller();
  info_log_add(format!("ego_tenant: wallet_cycle_recharge, canister_id: {}", canister_id).as_str());

  let management = IcManagement::new();

  let ego_store_id = canister_get_one("ego_store").unwrap();
  let ego_store = EgoStore::new(ego_store_id);

  info_log_add("1. get task by canister_id");
  let task = EGO_TENANT.with(|ego_tenant| {
    match ego_tenant.borrow().tasks.get(&canister_id) {
      None => {
        info_log_add("ego_tenant error, can not find task");
        Err(EgoError::from(CanisterNotFounded))
      }
      Some(task) => {
        Ok(task.clone())
      }
    }
  })?;

  EgoTenantService::wallet_cycle_recharge(management, ego_store, &task, cycles).await?;
  Ok(())
}

/********************  notify  ********************/
fn task_run() {
  info_log_add("ego-tenant: task_run");

  let sentinel = time().div(1e9 as u64);  // convert to second
  let tasks = EGO_TENANT.with(|ego_tenant| ego_tenant.borrow_mut().tasks_get(sentinel));

  for task in tasks {
    let ego_canister = EgoCanister::new();

    ego_canister.ego_cycle_check(task.canister_id);
  }
}
