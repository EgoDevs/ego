use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use serde::Serialize;

use ego_types::app::AppId;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct WalletProvider {
    pub wallet_provider: Principal,
    pub app_id: AppId,
}

impl WalletProvider {
    pub fn new(wallet_provider: &Principal, app_id: &AppId) -> Self {
        WalletProvider {
            wallet_provider: wallet_provider.clone(),
            app_id: app_id.clone(),
        }
    }
}
