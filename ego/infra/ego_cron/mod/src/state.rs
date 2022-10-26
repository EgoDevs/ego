use crate::ego_cron::EgoCron;
use std::cell::RefCell;

thread_local! {
    pub static EGO_CRON: RefCell<EgoCron> = RefCell::new(EgoCron::new());
}
