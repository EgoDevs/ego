use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_types::Principal;
use ego_utils::types::Cycles;
use serde::Serialize;


#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Canister {
  pub canister_id: Principal,
  pub wasm_id: String,
  pub last_check: u64,
  pub last_cycle: Cycles,
  pub current_cycle: Cycles
}

impl Canister {
  pub fn new(wasm_id: String, canister_id: Principal) -> Self {
    Canister {
      canister_id,
      wasm_id,
      last_check: 0,
      last_cycle: 0,
      current_cycle: 0
    }
  }
}
