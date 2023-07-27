use ic_cdk::api;
use candid::{Principal};

pub trait TEgoTenant {
    fn canister_main_track(
        &self,
        ego_tenant_id: Principal,
        wallet_id: Principal,
        canister_id: Principal,
    );
    fn canister_main_untrack(
        &self,
        ego_tenant_id: Principal,
        wallet_id: Principal,
        canister_id: Principal,
    );
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
    ) {
        let _result = api::call::notify(
            ego_tenant_id,
            "canister_main_track",
            (wallet_id, canister_id),
        );
    }

    fn canister_main_untrack(
        &self,
        ego_tenant_id: Principal,
        wallet_id: Principal,
        canister_id: Principal,
    ) {
        let _result = api::call::notify(
            ego_tenant_id,
            "canister_main_untrack",
            (wallet_id, canister_id),
        );
    }
}
