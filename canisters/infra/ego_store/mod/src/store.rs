use std::collections::BTreeMap;

use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_ledger_types::Memo;
use serde::Serialize;

use ego_types::app::{App, AppId, CashFlow, UserApp};
use ego_types::app::EgoError;

use crate::app::EgoStoreApp;
use crate::order::{Order, OrderStatus};
use crate::state::error_log_add;
use crate::tenant::Tenant;
use crate::types::EgoStoreErr;
use crate::wallet::*;
use crate::wallet_provider::WalletProvider;

/********************  app store  ********************/
#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub struct EgoStore {
  pub apps: BTreeMap<AppId, EgoStoreApp>,
  pub orders: BTreeMap<Memo, Order>,
  pub wallets: BTreeMap<Principal, Wallet>,
  pub tenants: BTreeMap<Principal, Tenant>,
  pub next_order_id: u64,
  pub wallet_providers: BTreeMap<Principal, WalletProvider>,
}

impl EgoStore {
  pub fn new() -> Self {
    EgoStore {
      apps: BTreeMap::new(),
      orders: BTreeMap::new(),
      wallets: BTreeMap::new(),
      tenants: BTreeMap::new(),
      next_order_id: 1,
      wallet_providers: BTreeMap::new(),
    }
  }

  pub fn app_main_list(&self) -> Result<Vec<App>, EgoError> {
    Ok(self
      .apps
      .iter()
      .map(|(_app_id, ego_store_app)| {
        ego_store_app.app.clone()
      })
      .collect())
  }

  pub fn app_main_get(&self, app_id: &AppId) -> Result<EgoStoreApp, EgoError> {
    match self.apps.get(app_id) {
      Some(app) => Ok(app.clone()),
      None => {
        error_log_add("app_main_get: app not exists");
        Err(EgoStoreErr::AppNotExists.into())
      },
    }
  }

  pub fn wallet_main_get(&self, wallet_id: Principal) -> Result<Wallet, EgoError> {
    match self.wallets.get(&wallet_id) {
      Some(wallet) => Ok(wallet.clone()),
      None => {
        error_log_add("wallet_main_get: wallet not exists");
        Err(EgoStoreErr::WalletNotExists.into())
      },
    }
  }

  pub fn wallet_main_register(
    &mut self,
    wallet_id: Principal,
    user_id: Principal,
  ) -> Result<Principal, EgoError> {
    match self.wallets.get(&wallet_id) {
      Some(wallet) => Ok(wallet.tenant_id),
      None => {
        let tenant_id = self.tenant_get()?;
        let wallet = self
          .wallets
          .entry(wallet_id)
          .or_insert(Wallet::new(tenant_id, wallet_id, user_id));

        Ok(wallet.tenant_id)
      }
    }
  }

  pub fn wallet_tenant_get(&self, wallet_id: &Principal) -> Result<Principal, EgoError> {
    match self.wallets.get(wallet_id) {
      Some(wallet) => Ok(wallet.tenant_id),
      None => {
        error_log_add("wallet_tenant_get: wallet not exists");
        Err(EgoStoreErr::WalletNotExists.into())
      },
    }
  }

  pub fn wallet_app_list(&self, wallet_id: &Principal) -> Result<Vec<UserApp>, EgoError> {
    match self.wallets.get(wallet_id) {
      None => {
        error_log_add("wallet_app_list: wallet not exists");
        Err(EgoStoreErr::WalletNotExists.into())
      },
      Some(wallet) => Ok(wallet
        .apps
        .iter()
        .map(|(_canister_id, user_app)| {
          let app = &self.apps.get(&user_app.app.app_id).unwrap().app;
          let mut ret_user_app = user_app.clone();
          ret_user_app.latest_version = app.current_version;
          ret_user_app
        })
        .collect()),
    }
  }

  pub fn wallet_app_get(
    &self,
    wallet_id: &Principal,
    canister_id: &Principal,
  ) -> Result<UserApp, EgoError> {
    match self.wallets.get(wallet_id) {
      None => {
        error_log_add("wallet_app_get: wallet not exists");
        Err(EgoStoreErr::WalletNotExists.into())
      },
      Some(wallet) => match wallet.apps.get(canister_id) {
        None => Err(EgoStoreErr::AppNotInstall.into()),
        Some(user_app) => {
          let app = &self.apps.get(&user_app.app.app_id).unwrap().app;
          let mut ret_user_app = user_app.clone();
          ret_user_app.latest_version = app.current_version;
          Ok(ret_user_app)
        }
      },
    }
  }

  pub fn wallet_app_install(
    &mut self,
    wallet_id: &Principal,
    user_app: &UserApp,
  ) {
    self.wallets
      .get_mut(wallet_id)
      .unwrap()
      .app_install(user_app);
  }

  pub fn wallet_app_upgrade(&mut self, wallet_id: &Principal, user_app: &UserApp, ego_store_app: &EgoStoreApp) {
    self.wallets
      .get_mut(wallet_id)
      .unwrap()
      .app_upgrade(user_app, ego_store_app);
  }

  pub fn wallet_app_remove(
    &mut self,
    wallet_id: &Principal,
    canister_id: &Principal,
  ) -> Result<(), EgoError> {
    match self.wallets.get_mut(wallet_id) {
      None => {
        error_log_add("wallet_app_remove: wallet not exists");
        Err(EgoStoreErr::WalletNotExists.into())
      },
      Some(wallet) => {
        wallet.app_remove(canister_id);
        Ok(())
      }
    }
  }

