use ic_cdk::api;
use ic_cdk::export::Principal;
use ego_cron_mod::types::{CronInterval, TaskMainAddRequest};

pub trait TEgoCron {
    fn task_main_add(&self, method: &str);
}

pub struct EgoCron {
    pub canister_id: Principal
}

impl EgoCron {
    pub fn new(canister_id: Principal) -> Self {
        EgoCron {canister_id}
    }
}

impl TEgoCron for EgoCron {
    fn task_main_add(&self, method: &str) {
        let req = TaskMainAddRequest {
            method: method.to_string(), interval: CronInterval::PerMinute
        };

        let _result = api::call::notify(self.canister_id, "task_main_add", (req,));
    }
}
