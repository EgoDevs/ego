use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CashFlow {
    pub cash_flow_type: CashFlowType,
    pub cycles: u128,
    pub balance: u128, // balance after the operation
    pub created_at: u64,
    pub operator: Principal,
    pub comment: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum CashFlowType {
    CHARGE,
    RECHARGE,
}

impl CashFlow {
    pub fn new(
        cash_flow_type: CashFlowType,
        cycles: u128,
        balance: u128,
        operator: Principal,
        comment: String,
    ) -> Self {
        let now = ic_cdk::api::time();
        CashFlow {
            cash_flow_type,
            cycles,
            balance,
            created_at: now,
            operator,
            comment,
        }
    }
}
