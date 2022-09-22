use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;
use ic_ledger_types::{AccountIdentifier, Memo, Tokens};

#[derive(Clone, Debug, Eq, PartialEq,  CandidType, Deserialize, Serialize)]
pub struct Payment {
  pub from: AccountIdentifier,
  pub to: AccountIdentifier,
  pub amount: Tokens,
  pub memo: Memo
}

impl Payment {
  pub fn new(from: AccountIdentifier, to: AccountIdentifier, amount: Tokens, memo: Memo) -> Self {
    Payment { from, to, amount, memo }
  }
}