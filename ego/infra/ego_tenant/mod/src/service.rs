use ic_cdk::export::Principal;
use ego_types::app::Wasm;
use ego_types::ego_error::EgoError;
use ego_utils::consts::CREATE_CANISTER_CYCLES_FEE;

use crate::c2c::ego_file::TEgoFile;
use crate::c2c::ic_management::TIcManagement;
use crate::state::EGO_TENANT;

pub struct EgoTenantService {

}

impl EgoTenantService {
    pub fn canister_main_track(wallet_id: Principal, canister_id: Principal) -> Result<bool, EgoError> {
        EGO_TENANT.with(|ego_tenant| ego_tenant.borrow_mut().canister_main_track(wallet_id, canister_id))
    }

    pub fn canister_main_untrack(wallet_id: Principal, canister_id: Principal) -> Result<bool, EgoError> {
        EGO_TENANT.with(|ego_tenant| ego_tenant.borrow_mut().canister_main_untrack(wallet_id, canister_id))
    }

    pub async fn app_main_install<F: TEgoFile, M: TIcManagement>(ego_file: F, management: M, wallet_id: Principal, user_id: Principal, wasm: Wasm) -> Result<Principal, EgoError> {
        // TODO: checked whether user has add tenant as one of the canister's controller

        ic_cdk::println!("1 create canister");
        let canister_id = management.canister_main_create(CREATE_CANISTER_CYCLES_FEE).await?;

        ic_cdk::println!("2 load wasm data for {}", wasm.id());
        let data = ego_file.file_main_read(wasm.canister_id.unwrap(), wasm.fid()).await?;

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

        ic_cdk::println!("1 load wasm data for {}", wasm.id());
        let data = ego_file.file_main_read(wasm.canister_id.unwrap(), wasm.fid()).await?;

        ic_cdk::println!("2 install code");
        management.canister_code_upgrade(canister_id, data).await?;

        Ok(true)
    }
}
