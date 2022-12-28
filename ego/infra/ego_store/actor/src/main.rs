mod actor;

#[allow(dead_code)]
#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[allow(dead_code)]
#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
  use crate::actor::InitArg;
  use ego_store_mod::types::*;
  use ego_types::app::EgoError;
  use ic_cdk::export::Principal;
  use ego_store_mod::order::Order;
  use ego_types::app::WalletApp;
  use ego_types::app::UserApp;
  use ego_types::app::{AppId, App};

  candid::export_service!();
  std::print!("{}", __export_service());
}
