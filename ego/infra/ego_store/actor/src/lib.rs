use candid::candid_method;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::storage;
use ic_cdk_macros::*;
use serde::Serialize;

use ego_store_mod::c2c::ego_tenant::EgoTenant;
use ego_store_mod::ego_store::EgoStore;
use ego_store_mod::service::*;
use ego_store_mod::state::EGO_STORE;
use ego_store_mod::types::*;
use ego_types::ego_error::EgoError;
use ego_users::inject_ego_users;

inject_ego_users!();


#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct InitArg {
  init_caller: Option<Principal>,
}

#[init]
#[candid_method(init)]
pub fn init(arg: InitArg) {
  let caller = arg.init_caller.unwrap_or(caller());
  ic_cdk::println!("ego-store: init, caller is {}", caller.clone());

  ic_cdk::println!("==> add caller as the owner");
  users_init(caller.clone());
}

#[derive(CandidType, Deserialize, Serialize)]
struct PersistState {
  pub ego_store: EgoStore,
  pub user: User,
}

#[pre_upgrade]
fn pre_upgrade() {
  ic_cdk::println!("ego-store: pre_upgrade");

  let ego_store = EGO_STORE.with(|ego_store| ego_store.borrow().clone());
  let user = users_pre_upgrade();

  let state = PersistState { ego_store, user };
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
pub fn app_main_get(request: AppMainGetRequest) -> Result<AppMainGetResponse, EgoError> {
  ic_cdk::println!("ego-store: app_main_get");
  match EgoStoreService::app_main_get(request.app_id) {
    Ok(app) => Ok(AppMainGetResponse { app }),
    Err(e) => Err(e),
  }
}

#[update(name = "wallet_main_register")]
#[candid_method(update, rename = "wallet_main_register")]
pub fn wallet_main_register(req: WalletMainRegisterRequest) -> Result<WalletMainRegisterResponse, EgoError> {
  ic_cdk::println!("ego-store: wallet_main_register");
  match EgoStoreService::wallet_main_register(ic_cdk::caller(), req.user_id) {
    Ok(tenant_id) => Ok(WalletMainRegisterResponse { tenant_id }),
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

#[update(name = "wallet_app_install")]
#[candid_method(update, rename = "wallet_app_install")]
pub async fn wallet_app_install(req: WalletAppInstallRequest) -> Result<WalletAppInstallResponse, EgoError> {
  ic_cdk::println!("ego_store: wallet_app_install");
  let ego_tenant = EgoTenant::new();
  let user_app = EgoStoreService::wallet_app_install(ego_tenant, ic_cdk::caller(), req.app_id).await?;
  Ok(WalletAppInstallResponse { user_app })
}

#[update(name = "wallet_app_upgrade")]
#[candid_method(update, rename = "wallet_app_upgrade")]
pub async fn wallet_app_upgrade(req: WalletAppUpgradeRequest) -> Result<WalletAppUpgradeResponse, EgoError> {
  ic_cdk::println!("ego_store: wallet_app_upgrade");
  let ego_tenant = EgoTenant::new();
  let user_app = EgoStoreService::wallet_app_upgrade(ego_tenant,ic_cdk::caller(), req.app_id).await?;
  Ok(WalletAppUpgradeResponse { user_app })
}

#[update(name = "wallet_app_remove")]
#[candid_method(update, rename = "wallet_app_remove")]
pub fn wallet_app_remove(req: WalletAppRemoveRequest) -> Result<WalletAppRemoveResponse, EgoError> {
  ic_cdk::println!("ego_store: wallet_app_remove");
  match EgoStoreService::wallet_app_remove(ic_cdk::caller(), req.app_id) {
    Ok(_) => Ok(WalletAppRemoveResponse { }),
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
    Ok(order) => Ok(WalletOrderNewResponse { memo: order.memo }),
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
  ic_cdk::println!("ego_store: app_main_release");

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
#[update(name = "admin_ego_tenant_add")]
#[candid_method(update, rename = "admin_ego_tenant_add")]
pub fn admin_ego_tenant_add(req: AdminEgoTenantAddRequest) -> Result<AdminEgoTenantAddResponse, EgoError> {
  ic_cdk::println!("ego_store: admin_ego_tenant_add");

  match EgoStoreService::admin_ego_tenant_add(req.tenant_id) {
    Ok(ret) => Ok(AdminEgoTenantAddResponse { ret }),
    Err(e) => Err(e),
  }
}

#[update(name = "admin_wallet_provider_add")]
#[candid_method(update, rename = "admin_wallet_provider_add")]
pub fn admin_wallet_provider_add(req: AdminWalletProviderAddRequest) -> Result<AdminWalletProviderAddResponse, EgoError> {
  ic_cdk::println!("ego_store: admin_wallet_provider_add");

  match EgoStoreService::admin_wallet_provider_add(&req.wallet_provider, &req.wallet_app_id) {
    Ok(ret) => Ok(AdminWalletProviderAddResponse { ret }),
    Err(e) => Err(e),
  }
}

/********************  wallet provider methods  ********************/
#[update(name = "wallet_main_new")]
#[candid_method(update, rename = "wallet_main_new")]
pub async fn wallet_main_new(req: WalletMainNewRequest) -> Result<WalletMainNewResponse, EgoError> {
  ic_cdk::println!("ego-store: wallet_main_new");

  let wallet_provider = caller();

  let app_id = EGO_STORE.with(|ego_store| {
    match ego_store
      .borrow().wallet_providers.get(&wallet_provider){
      None => Err(EgoError::from(EgoStoreErr::WalletProviderNotExists)),
      Some(walelet_provider) => {
        Ok(walelet_provider.app_id.clone())
      }
    }
  })?;

  let ego_tenant = EgoTenant::new();
  let user_app = EgoStoreService::wallet_controller_install(ego_tenant, req.user_id, app_id).await?;

  // match EgoStoreService::wallet_main_register(user_app.backend.as_ref().unwrap().canister_id, req.user_id) {
  //   Ok(_) => Ok(WalletMainNewResponse{user_app}),
  //   Err(e) => Err(e),
  // }
  Ok(WalletMainNewResponse{user_app})
}