use std::collections::BTreeMap;
use std::time::Duration;

use candid::candid_method;
use candid::Principal;
use ego_backup::inject_backup_api;
use ic_cdk::{caller, id};
use ic_cdk_macros::*;

use ego_lib::ego_canister::{EgoCanister, TEgoCanister};
use ego_macros::{inject_cycle_info_api, inject_ego_api};
use ego_tenant_mod::backup::*;
use ego_tenant_mod::c2c::ego_file::EgoFile;
use ego_tenant_mod::c2c::ego_store::EgoStore;
use ego_tenant_mod::c2c::ic_management::IcManagement;
use ego_tenant_mod::service::{EgoTenantService, NEXT_CHECK_DURATION};
use ego_tenant_mod::state::*;
use ego_tenant_mod::types::{AppMainInstallRequest, AppMainReInstallRequest, AppMainUpgradeRequest, DataExport, task};
use ego_tenant_mod::types::EgoTenantErr::CanisterNotFounded;
use ego_tenant_mod::types::stable_state::StableState;
use ego_tenant_mod::types::task::{MAX_TRY_COUNT, Task};
use ego_types::app::EgoError;
use ego_utils::util::time;

inject_ego_api!();
inject_cycle_info_api!();
inject_backup_api!();

pub const CHECK_DURATION: u64 = 600; // 每 10 分钟，检查有没有需要检查的Canister


#[init]
#[candid_method(init)]
fn init() {
  let caller = caller();
  info_log_add(format!("ego_tenant: init, caller is {}", caller.clone()).as_str());

  info_log_add("==> add caller as the owner");
  owner_add(caller.clone());

  let duration = Duration::from_secs(CHECK_DURATION);
  ic_cdk_timers::set_timer_interval(duration, task_run);
}

#[pre_upgrade]
fn pre_upgrade() {
  info_log_add("pre_upgrade");

  ego_tenant_mod::state::pre_upgrade();
}

#[post_upgrade]
fn post_upgrade() {
  info_log_add("post_upgrade");
  ego_tenant_mod::state::post_upgrade();

  let duration = Duration::from_secs(CHECK_DURATION);
  ic_cdk_timers::set_timer_interval(duration, task_run);
}

/********************  methods for ego_store   ********************/
#[update(name = "app_main_install", guard = "user_guard")]
#[candid_method(update, rename = "app_main_install")]
async fn app_main_install(req: AppMainInstallRequest) -> Result<Principal, EgoError> {
  info_log_add("app_main_install");

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
  info_log_add("app_main_upgrade");
  let management = IcManagement::new();
  let ego_file = EgoFile::new();

  let ego_canister = EgoCanister::new();

  let ret = EgoTenantService::app_main_upgrade(
    ego_file,
    management,
    ego_canister,
    req.canister_id,
    req.wasm,
    id(),
  )
    .await?;
  Ok(ret)
}

#[update(name = "app_main_reinstall", guard = "user_guard")]
#[candid_method(update, rename = "app_main_reinstall")]
async fn app_main_reinstall(req: AppMainReInstallRequest) -> Result<bool, EgoError> {
  info_log_add("app_main_reinstall");
  let management = IcManagement::new();
  let ego_file = EgoFile::new();

  let ego_canister = EgoCanister::new();

  let ret = EgoTenantService::app_main_reinstall(
    ego_file,
    management,
    ego_canister,
    req.canister_id,
    req.wasm,
    id(),
  )
    .await?;
  Ok(ret)
}

#[update(name = "app_main_delete", guard = "user_guard")]
#[candid_method(update, rename = "app_main_delete")]
async fn app_main_delete(canister_id: Principal) -> Result<(), EgoError> {
  info_log_add("app_main_delete");
  let management = IcManagement::new();

  EgoTenantService::app_main_delete(management, &canister_id).await
}

#[update(name = "canister_main_track", guard = "user_guard")]
#[candid_method(update, rename = "canister_main_track")]
fn canister_main_track(canister_id: Principal) -> Result<(), EgoError> {
  info_log_add("canister_main_track");

  let next_check_time = time() / 1000 + NEXT_CHECK_DURATION; // next_check_time

  EgoTenantService::canister_main_track(&canister_id, next_check_time);
  Ok(())
}

#[update(name = "canister_main_untrack", guard = "user_guard")]
#[candid_method(update, rename = "canister_main_untrack")]
fn canister_main_untrack(canister_id: Principal) -> Result<(), EgoError> {
  info_log_add("canister_main_untrack");

  EgoTenantService::canister_main_untrack(&canister_id);
  Ok(())
}

#[update(name = "ego_cycle_check_cb")]
#[candid_method(update, rename = "ego_cycle_check_cb")]
async fn ego_cycle_check_cb(records: Vec<CycleRecord>, threshold: u128) -> Result<(), EgoError> {
  let canister_id = caller();
  info_log_add(
    format!(
      "ego_cycle_check_cb, canister_id: {}",
      canister_id
    )
      .as_str(),
  );

  let management = IcManagement::new();

  let ego_store_id = canister_get_one("ego_store").unwrap();
  let ego_store = EgoStore::new(ego_store_id);

  let ego_canister = EgoCanister::new();

  info_log_add("1. get task by canister_id");
  let mut task = match Task::get(&canister_id) {
    None => {
      info_log_add("ego_tenant error, can not find task");
      Err(EgoError::from(CanisterNotFounded))
    }
    Some(task) => Ok(task.clone()),
  }?;

  EgoTenantService::ego_cycle_check_cb(
    management,
    ego_store,
    ego_canister,
    &mut task,
    &canister_id,
    &records,
    threshold,
  )
    .await?;
  Ok(())
}

