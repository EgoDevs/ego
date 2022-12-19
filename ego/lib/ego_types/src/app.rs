use std::fmt;

use candid::CandidType;
use ic_cdk::export::candid::Deserialize;
use ic_cdk::export::Principal;
use serde::Serialize;

use crate::version::Version;

pub type AppId = String;
pub type WasmId = String;
pub type FileId = String;

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
}

impl App {
  pub fn to_string(&self) -> String {
    format!(
      "app_id: {:?},category:{:?},current_version:{:?},",
      self.app_id, self.category, self.current_version
    )
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
    price: f32,
  ) -> Self {
    App {
      app_id,
      name,
      category,
      logo,
      description,
      current_version,
      price,
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

fn get_md5(data: &Vec<u8>) -> String {
  let digest = md5::compute(data);
  return format!("{:?}", digest);
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

#[derive(
CandidType, Serialize, Deserialize, Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq,
)]
pub enum DeployMode {
  SHARED,
  DEDICATED,
}
