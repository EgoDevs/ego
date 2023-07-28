use std::borrow::Cow;
use candid::{Decode, Encode};

use candid::{CandidType, Deserialize, Principal};
use ic_stable_structures::{BoundedStorable, Storable};
use ic_stable_structures::storable::Blob;
use serde::Serialize;
use ego_types::app::{CashFlowType, EgoError, Version};
use ego_utils::util::time;

use crate::memory::{WALLETS};
use crate::types::cash_flow::CashFlow;
use crate::types::EgoStoreErr;

use crate::types::user_app::UserApp;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Wallet {
    pub tenant_id: Principal,
    pub wallet_id: Principal,
    pub user_id: Principal,
    pub cycles: u128,
    pub last_update: u64    // second
}

impl Wallet {
    pub fn new(tenant_id: &Principal, wallet_id: &Principal, user_id: &Principal) -> Self {
        Wallet {
            tenant_id: tenant_id.clone(),
            wallet_id: wallet_id.clone(),
            user_id: user_id.clone(),
            cycles: 0,
            last_update: 0
        }
    }

    pub fn app_install(&mut self, user_app: &mut UserApp) {
        user_app.save();
    }

    pub fn app_upgrade(&mut self, user_app: &mut UserApp, latest_version: &Version) {
        user_app.app.current_version = latest_version.clone();
        user_app.save();
    }

    pub fn app_remove(&mut self, user_app: &UserApp) {
        user_app.remove();
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
                self.wallet_id,
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
            self.wallet_id,
            CashFlowType::RECHARGE,
            cycle,
            self.cycles,
            operator,
            comment,
        );
        cash_flow.save();
        Ok(())
    }

    pub fn by_last_update(last_update: u64) -> Vec<Wallet> {
        WALLETS.with(|cell| {
            let inst = cell.borrow();
            inst.iter()
              .filter(|(_, wallet)| wallet.last_update > last_update)
              .map(|(_, wallet)| {
                  wallet
              }).collect()
        })
    }

    pub fn list() -> Vec<Wallet> {
        WALLETS.with(|cell| {
            let inst = cell.borrow_mut();
            inst.iter().map(|(_, wallet)| wallet).collect()
        })
    }

    pub fn get(wallet_id: &Principal) -> Option<Wallet> {
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
            self.last_update = time() / 1000000000;
            inst.insert(key, self.clone());
        });
    }
}

impl Storable for Wallet {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self  {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Wallet {
    const MAX_SIZE: u32 = 128;
    const IS_FIXED_SIZE: bool = false;
}