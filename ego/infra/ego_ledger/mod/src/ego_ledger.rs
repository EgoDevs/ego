use std::collections::BTreeMap;

use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_ledger_types::{BlockIndex, Memo};
use serde::Serialize;

use crate::payment::Payment;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct EgoLedger {
    pub wait_for_notified: BTreeMap<Memo, Payment>,
    pub confirmed: BTreeMap<Memo, Payment>,
    pub start: BlockIndex,
}

impl EgoLedger {
    pub fn new() -> Self {
        EgoLedger {
            wait_for_notified: BTreeMap::new(),
            confirmed: BTreeMap::new(),
            start: 0u64,
        }
    }

    pub fn ledger_main_init(&mut self, start: BlockIndex) {
        self.start = start;
    }

    pub fn ledger_payment_add(&mut self, payment: Payment) {
        self.wait_for_notified
            .entry(payment.memo)
            .or_insert(payment);
    }

    pub fn ledger_payment_matched(&self) -> Vec<Memo> {
        let memos = self
            .wait_for_notified
            .iter()
            .filter_map(|(w_memo, w_payment)| match self.confirmed.get(w_memo) {
                Some(c_payment) => {
                    if c_payment == w_payment {
                        Some(w_memo.clone())
                    } else {
                        None
                    }
                }
                None => None,
            })
            .collect();

        memos
    }

    pub fn ledger_payment_remove(&mut self, successes_memos: Vec<Memo>) {
        for successes_memo in successes_memos {
            self.wait_for_notified.remove(&successes_memo);
            self.confirmed.remove(&successes_memo);
        }
    }
}
