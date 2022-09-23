use ic_types::Principal;
use ego_types::ego_error::EgoError;
use crate::state::EGO_CRON;
use crate::types::CronInterval;

pub struct EgoCronService {}

impl EgoCronService {
    pub fn task_main_add(canister_id: Principal, method: String, interval: CronInterval) -> Result<(), EgoError> {
        EGO_CRON.with(|s| {
            s.borrow_mut().task_add(canister_id, method, interval)
        })
    }

    pub fn task_main_cancel(canister_id: Principal, method: String) -> Result<(), EgoError> {
        let result = EGO_CRON.with(|s| {
            s.borrow().task_get(canister_id, method)
        });

        if result.is_some() {
            let task_id = result.unwrap();
            EGO_CRON.with(|s| {
                s.borrow_mut().task_cancel(task_id);
            });
        }

        Ok(())
    }
}