use std::fmt;
use std::str::FromStr;
use candid::{CandidType};
use serde::{Deserialize, Serialize};
use ic_types::Principal;

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

// app relative types
pub type AppId = String;
pub type WasmId = String;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum Category {
  System,
  Vault,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Wasm {
  pub id: String,
  pub app_id: AppId,
  pub version: Version,
  pub canister_type: CanisterType,
  /// share frontend canister id
  pub canister_id: Option<Principal>,
  /// unique id of wasm file
  pub fid: WasmId,
  pub file_id: Option<Principal>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum CanisterType {
  BACKEND,
  ASSET,
}

impl fmt::Display for CanisterType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}


impl Wasm {
  pub fn new(app_id: AppId, version: Version, canister_type: CanisterType, file_id: Option<Principal>) -> Self {
    let id = format!("{}|{}", app_id.clone(), canister_type);
    let fid = get_md5(&format!("{}|{}|{}", app_id.clone(), version.to_string(), canister_type).into_bytes());
    Wasm { id, app_id, version, canister_type, canister_id: None, fid, file_id }
  }
}

fn get_md5(data: &Vec<u8>) -> String {
  let digest = md5::compute(data);
  return format!("{:?}", digest);
}