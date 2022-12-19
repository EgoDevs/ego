use std::cell::RefCell;

use crate::ego_ops::EgoOps;

thread_local! {
  pub static EGO_OPS: RefCell<EgoOps> = RefCell::new(EgoOps::new());
}
