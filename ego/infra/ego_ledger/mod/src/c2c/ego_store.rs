use crate::c2c::c2c_types::WalletOrderNotifyRequest;
use async_trait::async_trait;
use ego_types::ego_error::EgoError;
use ic_cdk::api;
use ic_cdk::export::Principal;
use ic_ledger_types::Memo;

#[async_trait]
pub trait TEgoStore {
    async fn wallet_order_notify(
        &self,
        canister_id: Principal,
        memo: Memo,
    ) -> Result<bool, EgoError>;
}

pub struct EgoStore {}

impl EgoStore {
    pub fn new() -> Self {
        EgoStore {}
    }
}

#[async_trait]
impl TEgoStore for EgoStore {
    async fn wallet_order_notify(
        &self,
        canister_id: Principal,
        memo: Memo,
    ) -> Result<bool, EgoError> {
        let req = WalletOrderNotifyRequest { memo };

        let notify_result = api::call::notify(canister_id, "wallet_order_notify", (req,));

        match notify_result {
            Err(code) => {
                let code = code as u16;
                Err(EgoError {
                    code,
                    msg: "wallet_order_notify notify failed".to_string(),
                })
            }
            _ => Ok(true),
        }
    }
}
