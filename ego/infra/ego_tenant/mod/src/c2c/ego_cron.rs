use ic_cdk::api;
use ic_cdk::export::Principal;
use ego_cron_mod::types::{CronInterval, TaskMainAddRequest};

pub trait TEgoCron {
    fn task_main_add(
        &self,
        ego_cron_canister_id: Principal,
        method: &str
    );
}

pub struct EgoCron {}

impl EgoCron {
    pub fn new() -> Self {
        EgoCron {}
    }
}

impl TEgoCron for EgoCron {
    fn task_main_add(
        &self,
        ego_cron_canister_id: Principal,
        method: &str
    ) {
        let req = TaskMainAddRequest {
            method: method.to_string(), interval: CronInterval::PerMinute
        };

        let _result = api::call::notify(ego_cron_canister_id, "task_main_add", (req,));
    }
}
