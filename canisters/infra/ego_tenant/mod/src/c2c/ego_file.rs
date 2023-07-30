use async_trait::async_trait;
use candid::Principal;
use ic_cdk::api;
use ic_cdk::api::call::RejectionCode;

use ego_types::app::EgoError;
use ego_types::app::WasmId;

use crate::state::error_log_add;

#[async_trait]
pub trait TEgoFile {
  async fn file_main_read(
    &self,
    canister_id: Principal,
    fid: WasmId,
  ) -> Result<Vec<u8>, EgoError>;
}

pub struct EgoFile {}

impl EgoFile {
  pub fn new() -> Self {
    EgoFile {}
  }
}

#[async_trait]
impl TEgoFile for EgoFile {
  async fn file_main_read(
    &self,
    canister_id: Principal,
    fid: WasmId,
  ) -> Result<Vec<u8>, EgoError> {
    let call_result = api::call::call(canister_id, "file_main_read", (fid, )).await
      as Result<(Result<Vec<u8>, EgoError>, ), (RejectionCode, String)>;

    match call_result {
      Ok(resp) => match resp.0 {
        Ok(data) => Ok(data),
        Err(e) => {
          error_log_add(format!("file_main_read failed, err: {:?}", e).as_str());
          Err(e)
        }
      },
      Err((code, msg)) => {
        let code = code as u16;
        error_log_add(format!("file_main_read failed, code: {}, msg: {}", code, msg).as_str());
        Err(EgoError { code, msg })
      }
    }
  }
}
