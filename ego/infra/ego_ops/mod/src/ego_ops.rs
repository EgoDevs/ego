use std::collections::{BTreeMap};
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_types::Principal;
use serde::Serialize;
use ego_types::app::AppId;

#[derive(CandidType, Deserialize, Serialize)]
pub struct EgoOps {
  pub canisters: BTreeMap<AppId, Vec<Principal>>
}

impl EgoOps {
  pub fn new() -> Self {
    EgoOps {
      canisters: BTreeMap::new()
    }
  }

  pub fn app_canister_register(&mut self, app_id: AppId, canister_id: Principal) {
    let cans = self.canisters.entry(app_id).or_insert(vec![]);

    if !cans.contains(&canister_id) {
      cans.push(canister_id);
    }
  }
}