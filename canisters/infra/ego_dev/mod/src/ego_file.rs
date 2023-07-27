use std::cmp::Ordering;

use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct EgoFile {
    pub wasm_count: u16,
    pub canister_id: Principal,
}

impl Eq for EgoFile {}

impl PartialEq<Self> for EgoFile {
    fn eq(&self, other: &Self) -> bool {
        self.canister_id == other.canister_id
    }
}

impl PartialOrd<Self> for EgoFile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.wasm_count.cmp(&other.wasm_count))
    }
}

impl Ord for EgoFile {
    fn cmp(&self, other: &Self) -> Ordering {
        self.wasm_count.cmp(&other.wasm_count)
    }
}

impl EgoFile {
    pub fn new(canister_id: Principal) -> Self {
        EgoFile {
            canister_id,
            wasm_count: 0,
        }
    }
}
