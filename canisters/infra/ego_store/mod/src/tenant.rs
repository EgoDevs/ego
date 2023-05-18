use std::cmp::Ordering;

use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Tenant {
    pub wallet_count: u16,
    pub canister_id: Principal,
}

impl Eq for Tenant {}

impl PartialEq<Self> for Tenant {
    fn eq(&self, other: &Self) -> bool {
        self.canister_id == other.canister_id
    }
}

impl PartialOrd<Self> for Tenant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.wallet_count.cmp(&other.wallet_count))
    }
}

impl Ord for Tenant {
    fn cmp(&self, other: &Self) -> Ordering {
        self.wallet_count.cmp(&other.wallet_count)
    }
}

impl Tenant {
    pub fn new(tenant_id: Principal) -> Self {
        Tenant {
            canister_id: tenant_id,
            wallet_count: 0,
        }
    }
}
