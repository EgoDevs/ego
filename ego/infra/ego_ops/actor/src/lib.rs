use ic_cdk::{storage};
use ic_cdk_macros::*;
use ego_ops_mod::service::EgoOpsService;

use ego_types::ego_error::EgoError;
use candid::{candid_method};
use ego_ops_mod::ego_ops::EgoOps;
use ego_ops_mod::state::EGO_OPS;
use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;
use ego_ops_mod::types::{AdminAppCreateRequest, AdminAppCreateResponse, AdminAppDeployRequest, AdminAppDeployResponse, CanisterMainListResponse, CanisterMainRegisterRequest};

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
/// register the initial canister into the ego_ops
#[update(name = "canister_main_register")]
#[candid_method(update, rename = "canister_main_register")]
async fn canister_main_register(req: CanisterMainRegisterRequest) -> Result<(), EgoError> {
  ic_cdk::println!("ego-ops: canister_main_register");

  EgoOpsService::canister_main_register(req.app_id, req.canister_id);

  Ok(())
}

/// register the initial canister into the ego_ops
#[update(name = "canister_relation_update")]
#[candid_method(update, rename = "canister_relation_update")]
async fn canister_relation_update() -> Result<(), EgoError> {
  ic_cdk::println!("ego-ops: canister_relation_update");

  EgoOpsService::canister_relation_update().await?;

  Ok(())
}



#[query(name = "canister_main_list")]
#[candid_method(query, rename = "canister_main_list")]
async fn canister_main_list() -> Result<CanisterMainListResponse, EgoError> {
  ic_cdk::println!("ego-ops: canister_main_list");

  EGO_OPS.with(|ego_ops| {
    Ok(CanisterMainListResponse{ canisters: ego_ops.borrow().canisters.clone()})
  })
}

/// register ego infra app
#[update(name = "admin_app_create")]
#[candid_method(update, rename = "admin_app_create")]
async fn admin_app_create(req: AdminAppCreateRequest) -> Result<AdminAppCreateResponse, EgoError> {
  ic_cdk::println!("ego-ops: admin_app_create");

  match EgoOpsService::admin_app_create(req.app_id, req.name, req.version, req.backend_data, req.backend_hash, req.frontend).await {
    Ok(ret) => {
      Ok(AdminAppCreateResponse{ret})
    },
    Err(e) => Err(e)
  }
}

/// deploy ego infra canister
#[update(name = "admin_app_deploy")]
#[candid_method(update, rename = "admin_app_deploy")]
async fn admin_app_deploy(req: AdminAppDeployRequest) -> Result<AdminAppDeployResponse, EgoError> {
  ic_cdk::println!("ego-ops: admin_app_create");

  match EgoOpsService::admin_app_deploy(req.app_id).await {
    Ok(ret) => {
      Ok(AdminAppDeployResponse{ret})
    },
    Err(e) => Err(e)
  }
}
