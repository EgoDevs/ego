use astrox_macros::{inject_canister_log, inject_canister_registry};
use astrox_macros::inject_canister_users;
use ic_ledger_types::{AccountIdentifier, BlockIndex, Memo, Operation, Tokens, Transaction};

use ego_macros::inject_log;
use ego_types::ego_error::EgoError;

use crate::c2c::ego_store::TEgoStore;
use crate::c2c::ic_ledger::TIcLedger;
use crate::payment::{Payment, PaymentStatus};
use crate::state::EGO_LEDGER;

inject_log!();
inject_canister_users!();
inject_canister_registry!();
inject_canister_log!();

/********************  methods for ego_registry   ********************/
fn on_canister_added(name: &str, canister_id: Principal) {
  let _ = match name {
    "ego_store" => user_add(canister_id),
    _ => {}
  };
}

pub struct EgoLedgerService {}

impl EgoLedgerService {
  pub fn ledger_main_init(start: BlockIndex) {
    EGO_LEDGER.with(|ego_ledger| ego_ledger.borrow_mut().ledger_main_init(start))
  }

  pub fn ledger_payment_add(
    from: AccountIdentifier,
    to: AccountIdentifier,
    amount: Tokens,
    memo: Memo,
  ) {
    EGO_LEDGER.with(|ego_ledger| {
      let payment = Payment::new(from, to, amount, memo);
      ego_ledger.borrow_mut().ledger_payment_add(payment)
    })
  }

  pub async fn ledger_payment_match<S: TEgoStore, IL: TIcLedger>(ego_store: S, ic_ledger: IL) -> Result<(), EgoError> {
    ic_cdk::println!("1.query blocks");
    let start = EGO_LEDGER.with(|ego_ledger| ego_ledger.borrow().start);

    let blocks = ic_ledger.query_blocks(start).await?;

    ic_cdk::println!("2.add block to confirmed");
    EGO_LEDGER.with(|ego_ledger| {
      let mut e_l = ego_ledger.borrow_mut();
      for block in blocks {
        let trx: Transaction = block.clone().transaction;
        if trx.operation.is_some() {
          match trx.operation.unwrap() {
            Operation::Transfer {
              from,
              to,
              amount,
              fee: _,
            } => {
              ic_cdk::println!("3.from:{}, to:{}, amount:{}, memo:{:?}", from, to, amount, trx.memo);
              e_l.block_confirm(from, to, amount);
            }
            _ => {}
          }
        }
      }
    });

    ic_cdk::println!("4.notify ego_store");
    EGO_LEDGER.with(|ego_ledger| {
      for (_to, payment) in ego_ledger.borrow_mut().payments.iter_mut() {
        ego_store.wallet_order_notify(payment.memo);
        payment.status = PaymentStatus::NOTIFIED;
      }
    });

    ic_cdk::println!("5.remove notified successes memos");
    EGO_LEDGER.with(|ego_ledger| {
      ego_ledger.borrow_mut().payments.retain(|_to, payment| payment.status != PaymentStatus::NOTIFIED)
    });

    Ok(())
  }
}
