use std::collections::{BTreeMap};

use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_types::Principal;

use crate::types::{cron_interval, CronInterval, EgoCronError};

use ic_cron::implement_cron;
use ic_cron::types::{Iterations, SchedulingOptions};
use ego_types::ego_error::EgoError;
implement_cron!();

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CronTask {
    pub task_id: u64,
    pub canister_id: Principal,
    pub method: String,
    pub interval: CronInterval,
}

impl CronTask {
    pub fn new(task_id: u64, canister_id: Principal, method: String, interval: CronInterval) -> Self {
        CronTask {
            task_id,
            canister_id,
            method,
            interval
        }
    }
}

impl PartialEq for CronTask {
    fn eq(&self, other: &Self) -> bool {
        self.canister_id == other.canister_id && self.method == other.method && self.interval == other.interval
    }
}

pub struct TaskStore {
    pub cron_tasks: BTreeMap<u64, CronTask>,
}

impl TaskStore {
    pub fn new() -> Self {
        TaskStore {
            cron_tasks: BTreeMap::default()
        }
    }

    pub fn task_add(&mut self, canister_id: Principal, method: String, interval: CronInterval) -> Result<u64, EgoError> {
        let mut task = CronTask::new(0, canister_id.clone(),  method.clone(), interval.clone());

        match self.cron_tasks.values().find(|cron_task| **cron_task == task) {
            Some(task) => Ok(task.task_id),
            None => {
                let duration_nano = cron_interval(interval.clone());
                let res = cron_enqueue(
                    task.clone(),
                    SchedulingOptions {
                        delay_nano: 0,
                        interval_nano: duration_nano,
                        iterations: Iterations::Infinite,
                    },
                );

                let task_id = res.unwrap();
                task.task_id = task_id;

                self.cron_tasks.insert(task_id, task);

                Ok(task_id)
            }
        }
    }

    pub fn task_cancel(&mut self, task_id: u64) -> Result<bool, EgoError> {
        match self.cron_tasks.get(&task_id) {
            Some(_) => {
                cron_dequeue(task_id);
                Ok(true)
            },
            None => Err(EgoCronError::TaskNotFound.into())
        }
    }
}