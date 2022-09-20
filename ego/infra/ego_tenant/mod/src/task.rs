use std::cmp::Ordering;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_types::Principal;
use serde::Serialize;
use ego_utils::ic_management::Cycles;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Task {
  pub wallet_id: Principal,
  pub canister_id: Principal,
  pub last_check_time: u64,
  pub next_check_time: u64,
  pub last_cycle: Cycles,
  pub current_cycle: Cycles
}

impl Eq for Task {}

impl PartialEq<Self> for Task {
  fn eq(&self, other: &Self) -> bool {
    self.wallet_id == other.wallet_id && self.canister_id == other.canister_id
  }
}

impl PartialOrd<Self> for Task {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.next_check_time.cmp(&other.next_check_time))
  }
}

impl Ord for Task {
  fn cmp(&self, other: &Self) -> Ordering {
    self.next_check_time.cmp(&other.next_check_time)
  }
}

impl Task {
  pub fn new(wallet_id: Principal, canister_id: Principal, next_check_time: u64) -> Self {
    Task {
      wallet_id,
      canister_id,
      last_check_time: 0,
      next_check_time,
      last_cycle: 0,
      current_cycle: 0
    }
  }
}
