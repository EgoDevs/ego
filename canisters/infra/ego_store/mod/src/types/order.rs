use std::borrow::Cow;
use candid::{Decode, Encode};
use ic_ledger_types::{AccountIdentifier, Memo, Subaccount};
use ic_stable_structures::{BoundedStorable, Storable};
use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use ego_utils::util::time;
use crate::memory::ORDERS;
use crate::state::SEQ;

#[derive(
CandidType, Serialize, Deserialize, Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq,
)]
pub enum OrderStatus {
  NEW,
  SUCCESS,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Order {
  pub wallet_id: Principal,
  pub from: AccountIdentifier,
  pub to: AccountIdentifier,
  pub amount: f32,
  pub memo: Memo,
  pub status: OrderStatus,
  pub last_update: u64  // second
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OrderType {
  APP,
  RECHARGE,
}

impl Order {
  pub fn new(wallet_id: &Principal, store_id: &Principal, amount: f32) -> Self {
    let memo = SEQ.with(|cell| cell.borrow_mut().next_number("order", 0));
    let mut bytes = [0u8; 32];
    let mut subaccount = Subaccount(bytes);
    let from = AccountIdentifier::new(&wallet_id, &subaccount);

    bytes
      .split_at_mut(8)
      .0
      .copy_from_slice(memo.to_le_bytes().as_slice());
    subaccount = Subaccount(bytes);
    let to = AccountIdentifier::new(store_id, &subaccount);

    Order {
      wallet_id: wallet_id.clone(),
      from,
      to,
      amount,
      memo: Memo(memo),
      status: OrderStatus::NEW,
      last_update: 0
    }
  }

  pub fn list() -> Vec<Order> {
    ORDERS.with(|cell| {
      let inst = cell.borrow();
      inst.iter()
        .map(|(_, order)| {
          order
        }).collect()
    })
  }

  pub fn by_wallet_id(wallet_id: &Principal) -> Vec<Order> {
    ORDERS.with(|cell| {
      let inst = cell.borrow();
      inst.iter()
        .filter(|(_, order)| order.wallet_id == wallet_id.clone())
        .map(|(_, order)| {
          order
        }).collect()
    })
  }

  pub fn get(memo: Memo) -> Option<Order>{
    ORDERS.with(|cell| {
      let inst = cell.borrow_mut();

      inst.get(&memo.0)
    })
  }

  pub fn save(&mut self) {
    ORDERS.with(|cell| {
      let mut inst = cell.borrow_mut();
      self.last_update = time() / 1000000000;

      inst.insert(self.memo.0, self.clone());
    });
  }
}

impl Storable for Order {
  fn to_bytes(&self) -> Cow<[u8]> {
    Cow::Owned(Encode!(self).unwrap())
  }

  fn from_bytes(bytes: Cow<[u8]>) -> Self  {
    Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl BoundedStorable for Order {
  const MAX_SIZE: u32 = 256;
  const IS_FIXED_SIZE: bool = false;
}