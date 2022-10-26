use crate::ego_dev::EgoDev;
use std::cell::RefCell;

thread_local! {
  pub static EGO_DEV: RefCell<EgoDev> = RefCell::new(EgoDev::new());
}
