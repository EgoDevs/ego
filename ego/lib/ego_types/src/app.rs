use std::fmt;
use std::str::FromStr;

use ic_cdk::api::call::RejectionCode;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use serde::Serialize;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct EgoError {
  pub code: u16,
  pub msg: String,
}

impl EgoError {
  pub fn new(code: u16, msg: &str) -> Self {
    EgoError {
      code,
      msg: msg.to_string(),
    }
  }
}

impl From<std::string::String> for EgoError {
  fn from(msg: String) -> Self {
    EgoError { code: 255, msg }
  }
}

impl From<(RejectionCode, std::string::String)> for EgoError {
  fn from((code, msg): (RejectionCode, String)) -> Self {
    EgoError {
      code: code as u16,
      msg,
    }
  }
}

pub type AppId = String;
pub type WasmId = String;
pub type FileId = String;

#[derive(CandidType, Serialize, Deserialize, Clone, Copy, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
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

#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum Category {
  System,
  Vault,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct App {
  pub app_id: AppId,
  pub name: String,
  pub category: Category,
  pub logo: String,
  pub description: String,
  pub current_version: Version,
  pub price: f32,
  pub app_hash: String
}

impl App {
  pub fn to_string(&self) -> String {
    format!("app_id: {:?}, category: {:?}, current_version: {:?},", self.app_id, self.category, self.current_version)
  }
}

impl App {
  pub fn new(
    app_id: AppId,
    name: String,
    category: Category,
    logo: String,
    description: String,
    current_version: Version,
    price: f32
  ) -> Self {
    let data= &format!("{}|{}", app_id.clone(), current_version.to_string()).into_bytes();
    let app_hash = get_md5(data);
    App {
      app_id,
      name,
      category,
      logo,
      description,
      current_version,
      price,
      app_hash
    }
  }

  pub fn app_hash_update(&mut self){
    let data= &format!("{}|{}", self.app_id, self.current_version.to_string()).into_bytes();
    self.app_hash = get_md5(data);
  }
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


#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Canister {
  pub canister_id: Principal,
  pub canister_type: CanisterType,
}

impl Canister {
  pub fn new(canister_id: Principal, canister_type: CanisterType) -> Self {
    Canister {
      canister_id,
      canister_type,
    }
  }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UserApp {
  pub app: App,
  pub canister: Canister,
  pub latest_version: Version,
}

impl UserApp {
  pub fn new(app: &App, canister: Canister) -> Self {
    UserApp {
      app: app.clone(),
      latest_version: app.current_version.clone(),
      canister,
    }
  }
}


#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Wasm {
  pub app_id: AppId,
  pub version: Version,
  pub canister_type: CanisterType,
  /// when canister_type is ASSET, this will be the shared frontend canister id
  /// when canister_type is BACKEND, this will be the ego_file canister id used to store the wasm datas
  pub canister_id: Principal,
}


impl Wasm {
  pub fn new(
    app_id: AppId,
    version: Version,
    canister_type: CanisterType,
    canister_id: Principal,
  ) -> Self {
    Wasm {
      app_id,
      version,
      canister_type,
      canister_id,
    }
  }

  /// id of wasm, will be the same across different version
  pub fn id(&self) -> WasmId {
    format!("{}|{}", self.app_id.clone(), self.canister_type)
  }

  /// unique id of wasm file
  pub fn fid(&self) -> FileId {
    get_md5(
      &format!(
        "{}|{}|{}",
        self.app_id.clone(),
        self.canister_type,
        self.version.to_string()
      )
        .into_bytes(),
    )
  }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CashFlow {
  pub cash_flow_type: CashFlowType,
  pub cycles: u128,
  pub balance: u128,
  // balance after the operation
  pub created_at: u64,
  pub operator: Principal,
  pub comment: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum CashFlowType {
  CHARGE,
  RECHARGE,
}

impl CashFlow {
  pub fn new(
    cash_flow_type: CashFlowType,
    cycles: u128,
    balance: u128,
    operator: Principal,
    ts: u64,
    comment: String,
  ) -> Self {
    CashFlow {
      cash_flow_type,
      cycles,
      balance,
      created_at: ts,
      operator,
      comment,
    }
  }
}

fn get_md5(data: &Vec<u8>) -> String {
  let digest = md5::compute(data);
  return format!("{:?}", digest);
}