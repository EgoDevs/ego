use ic_ledger_types::Memo;
use ic_types::Principal;
use ego_types::app::{App, AppId};
use ego_types::ego_error::EgoError;

use crate::order::Order;
use crate::state::APP_STORE;
use crate::types::QueryParam;

pub struct EgoStoreService {}

impl EgoStoreService {
    pub fn app_main_list(query_param: QueryParam) -> Result<Vec<App>, EgoError> {
        APP_STORE.with(|app_store| app_store.borrow().app_main_list(&query_param))
    }

    pub fn app_main_get(app_id: AppId) -> Result<App, EgoError> {
        APP_STORE.with(
            |app_store| app_store.borrow().app_main_get(&app_id)
        )
    }

    pub fn wallet_main_new(
        wallet_id: Principal,
    ) -> Result<Principal, EgoError> {
        APP_STORE.with(|app_store| {
              app_store
                .borrow_mut().wallet_main_new(wallet_id)
        })
    }

    pub fn wallet_tenant_get(wallet_id: Principal) -> Result<Principal, EgoError> {
        APP_STORE.with(|app_store| {
            app_store
              .borrow().wallet_tenant_get(&wallet_id)
        })
    }

    pub fn wallet_app_list(wallet_id: Principal) -> Result<Vec<App>, EgoError>{
        APP_STORE.with(|app_store| {
            app_store
              .borrow().wallet_app_list(&wallet_id)
        })
    }

    pub fn wallet_app_install(wallet_id: Principal, app_id: String) -> Result<Vec<Principal>, EgoError> {
        APP_STORE.with(|app_store| {
            app_store
              .borrow_mut().wallet_app_install(&wallet_id, &app_id)
        })
    }

    pub fn wallet_app_upgrade(wallet_id: Principal, app_id: String) -> Result<Vec<Principal>, EgoError> {
        APP_STORE.with(|app_store| {
            app_store
              .borrow_mut().wallet_app_upgrade(&wallet_id, &app_id)
        })
    }

    pub fn wallet_app_remove(wallet_id: Principal, app_id: String) -> Result<Vec<Principal>, EgoError> {
        APP_STORE.with(|app_store| {
            app_store
              .borrow_mut().wallet_app_remove(&wallet_id, &app_id)
        })
    }

    pub fn wallet_order_list(wallet_id: Principal) -> Result<Vec<Order>, EgoError> {
        APP_STORE.with(|app_store| {
            app_store
              .borrow().wallet_order_list(&wallet_id)
        })
    }

    pub fn wallet_order_new(wallet_id: Principal, store_id: Principal, amount: f32) -> Result<Order, EgoError> {
        APP_STORE.with(|app_store| {
            app_store
              .borrow_mut().wallet_order_new(&wallet_id, &store_id, amount)
        })
    }

    pub fn wallet_order_notify(memo: Memo) -> Result<bool, EgoError> {
        APP_STORE.with(|app_store| {
            app_store.borrow_mut().wallet_order_notify(memo)
        })
    }

    pub fn admin_tenant_add(tenant_id: Principal) -> Result<bool, EgoError>{
        APP_STORE.with(|app_store| {
            app_store
              .borrow_mut().admin_tenant_add(&tenant_id)
        })
    }
}

/********************  guard methods  ********************/
// #[inline(always)]
// pub fn manager_guard() -> Result<(), String> {
//     if APP_STORE.with(|app_store| app_store.borrow().is_manager(api::caller())) {
//         Ok(())
//     } else {
//         trap(&format!("{} unauthorized", api::caller()));
//     }
// }
//
// #[inline(always)]
// pub fn auditer_guard() -> Result<(), String> {
//     if APP_STORE.with(|app_store| app_store.borrow().is_app_auditer(api::caller())) {
//         Ok(())
//     } else {
//         trap(&format!("{} unauthorized", api::caller()));
//     }
// }
//
// #[inline(always)]
// pub fn developer_guard() -> Result<(), String> {
//     if APP_STORE.with(|app_store| app_store.borrow().is_app_developer(api::caller())) {
//         Ok(())
//     } else {
//         trap(&format!("{} unauthorized", api::caller()));
//     }
// }
