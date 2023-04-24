use std::ops::{Div, Mul};
use ic_cdk::export::Principal;

use ego_lib::ego_canister::TEgoCanister;
use ego_lib::ic_management::controllers_update;
use ego_types::app::{CanisterType, Wasm};
use ego_types::app::EgoError;
use ego_types::cycle_info::{CycleRecord, DEFAULT_ESTIMATE};

use crate::c2c::ego_file::TEgoFile;
use crate::c2c::ego_store::TEgoStore;
use crate::c2c::ic_management::TIcManagement;
use crate::state::{canister_get_one, EGO_TENANT, info_log_add};
use crate::task::Task;
use crate::types::EgoTenantErr;
use crate::types::EgoTenantErr::CycleNotEnough;

pub struct EgoTenantService {}

pub const NEXT_CHECK_DURATION: u64 = 60 * 60; // 1 hour
pub const CREATE_CANISTER_CYCLES_FEE: u128 = 100_000_000_000;
// pub const CREATE_CANISTER_CYCLES_FEE: u128 = 200_000_000_000;


impl EgoTenantService {
  pub fn canister_main_track(
    wallet_id: Principal,
    canister_id: Principal,
    next_check_time: u64
  ) -> Result<(), EgoError> {
    EGO_TENANT.with(|ego_tenant| {
      ego_tenant
        .borrow_mut()
        .canister_main_track(wallet_id, canister_id, next_check_time)
    })
  }

  pub fn canister_main_untrack(
    canister_id: Principal,
  ) -> Result<(), EgoError> {
    EGO_TENANT.with(|ego_tenant| {
      ego_tenant
        .borrow_mut()
        .canister_main_untrack(canister_id)
    })
  }

  pub async fn app_main_install<F: TEgoFile, M: TIcManagement, EC: TEgoCanister>(
    ego_tenant_id: Principal,
    ego_file: F,
    management: M,
    ego_canister: EC,
    wallet_id: Principal,
    user_id: Principal,
    wasm: Wasm,
  ) -> Result<Principal, EgoError> {
    let ego_store_id = canister_get_one("ego_store").unwrap();

    // TODO: handle frontend wasm
    if wasm.canister_type == CanisterType::ASSET {
      return Err(EgoTenantErr::SystemError("not implemented".to_string()).into());
    }

    let canister_id = management
      .canister_main_create(CREATE_CANISTER_CYCLES_FEE)
      .await?;
    info_log_add(format!("1 create canister {}", canister_id).as_str());

    info_log_add(format!("2 load wasm data {}", wasm.fid()).as_str());
    let data = ego_file
      .file_main_read(wasm.canister_id, wasm.fid())
      .await?;

    info_log_add("3 install code");
    management.canister_code_install(canister_id, data).await?;

    // add ego_store_id to app
    info_log_add("4 add [ego_store, ego_tenant] to canister");
    ego_canister.ego_canister_add(canister_id, "ego_store".to_string(), ego_store_id);
    ego_canister.ego_canister_add(canister_id, "ego_tenant".to_string(), ego_tenant_id);

    info_log_add("5 add [ego_store, ego_tenant] as ops_user");
    ego_canister.ego_op_add(canister_id, ego_store_id);
    ego_canister.ego_op_add(canister_id, ego_tenant_id);

    // ego_canister.ego_controller_set(canister_id, vec![wallet_id, user_id, canister_id]).await?;

    info_log_add(format!("6 change canister owner to [wallet: {}, user: {}, self: {}]", wallet_id, user_id, canister_id).as_str());
    ego_canister.ego_owner_set(canister_id, vec![wallet_id, user_id, canister_id]);

    info_log_add(format!("7 set canister controller to [wallet: {}, user: {}, self: {}]", wallet_id, user_id, canister_id).as_str());
    controllers_update(canister_id, vec![wallet_id, user_id, canister_id]).await?;


    Ok(canister_id)
  }

