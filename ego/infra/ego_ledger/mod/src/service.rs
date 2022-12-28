
use ic_ledger_types::{AccountIdentifier, BlockIndex, Memo, Operation, Tokens, Transaction};

use ego_types::app::EgoError;

use crate::c2c::ego_store::TEgoStore;
use crate::c2c::ic_ledger::TIcLedger;
use crate::payment::{Payment, PaymentStatus};
use crate::state::{EGO_LEDGER, log_add};



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
    log_add("1.query blocks");
    let start = EGO_LEDGER.with(|ego_ledger| ego_ledger.borrow().start);

    let blocks = ic_ledger.query_blocks(start).await?;

    log_add("2.add block to confirmed");
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
              log_add(format!("3.from:{}, to:{}, amount:{}, memo:{:?}", from, to, amount, trx.memo).as_str());
              e_l.block_confirm(from, to, amount);
            }
            _ => {}
          }
        }
      }
    });

    log_add("4.notify ego_store");
    EGO_LEDGER.with(|ego_ledger| {
      for (_to, payment) in ego_ledger.borrow_mut().payments.iter_mut() {
        ego_store.wallet_order_notify(payment.memo);
        payment.status = PaymentStatus::NOTIFIED;
      }
    });

    log_add("5.remove notified successes memos");
    EGO_LEDGER.with(|ego_ledger| {
      ego_ledger.borrow_mut().payments.retain(|_to, payment| payment.status != PaymentStatus::NOTIFIED)
    });

    Ok(())
  }
}
