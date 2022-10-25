use ic_cdk::api;
use ic_cdk::export::Principal;

use async_trait::async_trait;
use ego_types::ego_error::EgoError;
use ic_cdk::api::call::RejectionCode;

#[async_trait]
pub trait TEgoCanister {
    async fn balance_get(&self, canister_id: Principal) -> Result<u128, EgoError>;
}

pub struct EgoCanister {}

impl EgoCanister {
    pub fn new() -> Self {
        EgoCanister {}
    }
}

#[async_trait]
impl TEgoCanister for EgoCanister {
    async fn balance_get(&self, canister_id: Principal) -> Result<u128, EgoError> {
        let call_result = api::call::call(canister_id, "balance_get", ()).await
            as Result<(u128,), (RejectionCode, String)>;

        match call_result {
            Ok(resp) => Ok(resp.0),
            Err((code, msg)) => {
                let code = code as u16;
                Err(EgoError { code, msg })
            }
        }
    }
}
