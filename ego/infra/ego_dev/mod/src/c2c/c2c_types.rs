use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;
use ego_types::app::{App, FileId};

// type for ego_file
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

// type for ego_store
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AppMainReleaseRequest {
  pub app: App
}