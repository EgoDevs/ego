use async_trait::async_trait;
use ic_cdk::api;
use ic_cdk::export::Principal;

#[async_trait]
pub trait TEgoRecord {
    fn record_add(&self, scope: String, event: String, message: String, created_at: Option<u64>);
}

pub struct EgoRecord {
    pub canister_id: Principal,
}

impl EgoRecord {
    pub fn new(canister_id: Principal) -> Self {
        EgoRecord { canister_id }
    }
}

#[async_trait]
impl TEgoRecord for EgoRecord {
    fn record_add(&self, scope: String, event: String, message: String, created_at: Option<u64>) {
        let _result = api::call::notify(
            self.canister_id,
            "record_add",
            (scope, event, message, created_at),
        );
    }
}
