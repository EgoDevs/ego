use ic_cdk::api;
use ic_cdk::export::Principal;


pub trait TEgoLog{
    fn canister_log_add(&self, message: &str);
}

pub struct EgoLog {
    pub canister_id: Principal
}

impl EgoLog {
    pub fn new(canister_id: Principal) -> Self {
        EgoLog {canister_id}
    }
}

impl TEgoLog for EgoLog {
    fn canister_log_add(&self, message: &str) {
        let _result = api::call::notify(self.canister_id, "canister_log_add", (message.to_string(),));
    }
}
