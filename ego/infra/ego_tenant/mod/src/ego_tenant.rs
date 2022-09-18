use std::collections::{BTreeMap, BTreeSet, HashMap};
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_types::Principal;
use serde::Serialize;
use ego_utils::types::{AppId, EgoError};
use crate::task::Task;
use crate::types::EgoTenantErr;
use crate::wallet::Wallet;

#[derive(CandidType, Deserialize, Serialize)]
pub struct EgoTenant {
  pub wallets: BTreeMap<Principal, Wallet>,
  pub tasks: BTreeSet<Task>
}

impl EgoTenant {
  pub fn new() -> Self {
    EgoTenant {
      wallets: Default::default(),
      tasks: Default::default()
    }
  }

  pub fn wallet_main_add(&mut self, wallet_id: Principal) -> Result<bool, EgoError> {
    match self.wallets.get(&wallet_id) {
      Some(_) => Err(EgoTenantErr::WalletExists.into()),
      None => {
        self.wallets.insert(wallet_id.clone(), Wallet::new(wallet_id.clone()));
        Ok(true)
      }
    }
  }

  pub fn wallet_main_remove(&mut self, wallet_id: &Principal) -> Result<bool, EgoError> {
    match self.wallets.get(&wallet_id) {
      Some(_) => {
        self.wallets.retain(|wid, _| wid != wallet_id);
        Ok(true)
      },
      None => {
        Err(EgoTenantErr::WalletNotExists.into())

      }
    }
  }

  pub fn wallet_main_get(&self, wallet_id: &Principal) -> Result<Wallet, EgoError> {
    match self.wallets.get(&wallet_id) {
      Some(wallet) => {
        Ok(wallet.clone())
      },
      None => {
        Err(EgoTenantErr::WalletNotExists.into())
      }
    }
  }

  pub fn wallet_main_get_mut(&mut self, wallet_id: &Principal) -> Result<Wallet, EgoError> {
    match self.wallets.get_mut(&wallet_id) {
      Some(wallet) => {
        Ok(wallet.clone())
      },
      None => {
        Err(EgoTenantErr::WalletNotExists.into())
      }
    }
  }

  pub fn wallet_app_install(&mut self, wallet_id: &Principal, app_id: AppId, canisters: HashMap<String, Principal>) -> Result<bool, EgoError> {
    match self.wallets.get_mut(wallet_id) {
      None => Err(EgoTenantErr::WalletNotExists.into()),
      Some(wallet) => {
        wallet.app_install(app_id, canisters)
      }
    }
  }
}
