mod actor;

#[allow(dead_code)]
#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[allow(dead_code)]
#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
  use crate::actor::InitArg;
  use ego_assets_mod::rc_bytes::*;
  use ego_assets_mod::state_machine::*;
  use ego_assets_mod::types::*;
  use ic_cdk::export::Principal;
  candid::export_service!();
  std::print!("{}", __export_service());
}
