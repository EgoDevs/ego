use std::collections::BTreeMap;

use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_ledger_types::{AccountIdentifier, BlockIndex, Tokens};
use serde::Serialize;

use crate::payment::{Payment, PaymentStatus};

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct EgoLedger {
    pub payments: BTreeMap<AccountIdentifier, Payment>,
    pub start: BlockIndex,
}

impl EgoLedger {
    pub fn new() -> Self {
        EgoLedger {
            payments: BTreeMap::new(),
            start: 0u64,
        }
    }

    pub fn ledger_main_init(&mut self, start: BlockIndex) {
        self.start = start;
    }

    pub fn ledger_payment_add(&mut self, payment: Payment) {
        self.payments
            .entry(payment.to)
            .or_insert(payment);
    }

    pub fn block_confirm(&mut self, _from: AccountIdentifier, to: AccountIdentifier, amount: Tokens) {
        if self.payments.contains_key(&to) {
            let payment = self.payments.get_mut(&to).unwrap();
            if payment.amount == amount {
                payment.status = PaymentStatus::CONFIRMED
            }
        }
    }
}
