use std::collections::BTreeMap;

use candid::candid_method;
use ic_cdk::{caller, id, storage};
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_cdk_macros::*;
use serde::Serialize;

use ego_lib::ego_canister::{EgoCanister, TEgoCanister};
use ego_macros::{inject_cycle_info_api, inject_ego_api};
use ego_ops_mod::c2c::ego_dev::EgoDev;
use ego_ops_mod::c2c::ego_store::{EgoStore, TEgoStore};
use ego_ops_mod::c2c::ego_tenant::{EgoTenant as EgoTenantInner, TEgoTenant as TEgoTenantInner};
use ego_ops_mod::ego_ops::EgoOps;
use ego_ops_mod::service::EgoOpsService;
use ego_ops_mod::state::*;
use ego_ops_mod::state::EGO_OPS;
use ego_ops_mod::types::{AdminAppCreateRequest, AdminWalletCycleRechargeRequest, AdminWalletProviderAddRequest};
use ego_types::app::EgoError;
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
  info_log_add(format!("ego-ops: init, caller is {}", caller.clone()).as_str());

  info_log_add("==> add caller as the owner");
  owner_add(caller.clone());
}

#[derive(CandidType, Deserialize, Serialize)]
struct PersistState {
  pub ego_ops: EgoOps,
  users: Option<User>,
  registry: Option<Registry>,
  cycle_info: Option<CycleInfo>,
}

