use ic_cdk::export::Principal;

use ego_lib::ego_canister::TEgoCanister;
use ego_types::app::{CanisterType, Wasm};
use ego_types::app::EgoError;
use ego_types::cycle_info::CycleRecord;

use crate::c2c::ego_file::TEgoFile;
use crate::c2c::ego_store::TEgoStore;
use crate::c2c::ic_management::TIcManagement;
use crate::state::{canister_get_one, EGO_TENANT, info_log_add};
use crate::task::Task;
use crate::types::EgoTenantErr;
use crate::types::EgoTenantErr::CycleNotEnough;

pub struct EgoTenantService {}

pub const NEXT_CHECK_DURATION: u64 = 60 * 5;
pub const CREATE_CANISTER_CYCLES_FEE: u128 = 100_000_000_000;
// pub const CREATE_CANISTER_CYCLES_FEE: u128 = 200_000_000_000;


impl EgoTenantService {
  pub fn canister_main_track(
    wallet_id: Principal,
    canister_id: Principal,
  ) -> Result<(), EgoError> {
    EGO_TENANT.with(|ego_tenant| {
      ego_tenant
        .borrow_mut()
        .canister_main_track(wallet_id, canister_id)
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

    info_log_add("1 create canister");
    let canister_id = management
      .canister_main_create(CREATE_CANISTER_CYCLES_FEE)
      .await?;

    info_log_add("2 load wasm data");
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

    info_log_add("6 set canister controller to [wallet, user, self]");
    ego_canister.ego_controller_set(canister_id, vec![wallet_id, user_id, canister_id]);

    info_log_add("7 change canister owner to [wallet, user, self]");
    ego_canister.ego_owner_set(canister_id, vec![wallet_id, user_id, canister_id]);

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

    if current_cycle < threshold {
      let cycle_required_to_top_up = threshold - current_cycle;

      info_log_add(format!("1. cycle_required_to_top_up: {}", cycle_required_to_top_up).as_str());
      EgoTenantService::wallet_cycle_recharge(management, ego_store, task, cycle_required_to_top_up).await?;

      current_cycle = threshold;
    }

    info_log_add(format!("2. check cycle last_cycle: {}, current_cycle: {}", last_cycle, current_cycle).as_str());
    if last_cycle == 0 {
      // for the first time checking, we will check it again after 30 minutes
      info_log_add("2.1 last cycle is 0. skip estimation")
    } else {
      let delta_cycle = last_cycle - current_cycle;
      let delta_time = current_ts - last_ts; // in second

      info_log_add(format!("3. delta cycle {}, delta time {}", delta_cycle, delta_time).as_str());
      if delta_time == 0 {
        info_log_add("3.1 delta_time is zero. skip estimation")
        // just checked, do nothing
      } else {
        let cycle_consume_per_second = delta_cycle / (delta_time as u128);
        info_log_add(format!("4. cycle_consume_per_second: {}", cycle_consume_per_second).as_str());

        if cycle_consume_per_second != 0 {
          // the remain cycles can be used in estimate_duration second
          let estimate_duration = (current_cycle / cycle_consume_per_second) as u64;

          info_log_add(format!("5. estimate_duration: {}", estimate_duration).as_str());
          ego_canister.ego_cycle_estimate_set(*canister_id, estimate_duration);
        }
      }
    }

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
