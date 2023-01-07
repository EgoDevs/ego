mod actor;

#[allow(dead_code)]
#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[allow(dead_code)]
#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
  use crate::actor::InitArg;
  use ego_dev_mod::types::*;
  use ego_dev_mod::app::*;
  use ego_dev_mod::developer::*;
  use ego_types::app::*;
  use ic_cdk::export::Principal;

  candid::export_service!();
  std::print!("{}", __export_service());
}