#[pre_upgrade]
fn pre_upgrade() {
  info_log_add("ego-ops: pre_upgrade");
  let ego_ops = EGO_OPS.with(|ego_ops| ego_ops.borrow().clone());

  let state = PersistState {
    ego_ops,
    users: Some(users_pre_upgrade()),
    registry: Some(registry_pre_upgrade()),
    cycle_info: Some(cycle_info_pre_upgrade()),
  };
  storage::stable_save((state, )).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
  info_log_add("ego-ops: post_upgrade");
  let (state, ): (PersistState, ) = storage::stable_restore().unwrap();
  EGO_OPS.with(|ego_ops| *ego_ops.borrow_mut() = state.ego_ops);

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


/********************   owner method   ********************/
#[update(name = "canister_relation_update", guard = "owner_guard")]
#[candid_method(update, rename = "canister_relation_update")]
pub fn canister_relation_update(name: String) {
  info_log_add(&format!("ego-ops: canister_relation_update {}", name));

  let ego_canister = EgoCanister::new();

  let ego_dev_id = canister_get_one("ego_dev").unwrap();
  let ego_file_ids = canister_get_all("ego_file");

  let ego_store_id = canister_get_one("ego_store").unwrap();

  let ego_tenant_ids = canister_get_all("ego_tenant");
  let ego_tenant_id = ego_tenant_ids.get(0).unwrap();

  let ego_ledger_id = canister_get_one("ego_ledger").unwrap();

  // ego_dev
  match name.as_str() {
    "ego_dev" => {
      for ego_file_id in ego_file_ids.iter() {
        ego_canister.ego_canister_add(ego_dev_id, "ego_file".to_string(), ego_file_id.clone());
      }
      ego_canister
        .ego_canister_add(ego_dev_id, "ego_store".to_string(), ego_store_id);
      ego_canister
        .ego_canister_add(ego_dev_id, "ego_tenant".to_string(), ego_tenant_id.clone());
    }
    "ego_file" => {
      for ego_file_id in ego_file_ids.iter() {
        ego_canister.ego_canister_add(ego_file_id.clone(), "ego_dev".to_string(), ego_dev_id);
        for ego_tenant_id in ego_tenant_ids.iter() {
          ego_canister.ego_canister_add(ego_file_id.clone(), "ego_tenant".to_string(), ego_tenant_id.clone());
        }
      }
    }
    "ego_store" => {
      ego_canister.ego_canister_add(ego_store_id, "ego_dev".to_string(), ego_dev_id);
      ego_canister.ego_canister_add(ego_store_id, "ego_ledger".to_string(), ego_ledger_id);
      for ego_tenant_id in ego_tenant_ids.iter() {
        ego_canister.ego_canister_add(ego_store_id, "ego_tenant".to_string(), ego_tenant_id.clone());
      }
    }
    "ego_tenant" => {
      for ego_tenant_id in ego_tenant_ids.iter() {
        ego_canister.ego_canister_add(ego_tenant_id.clone(), "ego_store".to_string(), ego_store_id);
      }
    }
    "ego_ledger" => {
      ego_canister.ego_canister_add(ego_ledger_id, "ego_store".to_string(), ego_store_id);
      ego_canister.ego_canister_add(ego_ledger_id, "ego_tenant".to_string(), ego_tenant_id.clone());
    }
    "ego_ops" => {
      let ego_store = EgoStore::new(ego_store_id);
      ego_store.admin_wallet_main_register(caller());
      canister_add("ego_tenant".to_string(), ego_tenant_id.clone());
    }
    _ => {}
  }
}

#[update(name = "canister_main_track", guard = "owner_guard")]
#[candid_method(update, rename = "canister_main_track")]
pub fn canister_main_track() {
  info_log_add("ego-ops: canister_main_track");

  let wallet_id = id();
  let ego_tenant = EgoTenantInner::new();

  let ego_canister = EgoCanister::new();


  let tracker_ego_tenant_id = canister_get_one("ego_tenant").unwrap();

  // ego_dev
  info_log_add("1 track ego_dev");
  let ego_dev_id = canister_get_one("ego_dev").unwrap();
  ego_canister.ego_op_add(ego_dev_id, tracker_ego_tenant_id);
  ego_tenant.canister_main_track(tracker_ego_tenant_id, wallet_id, ego_dev_id);

  // ego_file
  info_log_add("2 track ego_file");
  for ego_file_id in canister_get_all("ego_file") {
    ego_canister.ego_op_add(ego_file_id, tracker_ego_tenant_id);
    ego_tenant.canister_main_track(tracker_ego_tenant_id, wallet_id, ego_file_id);
  }

  // ego_store
  info_log_add("3 track ego_store");
  let ego_store_id = canister_get_one("ego_store").unwrap();
  ego_canister.ego_op_add(ego_store_id, tracker_ego_tenant_id);
  ego_tenant.canister_main_track(tracker_ego_tenant_id, wallet_id, ego_store_id);

  // ego_tenant
  info_log_add("4 track ego_tenant");
  for ego_tenant_id in canister_get_all("ego_tenant") {
    ego_tenant.canister_main_track(tracker_ego_tenant_id, wallet_id, ego_tenant_id);
  }

  // ego_ledger
  info_log_add("4 track ego_ledger");
  let ego_ledger_id = canister_get_one("ego_ledger").unwrap();
  ego_canister.ego_op_add(ego_ledger_id, tracker_ego_tenant_id);
  ego_tenant.canister_main_track(tracker_ego_tenant_id, wallet_id, ego_ledger_id);

  // ego_ops
  info_log_add("5 track ego_ops");
  op_add(tracker_ego_tenant_id);
  ego_tenant.canister_main_track(tracker_ego_tenant_id, wallet_id, wallet_id);
}

#[update(name = "admin_app_create", guard = "owner_guard")]
#[candid_method(update, rename = "admin_app_create")]
pub fn admin_app_create(
  req: AdminAppCreateRequest,
) -> Result<(), EgoError> {
  info_log_add("ego-ops: admin_app_create");

  let ego_dev = EgoDev::new();
  let ego_dev_id = canister_get_one("ego_dev").unwrap();


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
    req.backend_data_hash,
  )?;
  Ok(())
}

#[update(name = "admin_wallet_provider_add", guard = "owner_guard")]
#[candid_method(update, rename = "admin_wallet_provider_add")]
pub fn admin_wallet_provider_add(req: AdminWalletProviderAddRequest) -> Result<(), EgoError> {
  info_log_add("ego_ops: admin_wallet_provider_add");

  let ego_store_id = canister_get_one("ego_store").unwrap();
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
  info_log_add("ego_ops: admin_wallet_cycle_recharge");

  let ego_store_id = canister_get_one("ego_store").unwrap();
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
  info_log_add("ego_ops: admin_wallet_order_new");

  let ego_store_id = canister_get_one("ego_store").unwrap();
  let ego_store = EgoStore::new(ego_store_id);

  ego_store
    .admin_wallet_order_new(amount);

  Ok(())
}

/********************  methods for ego_cycle_threshold_get   ********************/
pub fn cycle_threshold_get() -> u128 {
  1_000_000_000_000
}

pub fn runtime_cycle_threshold_get() -> u128 {
  1_000_000_000_000
}