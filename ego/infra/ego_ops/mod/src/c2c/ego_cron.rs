use ic_cdk::api;
use ic_cdk::export::Principal;

use async_trait::async_trait;
use ego_types::ego_error::EgoError;
use crate::c2c::c2c_types::{CronInterval, TaskMainAddRequest};

#[async_trait]
pub trait TEgoCron {
  async fn task_main_add(&self, ego_cron_id: Principal, canister_id: Principal, method: String, interval: CronInterval) -> Result<bool, EgoError>;
}

pub struct EgoCron {
}

impl EgoCron{
  pub fn new() -> Self {
    EgoCron{}
  }
}

#[async_trait]
impl TEgoCron for EgoCron {
  async fn task_main_add(&self, ego_cron_id: Principal, canister_id: Principal, method: String, interval: CronInterval) -> Result<bool, EgoError>{
    let req = TaskMainAddRequest{
      canister_id,
      method,
      interval
    };

    let notify_result = api::call::notify(
      ego_cron_id,
      "task_main_add",
      (req,),
    );

    match notify_result {
      Err(code) => {
        let code = code as u16;
        Err(EgoError { code, msg: "task_main_add failed".to_string() })
      },
      _ => Ok(true)
    }
  }
}