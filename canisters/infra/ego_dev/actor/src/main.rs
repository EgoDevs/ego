mod actor;

#[allow(dead_code)]
#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[allow(dead_code)]
#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
  use ego_dev_mod::types::ego_dev_app::EgoDevApp;
  use ego_dev_mod::types::developer::*;
  use ego_dev_mod::types::app_version::AppVersion;
  use ego_dev_mod::types::*;
  use ego_types::app::*;
  use ego_types::cycle_info::*;
  use candid::Principal;
  use std::collections::BTreeMap;
  use ego_backup::backup_info::*;

  candid::export_service!();
  std::print!("{}", __export_service());
}
