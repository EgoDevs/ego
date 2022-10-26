use std::cell::RefCell;

use crate::ego_store::EgoStore;

thread_local! {
  pub static EGO_STORE: RefCell<EgoStore> = RefCell::new(EgoStore::new());
}
