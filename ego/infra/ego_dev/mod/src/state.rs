use std::cell::RefCell;

use crate::app::AppStore;

thread_local! {
  pub static APP_STORE: RefCell<AppStore> = RefCell::new(AppStore::new());
}