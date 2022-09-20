use ic_cdk::{caller, storage};
use ic_cdk_macros::*;
use ego_ops_mod::service::EgoOpsService;
use ego_ops_mod::types::{CanisterMainCreateRequest, CanisterMainCreateResponse, CanisterMainListResponse};

use ego_types::ego_error::EgoError;
use candid::{candid_method};
use ego_ops_mod::c2c::ego_file::EgoFile;
use ego_ops_mod::c2c::ic_management::IcManagement;
use ego_ops_mod::ego_ops::EgoOps;
use ego_ops_mod::state::EGO_OPS;

#[init]
fn init() {
  ic_cdk::println!("ego-ops: init, caller is {}", caller());
}

#[pre_upgrade]
fn pre_upgrade() {
  ic_cdk::println!("ego-ops: pre_upgrade");
  EGO_OPS.with(|ego_ops| storage::stable_save((ego_ops,)).unwrap());
}

#[post_upgrade]
fn post_upgrade() {
  ic_cdk::println!("ego-ops: post_upgrade");
  let (old_ego_ops,): (EgoOps,) = storage::stable_restore().unwrap();
  EGO_OPS.with(|ego_ops|
    *ego_ops.borrow_mut() = old_ego_ops
  );
}

/********************   owner method   ********************/
#[update(name = "canister_main_create")]
#[candid_method(update, rename = "canister_main_create")]
async fn canister_main_create(req: CanisterMainCreateRequest) -> Result<CanisterMainCreateResponse, EgoError> {
  ic_cdk::println!("ego-ops: canister_main_create");

  let ic_management = IcManagement::new();
  let ego_file = EgoFile::new();
  match EgoOpsService::canister_main_create(ego_file, ic_management, req.app_id, req.version, req.data, req.hash).await {
    Ok(ret) => Ok(CanisterMainCreateResponse{ret}),
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
