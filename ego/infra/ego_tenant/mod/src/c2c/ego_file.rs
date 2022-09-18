use ic_cdk::api;
use ic_types::Principal;
use ego_utils::types::{EgoError, WasmId};
use ego_file_mod::types::{FileMainReadRequest, FileMainReadResponse};
use async_trait::async_trait;

#[async_trait]
pub trait TEgoFile {
  async fn file_main_read(&self, canister_id: Principal, fid: WasmId) -> Result<Vec<u8>, EgoError>;
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
  async fn file_main_read(&self, canister_id: Principal, fid: WasmId) -> Result<Vec<u8>, EgoError>{
    let req = FileMainReadRequest {
      fid
    };

    let call_result = api::call::call(
      canister_id,
      "file_main_read",
      (req,),
    )
      .await as Result<(Result<FileMainReadResponse, EgoError>,), _>;

    match call_result.unwrap().0 {
      Ok(resp) => {
        Ok(resp.data)
      },
      Err(e) => {
        Err(e)
      }
    }
  }
}