use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_types::Principal;
use serde::Serialize;
use crate::version::Version;
use std::fmt;

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
  pub frontend: Wasm,
  pub backend: Wasm,
  pub price: f32
}


impl App {
  pub fn to_string(&self) -> String {
    format!("app_id: {:?},category:{:?},current_version:{:?},",
            self.app_id, self.category,self.current_version)
  }
}

impl App {
  pub fn new(app_id: AppId, name: String, category: Category, logo: String, description: String, current_version: Version, frontend: Wasm, backend: Wasm, price: f32) -> Self {
    App {
      app_id,
      name,
      category,
      logo,
      description,
      current_version,
      frontend,
      backend,
      price
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
  pub canister_id: Option<Principal>,

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
  pub fn new(app_id: AppId, version: Version, canister_type: CanisterType, canister_id: Option<Principal>) -> Self {
    Wasm { app_id, version, canister_type, canister_id }
  }

  /// id of wasm, will be the same across different version
  pub fn id(&self) -> WasmId {
    format!("{}|{}", self.app_id.clone(), self.canister_type)
  }

  /// unique id of wasm file
  pub fn fid(&self) -> FileId {
    get_md5(&format!("{}|{}|{}", self.app_id.clone(), self.canister_type, self.version.to_string()).into_bytes())
  }
}

fn get_md5(data: &Vec<u8>) -> String {
  let digest = md5::compute(data);
  return format!("{:?}", digest);
}