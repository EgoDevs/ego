use crate::ego_ledger::EgoLedger;
use std::cell::RefCell;
use ic_types::Principal;

thread_local! {
  pub static EGO_LEDGER: RefCell<EgoLedger> = RefCell::new(EgoLedger::new());
  pub static EGO_STORE_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
}