use std::ops::{Div, Mul};
use ic_cdk::export::Principal;
use ego_types::app::{CanisterType, Wasm};
use ego_types::ego_error::EgoError;
use ego_utils::consts::CREATE_CANISTER_CYCLES_FEE;

use crate::c2c::ego_file::TEgoFile;
use crate::c2c::ic_management::TIcManagement;
use crate::state::EGO_TENANT;
use crate::types::EgoTenantErr;
use num_traits::cast::ToPrimitive;
use crate::c2c::ego_store::TEgoStore;

pub struct EgoTenantService {

}

pub const HALF_HOUR: u64 = 1000 * 60 * 30; // 0.1T cycles

impl EgoTenantService {
    pub fn canister_main_track(wallet_id: Principal, canister_id: Principal) -> Result<(), EgoError> {
        EGO_TENANT.with(|ego_tenant| ego_tenant.borrow_mut().canister_main_track(wallet_id, canister_id))
    }

    pub fn canister_main_untrack(wallet_id: Principal, canister_id: Principal) -> Result<(), EgoError> {
        EGO_TENANT.with(|ego_tenant| ego_tenant.borrow_mut().canister_main_untrack(wallet_id, canister_id))
    }

    pub async fn app_main_install<F: TEgoFile, M: TIcManagement>(ego_file: F, management: M, wallet_id: Principal, user_id: Principal, wasm: Wasm) -> Result<Principal, EgoError> {
        // TODO: handle frontend wasm
        if wasm.canister_type == CanisterType::ASSET {
            return Err(EgoTenantErr::SystemError("not implemented".to_string()).into());
        }

        ic_cdk::println!("1 create canister");
        let canister_id = management.canister_main_create(CREATE_CANISTER_CYCLES_FEE).await?;

        ic_cdk::println!("2 load wasm data for {}", wasm.id());
        let data = ego_file.file_main_read(wasm.canister_id, wasm.fid()).await?;

        ic_cdk::println!("3 install code");
        management.canister_code_install(canister_id, data).await?;

        ic_cdk::println!("4 change canister controller to wallet");
        management.canister_controller_set(canister_id, vec![wallet_id]).await?;

        ic_cdk::println!("4 change canister owner to user");
        management.canister_owner_set(canister_id, user_id).await?;

        Ok(canister_id)
    }

    pub async fn app_main_upgrade<F: TEgoFile, M: TIcManagement>(ego_file: F, management: M, canister_id: Principal, wasm: Wasm) -> Result<bool, EgoError> {
        // TODO: checked whether user has add tenant as one of the canister's controller

        if wasm.canister_type == CanisterType::ASSET {
            return Err(EgoTenantErr::SystemError("not implemented".to_string()).into());
        }

        ic_cdk::println!("1 load wasm data for {}", wasm.id());
        let data = ego_file.file_main_read(wasm.canister_id, wasm.fid()).await?;

        ic_cdk::println!("2 install code");
        management.canister_code_upgrade(canister_id, data).await?;

        Ok(true)
    }

    pub async fn canister_cycles_check<M: TIcManagement, S: TEgoStore>(management: M, ego_store: S, sentinel: u64) -> Result<(), EgoError> {
        let tasks = EGO_TENANT.with(|ego_tenant| ego_tenant.borrow_mut().tasks_get(sentinel));
        for task in tasks {
            let status = management.canister_status_get(task.canister_id).await?;
            let mut current_cycle = status.cycles.0.to_u128().unwrap();
            let mut next_time = sentinel + HALF_HOUR;
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
                    let cycle_consume_per_nanosecond = delta_cycle.div(delta_time as u128);

                    if cycle_consume_per_nanosecond != 0 {
                        // the remain cycles can be used in estimate_duration nanosecond
                        let estimate_duration = (current_cycle / cycle_consume_per_nanosecond).mul(8).div(10) as u64;

                        if estimate_duration <= HALF_HOUR {
                            let ego_store_id = EGO_TENANT.with(|ego_tenant| {
                                ego_tenant.borrow().ego_store
                            });
                            let cycle_required_to_top_up = cycle_consume_per_nanosecond * HALF_HOUR as u128;
                            match ego_store.wallet_cycle_charge(ego_store_id, task.wallet_id, cycle_required_to_top_up, format!("wallet cycle charge, top up canister id {}", task.canister_id)).await?{
                                true => {
                                    management.canister_cycle_top_up(task.canister_id, cycle_required_to_top_up).await?;
                                    current_cycle = current_cycle + cycle_required_to_top_up;
                                }
                                false => {
                                    // TODO: in case wallet controller do not contains enough cycles
                                    continue
                                }
                            }
                        } else {
                            next_time = estimate_duration as u64 + sentinel;
                        }

                    }
                }
            }

            EGO_TENANT.with(|ego_tenant| ego_tenant.borrow_mut().task_update(task.canister_id, current_cycle, next_time));
        }
        Ok(())
    }
}
