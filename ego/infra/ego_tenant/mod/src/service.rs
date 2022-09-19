use std::collections::{BTreeMap};
use ic_types::Principal;
use ego_types::app::App;
use ego_types::ego_error::EgoError;
use ego_utils::consts::CREATE_CANISTER_CYCLES_FEE;

use crate::c2c::ego_file::TEgoFile;
use crate::c2c::ic_management::TIcManagement;
use crate::state::EGO_TENANT;
use crate::types::{EgoTenantErr};
use crate::wallet::Wallet;

pub struct EgoTenantService {

}

impl EgoTenantService {
    pub fn wallet_main_add(wallet_id: Principal) -> Result<bool, EgoError> {
        EGO_TENANT.with(|ego_tenant| ego_tenant.borrow_mut().wallet_main_add(wallet_id))
    }

    pub fn wallet_main_remove(wallet_id: &Principal) -> Result<bool, EgoError> {
        EGO_TENANT.with(|ego_tenant| ego_tenant.borrow_mut().wallet_main_remove(&wallet_id))
    }

    pub fn wallet_main_get(wallet_id: &Principal) -> Result<Wallet, EgoError> {
        EGO_TENANT.with(|ego_tenant| ego_tenant.borrow().wallet_main_get(&wallet_id))
    }

    pub async fn wallet_app_install<F: TEgoFile, M: TIcManagement>(wallet_id: Principal, ego_file: F, management: M, app: App) -> Result<BTreeMap<String, Principal>, EgoError> {
        let mut wallet = EGO_TENANT.with(|ego_tenant| ego_tenant.borrow_mut().wallet_main_get_mut(&wallet_id))?;

        let mut canisters = BTreeMap::<String, Principal>::new();

        if app.frontend.canister_id.is_some() {
            canisters.insert(app.frontend.id(), app.frontend.canister_id.unwrap());
        }

        if app.backend.canister_id.is_some() {
            ic_cdk::println!("1. create canisters");
            let wasm = app.backend;

            ic_cdk::println!("1.1 create canister");
            let canister_id = management.canister_main_create(CREATE_CANISTER_CYCLES_FEE).await?;
            canisters.insert(wasm.id(), canister_id);

            ic_cdk::println!("1.2 load wasm data for {}", wasm.id());
            let data = ego_file.file_main_read(wasm.canister_id.unwrap(), wasm.fid()).await?;

            ic_cdk::println!("1.3 install code");
            management.canister_code_install(canister_id, data).await?;

            ic_cdk::println!("1.4 change owner to wallet");
            management.canister_controller_set(canister_id, vec![wallet_id]).await?;

            wallet.app_install(app.app_id,canisters.clone())?;
        }

        Ok(canisters)
    }

    pub async fn wallet_app_upgrade<F: TEgoFile, M: TIcManagement>(wallet_id: Principal, ego_file: F, management: M, app: App) -> Result<bool, EgoError> {
        let wallet = EGO_TENANT.with(|ego_tenant| ego_tenant.borrow().wallet_main_get(&wallet_id))?;
        let wasm = app.backend;

        match wallet.canisters.get(&app.app_id){
            None => Err(EgoTenantErr::AppNotInstalled.into()),
            Some(canisters) => {
                match canisters.get(&wasm.id()) {
                    None => Err(EgoTenantErr::CanisterNotFounded.into()),
                    Some(canister) => {
                        // TODO: checked whether user has add tenant as one of the canister's controller

                        ic_cdk::println!("1.1 load wasm data for {}", wasm.id());
                        let data = ego_file.file_main_read(wasm.canister_id.unwrap(), wasm.fid()).await?;

                        ic_cdk::println!("1.2 install code");
                        management.canister_code_upgrade(canister.canister_id, data).await?;

                        Ok(true)
                    }
                }
            }
        }
    }
}
