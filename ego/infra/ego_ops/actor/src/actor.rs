use std::collections::BTreeMap;

use candid::candid_method;
use ic_cdk::{caller, id, storage};
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_cdk_macros::*;
use serde::Serialize;

use ego_ops_mod::c2c::ego_canister::{EgoCanister, TEgoCanister};
use ego_ops_mod::c2c::ego_dev::EgoDev;
use ego_ops_mod::c2c::ego_store::{EgoStore, TEgoStore};
use ego_ops_mod::c2c::ego_tenant::{EgoTenant, TEgoTenant};
use ego_ops_mod::ego_macros::inject_ego_macros;
use ego_ops_mod::ego_ops::EgoOps;
use ego_ops_mod::service::{ego_log, EgoOpsService, Registry, REGISTRY, User};
use ego_ops_mod::service::{canister_add, canister_list, is_owner, log_list, owner_add, registry_post_upgrade, registry_pre_upgrade, USER, user_add, users_post_upgrade, users_pre_upgrade};
use ego_ops_mod::state::EGO_OPS;
use ego_ops_mod::types::{AdminAppCreateRequest, AdminWalletCycleRechargeRequest, AdminWalletProviderAddRequest};
use ego_types::ego_error::EgoError;

inject_ego_macros!();

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct InitArg {
  init_caller: Option<Principal>,
}

#[init]
#[candid_method(init)]
pub fn init(arg: InitArg) {
  let caller = arg.init_caller.unwrap_or(caller());
  ego_log(format!("ego-ops: init, caller is {}", caller.clone()).as_str());

  ego_log("==> add caller as the owner");
  owner_add(caller.clone());
}

#[derive(CandidType, Deserialize, Serialize)]
struct PersistState {
  pub ego_ops: EgoOps,
  pub user: User,
  pub registry: Registry,
}

#[pre_upgrade]
fn pre_upgrade() {
  ego_log("ego-ops: pre_upgrade");
  let ego_ops = EGO_OPS.with(|ego_ops| ego_ops.borrow().clone());
  let user = users_pre_upgrade();
  let registry = registry_pre_upgrade();

  let state = PersistState { ego_ops, user, registry };
  storage::stable_save((state, )).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
  ego_log("ego-ops: post_upgrade");
  let (state, ): (PersistState, ) = storage::stable_restore().unwrap();
  EGO_OPS.with(|ego_ops| *ego_ops.borrow_mut() = state.ego_ops);
  users_post_upgrade(state.user);
  registry_post_upgrade(state.registry);
}


/********************   owner method   ********************/
#[update(name = "canister_relation_update", guard = "owner_guard")]
#[candid_method(update, rename = "canister_relation_update")]
pub fn canister_relation_update(name: String) {
  ego_log(&format!("ego-ops: canister_relation_update {}", name));

  REGISTRY.with(|register| {
    let ego_canister = EgoCanister::new();

    let ego_dev_id = register.borrow().canister_get_one("ego_dev").unwrap();
    let ego_file_ids = register.borrow().canister_get_all("ego_file");

    let ego_store_id = register.borrow().canister_get_one("ego_store").unwrap();
    let ego_tenant_ids = register.borrow().canister_get_all("ego_tenant");

    let ego_ledger_id = register.borrow().canister_get_one("ego_ledger").unwrap();

    // ego_dev
    match name.as_str() {
      "ego_dev" => {
        for ego_file_id in ego_file_ids.iter() {
          ego_canister.ego_canister_add(&ego_dev_id, "ego_file".to_string(), ego_file_id);
        }
        ego_canister
          .ego_canister_add(&ego_dev_id, "ego_store".to_string(), &ego_store_id);
      }
      "ego_file" => {
        for ego_file_id in ego_file_ids.iter() {
          ego_canister.ego_canister_add(ego_file_id, "ego_dev".to_string(), &ego_dev_id);
          for ego_tenant_id in ego_tenant_ids.iter() {
            ego_canister.ego_canister_add(ego_file_id, "ego_tenant".to_string(), ego_tenant_id);
          }
        }
      }
      "ego_store" => {
        ego_canister.ego_canister_add(&ego_store_id, "ego_dev".to_string(), &ego_dev_id);
        ego_canister.ego_canister_add(&ego_store_id, "ego_ledger".to_string(), &ego_ledger_id);
        for ego_tenant_id in ego_tenant_ids.iter() {
          ego_canister.ego_canister_add(&ego_store_id, "ego_tenant".to_string(), ego_tenant_id);
        }
      }
      "ego_tenant" => {
        for ego_tenant_id in ego_tenant_ids.iter() {
          ego_canister.ego_canister_add(ego_tenant_id, "ego_store".to_string(), &ego_store_id);
        }
      }
      "ego_ledger" => {
        ego_canister.ego_canister_add(&ego_ledger_id, "ego_store".to_string(), &ego_store_id);
      }
      "ego_ops" => {
        let ego_store = EgoStore::new(ego_store_id);
        ego_store.admin_wallet_main_register(caller());
      }
      _ => {}
    }
  });
}

