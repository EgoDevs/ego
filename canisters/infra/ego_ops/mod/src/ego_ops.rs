use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct EgoOps {}

impl EgoOps {
    pub fn new() -> Self {
        EgoOps {}
    }
}
