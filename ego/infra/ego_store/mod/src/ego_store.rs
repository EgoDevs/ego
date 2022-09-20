use std::collections::{BTreeMap};

use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_types::Principal;
use serde::Serialize;

use ic_ledger_types::{Memo};
use ego_types::app::{App, AppId};
use ego_types::ego_error::EgoError;

use crate::order::{Order, OrderStatus};
use crate::tenant::Tenant;
use crate::types::{EgoStoreErr, QueryParam};
use crate::wallet::*;

/********************  app store  ********************/
#[derive(CandidType, Deserialize, Serialize)]
pub struct EgoStore {
  pub apps: BTreeMap<AppId, App>,
  pub orders: BTreeMap<Memo, Order>,
  pub wallets: BTreeMap<Principal, Wallet>,
  pub tenants: BTreeMap<Principal, Tenant>,
  pub next_order_id: u64
}

impl EgoStore {
  pub fn new() -> Self {
    EgoStore {
      apps: BTreeMap::new(),
      orders: BTreeMap::new(),
      wallets: BTreeMap::new(),
      tenants: BTreeMap::new(),
      next_order_id: 1
    }
  }

  pub fn app_main_list(&self, query_param: &QueryParam) -> Result<Vec<App>, EgoError> {
    match query_param {
      QueryParam::ByCategory { category } => {
        Ok(self.apps.iter().filter_map(|(_app_id, app)| {
          if app.category == *category {
            Some(app.clone())
          } else {
            None
          }
        }).collect())
      }
    }
  }

  pub fn app_main_get(&self, app_id: &AppId) -> Result<App, EgoError> {
    match self.apps.get(app_id) {
      Some(app) => Ok(app.clone()),
      None => Err(EgoStoreErr::AppNotExists.into())
    }
  }


  pub fn wallet_main_new(&mut self, wallet_id: Principal) -> Result<Principal, EgoError> {
    match self.wallets.get(&wallet_id) {
      Some(_) => Err(EgoStoreErr::WalletExists.into()),
      None => {
        let tenant_id = self.tenant_get()?;
        let wallet = self.wallets.entry(wallet_id).or_insert(Wallet::new(tenant_id, wallet_id));

        Ok(wallet.tenant_id)
      }
    }
  }

  pub fn wallet_tenant_get(&self, wallet_id: &Principal) -> Result<Principal, EgoError> {
    match self.wallets.get(wallet_id) {
      Some(wallet) => Ok(wallet.tenant_id),
      None => Err(EgoStoreErr::WalletNotExists.into())
    }
  }

  pub fn wallet_app_list(&self, wallet_id: &Principal) -> Result<Vec<App>, EgoError>{
    match self.wallets.get(wallet_id) {
      None => Err(EgoStoreErr::WalletNotExists.into()),
      Some(wallet) => {
        Ok(wallet.apps.iter().map(|app_id| self.apps.get(app_id).unwrap().clone()).collect())
      }
    }
  }

  pub fn wallet_app_install(&mut self, wallet_id: &Principal, app_id: &str) -> Result<Vec<Principal>, EgoError> {
    match self.wallets.get_mut(wallet_id) {
      None => Err(EgoStoreErr::WalletNotExists.into()),
      Some(wallet) => {
        match self.apps.get(app_id) {
          None => {
            Err(EgoStoreErr::AppNotExists.into())
          }
          Some(app) => {
            Ok(wallet.app_install(app)?)
          }
        }
      }
    }
  }

  pub fn wallet_app_upgrade(&mut self, wallet_id: &Principal, app_id: &str) -> Result<Vec<Principal>, EgoError> {
    match self.wallets.get_mut(wallet_id) {
      None => Err(EgoStoreErr::WalletNotExists.into()),
      Some(wallet) => {
        match self.apps.get(app_id) {
          None => {
            Err(EgoStoreErr::AppNotExists.into())
          }
          Some(app) => {
            Ok(wallet.app_upgrade(app)?)
          }
        }
      }
    }
  }

  pub fn wallet_app_remove(&mut self, wallet_id: &Principal, app_id: &str) -> Result<Vec<Principal>, EgoError> {
    match self.wallets.get_mut(wallet_id) {
      None => Err(EgoStoreErr::WalletNotExists.into()),
      Some(wallet) => {
        match self.apps.get(app_id) {
          None => {
            Err(EgoStoreErr::AppNotExists.into())
          }
          Some(app) => {
            Ok(wallet.app_remove(app)?)
          }
        }
      }
    }
  }

  pub fn wallet_order_list(&self, wallet_id: &Principal) -> Result<Vec<Order>, EgoError>{
    match self.wallets.get(wallet_id) {
      None => Err(EgoStoreErr::WalletNotExists.into()),
      Some(wallet) => {
        Ok(wallet.orders.iter().map(|memo| self.orders.get(memo).unwrap().clone()).collect())
      }
    }
  }

  pub fn wallet_order_new(&mut self, wallet_id: &Principal, store_id: &Principal, amount: f32) -> Result<Order, EgoError>{
    match self.wallets.get_mut(wallet_id) {
      None => Err(EgoStoreErr::WalletNotExists.into()),
      Some(wallet) => {
        let order_id = self.next_order_id;
        self.next_order_id += 1;
        let order = wallet.order_new(store_id, amount, order_id);
        self.orders.insert(order.memo, order.clone());
        Ok(order)
      }
    }
  }

  pub fn wallet_order_notify(&mut self, memo: Memo) -> Result<bool, EgoError> {
    match self.orders.get_mut(&memo) {
      Some(order) => {
        order.status = OrderStatus::SUCCESS;
        Ok(true)
      },
      None => Err(EgoStoreErr::OrderNotExists.into())
    }
  }

  pub fn admin_tenant_add(&mut self, tenant_id: &Principal) -> Result<bool, EgoError> {
    match self.tenants.get(tenant_id) {
      Some(_) => Err(EgoStoreErr::TenantExists.into()),
      None => {
        self.tenants.insert(tenant_id.clone(), Tenant::new(tenant_id.clone()));
        Ok(true)
      }
    }
  }

  pub fn app_main_release(&mut self, app: App) -> Result<bool, EgoError>{
    self.apps.entry(app.app_id.clone()).and_modify(|exists_app| *exists_app = app.clone()).or_insert(app);

    Ok(true)
  }

  fn tenant_get(&self) -> Result<Principal, EgoError>{
    if self.tenants.len() == 0 {
      Err(EgoStoreErr::NoTenant.into())
    } else {
      Ok(self.tenants.values().min().unwrap().canister_id)
    }
  }
}

