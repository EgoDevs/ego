use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_ledger_types::{AccountIdentifier, Memo, Tokens};
use serde::Serialize;

#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize, Serialize)]
pub struct Payment {
    pub from: AccountIdentifier,
    pub to: AccountIdentifier,
    pub amount: Tokens,
    pub memo: Memo,
    pub status: PaymentStatus,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum PaymentStatus {
    PENDING,
    CONFIRMED,
    NOTIFIED,
}

impl Payment {
    pub fn new(from: AccountIdentifier, to: AccountIdentifier, amount: Tokens, memo: Memo) -> Self {
        Payment {
            from,
            to,
            amount,
            memo,
            status: PaymentStatus::PENDING,
        }
    }
}
