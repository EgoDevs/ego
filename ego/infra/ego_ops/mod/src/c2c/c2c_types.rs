use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_types::Principal;
use ego_types::app::FileId;

// type for ego_file
#[derive(CandidType, Deserialize)]
pub struct FileMainWriteRequest {
  pub fid: FileId,
  pub hash: String,
  pub data: Vec<u8>,
}

// type for ego_dev
#[derive(CandidType, Deserialize)]
pub struct AdminEgoFileAddRequest {
  pub canister_id: Principal,
}
