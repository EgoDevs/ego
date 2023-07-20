use std::collections::BTreeMap;

use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use serde::Serialize;

use ego_types::app::EgoError;

use crate::task::Task;

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub struct Tenant {
    pub tasks: BTreeMap<Principal, Task>,
}

impl Tenant {
    pub fn new() -> Self {
        Tenant {
            tasks: Default::default(),
        }
    }

    pub fn canister_main_track(
        &mut self,
        wallet_id: Principal,
        canister_id: Principal,
        next_check_time: u64,
    ) -> Result<(), EgoError> {
        self.tasks
            .entry(canister_id)
            .and_modify(|task| task.next_check_time = next_check_time)
            .or_insert(Task::new(wallet_id, canister_id, next_check_time));
        Ok(())
    }

    pub fn canister_main_untrack(&mut self, canister_id: Principal) -> Result<(), EgoError> {
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

    pub fn task_update(&mut self, canister_id: Principal, next_check_time: u64, last_cycle: u128) {
        self.tasks.entry(canister_id).and_modify(|task| {
            task.next_check_time = next_check_time;
            task.last_cycle = Some(last_cycle);
        });
    }
}
