use std::collections::HashMap;
use ic_types::Principal;
use ego_dev_mod::app::CanisterType::BACKEND;
use ego_dev_mod::types::AppId;
use ego_utils::consts::CREATE_CANISTER_CYCLES_FEE;
use ego_utils::types::Management;
use ego_utils::types::EgoError;
use crate::c2c::ego_dev::TEgoDev;
use crate::state::EGO_TENANT;
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

    pub async fn wallet_app_install<D: TEgoDev, M: Management>(wallet_id: Principal, dev: D, management: M, app_id: AppId) -> Result<HashMap<String, Principal>, EgoError> {
        ic_cdk::println!("1. get app");
        let app = dev.app_main_get(app_id.clone()).await?;

        let mut canisters = HashMap::<String, Principal>::new();

        ic_cdk::println!("2. create canisters");
        let wasms = app.wasm_release_find().unwrap();
        for wasm in wasms.iter().find(|wasm| wasm.canister_type == BACKEND) {
            ic_cdk::println!("2.1 load wasm data for {}", wasm.id);
            let data = dev.file_main_read(app_id.clone(), wasm.id.clone()).await?;

            ic_cdk::println!("2.2 create canister");
            let canister_id = management.canister_main_create(CREATE_CANISTER_CYCLES_FEE).await?;
            canisters.insert(wasm.id.clone(), canister_id);

            ic_cdk::println!("2.3 install code");
            management.canister_code_install(canister_id, data).await?;

            ic_cdk::println!("2.4 change owner to wallet");
            management.canister_controller_set(canister_id, vec![wallet_id]).await?;
        }

        EGO_TENANT.with(|ego_tenant| ego_tenant.borrow_mut().wallet_app_install(&wallet_id, app_id.clone(), canisters.clone()))?;

        Ok(canisters)
    }

    // pub async fn wallet_app_upgrade<D: TEgoDev, F: TEgoFile, M: Management>(wallet_id: Principal, dev: D, file: F, management: M, app_id: AppId) -> Result<bool, EgoError> {
    //     ic_cdk::println!("1. get app");
    //     let app = dev.app_main_get(app_id).await?;
    //
    //     ic_cdk::println!("2. create canisters");
    //     let wasms = app.find_release_wasms().unwrap();
    //     for wasm in wasms.iter().find(|wasm| wasm.canister_type == BACKEND) {
    //         ic_cdk::println!("2.1 load wasm data for {}", wasm.id);
    //         let data = file.file_main_read(wasm.file_id, wasm.id.clone()).await?;
    //
    //         ic_cdk::println!("2.2 upgrade code");
    //         management.canister_code_install(canister_id, data).await?;
    //     }
    //
    //     Ok(true)
    // }
}
