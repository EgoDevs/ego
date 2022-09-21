use std::collections::BTreeMap;
use serde::Serialize;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cron::types::{Iterations, SchedulingOptions};
use ic_types::Principal;
use ego_types::ego_error::EgoError;
use crate::types::{cron_interval, CronInterval, EgoCronError};
use ic_cron::implement_cron;
use crate::task::Task;

implement_cron!();

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub struct EgoCron {
  pub cron_tasks: BTreeMap<u64, Task>,
}

impl EgoCron {
  pub fn new() -> Self {
    EgoCron {
      cron_tasks: BTreeMap::default()
    }
  }

  pub fn task_add(&mut self, canister_id: Principal, method: String, interval: CronInterval) -> Result<u64, EgoError> {
    let mut task = Task::new(0, canister_id.clone(), method.clone(), interval.clone());

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