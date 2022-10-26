use crate::task::Task;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub struct EgoCron {
    pub cron_tasks: BTreeMap<u64, Task>,
}

impl EgoCron {
    pub fn new() -> Self {
        EgoCron {
            cron_tasks: BTreeMap::default(),
        }
    }

    pub fn task_add(&mut self, canister_id: Principal, method: String) -> Option<Task> {
        let task = Task::new(canister_id.clone(), method.clone());

        match self
            .cron_tasks
            .values()
            .find(|cron_task| **cron_task == task)
        {
            Some(_task) => None,
            None => Some(task),
        }
    }

    pub fn task_cancel(&mut self, task_id: u64) {
        self.cron_tasks.remove(&task_id);
    }

    pub fn task_get(&self, canister_id: Principal, method: String) -> Option<u64> {
        let cancel_task = Task::new(canister_id.clone(), method.clone());

        self.cron_tasks.iter().find_map(|(task_id, task)| {
            if *task == cancel_task {
                Some(task_id.clone())
            } else {
                None
            }
        })
    }
}
