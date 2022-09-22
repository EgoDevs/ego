use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;
use candid::candid_method;
use ic_cdk::{storage};
use ic_cdk_macros::*;
use ego_ledger_mod::ego_ledger::EgoLedger;
use ego_ledger_mod::service::EgoLedgerService;

use ego_ledger_mod::state::{EGO_LEDGER, EGO_STORE_CANISTER_ID};
use ego_ledger_mod::types::{AdminEgoStoreSetRequest, LedgerMainInitRequest, LedgerPaymentAddRequest};
use ego_types::ego_error::EgoError;

use ego_users::inject_ego_users;

inject_ego_users!();

#[init]
fn init() {
  let caller = ic_cdk::caller();
  ic_cdk::println!("ego-ledger: init, caller is {}", caller);

  ic_cdk::println!("==> add caller as the owner");
  users_init();
}


#[derive(CandidType, Deserialize, Serialize)]
struct PersistState {
  pub ego_ledger: EgoLedger,
  pub user: User,
  pub ego_store: Option<Principal>,
}

#[pre_upgrade]
fn pre_upgrade() {
  ic_cdk::println!("ego-ledger: pre_upgrade");
  let ego_ledger = EGO_LEDGER.with(|ego_ledger| ego_ledger.borrow().clone());
  let user = users_pre_upgrade();
  let ego_store = EGO_STORE_CANISTER_ID.with(|r_c| r_c.borrow().clone());

  let state = PersistState { ego_ledger, user, ego_store };
  storage::stable_save((state, )).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
  ic_cdk::println!("ego-ledger: post_upgrade");
  let (state, ): (PersistState, ) = storage::stable_restore().unwrap();
  EGO_LEDGER.with(|ego_ledger|
    *ego_ledger.borrow_mut() = state.ego_ledger
  );

  users_post_upgrade(state.user);

  EGO_STORE_CANISTER_ID.with(|r_c| *r_c.borrow_mut() = state.ego_store);
}

/********************  user  ********************/
#[update(name = "ledger_payment_add", guard = "user_guard")]
#[candid_method(update, rename = "ledger_payment_add")]
fn ledger_payment_add(req: LedgerPaymentAddRequest) -> Result<(), EgoError> {
  ic_cdk::println!("ego-ledger: ledger_payment_add");
  EgoLedgerService::ledger_payment_add(req.from, req.to, req.amount, req.memo);
  Ok(())
}

/********************  owner  ********************/
#[update(name = "admin_ego_store_set", guard = "owner_guard")]
#[candid_method(update, rename = "admin_ego_store_set")]
pub fn admin_ego_store_set(req: AdminEgoStoreSetRequest) -> Result<(), EgoError> {
  ic_cdk::println!("ego-ledger: admin_ego_store_set");
  EGO_STORE_CANISTER_ID.with(|rc| *rc.borrow_mut() = Some(req.canister_id));
  Ok(())
}

#[update(name = "ledger_main_init", guard = "owner_guard")]
#[candid_method(update, rename = "ledger_main_init")]
async fn ledger_main_init(req: LedgerMainInitRequest) -> Result<(), EgoError> {
  ic_cdk::println!("ego-ledger: ledger_main_init");
  EgoLedgerService::ledger_main_init(req.start);
  Ok(())
}


/********************  notify  ********************/
#[update(name = "message_main_notify", guard = "user_guard")]
#[candid_method(update, rename = "message_main_notify")]
async fn message_main_notify() -> Result<(), EgoError> {
  ic_cdk::println!("ego-ledger: message_main_notify");

  EgoLedgerService::ledger_block_query().await?;

  Ok(())
}
