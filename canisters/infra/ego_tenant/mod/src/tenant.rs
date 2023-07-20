use ic_cdk::export::Principal;
use ic_stable_structures::storable::Blob;

use ego_types::app::EgoError;
use crate::memory::TASKS;
use crate::types::Task;


pub struct Tenant {
}

impl Tenant {
    pub fn canister_main_track(
        wallet_id: Principal,
        canister_id: Principal,
        next_check_time: u64,
    ) -> Result<(), EgoError> {
        TASKS.with(|cell| {
            let mut inst = cell.borrow_mut();
            let key = Blob::try_from(canister_id.as_slice()).unwrap();
            inst.insert(key, Task::new(wallet_id, canister_id, next_check_time, None))
        });
        Ok(())
    }

    pub fn canister_main_untrack(canister_id: Principal) -> Result<(), EgoError> {
        TASKS.with(|cell| {
            let mut inst = cell.borrow_mut();
            let key = Blob::try_from(canister_id.as_slice()).unwrap();
            inst.remove(&key);
        });

        Ok(())
    }

    pub fn task_get(canister_id: Principal) -> Option<Task> {
        TASKS.with(|cell| {
            let inst = cell.borrow();
            let key = Blob::try_from(canister_id.as_slice()).unwrap();
            inst.get(&key)
        })
    }

    pub fn task_filter(sentinel: u64) -> Vec<Task> {
        TASKS.with(|cell| {
            let inst = cell.borrow();
            inst.iter().filter(|(_, task)| task.next_check_time <= sentinel)
              .map(|(_, task)| task.clone()).collect()
        })
    }

    pub fn task_all() -> Vec<Task> {
        TASKS.with(|cell| {
            let inst = cell.borrow();
            inst.iter()
              .map(|(_, task)| task.clone()).collect()
        })
    }

    pub fn task_update(wallet_id: Principal, canister_id: Principal, next_check_time: u64, last_cycle: u128) {
        TASKS.with(|cell| {
            let mut inst = cell.borrow_mut();
            let key = Blob::try_from(canister_id.as_slice()).unwrap();
            inst.insert(key, Task::new(wallet_id, canister_id, next_check_time, Some(last_cycle)))
        });
    }
}
