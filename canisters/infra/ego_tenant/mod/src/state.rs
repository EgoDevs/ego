use std::cell::RefCell;

use ego_macros::inject_ego_data;

use crate::tenant::Tenant;

inject_ego_data!();

/********************  methods for ego_registry   ********************/
fn on_canister_added(name: &str, canister_id: Principal) {
  let _ = match name {
    "ego_store" => user_add(canister_id),
    _ => {}
  };
}

thread_local! {
  pub static EGO_TENANT: RefCell<Tenant> = RefCell::new(Tenant::new());
}