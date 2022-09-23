use std::collections::BTreeMap;
use serde::Serialize;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cron::types::{Iterations, SchedulingOptions, TaskId};
use ic_types::Principal;
use ego_types::ego_error::EgoError;
use crate::types::{cron_interval, CronInterval};
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

  pub fn task_add(&mut self, canister_id: Principal, method: String, interval: CronInterval) -> Result<(), EgoError> {
    let task = Task::new(canister_id.clone(), method.clone());

    match self.cron_tasks.values().find(|cron_task| **cron_task == task) {
      Some(_task) => Ok(()),
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

        ic_cdk::println!("task created id:{} / principal:{} / method:{} / interval:{}", task_id, canister_id, method, duration_nano);

        self.cron_tasks.insert(task_id, task);

        Ok(())
      }
    }
  }

  pub fn task_cancel(&mut self, task_id: u64) {
    self.cron_tasks.remove(&task_id);
    cron_dequeue(task_id.clone() as TaskId);
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