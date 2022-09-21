use ic_cdk::{storage};
use ic_cdk_macros::*;
use ego_ops_mod::service::EgoOpsService;
use ego_ops_mod::types::{CanisterMainCreateRequest, CanisterMainCreateResponse, CanisterMainListResponse};

use ego_types::ego_error::EgoError;
use candid::{candid_method};
use ego_ops_mod::ego_ops::EgoOps;
use ego_ops_mod::state::EGO_OPS;
use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;

use ego_users::inject_ego_users;

inject_ego_users!();

#[init]
fn init() {
  ic_cdk::println!("ego-ops: init, caller is {}", caller());

  ic_cdk::println!("==> add caller as the owner");
  users_init();
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
#[update(name = "canister_main_create")]
#[candid_method(update, rename = "canister_main_create")]
async fn canister_main_create(req: CanisterMainCreateRequest) -> Result<CanisterMainCreateResponse, EgoError> {
  ic_cdk::println!("ego-ops: canister_main_create");

  match EgoOpsService::canister_main_create(req.app_id, req.version, req.data, req.hash).await {
    Ok(ret) => {
      Ok(CanisterMainCreateResponse{ret})
    },
    Err(e) => Err(e)
  }
}

#[query(name = "canister_main_list")]
#[candid_method(query, rename = "canister_main_list")]
async fn canister_main_list() -> Result<CanisterMainListResponse, EgoError> {
  ic_cdk::println!("ego-ops: canister_main_list");

  EGO_OPS.with(|ego_ops| {
    Ok(CanisterMainListResponse{ canisters: ego_ops.borrow().canisters.clone()})
  })
}

#[update(name = "app_main_create")]
#[candid_method(update, rename = "app_main_create")]
async fn app_main_create(req: CanisterMainCreateRequest) -> Result<CanisterMainCreateResponse, EgoError> {
  ic_cdk::println!("ego-ops: app_main_create");

  match EgoOpsService::canister_main_create(req.app_id, req.version, req.data, req.hash).await {
    Ok(ret) => {
      Ok(CanisterMainCreateResponse{ret})
    },
    Err(e) => Err(e)
  }
}