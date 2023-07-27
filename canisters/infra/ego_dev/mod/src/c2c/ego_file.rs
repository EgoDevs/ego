use async_trait::async_trait;
use candid::Principal;
use ic_cdk::api;
use ic_cdk::api::call::RejectionCode;

use ego_types::app::EgoError;
use ego_types::app::FileId;

use crate::state::error_log_add;

#[async_trait]
pub trait TEgoFile {
    async fn file_main_write(
        &self,
        canister_id: Principal,
        fid: FileId,
        hash: String,
        data: Vec<u8>,
    ) -> Result<bool, EgoError>;
}

pub struct EgoFile {}

impl EgoFile {
    pub fn new() -> Self {
        EgoFile {}
    }
}

#[async_trait]
impl TEgoFile for EgoFile {
    async fn file_main_write(
        &self,
        canister_id: Principal,
        fid: FileId,
        hash: String,
        data: Vec<u8>,
    ) -> Result<bool, EgoError> {
        let call_result = api::call::call(canister_id, "file_main_write", (fid, hash, data)).await
            as Result<(Result<bool, EgoError>,), (RejectionCode, String)>;

        match call_result {
            Ok(resp) => match resp.0 {
                Ok(resp) => Ok(resp),
                Err(e) => Err(e),
            },
            Err((code, msg)) => {
                let code = code as u16;
                error_log_add(
                    format!("Error calling file_main_write code: {}, msg: {}", code, msg).as_str(),
                );
                Err(EgoError { code, msg })
            }
        }
    }
}
