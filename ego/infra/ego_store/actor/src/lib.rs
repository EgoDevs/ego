use candid::candid_method;
use ic_cdk_macros::*;
use ic_cdk::storage;

use ego_store_mod::ego_store::EgoStore;
use ego_store_mod::service::*;
use ego_store_mod::state::{EGO_STORE};
use ego_store_mod::types::*;
use ego_types::ego_error::EgoError;
use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;

use ego_users::inject_ego_users;

inject_ego_users!();

#[init]
#[candid_method(init)]
fn init() {
  let caller = ic_cdk::caller();
  ic_cdk::println!("ego-store: init, caller is {}", caller);

  ic_cdk::println!("==> add caller as the owner");
  users_init();
}

#[derive(CandidType, Deserialize, Serialize)]
struct PersistState{
  pub ego_store: EgoStore,
  pub user: User
}

#[pre_upgrade]
fn pre_upgrade() {
  ic_cdk::println!("ego-store: pre_upgrade");

  let ego_store = EGO_STORE.with(|ego_store| ego_store.borrow().clone());
  let user = users_pre_upgrade();

  let state = PersistState{ego_store, user};
  storage::stable_save((state, )).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
  ic_cdk::println!("ego-store: post_upgrade");

  let (state, ): (PersistState, ) = storage::stable_restore().unwrap();
  EGO_STORE.with(|ego_store|
    *ego_store.borrow_mut() = state.ego_store
  );

  users_post_upgrade(state.user);
}

/********************  methods for wallet   ********************/
#[query(name = "app_main_list")]
#[candid_method(query, rename = "app_main_list")]
pub fn app_main_list(request: AppMainListRequest) -> Result<AppMainListResponse, EgoError> {
  ic_cdk::println!("ego-store: app_main_list");
  match EgoStoreService::app_main_list(request.query_param) {
    Ok(apps) => Ok(AppMainListResponse { apps }),
    Err(e) => Err(e),
  }
}

#[query(name = "app_main_get")]
#[candid_method(query, rename = "app_main_get")]
pub fn app_main_get(request: GetAppRequest) -> Result<GetAppResponse, EgoError> {
  ic_cdk::println!("ego-store: app_main_get");
  match EgoStoreService::app_main_get(request.app_id) {
    Ok(app) => Ok(GetAppResponse { app }),
    Err(e) => Err(e),
  }
}

#[update(name = "wallet_main_new")]
#[candid_method(update, rename = "wallet_main_new")]
pub fn wallet_main_new() -> Result<WalletMainNewResponse, EgoError> {
  ic_cdk::println!("ego-store: wallet_main_new");
  match EgoStoreService::wallet_main_new(ic_cdk::caller()) {
    Ok(tenant_id) => Ok(WalletMainNewResponse { tenant_id }),
    Err(e) => Err(e),
  }
}

#[query(name = "wallet_tenant_get")]
#[candid_method(query, rename = "wallet_tenant_get")]
pub fn wallet_tenant_get() -> Result<WalletTenantGetResponse, EgoError> {
  ic_cdk::println!("ego-store: wallet_tenant_get");
  match EgoStoreService::wallet_tenant_get(ic_cdk::caller()) {
    Ok(tenant_id) => Ok(WalletTenantGetResponse { tenant_id }),
    Err(e) => Err(e),
  }
}

#[query(name = "wallet_app_list")]
#[candid_method(query, rename = "wallet_app_list")]
pub fn wallet_app_list() -> Result<WalletAppListResponse, EgoError> {
  ic_cdk::println!("ego_store: wallet_app_list");
  match EgoStoreService::wallet_app_list(ic_cdk::caller()) {
    Ok(apps) => Ok(WalletAppListResponse { apps }),
    Err(e) => Err(e),
  }
}

#[query(name = "wallet_app_install")]
#[candid_method(query, rename = "wallet_app_install")]
pub fn wallet_app_install(req: WalletAppInstallRequest) -> Result<WalletAppInstallResponse, EgoError> {
  ic_cdk::println!("ego_store: wallet_app_install");
  match EgoStoreService::wallet_app_install(ic_cdk::caller(), req.app_id) {
    Ok(canister_ids) => Ok(WalletAppInstallResponse { canister_ids }),
    Err(e) => Err(e),
  }
}

