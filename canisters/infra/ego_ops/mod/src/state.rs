use std::cell::RefCell;

use ego_macros::{inject_cycle_info, inject_ego_data};

use crate::ego_ops::EgoOps;

inject_ego_data!();
inject_cycle_info!();

/********************  methods for ego_registry   ********************/
fn on_canister_added(name: &str, canister_id: Principal) {
    info_log_add(&format!(
        "on_canister_added name: {}, canister_id: {}",
        name, canister_id
    ));
}

thread_local! {
  pub static EGO_OPS: RefCell<EgoOps> = RefCell::new(EgoOps::new());
}
