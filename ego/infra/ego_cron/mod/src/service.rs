use ic_types::Principal;
use crate::state::TASKSTORE;
use ego_utils::types::EgoError;
use crate::types::CronInterval;

pub struct EgoCronService {}

impl EgoCronService {
    pub fn task_main_add(canister_id: Principal, method: String, interval: CronInterval) -> Result<u64, EgoError> {
        TASKSTORE.with(|s| {
            s.borrow_mut().task_add(canister_id, method, interval)
        })
    }

    pub fn task_main_cancel(task_id: u64) -> Result<bool, EgoError> {
        TASKSTORE.with(|s| {
            s.borrow_mut().task_cancel(task_id)
        })
    }
}