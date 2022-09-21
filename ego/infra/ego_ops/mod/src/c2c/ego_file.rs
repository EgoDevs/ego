use ic_cdk::api;
use ic_types::Principal;

use async_trait::async_trait;
use ego_types::app::FileId;
use ego_types::ego_error::EgoError;
use crate::c2c::c2c_types::{FileMainWriteRequest};

#[async_trait]
pub trait TEgoFile {
  async fn role_user_add(&self, canister_id: Principal, fid: FileId, hash: String, data: Vec<u8>) -> Result<bool, EgoError>;
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
  async fn file_main_write(&self, canister_id: Principal, fid: FileId, hash: String, data: Vec<u8>) -> Result<bool, EgoError>{
    let req = FileMainWriteRequest {
      fid, hash, data
    };

    let notify_result = api::call::notify(
      canister_id,
      "file_main_write",
      (req,),
    );

    match notify_result {
      Err(code) => {
        let code = code as u16;
        Err(EgoError { code, msg: "file_main_write failed".to_string() })
      },
      _ => Ok(true)
    }
  }
}