#[update(name = "wallet_cycle_recharge")]
#[candid_method(update, rename = "wallet_cycle_recharge")]
async fn wallet_cycle_recharge(cycles: u128) -> Result<(), EgoError> {
  let canister_id = caller();
  info_log_add(
    format!(
      "wallet_cycle_recharge, canister_id: {}",
      canister_id
    )
      .as_str(),
  );

  let management = IcManagement::new();

  let ego_store_id = canister_get_one("ego_store").unwrap();
  let ego_store = EgoStore::new(ego_store_id);

  info_log_add("1. get task by canister_id");
  let task = match Task::get(&canister_id) {
    None => {
      info_log_add("ego_tenant error, can not find task");
      Err(EgoError::from(CanisterNotFounded))
    }
    Some(task) => Ok(task.clone()),
  }?;

  EgoTenantService::wallet_cycle_recharge(management, ego_store, &task, cycles).await?;
  Ok(())
}

/********************  methods for astro_deployer   ********************/
#[update(name = "admin_task_list", guard = "owner_guard")]
#[candid_method(update, rename = "admin_task_list")]
pub fn admin_task_list(last_update: u64) -> Result<Vec<Task>, EgoError> {
  info_log_add("canister_task_list");

  Ok(Task::by_last_update(0, Task::len() as usize, last_update))
}

#[update(name = "reset_next_check_time", guard = "owner_guard")]
#[candid_method(update, rename = "reset_next_check_time")]
pub fn reset_next_check_time() {
  info_log_add("reset_next_check_time");

  let now = time() as i64;

  for task in Task::list(0, Task::len() as usize).iter_mut() {
    if (now - task.next_check_time as i64).abs() > NEXT_CHECK_DURATION as i64 {
      task.next_check_time = 0;
      task.try_count = MAX_TRY_COUNT - 1;
      task.save()
    }
  }
}

#[update(name = "admin_task_check", guard = "owner_guard")]
#[candid_method(update, rename = "admin_task_check")]
pub fn admin_task_check(canister_id: Principal) {
  let ego_canister = EgoCanister::new();
  info_log_add(format!("calling ego_cycle_check on {}", canister_id).as_str());

  ego_canister.ego_cycle_check(canister_id);
}

/********************  notify  ********************/
fn task_run() {
  info_log_add("task_run");

  let sentinel = time(); // convert to second
  let tasks = Task::by_sentinel(sentinel);

  for mut task in tasks {
    let ego_canister = EgoCanister::new();
    info_log_add(format!("calling ego_cycle_check on {}", task.canister_id).as_str());

    ego_canister.ego_cycle_check(task.canister_id);
    task.try_count += 1;
    task.save();
  }
}

/********************  methods for migrate   ********************/
/********************  数据导出   ********************/
#[update(name = "admin_export", guard = "owner_guard")]
#[candid_method(update, rename = "admin_export")]
pub fn admin_export() -> Vec<u8> {
  info_log_add("admin_export");

  let state = StableState::load();

  let data_export = DataExport {
    state,
    tasks: task::Task::list(0, Task::len() as usize),
  };

  serde_json::to_vec(&data_export).unwrap()
}

#[update(name = "admin_import", guard = "owner_guard")]
#[candid_method(update, rename = "admin_import")]
pub fn admin_import(tasks: Vec<Task>) {
  info_log_add("admin_task_add");

  tasks.iter().for_each(|task| {
    info_log_add(format!("task added canister_id:{}", task.canister_id).as_str());
    let mut t = Task::new(&task.canister_id, task.next_check_time, None);
    t.save();
  });
}

#[update(name = "delegate_controller_add", guard = "owner_guard")]
#[candid_method(update, rename = "delegate_controller_add")]
pub async fn delegate_controller_add(target_canister: Principal, principal: Principal) -> Result<String, String> {
  info_log_add(format!("delegate_controller_add. target_canister: {:?}, principal: {:?}", target_canister, principal).as_str());

  match ego_lib::ic_management::controller_add(target_canister.clone(), principal.clone()).await {
    Ok(_) => { Ok("Success".to_owned()) }
    Err(error) => {
      Err(error.msg)
    }
  }
}

#[update(name = "delegate_controller_remove", guard = "owner_guard")]
#[candid_method(update, rename = "delegate_controller_remove")]
pub async fn delegate_controller_remove(target_canister: Principal, principal: Principal) -> Result<String, String> {
  info_log_add(format!("delegate_controller_remove. target_canister: {:?}, principal: {:?}", target_canister, principal).as_str());

  match ego_lib::ic_management::controller_remove(target_canister.clone(), principal.clone()).await {
    Ok(_) => { Ok("Success".to_owned()) }
    Err(error) => {
      Err(error.msg)
    }
  }
}

/********************  methods for ego_cycle_threshold_get   ********************/
pub fn cycle_threshold_get() -> u128 {
  10_000_000_000_000
}

pub fn runtime_cycle_threshold_get() -> u128 {
  10_000_000_000_000
}