  pub async fn app_main_upgrade<F: TEgoFile, M: TIcManagement, EC: TEgoCanister>(
    ego_file: F,
    management: M,
    ego_canister: EC,
    canister_id: Principal,
    wasm: Wasm,
    tenant_id: Principal,
  ) -> Result<bool, EgoError> {
    // TODO: checked whether user has add tenant as one of the canister's controller

    if wasm.canister_type == CanisterType::ASSET {
      return Err(EgoTenantErr::SystemError("not implemented".to_string()).into());
    }

    info_log_add("1 load wasm data");
    let data = ego_file
      .file_main_read(wasm.canister_id, wasm.fid())
      .await?;

    info_log_add("2 install code");
    management.canister_code_upgrade(canister_id, data).await?;

    info_log_add("3 remove [ego_tenant] from canister controller");
    ego_canister.ego_controller_remove(canister_id, tenant_id);

    Ok(true)
  }

  pub async fn app_main_delete<M: TIcManagement>(
    management: M,
    canister_id: &Principal,
  ) -> Result<(), EgoError> {
    management.canister_main_delete(*canister_id).await
  }

  pub async fn ego_cycle_check_cb<M: TIcManagement, S: TEgoStore, EC: TEgoCanister>(
    management: M,
    ego_store: S,
    ego_canister: EC,
    task: &Task,
    canister_id: &Principal,
    records: &Vec<CycleRecord>,
    threshold: u128,
  ) -> Result<(), EgoError> {
    let mut current_cycle = records[0].balance;
    let current_ts: u64 = records[0].ts;  // second

    let mut last_cycle = 0;
    let mut last_ts: u64 = 0;

    if records.len() > 1 {
      last_cycle = records[1].balance;
      last_ts = records[1].ts; // second
    }

    let delta_cycle:i128 = last_cycle as i128 - current_cycle as i128;
    let delta_time = current_ts - last_ts; // in second

    let mut estimate_duration = DEFAULT_ESTIMATE;

    info_log_add(format!("1. compare current_cycle: {} and threshold: {}", current_cycle, threshold).as_str());
    if current_cycle < threshold {
      let cycle_required_to_top_up = threshold.mul(15).div(10) - current_cycle;

      info_log_add(format!("1.1. cycle_required_to_top_up: {}", cycle_required_to_top_up).as_str());
      EgoTenantService::wallet_cycle_recharge(management, ego_store, task, cycle_required_to_top_up).await?;

      current_cycle = threshold.mul(15).div(10);
    } else {
      info_log_add("1.2. cycle enough");
    }

    info_log_add(format!("2. delta cycle {}, delta time {}", delta_cycle, delta_time).as_str());

    if last_cycle == 0 {
      // for the first time checking, we will check it again after 30 minutes
      info_log_add("2.1 last cycle is 0. skip estimation");
    } else if delta_time == 0 {
      info_log_add("2.2 delta_time is zero. skip estimation");
    } else if delta_cycle < 0 {
      info_log_add("2.3. more cycle then before. use default estimation");
    } else {
      let cycle_consume_per_second = (delta_cycle / (delta_time as i128)) as u128;
      info_log_add(format!("3. cycle_consume_per_second: {}", cycle_consume_per_second).as_str());

      if cycle_consume_per_second == 0 {
        info_log_add("3.1. cycle_consume_per_second is zero. use default estimation");
      } else {
        // the remain cycles can be used in estimate_duration second
        estimate_duration = (current_cycle / cycle_consume_per_second) as u64;
      }
    }

    info_log_add(format!("4. estimate_duration: {}", estimate_duration).as_str());
    ego_canister.ego_cycle_estimate_set(*canister_id, estimate_duration);

    let next_time = current_ts + NEXT_CHECK_DURATION;
    EGO_TENANT.with(|ego_tenant| {
      ego_tenant
        .borrow_mut()
        .task_update(task.canister_id, next_time)
    });

    Ok(())
  }

  pub async fn wallet_cycle_recharge<M: TIcManagement, S: TEgoStore>(
    management: M,
    ego_store: S,
    task: &Task,
    cycles: u128,
  ) -> Result<(), EgoError> {
    let charge_ret = ego_store
      .wallet_cycle_charge(
        task.wallet_id,
        cycles,
        format!(
          "wallet cycle charge, top up canister id {}",
          task.canister_id
        ),
      ).await?;

    if charge_ret {
      management.canister_cycle_top_up(task.canister_id, cycles).await?;
      Ok(())
    } else {
      Err(CycleNotEnough.into())
    }
  }
}
