mod actor;

#[allow(dead_code)]
#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[allow(dead_code)]
#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
  use ego_types::app::EgoError;
  use ego_types::app::FileId;
  use ego_types::cycle_info::*;
  use candid::Principal;
  use std::collections::BTreeMap;

  candid::export_service!();
  std::print!("{}", __export_service());
}
