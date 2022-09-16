use ic_cdk::api;
use ic_types::Principal;
use ego_utils::types::EgoError;
use ego_file_mod::types::{FileMainWriteRequest, FileMainWriteResponse};
use async_trait::async_trait;

#[async_trait]
pub trait TEgoFile {
  async fn file_main_write(&self, canister_id: Principal, fid: String, hash: String, data: Vec<u8>) -> Result<bool, EgoError>;
}

pub struct EgoFile {
}

impl EgoFile{
  pub fn new() -> Self {
    EgoFile{}
  }
}

#[async_trait]
impl TEgoFile for EgoFile {
  async fn file_main_write(&self, canister_id: Principal, fid: String, hash: String, data: Vec<u8>) -> Result<bool, EgoError>{
    let req = FileMainWriteRequest {
      fid, hash, data
    };

    let call_result = api::call::call(
      canister_id,
      "file_main_write",
      (req,),
    )
      .await as Result<(Result<FileMainWriteResponse, EgoError>,), _>;

    match call_result.unwrap().0 {
      Ok(resp) => {
        Ok(resp.ret)
      },
      Err(e) => {
        Err(e)
      }
    }
  }
}