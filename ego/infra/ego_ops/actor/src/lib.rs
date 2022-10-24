use ic_cdk::{id, storage};
use ic_cdk_macros::*;
use ego_ops_mod::service::EgoOpsService;

use ego_types::ego_error::EgoError;
use candid::{candid_method};
use ego_ops_mod::ego_ops::EgoOps;
use ego_ops_mod::state::EGO_OPS;
use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;
use ego_ops_mod::c2c::ego_cron::EgoCron;
use ego_ops_mod::c2c::ego_dev::EgoDev;
use ego_ops_mod::c2c::ego_store::{EgoStore, TEgoStore};
use ego_ops_mod::c2c::ego_tenant::EgoTenant;
use ego_ops_mod::c2c::ego_user::EgoUser;
use ego_ops_mod::types::{AdminAppCreateRequest, AdminAppCreateResponse, AdminWalletProviderAddRequest, CanisterMainListResponse, CanisterMainRegisterRequest};

use ego_users::inject_ego_users;
use ego_macros::inject_balance_get;

inject_balance_get!();
inject_ego_users!();


#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct InitArg {
  init_caller: Option<Principal>,
}

#[init]
#[candid_method(init)]
pub fn init(arg: InitArg) {
  let caller = arg.init_caller.unwrap_or(caller());
  ic_cdk::println!("ego-ops: init, caller is {}", caller.clone());

  ic_cdk::println!("==> add caller as the owner");
  users_init(caller.clone());
}

#[derive(CandidType, Deserialize, Serialize)]
struct PersistState{
  pub ego_ops: EgoOps,
  pub user: User
}

#[pre_upgrade]
fn pre_upgrade() {
  ic_cdk::println!("ego-ops: pre_upgrade");
  let ego_ops = EGO_OPS.with(|ego_ops| ego_ops.borrow().clone());
  let user = users_pre_upgrade();

  let state = PersistState{ego_ops, user};
  storage::stable_save((state, )).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
  ic_cdk::println!("ego-ops: post_upgrade");
  let (state, ): (PersistState, ) = storage::stable_restore().unwrap();
  EGO_OPS.with(|ego_ops|
    *ego_ops.borrow_mut() = state.ego_ops
  );
  users_post_upgrade(state.user);
}

/********************   owner method   ********************/
#[update(name = "canister_main_register", guard = "owner_guard")]
#[candid_method(update, rename = "canister_main_register")]
pub fn canister_main_register(req: CanisterMainRegisterRequest) -> Result<(), EgoError> {
  ic_cdk::println!("ego-ops: canister_main_register");

  EgoOpsService::canister_main_register(req.app_id, req.canister_id);

  Ok(())
}

/// register the initial canister into the ego_ops
#[update(name = "canister_relation_update", guard = "owner_guard")]
#[candid_method(update, rename = "canister_relation_update")]
pub async fn canister_relation_update() -> Result<(), EgoError> {
  ic_cdk::println!("ego-ops: canister_relation_update");

  let ego_user = EgoUser::new();
  let ego_dev = EgoDev::new();
  let ego_store = EgoStore::new();
  let ego_cron = EgoCron::new();
  let ego_tenant = EgoTenant::new();

  EgoOpsService::canister_relation_update(ego_user, ego_dev, ego_store, ego_cron, ego_tenant).await?;

  Ok(())
}

#[update(name = "canister_main_track", guard = "owner_guard")]
#[candid_method(update, rename = "canister_main_track")]
pub async fn canister_main_track() -> Result<(), EgoError> {
  ic_cdk::println!("ego-ops: canister_main_track");

  let wallet_id = id();
  let ego_tenant = EgoTenant::new();

  EgoOpsService::canister_main_track(ego_tenant, wallet_id).await?;

  Ok(())
}

#[query(name = "canister_main_list", guard = "owner_guard")]
#[candid_method(query, rename = "canister_main_list")]
pub async fn canister_main_list() -> Result<CanisterMainListResponse, EgoError> {
  ic_cdk::println!("ego-ops: canister_main_list");

  EGO_OPS.with(|ego_ops| {
    Ok(CanisterMainListResponse{ canisters: ego_ops.borrow().canisters.clone()})
  })
}

#[update(name = "admin_app_create", guard = "owner_guard")]
#[candid_method(update, rename = "admin_app_create")]
pub async fn admin_app_create(req: AdminAppCreateRequest) -> Result<AdminAppCreateResponse, EgoError> {
  ic_cdk::println!("ego-ops: admin_app_create");

  match EgoOpsService::admin_app_create(req.app_id, req.name, req.version, req.category, req.logo, req.description, req.backend_data, req.backend_hash, req.frontend, req.deploy_mode).await {
    Ok(ret) => {
      Ok(AdminAppCreateResponse{ret})
    },
    Err(e) => Err(e)
  }
}

#[update(name = "admin_wallet_provider_add", guard = "owner_guard")]
#[candid_method(update, rename = "admin_wallet_provider_add")]
pub async fn admin_wallet_provider_add(req: AdminWalletProviderAddRequest) -> Result<(), EgoError> {
  ic_cdk::println!("ego_ops: admin_wallet_provider_add");
  let ego_store = EgoStore::new();

  let ego_store_id = EGO_OPS.with(|ego_ops| {
    ego_ops.borrow().app_canister_get("ego_store".to_string()).get(0).unwrap().clone()
  });

  ego_store.admin_wallet_provider_add(ego_store_id, req.wallet_provider, req.wallet_app_id).await?;

  Ok(())
}

#[update(name = "admin_wallet_cycle_recharge", guard = "owner_guard")]
#[candid_method(update, rename = "admin_wallet_cycle_recharge")]
pub async fn admin_wallet_cycle_recharge(req: AdminWalletProviderAddRequest) -> Result<(), EgoError> {
  ic_cdk::println!("ego_ops: admin_wallet_provider_add");
  let ego_store = EgoStore::new();

  let ego_store_id = EGO_OPS.with(|ego_ops| {
    ego_ops.borrow().app_canister_get("ego_store".to_string()).get(0).unwrap().clone()
  });

  ego_store.admin_wallet_provider_add(ego_store_id, req.wallet_provider, req.wallet_app_id).await?;

  Ok(())
}