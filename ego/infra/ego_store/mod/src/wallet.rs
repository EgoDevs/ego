use std::collections::BTreeMap;

use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_ledger_types::Memo;
use serde::Serialize;

use ego_types::app::AppId;
use ego_types::version::Version;

use crate::cash_flow::{CashFlow, CashFlowType};
use crate::order::Order;
use crate::user_app::WalletApp;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Wallet {
  pub tenant_id: Principal,
  pub orders: Vec<Memo>,
  pub apps: BTreeMap<AppId, WalletApp>,
  pub cycles: u128,
  pub wallet_id: Principal,
  pub user_id: Principal,
  pub cash_flowes: Vec<CashFlow>,
}

impl Wallet {
  pub fn new(tenant_id: Principal, wallet_id: Principal, user_id: Principal) -> Self {
    Wallet {
      tenant_id,
      orders: vec![],
      apps: BTreeMap::new(),
      cycles: 0,
      wallet_id,
      user_id,
      cash_flowes: vec![],
    }
  }

  pub fn app_install(&mut self, app_id: &AppId, user_app: &WalletApp) {
    self.apps.insert(app_id.clone(), user_app.clone());
  }

  pub fn app_upgrade(&mut self, app_id: &AppId, version: &Version) {
    self.apps
      .entry(app_id.clone())
      .and_modify(|user_app| user_app.current_version = version.clone());
  }

  pub fn app_remove(&mut self, app_id: &AppId) {
    self.apps.remove(app_id);
  }

  pub fn order_new(&mut self, store_id: &Principal, amount: f32, memo: u64) -> Order {
    let order = Order::new(self.wallet_id, store_id, amount, memo);
    self.orders.push(order.memo);

    order
  }

  pub fn cycle_charge(&mut self, cycle: u128, operator: Principal, ts: u64, comment: String) -> bool {
    if self.cycles > cycle {
      self.cycles -= cycle;
      self.cash_flowes.push(CashFlow::new(
        CashFlowType::CHARGE,
        cycle,
        self.cycles,
        operator,
        ts,
        comment,
      ));
      true
    } else {
      false
    }
  }

  pub fn cycle_recharge(&mut self, cycle: u128, operator: Principal, ts: u64, comment: String) -> bool {
    self.cycles += cycle;
    self.cash_flowes.push(CashFlow::new(
      CashFlowType::RECHARGE,
      cycle,
      self.cycles,
      operator,
      ts,
      comment,
    ));
    true
  }
}
