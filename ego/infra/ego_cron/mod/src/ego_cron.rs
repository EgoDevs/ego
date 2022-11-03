use crate::task::Task;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use serde::Serialize;
use std::collections::BTreeMap;
use ic_cron::types::TaskId;

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub struct EgoCron {
    pub cron_tasks: BTreeMap<TaskId, Task>,
}

impl EgoCron {
    pub fn new() -> Self {
        EgoCron {
            cron_tasks: BTreeMap::default(),
        }
    }

    pub fn task_add(&mut self, task_id: TaskId, task: Task) {
        self.cron_tasks.entry(task_id).or_insert(task);
    }

    pub fn task_cancel(&mut self, task_id: TaskId) {
        self.cron_tasks.remove(&task_id);
    }

    pub fn task_get(&self, canister_id: Principal, method: String) -> Option<TaskId> {
        self.cron_tasks.iter().find_map(|(task_id, task)| {
            if task.canister_id == canister_id && task.method == method {
                Some(task_id.clone())
            } else {
                None
            }
        })
    }
}
