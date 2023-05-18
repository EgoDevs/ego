use async_trait::async_trait;
use ego_types::app::EgoError;
use ego_types::cycle_info::CycleRecord;
use ic_cdk::api;
use ic_cdk::api::call::RejectionCode;
use ic_cdk::export::Principal;
use tracing::error;

#[async_trait]
pub trait TEgoTenant {
    fn ego_cycle_check_cb(&self, records: Vec<CycleRecord>, threshold: u128);

    // cycle recharge
    async fn wallet_cycle_recharge(&self, cycles: u128) -> Result<(), EgoError>;
}

#[derive(Copy, Clone)]
pub struct EgoTenant {
    pub canister_id: Principal,
}

impl EgoTenant {
    pub fn new(canister_id: Principal) -> Self {
        EgoTenant { canister_id }
    }
}

#[async_trait]
impl TEgoTenant for EgoTenant {
    fn ego_cycle_check_cb(&self, records: Vec<CycleRecord>, threshold: u128) {
        let _result =
            api::call::notify(self.canister_id, "ego_cycle_check_cb", (records, threshold));
    }

    async fn wallet_cycle_recharge(&self, cycles: u128) -> Result<(), EgoError> {
        let call_result = api::call::call(self.canister_id, "wallet_cycle_recharge", (cycles,))
            .await
            as Result<(Result<(), EgoError>,), (RejectionCode, String)>;

        match call_result {
            Ok(resp) => match resp.0 {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            Err((code, msg)) => {
                let code = code as u16;
                error!(
                    error_code = code,
                    error_message = msg.as_str(),
                    "Error calling wallet_cycle_recharge"
                );
                Err(EgoError { code, msg })
            }
        }
    }
}
