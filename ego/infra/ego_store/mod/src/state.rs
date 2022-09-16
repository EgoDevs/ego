use crate::ego_store::{AppStore};
use std::cell::RefCell;

thread_local! {
  pub static APP_STORE: RefCell<AppStore> = RefCell::new(AppStore::new());
}