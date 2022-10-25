use crate::ego_tenant::EgoTenant;
use std::cell::RefCell;

thread_local! {
  pub static EGO_TENANT: RefCell<EgoTenant> = RefCell::new(EgoTenant::new());
}
