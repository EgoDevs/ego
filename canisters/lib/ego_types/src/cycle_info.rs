use candid::{CandidType, Deserialize};
use serde::Serialize;

pub const DEFAULT_ESTIMATE: u64 = 86400 * 7;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CycleRecord {
  pub balance: u128,
  pub ts: u64, // timestamp in seconds
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CycleInfo {
  pub records: Vec<CycleRecord>,
  // estimated seconds canister cycle will be exhausted
  pub estimate_remaining: u64, // duration in seconds
}

impl Default for CycleInfo {
  fn default() -> Self {
    CycleInfo {
      estimate_remaining: DEFAULT_ESTIMATE,
      records: vec![],
    }
  }
}

impl CycleInfo {
  pub fn new() -> Self {
    CycleInfo {
      estimate_remaining: 0,
      records: vec![],
    }
  }

  pub fn record_add(&mut self, balance: u128, ts: u64) {
    self.records.insert(0, CycleRecord { balance, ts });
    if self.records.len() > 12 {
      self.records.truncate(12);
    }
  }

  pub fn record_list(&self) -> Vec<CycleRecord> {
    self.records.to_vec()
  }

  pub fn estimate_remaining_set(&mut self, estimate: u64) {
    self.estimate_remaining = estimate;
  }
}
