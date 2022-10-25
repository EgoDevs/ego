use crate::ego_log::EgoLog;
use std::cell::RefCell;

thread_local! {
  pub static EGO_LOG: RefCell<EgoLog> = RefCell::new(EgoLog::new());
}