use std::cell::RefCell;
use ic_cdk::export::Principal;
use crate::ego_dev::EgoDev;

thread_local! {
  pub static EGO_DEV: RefCell<EgoDev> = RefCell::new(EgoDev::new());
  pub static EGO_STORE_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
}