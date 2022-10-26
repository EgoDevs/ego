use crate::ego_ledger::EgoLedger;
use std::cell::RefCell;

thread_local! {
  pub static EGO_LEDGER: RefCell<EgoLedger> = RefCell::new(EgoLedger::new());
}
