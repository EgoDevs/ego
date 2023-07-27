use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_ledger_types::Memo;
use serde::Serialize;

use ego_types::app::EgoError;
use ego_types::app::{App, AppId};


use crate::memory::{EGO_STORE_APPS};
use crate::state::{error_log_add};
use crate::types::ego_store_app::EgoStoreApp;
use crate::types::{EgoStoreErr};
use crate::types::order::{Order, OrderStatus};
use crate::types::tenant::Tenant;
use crate::types::user_app::UserApp;
use crate::types::wallet::*;

/********************  app store  ********************/
#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub struct EgoStore {

}

impl EgoStore {
    pub fn app_main_list() -> Result<Vec<App>, EgoError> {
        let apps = EGO_STORE_APPS.with(|cell| {
            let inst = cell.borrow();
            inst.iter().map(|(_app_id, app_value)| app_value.app.clone()).collect()
        });
        Ok(apps)
    }

    pub fn app_main_get(app_id: &AppId) -> Result<EgoStoreApp, EgoError> {
        match EgoStoreApp::get(app_id)  {
            None => {
                error_log_add("app_main_get: app not exists");
                Err(EgoStoreErr::AppNotExists.into())
            }
            Some(ego_store_app) => {Ok(ego_store_app)}
        }
    }

    pub fn wallet_main_get(wallet_id: &Principal) -> Result<Wallet, EgoError> {
        match Wallet::get(wallet_id) {
            None => {
                error_log_add("wallet_main_get: wallet not exists");
                Err(EgoStoreErr::WalletNotExists.into())
            }
            Some(wallet) => {
                Ok(wallet)
            }
        }
    }

    pub fn wallet_main_register(
        wallet_id: &Principal,
        user_id: &Principal,
    ) -> Result<Principal, EgoError> {
        let tenant_id = EgoStore::tenant_get()?;
        let mut wallet = Wallet::new(&tenant_id, wallet_id, user_id);
        wallet.save();
        Ok(tenant_id)
    }

    pub fn wallet_app_list(wallet_id: &Principal) -> Vec<UserApp> {
        UserApp::by_wallet_id(wallet_id)
    }

    pub fn wallet_app_get(
        wallet_id: &Principal,
        canister_id: &Principal,
    ) -> Result<UserApp, EgoError> {
        match UserApp::get(canister_id){
            None => {
                error_log_add("wallet_app_get: app not exists");
                Err(EgoStoreErr::AppNotExists.into())
            }
            Some(user_app) => {
                match user_app.wallet_id.is_some() && user_app.wallet_id.unwrap() == *wallet_id {
                    true => {
                        Ok(user_app)
                    }
                    false => {
                        error_log_add("wallet_app_get: app not exists");
                        Err(EgoStoreErr::AppNotExists.into())
                    }
                }
            }
        }

    }

    pub fn wallet_app_install(wallet_id: &Principal, user_app: &mut UserApp) {
        match Wallet::get(wallet_id){
            None => {
                error_log_add("wallet_app_install: wallet not exists");
            }
            Some(mut wallet) => {
                wallet.app_install(user_app)
            }
        };
    }

    pub fn wallet_app_upgrade(
        wallet_id: &Principal,
        user_app: &mut UserApp,
        ego_store_app: &EgoStoreApp,
    ) {
        match Wallet::get(wallet_id){
            None => {
                error_log_add("wallet_app_upgrade: wallet not exists");
            }
            Some(mut wallet) => {
                wallet.app_upgrade(user_app, &ego_store_app.app.current_version)
            }
        };
    }

    pub fn wallet_app_remove(
        wallet_id: &Principal,
        user_app: &mut UserApp
    ) {
        match Wallet::get(wallet_id){
            None => {
                error_log_add("wallet_app_remove: wallet not exists");
            }
            Some(mut wallet) => {
                wallet.app_remove(user_app)
            }
        };
    }

    pub fn wallet_order_new(
        wallet_id: &Principal,
        store_id: &Principal,
        amount: f32,
    ) -> Result<Order, EgoError> {
        match Wallet::get(wallet_id) {
            None => {
                error_log_add("wallet_app_remove: wallet not exists");
                Err(EgoStoreErr::WalletNotExists.into())
            }
            Some(_) => {
                let mut order = Order::new(wallet_id, store_id, amount);
                order.save();
                Ok(order)
            }
        }
    }

    pub fn wallet_order_notify(
        memo: Memo,
        operator: &Principal,
    ) -> Result<(), EgoError> {
        match Order::get(memo) {
            None => {
                error_log_add("wallet_order_notify: order not exists");
                Err(EgoStoreErr::OrderNotExists.into())
            }
            Some(mut order) => {
                order.status = OrderStatus::SUCCESS;
                order.save();

                match Wallet::get(&order.wallet_id) {
                    None => {
                        error_log_add("wallet_order_notify: wallet not exists");
                        Err(EgoStoreErr::WalletNotExists.into())
                    }
                    Some(mut wallet) => {
                        let cycle = (order.amount.clone() * 1_000_000f32) as u128;
                        wallet.cycle_recharge(
                            cycle,
                            operator,
                            format!("wallet cycle recharge, order memo {}", memo.0)
                        )
                    }
                }
            }
        }
    }

    pub fn wallet_cycle_balance(wallet_id: &Principal) -> Result<u128, EgoError> {
        match Wallet::get(wallet_id) {
            None => {
                error_log_add("wallet_app_remove: wallet not exists");
                Err(EgoStoreErr::WalletNotExists.into())
            }
            Some(wallet) => {
                Ok(wallet.cycles)
            }
        }
    }

    pub fn wallet_cycle_charge(
        wallet_id: &Principal,
        cycle: u128,
        operator: &Principal,
        comment: String,
    ) -> Result<(), EgoError> {
        match Wallet::get(wallet_id) {
            None => {
                error_log_add("wallet_app_remove: wallet not exists");
                Err(EgoStoreErr::WalletNotExists.into())
            }
            Some(mut wallet) => {
                wallet.cycle_charge(cycle, operator, comment)
            }
        }
    }

    pub fn wallet_cycle_recharge(
        wallet_id: &Principal,
        cycle: u128,
        operator: &Principal,
        comment: String,
    ) -> Result<(), EgoError> {
        match Wallet::get(wallet_id) {
            None => {
                error_log_add("wallet_app_remove: wallet not exists");
                Err(EgoStoreErr::WalletNotExists.into())
            }
            Some(mut wallet) => {
                wallet.cycle_recharge(cycle, operator, comment)
            }
        }
    }

    pub fn app_main_release(ego_store_app: EgoStoreApp) -> Result<bool, EgoError> {
      ego_store_app.save();
      Ok(true)
    }

    pub fn tenant_get() -> Result<Principal, EgoError> {
        let tenants = Tenant::list();

        if tenants.len() == 0 {
            error_log_add("tenant_get: no tenant");
            Err(EgoStoreErr::NoTenant.into())
        } else {
            Ok(tenants.iter().min().unwrap().canister_id)
        }
    }
}
