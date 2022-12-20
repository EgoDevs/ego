use std::collections::BTreeMap;
use std::time::Duration;

use candid::candid_method;
use ic_cdk::{api, caller, id, storage, trap};
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_cdk::timer::set_timer_interval;
use ic_cdk_macros::*;
use ic_ledger_types::MAINNET_LEDGER_CANISTER_ID;
use serde::Serialize;

use ego_ledger_mod::c2c::ego_store::EgoStore;
use ego_ledger_mod::c2c::ic_ledger::IcLedger;
use ego_ledger_mod::ego_ledger::EgoLedger;
use ego_ledger_mod::ego_macros::inject_ego_macros;
use ego_ledger_mod::payment::Payment;
use ego_ledger_mod::service::{canister_add, canister_get_one, canister_list, ego_log, EgoLedgerService, is_owner, log_list_after, LogEntry, owner_add, Registry, registry_post_upgrade, registry_pre_upgrade, User, USER, user_add, users_post_upgrade, users_pre_upgrade};
use ego_ledger_mod::state::EGO_LEDGER;
use ego_ledger_mod::types::{
  LedgerMainInitRequest, LedgerPaymentAddRequest,
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
  ego_log(format!("ego-ledger: init, caller is {}", caller.clone()).as_str());

  ego_log("==> add caller as the owner");
  owner_add(caller.clone());

  let duration = Duration::new(60, 0);
  set_timer_interval(duration, || {
    let _result = api::call::notify(id(), "message_main_notify", ());
  });
}

#[derive(CandidType, Deserialize, Serialize)]
struct PersistState {
  pub ego_ledger: EgoLedger,
  pub user: User,
  pub registry: Registry,
}

#[pre_upgrade]
fn pre_upgrade() {
  ego_log("ego-ledger: pre_upgrade");
  let ego_ledger = EGO_LEDGER.with(|ego_ledger| ego_ledger.borrow().clone());
  let user = users_pre_upgrade();
  let registry = registry_pre_upgrade();

  let state = PersistState {
    ego_ledger,
    user,
    registry,
  };
  storage::stable_save((state, )).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
  ego_log("ego-ledger: post_upgrade");
  let (state, ): (PersistState, ) = storage::stable_restore().unwrap();
  EGO_LEDGER.with(|ego_ledger| *ego_ledger.borrow_mut() = state.ego_ledger);

  users_post_upgrade(state.user);
  registry_post_upgrade(state.registry);

  let duration = Duration::new(60, 0);
  set_timer_interval(duration, || {
    let _result = api::call::notify(id(), "message_main_notify", ());
  });
}

/********************  user  ********************/
#[update(name = "ledger_payment_add", guard = "user_guard")]
#[candid_method(update, rename = "ledger_payment_add")]
fn ledger_payment_add(req: LedgerPaymentAddRequest) -> Result<(), EgoError> {
  ego_log(format!("ego-ledger: ledger_payment_add from:{} to:{} memo:{:?}", req.from, req.to, req.memo).as_str());

  EgoLedgerService::ledger_payment_add(req.from, req.to, req.amount, req.memo);
  Ok(())
}

/********************  owner  ********************/
#[update(name = "ledger_main_init", guard = "owner_guard")]
#[candid_method(update, rename = "ledger_main_init")]
fn ledger_main_init(req: LedgerMainInitRequest) -> Result<(), EgoError> {
  ego_log("ego-ledger: ledger_main_init");
  EgoLedgerService::ledger_main_init(req.start);
  Ok(())
}

#[update(name = "ledger_payment_list", guard = "owner_guard")]
#[candid_method(update, rename = "ledger_payment_list")]
fn ledger_payment_list() -> Result<Vec<Payment>, EgoError> {
  ego_log("ego-ledger: ledger_payment_list");

  let payments = EGO_LEDGER.with(|ego_ledger| ego_ledger.borrow().payments.values().cloned().collect());

  Ok(payments)
}

/********************  notify  ********************/
#[update(name = "message_main_notify")]
#[candid_method(update, rename = "message_main_notify")]
async fn message_main_notify() {
  ego_log("ego-ledger: message_main_notify");

  let ego_store_id = canister_get_one("ego_store").unwrap();
  let ego_store = EgoStore::new(ego_store_id);

  let ic_ledger = IcLedger::new(MAINNET_LEDGER_CANISTER_ID);

  match EgoLedgerService::ledger_payment_match(ego_store, ic_ledger).await {
    Ok(_) => {}
    Err(_) => {}
  }
}
