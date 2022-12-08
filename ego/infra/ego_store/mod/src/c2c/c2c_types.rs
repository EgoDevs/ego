use ic_cdk::export::candid::{Deserialize};
use ic_ledger_types::{AccountIdentifier, Memo, Tokens};
use serde::Serialize;
use candid::CandidType;

// type for ego_ledger
#[derive(CandidType, Deserialize, Serialize)]
pub struct LedgerPaymentAddRequest {
  pub from: AccountIdentifier,
  pub to: AccountIdentifier,
  pub amount: Tokens,
  pub memo: Memo,
}