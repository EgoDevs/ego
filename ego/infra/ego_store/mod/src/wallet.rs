use ic_ledger_types::Memo;
use ic_types::Principal;
use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;
use ego_types::app::{App, AppId};
use ego_types::ego_error::EgoError;
use crate::order::Order;
use crate::types::EgoStoreErr;


#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Wallet {
  pub tenant_id: Principal,
  pub orders: Vec<Memo>,
  pub apps: Vec<AppId>,
  pub cycles: u64,
  pub wallet_id: Principal,
}

impl Wallet{
  pub fn new(tenant_id: Principal, wallet_id: Principal) -> Self{
    Wallet{tenant_id, orders: vec![], apps: vec![], cycles: 0, wallet_id}
  }

  // TODO: add actual install logic
  pub fn app_install(&mut self, app: &App) -> Result<Vec<Principal>, EgoError> {
    if self.apps.contains(&app.app_id) {
      Err(EgoStoreErr::AppAlreadyInstall.into())
    } else {
      self.apps.push(app.app_id.clone());
      Ok(vec![Principal::from_text("qaa6y-5yaaa-aaaaa-aaafa-cai".to_string()).unwrap()])
    }
  }

  // TODO: add actual upgrade logic
  pub fn app_upgrade(&mut self, app: &App) -> Result<Vec<Principal>, EgoError> {
    if self.apps.contains(&app.app_id) {
      Ok(vec![Principal::from_text("qaa6y-5yaaa-aaaaa-aaafa-cai".to_string()).unwrap()])
    } else {
      Err(EgoStoreErr::AppNotInstall.into())
    }
  }

  // TODO: add actual upgrade logic
  pub fn app_remove(&mut self, app: &App) -> Result<Vec<Principal>, EgoError> {
    if self.apps.contains(&app.app_id) {
      self.apps.retain(|app_id| *app_id != app.app_id);

      Ok(vec![Principal::from_text("qaa6y-5yaaa-aaaaa-aaafa-cai".to_string()).unwrap()])
    } else {
      Err(EgoStoreErr::AppNotInstall.into())
    }
  }

  pub fn order_new(&mut self, store_id: &Principal, amount: f32, memo: u64) -> Order{
    let order = Order::new(self.wallet_id, store_id, amount, memo);
    self.orders.push(order.memo);

    order
  }
}