use ic_cron::types::TaskId;
use ic_types::Principal;
use crate::state::EGO_CRON;
use crate::task::Task;

pub struct EgoCronService {}

impl EgoCronService {
    pub fn task_main_add(canister_id: Principal, method: String) -> Option<Task> {
        EGO_CRON.with(|s| {
            s.borrow_mut().task_add(canister_id, method)
        })
    }

    pub fn task_main_cancel(canister_id: Principal, method: String) -> Option<TaskId> {
        let result = EGO_CRON.with(|s| {
            s.borrow().task_get(canister_id, method)
        });

        if result.is_some() {
            let task_id = result.unwrap();
            EGO_CRON.with(|s| {
                s.borrow_mut().task_cancel(task_id);
            });
            Some(task_id)
        } else {
            None
        }
    }
}