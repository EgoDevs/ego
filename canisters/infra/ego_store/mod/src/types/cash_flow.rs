use std::borrow::Cow;

use candid::{Decode, Encode};
use candid::{CandidType, Deserialize, Principal};
use ic_stable_structures::{BoundedStorable, Storable};
use serde::Serialize;

use ego_types::app::CashFlowType;
use ego_utils::util::time;

use crate::memory::CASH_FLOWS;
use crate::state::SEQ;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct CashFlow {
  pub id: u64,
  pub wallet_id: Principal,
  pub cash_flow_type: CashFlowType,
  pub cycles: u128,
  pub balance: u128,
  // balance after the operation
  pub created_at: u64,
  // second
  pub operator: Principal,
  pub comment: String,
}

impl CashFlow {
  pub fn new(
    wallet_id: &Principal,
    cash_flow_type: CashFlowType,
    cycles: u128,
    balance: u128,
    operator: &Principal,
    comment: String,
  ) -> Self {
    let next_id = SEQ.with(|cell| cell.borrow_mut().next_number("cash_flow", 0));
    Self {
      id: next_id,
      wallet_id: *wallet_id,
      cash_flow_type,
      cycles,
      balance,
      created_at: 0,
      operator: *operator,
      comment,
    }
  }

  pub fn len() -> u64 {
    CASH_FLOWS.with(|cell| {
      let inst = cell.borrow();
      inst.len()
    })
  }

  pub fn list() -> Vec<Self> {
    Self::iter(|(_, cash_flow)| Some(cash_flow))
  }

  pub fn by_last_update(last_update: u64) -> Vec<Self> {
    Self::iter(|(_, cash_flow)| match cash_flow.created_at >= last_update {
      true => {
        Some(cash_flow)
      }
      false => { None }
    })
  }

  pub fn by_wallet_id(wallet_id: &Principal) -> Vec<Self> {
    Self::iter(|(_, cash_flow)| match cash_flow.wallet_id == *wallet_id {
      true => {
        Some(cash_flow)
      }
      false => { None }
    })
  }

  pub fn save(&mut self) {
    CASH_FLOWS.with(|cell| {
      let mut inst = cell.borrow_mut();
      if self.created_at == 0 {
        self.created_at = time();
      }
      inst.insert(self.id, self.clone());
    });
  }

  fn iter<F>(filter: F) -> Vec<Self>
    where F: FnMut((u64, Self)) -> Option<Self> {
    CASH_FLOWS.with(|cell| {
      let inst = cell.borrow();
      inst.iter().filter_map(filter).collect()
    })
  }
}

impl Into<ego_types::app::CashFlow> for CashFlow {
  fn into(self) -> ego_types::app::CashFlow {
    ego_types::app::CashFlow {
      cash_flow_type: self.cash_flow_type.clone(),
      cycles: self.cycles,
      balance: self.balance,
      created_at: self.created_at,
      operator: self.operator,
      comment: self.comment.clone(),
    }
  }
}

impl Storable for CashFlow {
  fn to_bytes(&self) -> Cow<[u8]> {
    Cow::Owned(Encode!(self).unwrap())
  }

  fn from_bytes(bytes: Cow<[u8]>) -> Self {
    Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl BoundedStorable for CashFlow {
  const MAX_SIZE: u32 = 1024;
  const IS_FIXED_SIZE: bool = false;
}