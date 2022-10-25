use ic_cdk::export::candid::{ Deserialize};
use candid::{CandidType, Principal};
use serde::Serialize;

use std::collections::BTreeMap;


pub trait CanisterTrait {
  // add canister under the specified name
  fn canister_add(&mut self, name: String, canister_id: Principal);

  // remove canister from the specified name
  fn canister_remove(&mut self, name: String, canister_id: Principal);

  // get canister under the specified name
  fn canister_get(&self, name: String) -> Vec<Principal>;
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Registry {
  canisters: BTreeMap<String, Vec<Principal>>
}

impl Default for Registry {
  fn default() -> Self {
    Registry { canisters: BTreeMap::new() }
  }
}

impl CanisterTrait for Registry {

  fn canister_add(&mut self, name: String, canister_id: Principal) {
    let canister_ids = self.canisters.entry(name).or_insert(vec![]);

    if !canister_ids.contains(&canister_id) {
      canister_ids.push(canister_id);
    }
  }


  fn canister_remove(&mut self, name: String, canister_id: Principal) {
    let canister_ids = self.canisters.entry(name).or_insert(vec![]);

    canister_ids.retain(|exists_canister_id| *exists_canister_id == canister_id)
  }

  fn canister_get(&self, name: String) -> Vec<Principal> {
    self.canisters.get(&name).cloned().unwrap()
  }
}
