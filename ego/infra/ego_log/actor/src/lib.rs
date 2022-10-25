use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;
use candid::candid_method;
use ic_cdk::{storage};
use ic_cdk::api::time;

use ic_cdk_macros::*;
use ego_log_mod::ego_log::EgoLog;
use ego_log_mod::log::Log;
use ego_log_mod::state::{EGO_LOG};

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
  ic_cdk::println!("ego-log: init, caller is {}", caller.clone());

  ic_cdk::println!("==> add caller as the owner");
  users_init(caller.clone());
}


#[derive(CandidType, Deserialize, Serialize)]
struct PersistState {
  pub ego_log: EgoLog,
  pub user: User
}

#[pre_upgrade]
fn pre_upgrade() {
  ic_cdk::println!("ego-log: pre_upgrade");
  let ego_log = EGO_LOG.with(|ego_log| ego_log.borrow().clone());
  let user = users_pre_upgrade();

  let state = PersistState { ego_log, user };
  storage::stable_save((state, )).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
  ic_cdk::println!("ego-log: post_upgrade");
  let (state, ): (PersistState, ) = storage::stable_restore().unwrap();
  EGO_LOG.with(|ego_log|
    *ego_log.borrow_mut() = state.ego_log
  );

  users_post_upgrade(state.user);
}

/********************  user  ********************/
#[update(name = "canister_log_add", guard = "user_guard")]
#[candid_method(update, rename = "canister_log_add")]
fn canister_log_add(message: String) {
  ic_cdk::println!("ego-log: canister_log_add");
  EGO_LOG.with(|ego_log| {
    ego_log.borrow_mut().canister_log_add(caller(), time(), message)
  });
}

#[query(name = "canister_log_get", guard = "user_guard")]
#[candid_method(query, rename = "canister_log_get")]
fn canister_log_get(from_ts: u64, to_ts: u64) -> Vec<Log>{
  ic_cdk::println!("ego-log: canister_log_get");
  EGO_LOG.with(|ego_log| {
    ego_log.borrow().canister_log_get(from_ts, to_ts)
  })
}

/********************  notify  ********************/
#[update(name = "message_main_notify", guard = "user_guard")]
#[candid_method(update, rename = "message_main_notify")]
async fn message_main_notify() {
  ic_cdk::println!("ego-log: message_main_notify");

}
