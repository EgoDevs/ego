use std::str::FromStr;
use ic_types::Principal;
use candid::{CandidType};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use ic_cdk::api::management_canister::main::CanisterStatusResponse;

// canister util types
#[derive(CandidType, Deserialize)]
pub struct CanisterUninstall {
  pub canister_id: Principal,
  pub arg: Vec<u8>,
}

// cycle util types
pub type Cycles = u128;

// error type
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct EgoError {
  pub code: u16,
  pub msg: String
}

impl EgoError{
  pub fn new(code: u16, msg: &str) -> Self{
    EgoError{code, msg: msg.to_string()}
  }
}

impl From<std::string::String> for EgoError {
  fn from(msg: String) -> Self {
    EgoError{code:255, msg}
  }
}

// version types

#[derive(
CandidType, Serialize, Deserialize, Clone, Copy, Debug, Default, Ord, PartialOrd, Eq, PartialEq,
)]
pub struct Version {
  pub major: u32,
  pub minor: u32,
  pub patch: u32,
}

impl Version {
  pub fn new(major: u32, minor: u32, patch: u32) -> Version {
    Version {
      major,
      minor,
      patch,
    }
  }

  pub fn min() -> Version {
    Version {
      major: 0,
      minor: 0,
      patch: 0,
    }
  }
}

impl FromStr for Version {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts: Vec<_> = s.split('.').collect();
    if parts.len() != 3 {
      return Err(format!("Unable to parse version: {}", s));
    }

    let major = u32::from_str(parts[0]).map_err(|e| e.to_string())?;
    let minor = u32::from_str(parts[1]).map_err(|e| e.to_string())?;
    let patch = u32::from_str(parts[2]).map_err(|e| e.to_string())?;

    Ok(Version {
      major,
      minor,
      patch,
    })
  }
}

impl ToString for Version {
  fn to_string(&self) -> String {
    format!("{}.{}.{}", self.major, self.minor, self.patch)
  }
}

// canister trait
#[async_trait]
pub trait Management {
  // canister relative methods
  async fn canister_main_create(&self, cycles_to_use: Cycles) -> Result<Principal, EgoError>;
  async fn canister_code_install(&self, canister_id: Principal, wasm_module: Vec<u8>) -> Result<(), EgoError>;
  async fn canister_code_upgrade(&self, canister_id: Principal, wasm_module: Vec<u8>) -> Result<(), EgoError>;

  async fn canister_status_get(&self, canister_id: Principal) -> Result<CanisterStatusResponse, EgoError>;
  async fn canister_controller_add(&self, canister_id: Principal, user_id: Principal) -> Result<(), EgoError>;
  async fn canister_controller_remove(&self, canister_id: Principal, user_id: Principal) -> Result<(), EgoError>;

  // cycle relative methods
  async fn canister_cycle_top_up(&self, canister_id: Principal, cycles_to_use: Cycles) -> Result<(), EgoError>;
}