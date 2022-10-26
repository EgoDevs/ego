use async_trait::async_trait;
use ic_cdk::api;
use ic_cdk::export::Principal;

use ego_tenant_mod::types::{
    AppMainInstallRequest, AppMainInstallResponse, AppMainUpgradeRequest, AppMainUpgradeResponse,
    CanisterMainTrackRequest, CanisterMainUnTrackRequest,
};
use ego_types::app::Wasm;
use ego_types::ego_error::EgoError;

#[async_trait]
pub trait TEgoTenant {
    async fn app_main_install(
        &self,
        ego_tenant_id: Principal,
        wallet_id: Principal,
        user_id: Principal,
        wasm: Wasm,
    ) -> Result<Principal, EgoError>;
    async fn app_main_upgrade(
        &self,
        ego_tenant_id: Principal,
        canister_id: Principal,
        wasm: Wasm,
    ) -> Result<bool, EgoError>;
    async fn canister_main_track(
        &self,
        ego_tenant_id: Principal,
        wallet_id: Principal,
        canister_id: Principal,
    ) -> Result<(), EgoError>;
    async fn canister_main_untrack(
        &self,
        ego_tenant_id: Principal,
        wallet_id: Principal,
        canister_id: Principal,
    ) -> Result<(), EgoError>;
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
        wasm: Wasm,
    ) -> Result<Principal, EgoError> {
        let req = AppMainInstallRequest {
            wallet_id,
            user_id,
            wasm,
        };

        let call_result = api::call::call(ego_tenant_id, "app_main_install", (req,)).await
            as Result<(Result<AppMainInstallResponse, EgoError>,), _>;

        match call_result {
            Ok(resp) => match resp.0 {
                Ok(resp) => Ok(resp.canister_id),
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
        wasm: Wasm,
    ) -> Result<bool, EgoError> {
        let req = AppMainUpgradeRequest { canister_id, wasm };

        let call_result = api::call::call(ego_tenant_id, "app_main_upgrade", (req,)).await
            as Result<(Result<AppMainUpgradeResponse, EgoError>,), _>;

        match call_result {
            Ok(resp) => match resp.0 {
                Ok(resp) => Ok(resp.ret),
                Err(e) => Err(e),
            },
            Err((code, msg)) => {
                let code = code as u16;
                Err(EgoError { code, msg })
            }
        }
    }

    async fn canister_main_track(
        &self,
        ego_tenant_id: Principal,
        wallet_id: Principal,
        canister_id: Principal,
    ) -> Result<(), EgoError> {
        let req = CanisterMainTrackRequest {
            wallet_id,
            canister_id,
        };

        let call_result = api::call::call(ego_tenant_id, "canister_main_track", (req,)).await
            as Result<(Result<(), EgoError>,), _>;

        match call_result {
            Ok(resp) => match resp.0 {
                Ok(resp) => Ok(resp),
                Err(e) => Err(e),
            },
            Err((code, msg)) => {
                let code = code as u16;
                Err(EgoError { code, msg })
            }
        }
    }

    async fn canister_main_untrack(
        &self,
        ego_tenant_id: Principal,
        wallet_id: Principal,
        canister_id: Principal,
    ) -> Result<(), EgoError> {
        let req = CanisterMainUnTrackRequest {
            wallet_id,
            canister_id,
        };

        let call_result = api::call::call(ego_tenant_id, "canister_main_untrack", (req,)).await
            as Result<(Result<(), EgoError>,), _>;

        match call_result {
            Ok(resp) => match resp.0 {
                Ok(resp) => Ok(resp),
                Err(e) => Err(e),
            },
            Err((code, msg)) => {
                let code = code as u16;
                Err(EgoError { code, msg })
            }
        }
    }
}
