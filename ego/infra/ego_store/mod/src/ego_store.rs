use std::collections::BTreeMap;

use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_ledger_types::Memo;
use serde::Serialize;

use ego_types::app::{App, AppId};
use ego_types::ego_error::EgoError;
use ego_types::version::Version;

use crate::app::EgoStoreApp;
use crate::cash_flow::CashFlow;
use crate::order::{Order, OrderStatus};
use crate::tenant::Tenant;
use crate::types::{EgoStoreErr, QueryParam};
use crate::user_app::{UserApp, WalletApp};
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

  pub fn app_main_list(&self, query_param: &QueryParam) -> Result<Vec<App>, EgoError> {
    match query_param {
      QueryParam::ByCategory { category } => Ok(self
        .apps
        .iter()
        .filter_map(|(_app_id, app)| {
          if app.category == *category {
            Some(App::from(app.clone()))
          } else {
            None
          }
        })
        .collect()),
    }
  }

  pub fn app_main_get(&self, app_id: &AppId) -> Result<EgoStoreApp, EgoError> {
    match self.apps.get(app_id) {
      Some(app) => Ok(app.clone()),
      None => Err(EgoStoreErr::AppNotExists.into()),
    }
  }

  pub fn wallet_main_get(&self, wallet_id: Principal) -> Result<Wallet, EgoError> {
    match self.wallets.get(&wallet_id) {
      Some(wallet) => Ok(wallet.clone()),
      None => Err(EgoStoreErr::WalletNotExists.into()),
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
      None => Err(EgoStoreErr::WalletNotExists.into()),
    }
  }

  pub fn wallet_app_list(&self, wallet_id: &Principal) -> Result<Vec<UserApp>, EgoError> {
    match self.wallets.get(wallet_id) {
      None => Err(EgoStoreErr::WalletNotExists.into()),
      Some(wallet) => Ok(wallet
        .apps
        .iter()
        .map(|(app_id, user_app)| {
          let app = self.apps.get(app_id).unwrap();
          UserApp::new(user_app, app)
        })
        .collect()),
    }
  }

  pub fn wallet_app_get(
    &self,
    wallet_id: &Principal,
    app_id: AppId,
  ) -> Result<UserApp, EgoError> {
    match self.wallets.get(wallet_id) {
      None => Err(EgoStoreErr::WalletNotExists.into()),
      Some(wallet) => match wallet.apps.get(&app_id) {
        None => Err(EgoStoreErr::AppNotInstall.into()),
        Some(user_app) => {
          let app = self.apps.get(&app_id).unwrap();
          Ok(UserApp::new(user_app, app))
        }
      },
    }
  }

  pub fn wallet_app_install(
    &mut self,
    wallet_id: &Principal,
    app_id: &AppId,
    user_app: &WalletApp,
  ) {
    self.wallets
      .get_mut(wallet_id)
      .unwrap()
      .app_install(app_id, user_app);
  }

  pub fn wallet_app_upgrade(&mut self, wallet_id: &Principal, app_id: &AppId, version: &Version) {
    self.wallets
      .get_mut(wallet_id)
      .unwrap()
      .app_upgrade(app_id, version);
  }

  pub fn wallet_app_remove(
    &mut self,
    wallet_id: &Principal,
    app_id: &AppId,
  ) -> Result<(), EgoError> {
    match self.wallets.get_mut(wallet_id) {
      None => Err(EgoStoreErr::WalletNotExists.into()),
      Some(wallet) => match self.apps.get(app_id) {
        None => Err(EgoStoreErr::AppNotExists.into()),
        Some(_app) => {
          wallet.app_remove(&app_id);
          Ok(())
        }
      },
    }
  }

  pub fn wallet_order_list(&self, wallet_id: &Principal) -> Result<Vec<Order>, EgoError> {
    match self.wallets.get(wallet_id) {
      None => Err(EgoStoreErr::WalletNotExists.into()),
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

  pub fn wallet_cycle_list(&self, wallet_id: &Principal) -> Result<Vec<CashFlow>, EgoError> {
    match self.wallets.get(wallet_id) {
      None => Err(EgoStoreErr::WalletNotExists.into()),
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
          None => Err(EgoStoreErr::WalletNotExists.into()),
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
      None => Err(EgoStoreErr::OrderNotExists.into()),
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
      None => Err(EgoStoreErr::WalletNotExists.into()),
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
      None => Err(EgoStoreErr::WalletNotExists.into()),
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

  pub fn app_main_release(&mut self, app: EgoStoreApp) -> Result<bool, EgoError> {
    self.apps
      .entry(app.app_id.clone())
      .and_modify(|exists_app| *exists_app = app.clone())
      .or_insert(app);

    Ok(true)
  }

  pub fn user_app_get(&self, wallet_id: &Principal, app_id: &AppId) -> Result<WalletApp, EgoError> {
    match self.wallets.get(wallet_id) {
      None => Err(EgoStoreErr::WalletNotExists.into()),
      Some(wallet) => match wallet.apps.get(app_id) {
        None => Err(EgoStoreErr::AppNotInstall.into()),
        Some(user_app) => Ok(user_app.clone()),
      },
    }
  }

  pub fn tenant_get(&self) -> Result<Principal, EgoError> {
    if self.tenants.len() == 0 {
      Err(EgoStoreErr::NoTenant.into())
    } else {
      Ok(self.tenants.values().min().unwrap().canister_id)
    }
  }
}
