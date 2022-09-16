use std::cell::RefCell;
use crate::ego_dev::EgoDev;

thread_local! {
  pub static EGO_DEV: RefCell<EgoDev> = RefCell::new(EgoDev::new());
}