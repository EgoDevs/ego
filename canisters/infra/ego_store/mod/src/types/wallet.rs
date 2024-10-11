use std::borrow::Cow;

use candid::{Decode, Encode};
use candid::{CandidType, Deserialize, Principal};
use ic_stable_structures::{BoundedStorable, Storable};
use ic_stable_structures::storable::Blob;
use serde::Serialize;

use ego_types::app::{CashFlowType, EgoError};
use ego_utils::util::time;

use crate::memory::WALLETS;
use crate::types::cash_flow::CashFlow;
use crate::types::EgoStoreErr;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Wallet {
  pub tenant_id: Principal,
  pub wallet_id: Principal,
  pub user_id: Principal,
  pub cycles: u128,
  pub last_update: u64,    // mini second
}

impl Wallet {
  pub fn new(tenant_id: &Principal, wallet_id: &Principal, user_id: &Principal) -> Self {
    Self {
      tenant_id: tenant_id.clone(),
      wallet_id: wallet_id.clone(),
      user_id: user_id.clone(),
      cycles: 0,
      last_update: 0,
    }
  }

  pub fn cycle_charge(
    &mut self,
    cycle: u128,
    operator: &Principal,
    comment: String,
  ) -> Result<(), EgoError> {
    if self.cycles > cycle {
      self.cycles -= cycle;
      self.save();

      let mut cash_flow = CashFlow::new(
        &self.wallet_id,
        CashFlowType::CHARGE,
        cycle,
        self.cycles,
        operator,
        comment,
      );
      cash_flow.save();

      Ok(())
    } else {
      Err(EgoStoreErr::CyclesNotEnouth.into())
    }
  }

  pub fn cycle_recharge(
    &mut self,
    cycle: u128,
    operator: &Principal,
    comment: String,
  ) -> Result<(), EgoError> {
    self.cycles += cycle;
    self.save();

    let mut cash_flow = CashFlow::new(
      &self.wallet_id,
      CashFlowType::RECHARGE,
      cycle,
      self.cycles,
      operator,
      comment,
    );
    cash_flow.save();
    Ok(())
  }

  pub fn len() -> u64 {
    WALLETS.with(|cell| {
      let inst = cell.borrow();
      inst.len()
    })
  }

  pub fn by_last_update(start: usize, end: usize, last_update: u64) -> Vec<Self> {
    Self::iter(start, end, |(_, wallet)| match wallet.last_update >= last_update {
      true => { Some(wallet) }
      false => { None }
    })
  }

  pub fn list(start: usize, end: usize) -> Vec<Self> {
    Self::iter(start, end, |(_, wallet)| Some(wallet))
  }

  pub fn get(wallet_id: &Principal) -> Option<Self> {
    WALLETS.with(|cell| {
      let inst = cell.borrow_mut();
      let key = Blob::try_from(wallet_id.as_slice()).unwrap();
      inst.get(&key)
    })
  }

  pub fn save(&mut self) {
    WALLETS.with(|cell| {
      let mut inst = cell.borrow_mut();
      let key = Blob::try_from(self.wallet_id.as_slice()).unwrap();
      self.last_update = time();
      inst.insert(key, self.clone());
    });
  }

  fn iter<F>(start: usize, end: usize, filter: F) -> Vec<Self>
  where
    F: Fn((Blob<29>, Self)) -> Option<Self>,
  {
    WALLETS.with(|cell| {
      let inst = cell.borrow();
      inst.iter().skip(start).take(end - start).filter_map(|entry| {
        filter(entry)
      }).collect()
    })
  }
}

impl Storable for Wallet {
  fn to_bytes(&self) -> Cow<[u8]> {
    Cow::Owned(Encode!(self).unwrap())
  }

  fn from_bytes(bytes: Cow<[u8]>) -> Self {
    Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl BoundedStorable for Wallet {
  const MAX_SIZE: u32 = 128;
  const IS_FIXED_SIZE: bool = false;
}