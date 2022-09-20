use crate::ego_store::{EgoStore};
use std::cell::RefCell;

thread_local! {
  pub static APP_STORE: RefCell<EgoStore> = RefCell::new(EgoStore::new());
}