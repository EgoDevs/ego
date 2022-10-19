use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;

// type for ego_cron
#[derive(CandidType, Deserialize)]
pub enum CronInterval {
  PerSecond,
  PerMinute,
  PerHour,
  PerDay,
}

#[derive(CandidType, Deserialize)]
pub struct TaskMainAddRequest {
  pub canister_id: Principal,
  pub method: String,
  pub interval: CronInterval,
}