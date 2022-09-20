use ic_cdk::export::candid::{CandidType, Deserialize};
use ego_types::app::FileId;

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