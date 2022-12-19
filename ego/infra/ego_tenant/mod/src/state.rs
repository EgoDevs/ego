use std::cell::RefCell;

use crate::ego_tenant::EgoTenant;

thread_local! {
  pub static EGO_TENANT: RefCell<EgoTenant> = RefCell::new(EgoTenant::new());
}