  pub fn wallet_order_list(&self, wallet_id: &Principal) -> Result<Vec<Order>, EgoError> {
    match self.wallets.get(wallet_id) {
      None => {
        error_log_add("wallet_order_list: wallet not exists");
        Err(EgoStoreErr::WalletNotExists.into())
      },
      Some(wallet) => Ok(wallet
        .orders
        .iter()
        .map(|memo| self.orders.get(memo).unwrap().clone())
        .collect()),
    }
  }

  pub fn wallet_order_list_all(&self) -> Vec<Order> {
    self.orders.values().cloned().collect()
  }

  pub fn wallet_order_new(
    &mut self,
    wallet_id: &Principal,
    store_id: &Principal,
    amount: f32,
  ) -> Result<Order, EgoError> {
    match self.wallets.get_mut(wallet_id) {
      None => {
        error_log_add("wallet_order_new: wallet not exists");
        Err(EgoStoreErr::WalletNotExists.into())
      },
      Some(wallet) => {
        let order_id = self.next_order_id;
        self.next_order_id += 1;
        let order = wallet.order_new(store_id, amount, order_id);
        self.orders.insert(order.memo, order.clone());
        Ok(order)
      }
    }
  }

  pub fn wallet_cycle_list(&self, wallet_id: &Principal) -> Result<Vec<CashFlow>, EgoError> {
    match self.wallets.get(wallet_id) {
      None => {
        error_log_add("wallet_cycle_list: wallet not exists");
        Err(EgoStoreErr::WalletNotExists.into())
      },
      Some(wallet) => {
        Ok(wallet.cash_flowes.clone())
      }
    }
  }

  pub fn wallet_order_notify(
    &mut self,
    memo: Memo,
    operator: Principal,
    ts: u64,
  ) -> Result<bool, EgoError> {
    match self.orders.get_mut(&memo) {
      Some(order) => {
        order.status = OrderStatus::SUCCESS;

        match self.wallets.get_mut(&order.wallet_id) {
          None => {
            error_log_add("wallet_order_notify: wallet not exists");
            Err(EgoStoreErr::WalletNotExists.into())
          },
          Some(wallet) => {
            // TODO: Add Real Recharge Logic
            let cycle = (order.amount * 1_000_000f32) as u128;
            Ok(wallet.cycle_recharge(
              cycle,
              operator,
              ts,
              format!("wallet cycle recharge, order memo {}", memo.0),
            ))
          }
        }
      }
      None => {
        error_log_add("wallet_order_notify: order not exists");
        Err(EgoStoreErr::OrderNotExists.into())
      },
    }
  }

  pub fn wallet_cycle_balance(&self, wallet_id: &Principal) -> Result<u128, EgoError> {
    match self.wallets.get(&wallet_id) {
      None => {
        error_log_add("wallet_cycle_balance: wallet not exists");
        Err(EgoStoreErr::WalletNotExists.into())
      },
      Some(wallet) => Ok(wallet.cycles),
    }
  }

  pub fn wallet_cycle_charge(
    &mut self,
    wallet_id: Principal,
    cycle: u128,
    operator: Principal,
    ts: u64,
    comment: String,
  ) -> Result<bool, EgoError> {
    match self.wallets.get_mut(&wallet_id) {
      None => {
        error_log_add("wallet_cycle_charge: wallet not exits");
        Err(EgoStoreErr::WalletNotExists.into())
      },
      Some(wallet) => Ok(wallet.cycle_charge(cycle, operator, ts, comment)),
    }
  }

  pub fn wallet_cycle_recharge(
    &mut self,
    wallet_id: Principal,
    cycle: u128,
    operator: Principal,
    ts: u64,
    comment: String,
  ) -> Result<bool, EgoError> {
    match self.wallets.get_mut(&wallet_id) {
      None => {
        error_log_add("wallet_cycle_recharge: wallet not exits");
        Err(EgoStoreErr::WalletNotExists.into())
      },
      Some(wallet) => Ok(wallet.cycle_recharge(cycle, operator, ts, comment)),
    }
  }

  pub fn admin_tenant_add(&mut self, tenant_id: Principal) {
    self.tenants.entry(tenant_id).or_insert(Tenant::new(tenant_id));
  }

  pub fn admin_wallet_provider_add(
    &mut self,
    wallet_provider: &Principal,
    wallet_id: &AppId,
  ) {
    self.wallet_providers.entry(wallet_provider.clone()).and_modify(|provider| provider.app_id = wallet_id.clone()).or_insert(WalletProvider::new(wallet_provider, wallet_id));
  }

  pub fn app_main_release(&mut self, ego_store_app: EgoStoreApp) -> Result<bool, EgoError> {
    self.apps
      .entry(ego_store_app.app.app_id.clone())
      .and_modify(|exists_app| *exists_app = ego_store_app.clone())
      .or_insert(ego_store_app);

    Ok(true)
  }

  pub fn tenant_get(&self) -> Result<Principal, EgoError> {
    if self.tenants.len() == 0 {
      error_log_add("tenant_get: no tenant");
      Err(EgoStoreErr::NoTenant.into())
    } else {
      Ok(self.tenants.values().min().unwrap().canister_id)
    }
  }
}
