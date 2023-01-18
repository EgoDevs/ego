mod actor;

#[allow(dead_code)]
#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[allow(dead_code)]
#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
  use crate::actor::InitArg;
  use ego_ledger_mod::types::*;
  use ego_types::app::EgoError;
  use ego_types::cycle_info::*;
  use ic_cdk::export::Principal;
  use ego_ledger_mod::payment::Payment;

  candid::export_service!();
  std::print!("{}", __export_service());
}
