mod lib;

#[allow(dead_code)]
#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[allow(dead_code)]
#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
  use crate::lib::InitArg;
  use ego_ledger_mod::types::*;
  use ego_ledger_mod::service::LogEntry;
  use ego_types::ego_error::EgoError;
  use ic_cdk::export::Principal;
  use std::collections::BTreeMap;
  use ego_ledger_mod::payment::Payment;

  candid::export_service!();
  std::print!("{}", __export_service());
}
