use crate::state::EGO_CRON;
use crate::task::Task;
use ic_cdk::export::Principal;
use ic_cron::types::TaskId;

pub struct EgoCronService {}

impl EgoCronService {
    pub fn task_main_add(task_id: TaskId, task: Task) {
        EGO_CRON.with(|s| s.borrow_mut().task_add(task_id, task));
    }

    pub fn task_main_get(canister_id: Principal, method: String) -> Option<TaskId> {
        EGO_CRON.with(|s| s.borrow().task_get(canister_id, method))
    }

    pub fn task_main_cancel(task_id: TaskId) {
        EGO_CRON.with(|s| s.borrow_mut().task_cancel(task_id));
    }
}
