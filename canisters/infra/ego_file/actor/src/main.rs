mod actor;

#[allow(dead_code)]
#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[allow(dead_code)]
#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
  use crate::actor::InitArg;
  use ego_types::app::EgoError;
  use ic_cdk::export::Principal;
  use ego_types::app::FileId;
  use ego_types::cycle_info::*;

  candid::export_service!();
  std::print!("{}", __export_service());
}