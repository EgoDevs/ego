use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;
use ic_types::Principal;
use crate::types::CronInterval;


#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub struct Task {
    pub task_id: u64,
    pub canister_id: Principal,
    pub method: String,
    pub interval: CronInterval,
}

impl Task {
    pub fn new(task_id: u64, canister_id: Principal, method: String, interval: CronInterval) -> Self {
        Task {
            task_id,
            canister_id,
            method,
            interval
        }
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.canister_id == other.canister_id && self.method == other.method && self.interval == other.interval
    }
}