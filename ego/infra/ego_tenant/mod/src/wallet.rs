use std::collections::{BTreeMap};
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_types::Principal;
use serde::Serialize;
use ego_utils::types::{AppId, EgoError};
use crate::canister::Canister;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Wallet {
  pub wallet_id: Principal,
  pub canisters: BTreeMap<AppId, BTreeMap<String, Canister>>,
}

impl Wallet {
  pub fn new(wallet_id: Principal) -> Self {
    Wallet {
      wallet_id,
      canisters: BTreeMap::new(),
    }
  }

  pub fn app_install(&mut self, app_id: AppId, canisters: BTreeMap<String, Principal>) -> Result<bool, EgoError> {
    let canisters= canisters.iter().map(|(wasm_id, canister_id)| (wasm_id.clone(), Canister::new(wasm_id.clone(), canister_id.clone()))).collect();
    self.canisters.entry(app_id).or_insert(canisters);
    Ok(true)
  }
}
