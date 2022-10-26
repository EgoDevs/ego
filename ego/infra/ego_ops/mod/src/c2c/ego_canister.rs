use ic_cdk::api;
use ic_cdk::export::Principal;

use async_trait::async_trait;

#[async_trait]
pub trait TEgoCanister {
    fn role_user_add(&self, canister_id: Principal, principal: Principal);
    fn canister_add(&self, canister_id: &Principal, name: String, principal: &Principal);
}

pub struct EgoCanister {}

impl EgoCanister {
    pub fn new() -> Self {
        EgoCanister {}
    }
}

#[async_trait]
impl TEgoCanister for EgoCanister {
    fn role_user_add(&self, canister_id: Principal, principal: Principal) {
        let _result = api::call::notify(canister_id, "role_user_add", (principal,));
    }

    fn canister_add(&self, canister_id: &Principal, name: String, principal: &Principal) {
        let _result = api::call::notify(canister_id.clone(), "canister_add", (name, principal.clone()));
    }
}
