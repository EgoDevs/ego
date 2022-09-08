use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize)]
pub struct WalletAppInstallRequest {
    pub app_id: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize, Eq, PartialEq)]
pub struct WalletAppInstallResponse {
    pub canister_id: Principal
}