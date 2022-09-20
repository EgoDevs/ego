use std::collections::{BTreeSet};
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_types::Principal;
use serde::Serialize;
use ego_types::ego_error::EgoError;
use crate::task::Task;

#[derive(CandidType, Deserialize, Serialize)]
pub struct EgoTenant {
  pub tasks: BTreeSet<Task>
}

impl EgoTenant {
  pub fn new() -> Self {
    EgoTenant {
      tasks: Default::default()
    }
  }

  pub fn canister_main_track(&mut self, wallet_id: Principal, canister_id: Principal) -> Result<bool, EgoError> {
    match self.tasks.iter().find(|task| task.wallet_id == wallet_id && task.canister_id == canister_id) {
      None => {
        self.tasks.insert(Task::new(wallet_id, canister_id, 0));
        Ok(true)
      }
      Some(_) => {
        Ok(true)
      }
    }
  }

  pub fn canister_main_untrack(&mut self, wallet_id: Principal, canister_id: Principal) -> Result<bool, EgoError> {
    self.tasks.retain(|task| task.wallet_id != wallet_id && task.canister_id != canister_id);
    Ok(true)
  }
}