#[query(name = "wallet_app_upgrade")]
#[candid_method(query, rename = "wallet_app_upgrade")]
pub fn wallet_app_upgrade(req: WalletAppUpgradeRequest) -> Result<WalletAppUpgradeResponse, EgoError> {
  ic_cdk::println!("ego_store: wallet_app_upgrade");
  match EgoStoreService::wallet_app_upgrade(ic_cdk::caller(), req.app_id) {
    Ok(canister_ids) => Ok(WalletAppUpgradeResponse { canister_ids }),
    Err(e) => Err(e),
  }
}

#[query(name = "wallet_app_remove")]
#[candid_method(query, rename = "wallet_app_remove")]
pub fn wallet_app_remove(req: WalletAppRemoveRequest) -> Result<WalletAppRemoveResponse, EgoError> {
  ic_cdk::println!("ego_store: wallet_app_remove");
  match EgoStoreService::wallet_app_remove(ic_cdk::caller(), req.app_id) {
    Ok(canister_ids) => Ok(WalletAppRemoveResponse { canister_ids }),
    Err(e) => Err(e),
  }
}


#[query(name = "wallet_order_list")]
#[candid_method(query, rename = "wallet_order_list")]
pub fn wallet_order_list() -> Result<WalletOrderListResponse, EgoError> {
  ic_cdk::println!("ego_store: wallet_order_list");

  match EgoStoreService::wallet_order_list(ic_cdk::caller()) {
    Ok(orders) => Ok(WalletOrderListResponse { orders }),
    Err(e) => Err(e)
  }
}

#[update(name = "wallet_order_new")]
#[candid_method(update, rename = "wallet_order_new")]
pub async fn wallet_order_new(request: WalletOrderNewRequest) -> Result<WalletOrderNewResponse, EgoError> {
  ic_cdk::println!("ego_store: wallet_order_new");

  match EgoStoreService::wallet_order_new(ic_cdk::caller(), ic_cdk::id(), request.amount) {
    Ok(order) => Ok(WalletOrderNewResponse { order }),
    Err(e) => Err(e)
  }
}

// TODO: wallet_cycle_list


/********************  for ego_tenant  ********************/
// TODO: wallet_cycle_charge

/********************  for ego_dev  ********************/
#[update(name = "app_main_release")]
#[candid_method(update, rename = "app_main_release")]
pub async fn app_main_release(request: AppMainReleaseRequest) -> Result<AppMainReleaseResponse, EgoError> {
  ic_cdk::println!("ego_store: wallet_order_new");

  match EgoStoreService::app_main_release(request.app) {
    Ok(ret) => Ok(AppMainReleaseResponse { ret }),
    Err(e) => Err(e)
  }
}

/********************  ego-ledger callback  ********************/
// TODO: should add guard here, only ledger can call this method
#[update(name = "wallet_order_notify")]
#[candid_method(update, rename = "wallet_order_notify")]
pub fn wallet_order_notify(request: WalletOrderNotifyRequest) -> Result<WalletOrderNotifyResponse, EgoError> {
  ic_cdk::println!("ego_store: wallet_order_notify");

  match EgoStoreService::wallet_order_notify(request.memo) {
    Ok(ret) => Ok(WalletOrderNotifyResponse { ret }),
    Err(e) => Err(e),
  }
}

/********************  owner methods  ********************/
#[query(name = "admin_tenant_add")]
#[candid_method(query, rename = "admin_tenant_add")]
pub fn admin_ego_tenant_add(req: AdminEgoTenantAddRequest) -> Result<AdminEgoTenantAddResponse, EgoError> {
  ic_cdk::println!("ego_store: admin_tenant_add");

  match EgoStoreService::admin_ego_tenant_add(req.tenant_id) {
    Ok(ret) => Ok(AdminEgoTenantAddResponse { ret }),
    Err(e) => Err(e),
  }
}
