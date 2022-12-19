use std::ops::{Div, Mul};

use astrox_macros::{inject_canister_log, inject_canister_registry};
use astrox_macros::inject_canister_users;
use ego_lib::ego_canister::TEgoCanister;

use ego_macros::inject_log;
use ego_types::app::{CanisterType, Wasm};
use ego_types::ego_error::EgoError;
use ego_utils::consts::CREATE_CANISTER_CYCLES_FEE;

use crate::c2c::ego_file::TEgoFile;
use crate::c2c::ego_store::TEgoStore;
use crate::c2c::ic_management::TIcManagement;
use crate::state::EGO_TENANT;
use crate::task::Task;
use crate::types::EgoTenantErr;

pub struct EgoTenantService {}

pub const HALF_HOUR: u64 = 1000 * 60 * 30; // 0.1T cycles

inject_log!();
inject_canister_users!();
inject_canister_registry!();
inject_canister_log!();

/********************  methods for ego_registry   ********************/
fn on_canister_added(name: &str, canister_id: Principal) {
  let _ = match name {
    "ego_store" => user_add(canister_id),
    _ => {}
  };
}

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
    wallet_id: Principal,
    canister_id: Principal,
  ) -> Result<(), EgoError> {
    EGO_TENANT.with(|ego_tenant| {
      ego_tenant
        .borrow_mut()
        .canister_main_untrack(wallet_id, canister_id)
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

    ego_log("1 create canister");
    let canister_id = management
      .canister_main_create(CREATE_CANISTER_CYCLES_FEE)
      .await?;

    ego_log("2 load wasm data");
    let data = ego_file
      .file_main_read(wasm.canister_id, wasm.fid())
      .await?;

    ego_log("3 install code");
    management.canister_code_install(canister_id, data).await?;

    // add ego_store_id to app
    ego_log("4 register canister");
    ego_canister.ego_canister_add(canister_id, "ego_store".to_string(), ego_store_id);
    ego_canister.ego_canister_add(canister_id, "ego_tenant".to_string(), ego_tenant_id);

    ego_log("5 add ops_user");
    ego_canister.ego_op_add(canister_id, ego_store_id);
    ego_canister.ego_op_add(canister_id, ego_tenant_id);

    ego_log("6 set canister controller to [wallet, user, self]");
    let _result = management.canister_controller_set(canister_id, vec![wallet_id, user_id, canister_id]).await?;

    ego_log("7 change canister owner to [wallet, user]");
    ego_canister.ego_owner_set(canister_id, vec![wallet_id, user_id]);

    Ok(canister_id)
  }

  pub async fn app_main_upgrade<F: TEgoFile, M: TIcManagement>(
    ego_file: F,
    management: M,
    canister_id: Principal,
    wasm: Wasm,
  ) -> Result<bool, EgoError> {
    // TODO: checked whether user has add tenant as one of the canister's controller

    if wasm.canister_type == CanisterType::ASSET {
      return Err(EgoTenantErr::SystemError("not implemented".to_string()).into());
    }

    ego_log("1 load wasm data");
    let data = ego_file
      .file_main_read(wasm.canister_id, wasm.fid())
      .await?;

    ego_log("2 install code");
    management.canister_code_upgrade(canister_id, data).await?;

    Ok(true)
  }

  pub async fn canister_cycles_check<M: TIcManagement, S: TEgoStore, EC: TEgoCanister>(
    management: M,
    ego_store: S,
    canister: EC,
    sentinel: u64,
    task: Task,
  ) -> Result<(), EgoError> {
    let ego_store_id = canister_get_one("ego_store").unwrap();

    let cycle = canister.balance_get(task.canister_id).await?;

    let mut current_cycle = cycle;
    let mut next_time = sentinel + HALF_HOUR;

    ego_log(format!("last_cycle: {}, current_cycle: {}", task.last_cycle, current_cycle).as_str());
    if task.last_cycle == 0 {
      // for the first time checking, we will check it again after 30 minutes
    } else if task.last_cycle <= current_cycle {
      // more cycle then before, do nothing
    } else {
      let delta_cycle = task.last_cycle - current_cycle;
      let delta_time = sentinel - task.last_check_time;
      if delta_time == 0 {
        // just checked, do nothing
      } else {
        let cycle_consume_per_nanosecond = delta_cycle / (delta_time as u128);

        if cycle_consume_per_nanosecond != 0 {
          // the remain cycles can be used in estimate_duration nanosecond
          let estimate_duration = (current_cycle / cycle_consume_per_nanosecond)
            .mul(8)
            .div(10) as u64;

          if estimate_duration <= HALF_HOUR {
            let cycle_required_to_top_up =
              cycle_consume_per_nanosecond * HALF_HOUR as u128;
            match ego_store
              .wallet_cycle_charge(
                ego_store_id,
                task.wallet_id,
                cycle_required_to_top_up,
                format!(
                  "wallet cycle charge, top up canister id {}",
                  task.canister_id
                ),
              )
              .await?
            {
              true => {
                management
                  .canister_cycle_top_up(
                    task.canister_id,
                    cycle_required_to_top_up,
                  );
                current_cycle = current_cycle + cycle_required_to_top_up;
              }
              false => {
                // TODO: in case wallet controller do not contains enough cycles
              }
            }
          } else {
            next_time = estimate_duration as u64 + sentinel;
          }
        }
      }
    }

    EGO_TENANT.with(|ego_tenant| {
      ego_tenant
        .borrow_mut()
        .task_update(task.canister_id, current_cycle, next_time)
    });

    Ok(())
  }
}
