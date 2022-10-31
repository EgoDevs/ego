use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_ledger_types::{AccountIdentifier, Memo, Tokens};
use serde::Serialize;

// type for ego_ledger
#[derive(CandidType, Deserialize, Serialize)]
pub struct LedgerPaymentAddRequest {
  pub from: AccountIdentifier,
  pub to: AccountIdentifier,
  pub amount: Tokens,
  pub memo: Memo,
}