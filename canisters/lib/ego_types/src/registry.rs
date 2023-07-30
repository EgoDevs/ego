use std::collections::BTreeMap;

use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Registry {
  canisters: BTreeMap<String, Vec<Principal>>,
}

impl Default for Registry {
  fn default() -> Self {
    Registry {
      canisters: BTreeMap::new(),
    }
  }
}

impl Registry {
  pub fn canister_add(&mut self, name: String, canister_id: Principal) {
    let canister_ids = self.canisters.entry(name).or_insert(vec![]);

    if !canister_ids.contains(&canister_id) {
      canister_ids.push(canister_id);
    }
  }

  pub fn canister_remove(&mut self, name: String, canister_id: Principal) {
    let canister_ids = self.canisters.entry(name).or_insert(vec![]);

    canister_ids.retain(|exists_canister_id| *exists_canister_id != canister_id)
  }

  pub fn canister_remove_all(&mut self, name: String) {
    self.canisters.remove(&name);
  }

  pub fn canister_get_all(&self, name: &str) -> Vec<Principal> {
    self.canisters.get(name).cloned().unwrap()
  }

  pub fn canister_get_one(&self, name: &str) -> Option<Principal> {
    match self.canisters.get(name) {
      None => None,
      Some(canisters) => {
        if canisters.is_empty() {
          None
        } else {
          Some(canisters.get(0).unwrap().clone())
        }
      }
    }
  }

  pub fn canister_list_all(&self) -> BTreeMap<String, Vec<Principal>> {
    self.canisters.clone()
  }
}
