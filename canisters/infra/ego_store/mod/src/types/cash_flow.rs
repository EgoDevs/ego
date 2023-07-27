use std::borrow::Cow;
use candid::{Decode, Encode, Principal};
use ic_cdk::api::time;
use ic_stable_structures::{BoundedStorable, Storable};
use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;
use ego_types::app::CashFlowType;
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
  pub operator: Principal,
  pub comment: String,
}

impl CashFlow {
  pub fn new(
    wallet_id: Principal,
    cash_flow_type: CashFlowType,
    cycles: u128,
    balance: u128,
    operator: &Principal,
    comment: String,
  ) -> Self {
    let next_id = SEQ.with(|cell| cell.borrow_mut().next_number("cash_flow", 0));
    CashFlow {
      id: next_id,
      wallet_id: wallet_id.clone(),
      cash_flow_type,
      cycles,
      balance,
      created_at: 0,
      operator: operator.clone(),
      comment,
    }
  }

  pub fn by_last_update(last_update: u64)  -> Vec<CashFlow> {
    CASH_FLOWS.with(|cell| {
      let inst = cell.borrow();
      inst.iter()
        .filter(|(_, cash_flow)| cash_flow.created_at > last_update)
        .map(|(_, cash_flow)| {
          cash_flow
        }).collect()
    })
  }

  pub fn by_wallet_id(wallet_id: &Principal) -> Vec<CashFlow> {
    CASH_FLOWS.with(|cell| {
      let inst = cell.borrow();
      inst.iter()
        .filter(|(_, cash_flow)| cash_flow.wallet_id == wallet_id.clone())
        .map(|(_, cash_flow)| {
          cash_flow
        }).collect()
    })
  }

  pub fn save(&mut self) {
    CASH_FLOWS.with(|cell| {
      let mut inst = cell.borrow_mut();
      if self.created_at == 0 {
        self.created_at = time() / 1000000000;
      }
      inst.insert(self.id, self.clone());
    });
  }

  pub fn into_ego_cash_flow(&self) -> ego_types::app::CashFlow {
    ego_types::app::CashFlow{
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

  fn from_bytes(bytes: Cow<[u8]>) -> Self  {
    Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl BoundedStorable for CashFlow {
  const MAX_SIZE: u32 = 1024;
  const IS_FIXED_SIZE: bool = false;
}