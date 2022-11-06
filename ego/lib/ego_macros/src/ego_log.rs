use ic_cdk::api;
use ic_cdk::export::Principal;


pub trait TEgoLogCanister{
    fn canister_log_add(&self, message: &str);
}

pub struct EgoLogCanister {
    pub canister_id: Principal
}

impl EgoLogCanister {
    pub fn new(canister_id: Principal) -> Self {
        EgoLogCanister {canister_id}
    }
}

impl TEgoLogCanister for EgoLogCanister {
    fn canister_log_add(&self, message: &str) {
        let _result = api::call::notify(self.canister_id, "canister_log_add", (message.to_string(),));
    }
}
