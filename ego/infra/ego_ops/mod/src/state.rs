use crate::ego_ops::EgoOps;
use std::cell::RefCell;

thread_local! {
  pub static EGO_OPS: RefCell<EgoOps> = RefCell::new(EgoOps::new());
}
