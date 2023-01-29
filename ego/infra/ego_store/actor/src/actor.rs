use std::collections::BTreeMap;

use candid::candid_method;
use ic_cdk::{caller, storage};
use ic_cdk::api::time;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_cdk_macros::*;
use serde::Serialize;

use ego_lib::ego_canister::EgoCanister;
use ego_macros::{inject_cycle_info_api, inject_ego_api};
use ego_store_mod::app::EgoStoreApp;
use ego_store_mod::c2c::ego_ledger::EgoLedger;
use ego_store_mod::c2c::ego_tenant::EgoTenant as EgoTenantInner;
use ego_store_mod::store::EgoStore;
use ego_store_mod::service::*;
use ego_store_mod::state::*;
use ego_store_mod::state::EGO_STORE;
use ego_store_mod::types::*;
use ego_types::app::{App, AppId};
use ego_types::app::EgoError;
use ego_types::app::UserApp;
use ego_types::app::CashFlow;
use ic_ledger_types::Memo;
use ego_store_mod::order::Order;
use ego_types::registry::Registry;
use ego_types::user::User;

inject_ego_api!();
inject_cycle_info_api!();

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct InitArg {
  init_caller: Option<Principal>,
}

#[init]
#[candid_method(init)]
pub fn init(arg: InitArg) {
  let caller = arg.init_caller.unwrap_or(caller());
  log_add(format!("ego_store: init, caller is {}", caller.clone()).as_str());

  log_add("==> add caller as the owner");
  owner_add(caller.clone());
}

#[derive(CandidType, Deserialize, Serialize)]
struct PersistState {
  ego_store: EgoStore,
  users: Option<User>,
  registry: Option<Registry>,
  cycle_info: Option<CycleInfo>
}

#[pre_upgrade]
fn pre_upgrade() {
  log_add("ego_store: pre_upgrade");

  let ego_store = EGO_STORE.with(|ego_store| ego_store.borrow().clone());

  let state = PersistState {
    ego_store,
    users: Some(users_pre_upgrade()),
    registry: Some(registry_pre_upgrade()),
    cycle_info: Some(cycle_info_pre_upgrade())
  };
  storage::stable_save((state, )).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
  log_add("ego_store: post_upgrade");

  let (state, ): (PersistState, ) = storage::stable_restore().unwrap();
  EGO_STORE.with(|ego_store| *ego_store.borrow_mut() = state.ego_store);

  match state.users {
    None => {}
    Some(users) => {
      users_post_upgrade(users);
    }
  }

  match state.registry {
    None => {}
    Some(registry) => {
      registry_post_upgrade(registry);
    }
  }

  match state.cycle_info {
    None => {}
    Some(cycle_info) => {
      cycle_info_post_upgrade(cycle_info);
    }
  }
}


/********************  methods for wallet   ********************/
#[update(name = "app_main_list")]
#[candid_method(update, rename = "app_main_list")]
pub fn app_main_list() -> Result<Vec<App>, EgoError> {
  log_add("ego_store: app_main_list");
  match EgoStoreService::app_main_list() {
    Ok(apps) => Ok(apps),
    Err(e) => Err(e),
  }
}

#[update(name = "app_main_get")]
#[candid_method(update, rename = "app_main_get")]
pub fn app_main_get(app_id: AppId) -> Result<App, EgoError> {
  log_add("ego_store: app_main_get");
  match EgoStoreService::app_main_get(&app_id) {
    Ok(ego_store_app) => Ok(ego_store_app.app),
    Err(e) => Err(e),
  }
}

#[update(name = "wallet_main_register")]
#[candid_method(update, rename = "wallet_main_register")]
pub fn wallet_main_register(
  user_id: Principal,
) -> Result<Principal, EgoError> {
  log_add("ego_store: wallet_main_register");
  let tenant_id = EgoStoreService::wallet_main_register(ic_cdk::caller(), user_id)?;

  Ok(tenant_id)
}

#[update(name = "wallet_tenant_get")]
#[candid_method(update, rename = "wallet_tenant_get")]
pub fn wallet_tenant_get() -> Result<Principal, EgoError> {
  log_add("ego_store: wallet_tenant_get");
  match EgoStoreService::wallet_tenant_get(ic_cdk::caller()) {
    Ok(tenant_id) => Ok(tenant_id),
    Err(e) => Err(e),
  }
}

#[update(name = "wallet_app_list")]
#[candid_method(update, rename = "wallet_app_list")]
pub fn wallet_app_list() -> Result<Vec<UserApp>, EgoError> {
  log_add("ego_store: wallet_app_list");
  let wallet_id = ic_cdk::caller();
  match EgoStoreService::wallet_app_list(&wallet_id) {
    Ok(apps) => Ok(apps),
    Err(e) => Err(e),
  }
}

