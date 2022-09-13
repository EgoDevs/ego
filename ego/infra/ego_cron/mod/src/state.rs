use std::cell::RefCell;

use crate::task::TaskStore;

thread_local! {
    pub static TASKSTORE: RefCell<TaskStore> = RefCell::new(TaskStore::new());
}
