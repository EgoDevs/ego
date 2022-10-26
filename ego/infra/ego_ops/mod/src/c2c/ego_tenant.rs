use ic_cdk::export::Principal;

use ego_tenant_mod::types::{
    CanisterMainTrackRequest, CanisterMainUnTrackRequest
};

use ic_cdk::api;

pub trait TEgoTenant {
    fn canister_main_track(
        &self,
        ego_tenant_id: Principal,
        wallet_id: Principal,
        canister_id: Principal,
    ) ;
    fn canister_main_untrack(
        &self,
        ego_tenant_id: Principal,
        wallet_id: Principal,
        canister_id: Principal,
    ) ;
}

pub struct EgoTenant {}

impl EgoTenant {
    pub fn new() -> Self {
        EgoTenant {}
    }
}

impl TEgoTenant for EgoTenant {
    fn canister_main_track(
        &self,
        ego_tenant_id: Principal,
        wallet_id: Principal,
        canister_id: Principal,
    )  {
        let req = CanisterMainTrackRequest {
            wallet_id,
            canister_id,
        };

        let _result = api::call::notify(ego_tenant_id, "canister_main_track", (req,));
    }

    fn canister_main_untrack(
        &self,
        ego_tenant_id: Principal,
        wallet_id: Principal,
        canister_id: Principal,
    )  {
        let req = CanisterMainUnTrackRequest {
            wallet_id,
            canister_id,
        };

        let _result = api::call::notify(ego_tenant_id, "canister_main_untrack", (req,));
    }
}
