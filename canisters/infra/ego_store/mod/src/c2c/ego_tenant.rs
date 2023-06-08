use async_trait::async_trait;
use ic_cdk::api;
use ic_cdk::export::Principal;

use ego_types::app::EgoError;
use ego_types::app::Wasm;

use crate::c2c::c2c_types::{AppMainInstallRequest, AppMainReInstallRequest, AppMainUpgradeRequest};

#[async_trait]
pub trait TEgoTenant {
    async fn app_main_install(
        &self,
        ego_tenant_id: Principal,
        wallet_id: Principal,
        user_id: Principal,
        wasm: &Wasm,
    ) -> Result<Principal, EgoError>;
    async fn app_main_upgrade(
        &self,
        ego_tenant_id: Principal,
        canister_id: Principal,
        wasm: &Wasm,
    ) -> Result<bool, EgoError>;
    async fn app_main_reinstall(
        &self,
        ego_tenant_id: Principal,
        canister_id: Principal,
        wasm: &Wasm,
    ) -> Result<bool, EgoError>;
    fn canister_main_track(
        &self,
        ego_tenant_id: Principal,
        wallet_id: &Principal,
        canister_id: &Principal,
    );
    fn canister_main_untrack(&self, ego_tenant_id: Principal, canister_id: &Principal);
    fn app_main_delete(&self, ego_tenant_id: Principal, canister_id: &Principal);
}

pub struct EgoTenant {}

impl EgoTenant {
    pub fn new() -> Self {
        EgoTenant {}
    }
}

#[async_trait]
impl TEgoTenant for EgoTenant {
    async fn app_main_install(
        &self,
        ego_tenant_id: Principal,
        wallet_id: Principal,
        user_id: Principal,
        wasm: &Wasm,
    ) -> Result<Principal, EgoError> {
        let req = AppMainInstallRequest {
            wallet_id,
            user_id,
            wasm: wasm.clone(),
        };

        let call_result = api::call::call(ego_tenant_id, "app_main_install", (req,)).await
            as Result<(Result<Principal, EgoError>,), _>;

        match call_result {
            Ok(resp) => match resp.0 {
                Ok(canister_id) => Ok(canister_id),
                Err(e) => Err(e),
            },
            Err((code, msg)) => {
                let code = code as u16;
                Err(EgoError { code, msg })
            }
        }
    }

    async fn app_main_upgrade(
        &self,
        ego_tenant_id: Principal,
        canister_id: Principal,
        wasm: &Wasm,
    ) -> Result<bool, EgoError> {
        let req = AppMainUpgradeRequest {
            canister_id,
            wasm: wasm.clone(),
        };

        let call_result = api::call::call(ego_tenant_id, "app_main_upgrade", (req,)).await
            as Result<(Result<bool, EgoError>,), _>;

        match call_result {
            Ok(resp) => match resp.0 {
                Ok(ret) => Ok(ret),
                Err(e) => Err(e),
            },
            Err((code, msg)) => {
                let code = code as u16;
                Err(EgoError { code, msg })
            }
        }
    }

    async fn app_main_reinstall(
        &self,
        ego_tenant_id: Principal,
        canister_id: Principal,
        wasm: &Wasm,
    ) -> Result<bool, EgoError> {
        let req = AppMainReInstallRequest {
            canister_id,
            wasm: wasm.clone(),
        };

        let call_result = api::call::call(ego_tenant_id, "app_main_reinstall", (req,)).await
          as Result<(Result<bool, EgoError>,), _>;

        match call_result {
            Ok(resp) => match resp.0 {
                Ok(ret) => Ok(ret),
                Err(e) => Err(e),
            },
            Err((code, msg)) => {
                let code = code as u16;
                Err(EgoError { code, msg })
            }
        }
    }

    fn canister_main_track(
        &self,
        ego_tenant_id: Principal,
        wallet_id: &Principal,
        canister_id: &Principal,
    ) {
        let _result = api::call::notify(
            ego_tenant_id,
            "canister_main_track",
            (wallet_id, canister_id),
        );
    }

    fn canister_main_untrack(&self, ego_tenant_id: Principal, canister_id: &Principal) {
        let _result = api::call::notify(ego_tenant_id, "canister_main_untrack", (canister_id,));
    }

    fn app_main_delete(&self, ego_tenant_id: Principal, canister_id: &Principal) {
        let _result = api::call::notify(ego_tenant_id, "app_main_delete", (canister_id,));
    }
}
