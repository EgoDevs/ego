use std::collections::BTreeMap;

use candid::{CandidType, Deserialize, Principal};
use candid::candid_method;
use ic_cdk::{caller, storage};
use ic_cdk_macros::*;
use ic_ledger_types::MAINNET_LEDGER_CANISTER_ID;
use serde::Serialize;

use ego_ledger_mod::c2c::ego_store::EgoStore;
use ego_ledger_mod::c2c::ic_ledger::IcLedger;
use ego_ledger_mod::ego_ledger::EgoLedger;
use ego_ledger_mod::payment::Payment;
use ego_ledger_mod::service::EgoLedgerService;
use ego_ledger_mod::state::*;
use ego_ledger_mod::state::EGO_LEDGER;
use ego_ledger_mod::types::{LedgerMainInitRequest, LedgerPaymentAddRequest};
use ego_macros::{inject_cycle_info_api, inject_ego_api};
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
  info_log_add(format!("ego-ledger: init, caller is {}", caller.clone()).as_str());

  info_log_add("==> add caller as the owner");
  owner_add(caller.clone());

  // remove order check
  // let duration = Duration::new(60, 0);
  // set_timer_interval(duration, || {
  //   let _result = api::call::notify(id(), "message_main_notify", ());
  // });
}

#[derive(CandidType, Deserialize, Serialize)]
struct PersistState {
  pub ego_ledger: EgoLedger,
  users: Option<User>,
  registry: Option<Registry>,
  cycle_info: Option<CycleInfo>,
}

#[pre_upgrade]
fn pre_upgrade() {
  info_log_add("ego-ledger: pre_upgrade");
  let ego_ledger = EGO_LEDGER.with(|ego_ledger| ego_ledger.borrow().clone());

  let state = PersistState {
    ego_ledger,
    users: Some(users_pre_upgrade()),
    registry: Some(registry_pre_upgrade()),
    cycle_info: Some(cycle_info_pre_upgrade()),
  };
  storage::stable_save((state, )).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
  info_log_add("ego-ledger: post_upgrade");
  let (state, ): (PersistState, ) = storage::stable_restore().unwrap();
  EGO_LEDGER.with(|ego_ledger| *ego_ledger.borrow_mut() = state.ego_ledger);

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

  // remove order check
  // let duration = Duration::new(60, 0);
  // set_timer_interval(duration, || {
  //   let _result = api::call::notify(id(), "message_main_notify", ());
  // });
}

/********************  user  ********************/
#[update(name = "ledger_payment_add", guard = "user_guard")]
#[candid_method(update, rename = "ledger_payment_add")]
fn ledger_payment_add(req: LedgerPaymentAddRequest) -> Result<(), EgoError> {
  info_log_add(
    format!(
      "ego-ledger: ledger_payment_add from:{} to:{} memo:{:?}",
      req.from, req.to, req.memo
    )
      .as_str(),
  );

  EgoLedgerService::ledger_payment_add(req.from, req.to, req.amount, req.memo);
  Ok(())
}

/********************  owner  ********************/
#[update(name = "ledger_main_init", guard = "owner_guard")]
#[candid_method(update, rename = "ledger_main_init")]
fn ledger_main_init(req: LedgerMainInitRequest) -> Result<(), EgoError> {
  info_log_add("ego-ledger: ledger_main_init");
  EgoLedgerService::ledger_main_init(req.start);
  Ok(())
}

#[update(name = "ledger_payment_list", guard = "owner_guard")]
#[candid_method(update, rename = "ledger_payment_list")]
fn ledger_payment_list() -> Result<Vec<Payment>, EgoError> {
  info_log_add("ego-ledger: ledger_payment_list");

  let payments =
    EGO_LEDGER.with(|ego_ledger| ego_ledger.borrow().payments.values().cloned().collect());

  Ok(payments)
}

/********************  notify  ********************/
#[update(name = "message_main_notify")]
#[candid_method(update, rename = "message_main_notify")]
async fn message_main_notify() {
  info_log_add("ego-ledger: message_main_notify");

  let ego_store_id = canister_get_one("ego_store").unwrap();
  let ego_store = EgoStore::new(ego_store_id);

  let ic_ledger = IcLedger::new(MAINNET_LEDGER_CANISTER_ID);

  match EgoLedgerService::ledger_payment_match(ego_store, ic_ledger).await {
    Ok(_) => {}
    Err(_) => {}
  }
}

/********************  methods for ego_cycle_threshold_get   ********************/
pub fn cycle_threshold_get() -> u128 {
  1_000_000_000_000
}

pub fn runtime_cycle_threshold_get() -> u128 {
  1_000_000_000_000
}
