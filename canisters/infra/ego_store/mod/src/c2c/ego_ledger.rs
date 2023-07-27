use ic_cdk::api;
use ic_cdk::export::Principal;
use ic_ledger_types::Tokens;

use crate::c2c::c2c_types::LedgerPaymentAddRequest;
use crate::types::order::Order;

pub trait TEgoLedger {
    fn ledger_payment_add(&self, order: &Order);
}

pub struct EgoLedger {
    pub canister_id: Principal,
}

impl EgoLedger {
    pub fn new(canister_id: Principal) -> Self {
        EgoLedger { canister_id }
    }
}

impl TEgoLedger for EgoLedger {
    fn ledger_payment_add(&self, order: &Order) {
        let req = LedgerPaymentAddRequest {
            from: order.from,
            to: order.to,
            amount: Tokens::from_e8s((order.amount * 1e8) as u64),
            memo: order.memo,
        };
        let _result = api::call::notify(self.canister_id, "ledger_payment_add", (req,));
    }
}
