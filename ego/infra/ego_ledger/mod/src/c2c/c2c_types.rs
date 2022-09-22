use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_ledger_types::Memo;
use serde::Serialize;

// type for ego_store
#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletOrderNotifyRequest {
  pub memo: Memo
}