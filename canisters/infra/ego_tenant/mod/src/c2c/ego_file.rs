use async_trait::async_trait;
use ic_cdk::api;
use ic_cdk::api::call::RejectionCode;
use ic_cdk::export::Principal;

use ego_types::app::EgoError;
use ego_types::app::WasmId;

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
        let call_result = api::call::call(canister_id, "file_main_read", (fid,)).await
            as Result<(Result<Vec<u8>, EgoError>,), (RejectionCode, String)>;

        match call_result {
            Ok(resp) => match resp.0 {
                Ok(data) => Ok(data),
                Err(e) => Err(e),
            },
            Err((code, msg)) => {
                let code = code as u16;
                Err(EgoError { code, msg })
            }
        }
    }
}
