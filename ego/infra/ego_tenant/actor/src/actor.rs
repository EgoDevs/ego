use std::collections::BTreeMap;
use std::time::Duration;

use candid::candid_method;
use ic_cdk::{api, caller, id, storage};
use ic_cdk::api::time;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_cdk::timer::set_timer_interval;
use ic_cdk_macros::*;
use serde::Serialize;

use ego_lib::ego_canister::EgoCanister;
use ego_macros::inject_ego_api;
use ego_tenant_mod::c2c::ego_file::EgoFile;
use ego_tenant_mod::c2c::ego_store::EgoStore;
use ego_tenant_mod::c2c::ic_management::IcManagement;
use ego_tenant_mod::ego_tenant::EgoTenant;
use ego_tenant_mod::service::EgoTenantService;
use ego_tenant_mod::state::{canister_add, is_op, is_owner, is_user, log_add, log_list, op_add, owner_add, owner_remove, owners_set, registry_post_upgrade, registry_pre_upgrade, user_add, user_remove, users_post_upgrade, users_pre_upgrade, users_set};
use ego_tenant_mod::state::EGO_TENANT;
use ego_tenant_mod::types::{
  AppMainInstallRequest, AppMainUpgradeRequest,
};
use ego_types::app::EgoError;
use ego_types::registry::Registry;
use ego_types::user::User;

inject_ego_api!();


#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct InitArg {
  init_caller: Option<Principal>,
}

#[init]
#[candid_method(init)]
pub fn init(arg: InitArg) {
  let caller = arg.init_caller.unwrap_or(caller());
  log_add(format!("ego-tenant: init, caller is {}", caller.clone()).as_str());

  log_add("==> add caller as the owner");
  owner_add(caller.clone());

  let duration = Duration::new(1800, 0);
  set_timer_interval(duration, || {
    let _result = api::call::notify(id(), "message_main_notify", ());
  });
}

#[derive(CandidType, Deserialize, Serialize)]
struct PersistState {
  pub ego_tenant: EgoTenant,
  users: Option<User>,
  registry: Option<Registry>,
}

#[pre_upgrade]
fn pre_upgrade() {
  log_add("ego-tenant: pre_upgrade");
  let ego_tenant = EGO_TENANT.with(|ego_tenant| ego_tenant.borrow().clone());

  let state = PersistState {
    ego_tenant,
    users: Some(users_pre_upgrade()),
    registry: Some(registry_pre_upgrade()),
  };
  storage::stable_save((state, )).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
  log_add("ego-tenant: post_upgrade");
  let (state, ): (PersistState, ) = storage::stable_restore().unwrap();
  EGO_TENANT.with(|ego_tenant| *ego_tenant.borrow_mut() = state.ego_tenant);

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

  let duration = Duration::new(1800, 0);
  set_timer_interval(duration, || {
    let _result = api::call::notify(id(), "message_main_notify", ());
  });
}


/********************  methods for ego_store   ********************/
#[update(name = "app_main_install", guard = "user_guard")]
#[candid_method(update, rename = "app_main_install")]
async fn app_main_install(req: AppMainInstallRequest) -> Result<Principal, EgoError> {
  log_add("ego_tenant: app_main_install");

  let ego_tenant_id = id();
  let management = IcManagement::new();
  let ego_file = EgoFile::new();
  let ego_canister = EgoCanister::new();

  let canister_id = EgoTenantService::app_main_install(
    ego_tenant_id,
    ego_file,
    management,
    ego_canister,
    req.wallet_id,
    req.user_id,
    req.wasm,
  )
    .await?;
  Ok(canister_id)
}

#[update(name = "app_main_upgrade", guard = "user_guard")]
#[candid_method(update, rename = "app_main_upgrade")]
async fn app_main_upgrade(req: AppMainUpgradeRequest) -> Result<bool, EgoError> {
  log_add("ego_tenant: app_main_upgrade");
  let management = IcManagement::new();
  let ego_file = EgoFile::new();

  let ego_canister = EgoCanister::new();

  let ret =
    EgoTenantService::app_main_upgrade(ego_file, management, ego_canister, req.canister_id, req.wasm, id()).await?;
  Ok(ret)
}

#[update(name = "app_main_delete", guard = "user_guard")]
#[candid_method(update, rename = "app_main_delete")]
async fn app_main_delete(canister_id: Principal) -> Result<(), EgoError> {
  log_add("ego_tenant: app_main_upgrade");
  let management = IcManagement::new();

  EgoTenantService::app_main_delete(management, &canister_id).await
}

#[update(name = "canister_main_track", guard = "user_guard")]
#[candid_method(update, rename = "canister_main_track")]
fn canister_main_track(wallet_id: Principal, canister_id: Principal) -> Result<(), EgoError> {
  log_add("ego_tenant: canister_main_track");

  EgoTenantService::canister_main_track(wallet_id, canister_id)?;
  Ok(())
}

#[update(name = "canister_main_untrack", guard = "user_guard")]
#[candid_method(update, rename = "canister_main_untrack")]
fn canister_main_untrack(wallet_id: Principal, canister_id: Principal) -> Result<(), EgoError> {
  log_add("ego_tenant: canister_main_untrack");

  EgoTenantService::canister_main_untrack(wallet_id, canister_id)?;
  Ok(())
}

/********************  notify  ********************/
#[update(name = "message_main_notify")]
#[candid_method(update, rename = "message_main_notify")]
async fn message_main_notify() {
  log_add("ego-tenant: message_main_notify");

  let sentinel = time();
  let tasks = EGO_TENANT.with(|ego_tenant| ego_tenant.borrow_mut().tasks_get(sentinel));

  for task in tasks {
    let management = IcManagement::new();
    let ego_store = EgoStore::new();
    let ego_canister = EgoCanister::new();

    match EgoTenantService::canister_cycles_check(
      management,
      ego_store,
      ego_canister,
      sentinel,
      task,
    )
      .await
    {
      Ok(_) => {}
      Err(e) => {
        log_add(&format!("canister_cycles_check failed: {:?}", e));
      }
    }
  }
}