#[update(name = "canister_main_track", guard = "owner_guard")]
#[candid_method(update, rename = "canister_main_track")]
pub fn canister_main_track() {
  ego_log("ego-ops: canister_main_track");

  let wallet_id = id();
  let ego_tenant = EgoTenant::new();

  REGISTRY.with(|register| {
    let tracker_ego_tenant_id = register.borrow().canister_get_one("ego_tenant").unwrap();

    // ego_dev
    ego_log("1 track ego_dev");
    let ego_dev_id = register.borrow().canister_get_one("ego_dev").unwrap();
    ego_tenant.canister_main_track(tracker_ego_tenant_id, wallet_id, ego_dev_id);

    // ego_file
    ego_log("2 track ego_file");
    for ego_file_id in register.borrow().canister_get_all("ego_file") {
      ego_tenant.canister_main_track(tracker_ego_tenant_id, wallet_id, ego_file_id);
    }

    // ego_store
    ego_log("3 track ego_store");
    let ego_store_id = register.borrow().canister_get_one("ego_store").unwrap();
    ego_tenant
      .canister_main_track(tracker_ego_tenant_id, wallet_id, ego_store_id);

    // ego_tenant
    ego_log("4 track ego_tenant");
    for ego_tenant_id in register.borrow().canister_get_all("ego_tenant") {
      ego_tenant
        .canister_main_track(tracker_ego_tenant_id, wallet_id, ego_tenant_id);
    }

    // ego_ledger
    ego_log("6 track ego_ledger");
    let ego_ledger_id = register.borrow().canister_get_one("ego_ledger").unwrap();
    ego_tenant
      .canister_main_track(tracker_ego_tenant_id, wallet_id, ego_ledger_id);

    // ego_ops
    ego_log("7 track ego_ops");
    ego_tenant
      .canister_main_track(tracker_ego_tenant_id, wallet_id, wallet_id);
  });
}

#[update(name = "admin_app_create", guard = "owner_guard")]
#[candid_method(update, rename = "admin_app_create")]
pub fn admin_app_create(
  req: AdminAppCreateRequest,
) -> Result<(), EgoError> {
  ego_log("ego-ops: admin_app_create");

  let ego_dev = EgoDev::new();
  let ego_dev_id = REGISTRY.with(|r| r.borrow().canister_get_one("ego_dev")).unwrap();


  EgoOpsService::admin_app_create(
    ego_dev,
    ego_dev_id,
    req.app_id,
    req.name,
    req.version,
    req.category,
    req.logo,
    req.description,
    req.backend_data,
    req.backend_hash,
    req.frontend,
    req.deploy_mode,
  )?;
  Ok(())
}

#[update(name = "admin_wallet_provider_add", guard = "owner_guard")]
#[candid_method(update, rename = "admin_wallet_provider_add")]
pub fn admin_wallet_provider_add(req: AdminWalletProviderAddRequest) -> Result<(), EgoError> {
  ego_log("ego_ops: admin_wallet_provider_add");

  let ego_store_id = REGISTRY.with(|r| r.borrow().canister_get_one("ego_store")).unwrap();
  let ego_store = EgoStore::new(ego_store_id);

  ego_store
    .admin_wallet_provider_add(req.wallet_provider, req.wallet_app_id);

  Ok(())
}

#[update(name = "admin_wallet_cycle_recharge", guard = "owner_guard")]
#[candid_method(update, rename = "admin_wallet_cycle_recharge")]
pub fn admin_wallet_cycle_recharge(
  req: AdminWalletCycleRechargeRequest,
) -> Result<(), EgoError> {
  ego_log("ego_ops: admin_wallet_cycle_recharge");

  let ego_store_id = REGISTRY.with(|r| r.borrow().canister_get_one("ego_store")).unwrap();
  let ego_store = EgoStore::new(ego_store_id);

  ego_store
    .admin_wallet_cycle_recharge(req.wallet_id, req.cycle, req.comment);

  Ok(())
}

#[update(name = "admin_wallet_order_new", guard = "owner_guard")]
#[candid_method(update, rename = "admin_wallet_order_new")]
pub fn admin_wallet_order_new(
  amount: f32,
) -> Result<(), EgoError> {
  ego_log("ego_ops: admin_wallet_order_new");

  let ego_store_id = REGISTRY.with(|r| r.borrow().canister_get_one("ego_store")).unwrap();
  let ego_store = EgoStore::new(ego_store_id);

  ego_store
    .admin_wallet_order_new(amount);

  Ok(())
}
