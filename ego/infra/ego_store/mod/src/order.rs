use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_ledger_types::{AccountIdentifier, Memo, Subaccount};
use serde::Serialize;
use ic_types::Principal;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum OrderStatus {
  NEW,
  SUCCESS
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Order {
  pub wallet_id: Principal,
  pub from: AccountIdentifier,
  pub to: AccountIdentifier,
  pub amount: f32,
  pub memo: Memo,
  pub status: OrderStatus
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OrderType {
  APP,
  RECHARGE
}

impl Order {
  pub fn new(wallet_id: Principal, store_id: &Principal, amount: f32, memo: u64) -> Self{
    let mut bytes = [0u8;32];
    let mut subaccount = Subaccount(bytes);
    let from = AccountIdentifier::new(&wallet_id, &subaccount);

    bytes.split_at_mut(8).0.copy_from_slice(memo.to_le_bytes().as_slice());
    subaccount = Subaccount(bytes);
    let to = AccountIdentifier::new(store_id, &subaccount);

    Order{wallet_id, from, to, amount, memo: Memo(memo), status: OrderStatus::NEW}
  }
}