use crate::ego_store::{EgoStore};
use std::cell::RefCell;

thread_local! {
  pub static EGO_STORE: RefCell<EgoStore> = RefCell::new(EgoStore::new());
}