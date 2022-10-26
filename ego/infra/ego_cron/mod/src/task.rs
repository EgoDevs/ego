use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use serde::Serialize;

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, Eq)]
pub struct Task {
    pub canister_id: Principal,
    pub method: String,
}

impl Task {
    pub fn new(canister_id: Principal, method: String) -> Self {
        Task {
            canister_id,
            method,
        }
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.canister_id == other.canister_id && self.method == other.method
    }
}
