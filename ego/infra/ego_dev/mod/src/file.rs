use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;
use ic_types::Principal;
use std::cmp::Ordering;


#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct File {
  pub wasm_count: u16,
  pub canister_id: Principal,
}

impl Eq for File {}

impl PartialEq<Self> for File {
  fn eq(&self, other: &Self) -> bool {
    self.canister_id == other.canister_id
  }
}

impl PartialOrd<Self> for File {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.wasm_count.cmp(&other.wasm_count))
  }
}

impl Ord for File {
  fn cmp(&self, other: &Self) -> Ordering {
    self.wasm_count.cmp(&other.wasm_count)
  }
}

impl File {
  pub fn new(canister_id: Principal) -> Self{
    File{canister_id, wasm_count: 0}
  }
}