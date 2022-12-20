use std::collections::BTreeMap;
use std::time::Duration;

use candid::candid_method;
use ic_cdk::{api, caller, id, storage, trap};
use ic_cdk::api::time;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_cdk::timer::set_timer_interval;
use ic_cdk_macros::*;
use serde::Serialize;

use ego_tenant_mod::c2c::ego_file::EgoFile;
use ego_tenant_mod::c2c::ego_store::EgoStore;
use ego_tenant_mod::c2c::ic_management::IcManagement;
use ego_tenant_mod::ego_lib::ego_canister::EgoCanister;
use ego_tenant_mod::ego_macros::inject_ego_macros;
use ego_tenant_mod::ego_tenant::EgoTenant;
use ego_tenant_mod::service::{EgoTenantService, Registry, User};
use ego_tenant_mod::service::{canister_add, canister_list, is_owner, log_list_after, LogEntry, owner_add, registry_post_upgrade, registry_pre_upgrade, USER, user_add, users_post_upgrade, users_pre_upgrade};
use ego_tenant_mod::service::ego_log;
use ego_tenant_mod::state::EGO_TENANT;
use ego_tenant_mod::types::{
  AppMainInstallRequest, AppMainInstallResponse, AppMainUpgradeRequest, AppMainUpgradeResponse,
  CanisterMainTrackRequest, CanisterMainUnTrackRequest,
};
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
  ego_log(format!("ego-tenant: init, caller is {}", caller.clone()).as_str());

  ego_log("==> add caller as the owner");
  owner_add(caller.clone());

  let duration = Duration::new(1800, 0);
  set_timer_interval(duration, || {
    let _result = api::call::notify(id(), "message_main_notify", ());
  });
}

#[derive(CandidType, Deserialize, Serialize)]
struct PersistState {
  pub ego_tenant: EgoTenant,
  pub user: User,
  pub registry: Registry,
}

#[pre_upgrade]
fn pre_upgrade() {
  ego_log("ego-tenant: pre_upgrade");
  let ego_tenant = EGO_TENANT.with(|ego_tenant| ego_tenant.borrow().clone());
  let user = users_pre_upgrade();
  let registry = registry_pre_upgrade();

  let state = PersistState {
    ego_tenant,
    user,
    registry,
  };
  storage::stable_save((state, )).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
  ego_log("ego-tenant: post_upgrade");
  let (state, ): (PersistState, ) = storage::stable_restore().unwrap();
  EGO_TENANT.with(|ego_tenant| *ego_tenant.borrow_mut() = state.ego_tenant);

  users_post_upgrade(state.user);
  registry_post_upgrade(state.registry);

  let duration = Duration::new(1800, 0);
  set_timer_interval(duration, || {
    let _result = api::call::notify(id(), "message_main_notify", ());
  });
}


/********************  methods for ego_store   ********************/
#[update(name = "app_main_install", guard = "user_guard")]
#[candid_method(update, rename = "app_main_install")]
async fn app_main_install(req: AppMainInstallRequest) -> Result<AppMainInstallResponse, EgoError> {
  ego_log("ego_tenant: app_main_install");

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
  Ok(AppMainInstallResponse { canister_id })
}

#[update(name = "app_main_upgrade", guard = "user_guard")]
#[candid_method(update, rename = "app_main_upgrade")]
async fn app_main_upgrade(req: AppMainUpgradeRequest) -> Result<AppMainUpgradeResponse, EgoError> {
  ego_log("ego_tenant: app_main_upgrade");
  let management = IcManagement::new();
  let ego_file = EgoFile::new();

  let ret =
    EgoTenantService::app_main_upgrade(ego_file, management, req.canister_id, req.wasm).await?;
  Ok(AppMainUpgradeResponse { ret })
}

#[update(name = "canister_main_track", guard = "user_guard")]
#[candid_method(update, rename = "canister_main_track")]
fn canister_main_track(req: CanisterMainTrackRequest) -> Result<(), EgoError> {
  ego_log("ego_tenant: canister_main_track");

  EgoTenantService::canister_main_track(req.wallet_id, req.canister_id)?;
  Ok(())
}

#[update(name = "canister_main_untrack", guard = "user_guard")]
#[candid_method(update, rename = "canister_main_untrack")]
fn canister_main_untrack(req: CanisterMainUnTrackRequest) -> Result<(), EgoError> {
  ego_log("ego_tenant: canister_main_untrack");

  EgoTenantService::canister_main_untrack(req.wallet_id, req.canister_id)?;
  Ok(())
}

/********************  notify  ********************/
#[update(name = "message_main_notify")]
#[candid_method(update, rename = "message_main_notify")]
async fn message_main_notify() {
  ego_log("ego-tenant: message_main_notify");

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
        ego_log(&format!("canister_cycles_check failed: {:?}", e));
      }
    }
  }
}
