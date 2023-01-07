use std::collections::BTreeMap;

use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use serde::Serialize;

use ego_types::app::EgoError;

use crate::task::Task;

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub struct EgoTenant {
  pub tasks: BTreeMap<Principal, Task>,
}

impl EgoTenant {
  pub fn new() -> Self {
    EgoTenant {
      tasks: Default::default(),
    }
  }

  pub fn canister_main_track(
    &mut self,
    wallet_id: Principal,
    canister_id: Principal,
  ) -> Result<(), EgoError> {
    self.tasks
      .entry(canister_id)
      .or_insert(Task::new(wallet_id, canister_id));
    Ok(())
  }

  pub fn canister_main_untrack(
    &mut self,
    _wallet_id: Principal,
    canister_id: Principal,
  ) -> Result<(), EgoError> {
    self.tasks.remove(&canister_id);

    Ok(())
  }

  pub fn tasks_get(&self, sentinel: u64) -> Vec<Task> {
    self.tasks
      .values()
      .filter(|&task| task.next_check_time <= sentinel)
      .cloned()
      .collect()
  }

  pub fn task_update(
    &mut self,
    canister_id: Principal,
    current_cycle: u128,
    next_check_time: u64,
  ) {
    self.tasks.entry(canister_id).and_modify(|task| {
      task.last_cycle = current_cycle;
      task.last_check_time = task.next_check_time;
      task.next_check_time = next_check_time;
    });
  }
}
