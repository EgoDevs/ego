use std::cell::RefCell;

use crate::ego_ledger::EgoLedger;

thread_local! {
  pub static EGO_LEDGER: RefCell<EgoLedger> = RefCell::new(EgoLedger::new());
}