#[update(name = "wallet_app_install")]
#[candid_method(update, rename = "wallet_app_install")]
pub async fn wallet_app_install(
  app_id: AppId,
) -> Result<UserApp, EgoError> {
  log_add("ego_store: wallet_app_install");

  log_add("1 get app to be install");
  let app = EGO_STORE.with(|ego_store| ego_store.borrow().app_main_get(&app_id).clone())?;

  log_add("2 get wallet_id");
  let wallet_id = caller();

  let ego_tenant = EgoTenantInner::new();
  let ego_canister = EgoCanister::new();

  let user_app =
    EgoStoreService::wallet_app_install(ego_tenant, ego_canister, wallet_id, app).await?;
  Ok(user_app)
}

#[update(name = "wallet_app_upgrade")]
#[candid_method(update, rename = "wallet_app_upgrade")]
pub async fn wallet_app_upgrade(wallet_id: Principal) -> Result<(), EgoError> {
  let canister_id = caller();
  let ego_tenant = EgoTenantInner::new();
  let ego_canister = EgoCanister::new();

  log_add(format!("ego_store: wallet_app_upgrade wallet_id: {}, canister_id: {}", wallet_id, canister_id).as_str());

  EgoStoreService::wallet_app_upgrade(ego_tenant, ego_canister, &wallet_id, &canister_id).await?;
  Ok(())
}

#[update(name = "wallet_app_remove")]
#[candid_method(update, rename = "wallet_app_remove")]
pub fn wallet_app_remove(wallet_id: Principal) -> Result<(), EgoError> {
  let canister_id = caller();
  let ego_tenant = EgoTenantInner::new();

  log_add(format!("ego_store: wallet_app_remove wallet_id: {}, canister_id: {}", wallet_id, canister_id).as_str());

  match EgoStoreService::wallet_app_remove(ego_tenant, &wallet_id, &canister_id) {
    Ok(_) => Ok(()),
    Err(e) => Err(e),
  }
}

#[update(name = "wallet_canister_track", guard = "user_guard")]
#[candid_method(update, rename = "wallet_canister_track")]
pub fn wallet_canister_track(canister_id: Principal) -> Result<(), EgoError> {
  log_add("ego_store: canister_main_track");

  let ego_tenant = EgoTenantInner::new();
  let wallet_id = caller();

  EgoStoreService::wallet_canister_track(ego_tenant, &wallet_id, &canister_id)
}

#[update(name = "wallet_canister_untrack", guard = "user_guard")]
#[candid_method(update, rename = "wallet_canister_untrack")]
pub fn wallet_canister_untrack(canister_id: Principal) -> Result<(), EgoError> {
  log_add("ego_store: canister_main_untrack");

  let ego_tenant = EgoTenantInner::new();
  let wallet_id = caller();

  EgoStoreService::wallet_canister_untrack(ego_tenant, &wallet_id, &canister_id)
}

#[update(name = "wallet_order_list")]
#[candid_method(update, rename = "wallet_order_list")]
pub fn wallet_order_list() -> Result<Vec<Order>, EgoError> {
  log_add("ego_store: wallet_order_list");

  match EgoStoreService::wallet_order_list(ic_cdk::caller()) {
    Ok(orders) => Ok( orders ),
    Err(e) => Err(e),
  }
}

#[update(name = "wallet_order_new")]
#[candid_method(update, rename = "wallet_order_new")]
pub async fn wallet_order_new(amount: f32) -> Result<Memo, EgoError> {
  log_add("ego_store: wallet_order_new");

  let ego_ledger_id = canister_get_one("ego_ledger").unwrap();
  let ego_ledger = EgoLedger::new(ego_ledger_id);

  match EgoStoreService::wallet_order_new(ego_ledger, ic_cdk::caller(), ic_cdk::id(), amount) {
    Ok(order) => Ok(order.memo),
    Err(e) => {
      log_add(&format!("ego_store: wallet_order_new {:?}", e));
      Err(e)
    }
  }
}

#[update(name = "wallet_cycle_balance")]
#[candid_method(update, rename = "wallet_cycle_balance")]
pub fn wallet_cycle_balance() -> Result<u128, EgoError> {
  log_add("ego_store: wallet_cycle_balance");

  let wallet_id = caller();

  match EgoStoreService::wallet_cycle_balance(wallet_id) {
    Ok(balance) => Ok(balance),
    Err(e) => {
      log_add(&format!("ego_store: wallet_cycle_balance {:?}", e));
      Err(e)
    }
  }
}


