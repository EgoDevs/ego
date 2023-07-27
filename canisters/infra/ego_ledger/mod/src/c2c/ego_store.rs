use candid::Principal;
use ic_cdk::api;
use ic_ledger_types::Memo;

pub trait TEgoStore {
    fn wallet_order_notify(&self, memo: Memo);
}

pub struct EgoStore {
    pub canister_id: Principal,
}

impl EgoStore {
    pub fn new(canister_id: Principal) -> Self {
        EgoStore { canister_id }
    }
}

impl TEgoStore for EgoStore {
    fn wallet_order_notify(&self, memo: Memo) {
        let _result = api::call::notify(self.canister_id, "wallet_order_notify", (memo,));
    }
}
