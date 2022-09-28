use ic_ledger_types::Memo;
use ic_cdk::export::Principal;
use ego_types::app::{App, AppId, Canister};
use ego_types::ego_error::EgoError;

use crate::order::Order;
use crate::state::EGO_STORE;
use crate::types::QueryParam;

pub struct EgoStoreService {}

impl EgoStoreService {
    pub fn app_main_list(query_param: QueryParam) -> Result<Vec<App>, EgoError> {
        EGO_STORE.with(|ego_store| ego_store.borrow().app_main_list(&query_param))
    }

    pub fn app_main_get(app_id: AppId) -> Result<App, EgoError> {
        EGO_STORE.with(
            |ego_store| ego_store.borrow().app_main_get(&app_id)
        )
    }

    pub fn wallet_main_new(
        wallet_id: Principal,
    ) -> Result<Principal, EgoError> {
        EGO_STORE.with(|ego_store| {
              ego_store
                .borrow_mut().wallet_main_new(wallet_id)
        })
    }

    pub fn wallet_tenant_get(wallet_id: Principal) -> Result<Principal, EgoError> {
        EGO_STORE.with(|ego_store| {
            ego_store
              .borrow().wallet_tenant_get(&wallet_id)
        })
    }

    pub fn wallet_app_list(wallet_id: Principal) -> Result<Vec<App>, EgoError>{
        EGO_STORE.with(|ego_store| {
            ego_store
              .borrow().wallet_app_list(&wallet_id)
        })
    }

    pub fn wallet_app_install(wallet_id: Principal, app_id: String) -> Result<Vec<Canister>, EgoError> {
        // TODO: add actual implementation
        EGO_STORE.with(|ego_store| {
            ego_store
              .borrow_mut().wallet_app_install(&wallet_id, &app_id)
        })
    }

    pub fn wallet_app_upgrade(wallet_id: Principal, app_id: String) -> Result<Vec<Canister>, EgoError> {
        // TODO: add actual implementation
        EGO_STORE.with(|ego_store| {
            ego_store
              .borrow_mut().wallet_app_upgrade(&wallet_id, &app_id)
        })
    }

    pub fn wallet_app_remove(wallet_id: Principal, app_id: String) -> Result<Vec<Canister>, EgoError> {
        EGO_STORE.with(|ego_store| {
            ego_store
              .borrow_mut().wallet_app_remove(&wallet_id, &app_id)
        })
    }

    pub fn wallet_order_list(wallet_id: Principal) -> Result<Vec<Order>, EgoError> {
        EGO_STORE.with(|ego_store| {
            ego_store
              .borrow().wallet_order_list(&wallet_id)
        })
    }

    pub fn wallet_order_new(wallet_id: Principal, store_id: Principal, amount: f32) -> Result<Order, EgoError> {
        EGO_STORE.with(|ego_store| {
            ego_store
              .borrow_mut().wallet_order_new(&wallet_id, &store_id, amount)
        })
    }

    pub fn wallet_order_notify(memo: Memo) -> Result<bool, EgoError> {
        EGO_STORE.with(|ego_store| {
            ego_store.borrow_mut().wallet_order_notify(memo)
        })
    }

    pub fn admin_ego_tenant_add(tenant_id: Principal) -> Result<bool, EgoError>{
        EGO_STORE.with(|ego_store| {
            ego_store
              .borrow_mut().admin_tenant_add(&tenant_id)
        })
    }

    pub fn app_main_release(app: App) -> Result<bool, EgoError> {
        EGO_STORE.with(|ego_store| {
            ego_store
              .borrow_mut().app_main_release(app)
        })
    }
}