#[update(name = "wallet_cycle_list")]
#[candid_method(update, rename = "wallet_cycle_list")]
pub fn wallet_cycle_list() -> Result<Vec<CashFlow>, EgoError> {
  log_add("ego_store: wallet_cycle_list");

  let wallet_id = caller();

  match EgoStoreService::wallet_cycle_list(wallet_id) {
    Ok(cash_flows) => Ok(cash_flows),
    Err(e) => {
      log_add(&format!("ego_store: wallet_cycle_list {:?}", e));
      Err(e)
    }
  }
}

/********************  methods for ego_tenant  ********************/
#[update(name = "wallet_cycle_charge", guard = "user_guard")]
#[candid_method(update, rename = "wallet_cycle_charge")]
pub fn wallet_cycle_charge(
  request: WalletCycleChargeRequest,
) -> Result<WalletCycleChargeResponse, EgoError> {
  log_add("ego_store: wallet_cycle_charge");

  // the tenant id or something else
  let operator = caller();

  match EgoStoreService::wallet_cycle_charge(
    request.wallet_id,
    request.cycle,
    operator,
    time(),
    request.comment,
  ) {
    Ok(ret) => Ok(WalletCycleChargeResponse { ret }),
    Err(e) => Err(e),
  }
}

/********************  methods for ego_dev  ********************/
#[update(name = "app_main_release", guard = "user_guard")]
#[candid_method(update, rename = "app_main_release")]
pub async fn app_main_release(app: EgoStoreApp) -> Result<bool, EgoError> {
  log_add("ego_store: app_main_release");

  match EgoStoreService::app_main_release(app) {
    Ok(ret) => Ok(ret),
    Err(e) => Err(e),
  }
}

/********************  methods for ego-ledger callback  ********************/
#[update(name = "wallet_order_notify", guard = "user_guard")]
#[candid_method(update, rename = "wallet_order_notify")]
pub fn wallet_order_notify(memo: Memo) -> Result<bool, EgoError> {
  log_add("ego_store: wallet_order_notify");

  // the ego_ledger id
  let operator = caller();

  match EgoStoreService::wallet_order_notify(memo, operator, ic_cdk::api::time()) {
    Ok(ret) => Ok( ret ),
    Err(e) => Err(e),
  }
}

/******************** owner methods  ********************/
#[update(name = "admin_wallet_provider_add")]
#[candid_method(update, rename = "admin_wallet_provider_add")]
pub fn admin_wallet_provider_add(
  req: AdminWalletProviderAddRequest,
) -> Result<(), EgoError> {
  log_add("ego_store: admin_wallet_provider_add");

  log_add(&format!("wallet_provider: {}, app_id: {}",
                   req.wallet_provider,
                   req.wallet_app_id));

  EgoStoreService::admin_wallet_provider_add(&req.wallet_provider, &req.wallet_app_id);
  Ok(())
}


#[update(name = "admin_wallet_cycle_recharge", guard = "owner_guard")]
#[candid_method(update, rename = "admin_wallet_cycle_recharge")]
pub fn admin_wallet_cycle_recharge(req: AdminWalletCycleRechargeRequest) -> Result<bool, EgoError> {
  log_add("ego_store: admin_wallet_cycle_recharge");

  // the ego_ops id
  let operator = caller();

  EgoStoreService::admin_wallet_cycle_recharge(req.wallet_id, req.cycle, operator, time(), req.comment)
}

#[update(name = "admin_wallet_order_list", guard = "owner_guard")]
#[candid_method(update, rename = "admin_wallet_order_list")]
pub fn admin_wallet_order_list() -> Result<Vec<Order>, EgoError> {
  log_add("ego_store: admin_wallet_order_list");

  Ok(EgoStoreService::wallet_order_list_all())
}

/********************  methods for wallet provider  ********************/
#[update(name = "wallet_main_new")]
#[candid_method(update, rename = "wallet_main_new")]
pub async fn wallet_main_new(user_id: Principal) -> Result<UserApp, EgoError> {
  log_add("ego_store: wallet_main_new");

  let wallet_provider = caller();

  log_add(&format!("wallet_provider is {}", wallet_provider));

  let app_id = EGO_STORE.with(|ego_store| {
    match ego_store.borrow().wallet_providers.get(&wallet_provider) {
      None => Err(EgoError::from(EgoStoreErr::WalletProviderNotExists)),
      Some(provider) => Ok(provider.app_id.clone()),
    }
  })?;

  log_add(&format!("1 get controller app_id {}", app_id));

  let ego_tenant = EgoTenantInner::new();
  let ego_canister = EgoCanister::new();
  let user_app =
    EgoStoreService::wallet_controller_install(ego_tenant, ego_canister, wallet_provider, user_id, app_id).await?;

  Ok(user_app)
}
