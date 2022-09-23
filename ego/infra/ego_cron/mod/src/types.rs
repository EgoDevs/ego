use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;
use ic_cdk::export::Principal;
use ego_types::ego_error::EgoError;


#[derive(CandidType, Deserialize, Serialize)]
pub struct TaskMainAddRequest {
    pub canister_id: Principal,
    pub method: String,
    pub interval: CronInterval,
}


#[derive(CandidType, Deserialize, Serialize)]
pub struct TaskMainCancelRequest {
    pub canister_id: Principal,
    pub method: String
}


#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub enum CronInterval {
    PerSecond,
    PerMinute,
    PerHour,
    PerDay,
}

pub fn cron_interval(interval: CronInterval) -> u64 {
    match interval {
        CronInterval::PerSecond => 1_000_000_000,
        CronInterval::PerMinute => 1_000_000_000 * 60,
        CronInterval::PerHour => 1_000_000_000 * 60 * 60,
        CronInterval::PerDay => 1_000_000_000 * 60 * 60 * 24,
    }
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum EgoCronError {
    AlreadyHasTask,
    TaskNotFound,
    CancelFail(u64),
    UnknownError(String),
}


impl From<EgoCronError> for EgoError {
    fn from(e: EgoCronError) -> Self {
        match e {
            EgoCronError::TaskNotFound => EgoError::new(2001, "ego_cron: task not found"),
            EgoCronError::CancelFail(_)=> EgoError::new(2002, "ego_cron: cancel fail"),
            EgoCronError::UnknownError(_) => EgoError::new(2003, "ego_cron: unknown error"),
            _ =>  EgoError::new(2003, "ego_cron: unknown error"),
        }
    }
}
