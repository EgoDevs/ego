use std::cell::RefCell;
use crate::ego_cron::EgoCron;

thread_local! {
    pub static EGO_CRON: RefCell<EgoCron> = RefCell::new(EgoCron::new());
}
