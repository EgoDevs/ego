use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use serde::Serialize;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Task {
    pub wallet_id: Principal,
    pub canister_id: Principal,
    pub next_check_time: u64, // second
    pub last_cycle: Option<u128>
}

impl Task {
    pub fn new(wallet_id: Principal, canister_id: Principal, next_check_time: u64) -> Self {
        Task {
            wallet_id,
            canister_id,
            next_check_time,
            last_cycle: None
        }
    }
}
