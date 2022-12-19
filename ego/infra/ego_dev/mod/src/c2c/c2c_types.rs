use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;

use ego_types::app::{AppId, Category, DeployMode, Wasm};
use ego_types::version::Version;

// type for ego_store
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AppMainReleaseRequest {
  pub app: EgoStoreApp,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct EgoStoreApp {
  pub app_id: AppId,
  pub name: String,
  pub category: Category,
  pub logo: String,
  pub description: String,
  pub current_version: Version,
  pub frontend: Option<Wasm>,
  pub backend: Option<Wasm>,
  pub price: f32,
  pub deploy_mode: DeployMode,
}

// type for ego_file
pub type FileId = String;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct FileMainWriteRequest {
  pub fid: FileId,
  pub hash: String,
  pub data: Vec<u8>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct FileMainWriteResponse {
  pub ret: bool,
}