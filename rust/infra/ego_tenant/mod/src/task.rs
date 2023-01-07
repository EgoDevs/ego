use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use serde::Serialize;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Task {
  pub wallet_id: Principal,
  pub canister_id: Principal,
  pub last_check_time: u64,
  pub next_check_time: u64,
  pub last_cycle: u128,
}

impl Task {
  pub fn new(wallet_id: Principal, canister_id: Principal) -> Self {
    Task {
      wallet_id,
      canister_id,
      last_check_time: 0,
      next_check_time: 0,
      last_cycle: 0,
    }
  }
}
