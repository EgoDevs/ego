use ic_cdk::api;
use ic_cdk::export::Principal;

use ego_types::app::AppId;

use crate::c2c::c2c_types::{
    AdminWalletCycleRechargeRequest, AdminWalletProviderAddRequest, WalletMainRegisterRequest,
    WalletOrderNewRequest,
};

pub trait TEgoStore {
    fn admin_wallet_provider_add(&self, wallet_provider: Principal, wallet_app_id: AppId);

    fn admin_wallet_cycle_recharge(&self, wallet_id: Principal, cycle: u128, comment: String);

    fn admin_wallet_order_new(&self, amount: f32);

    fn admin_wallet_main_register(&self, user_id: Principal);
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
    fn admin_wallet_provider_add(&self, wallet_provider: Principal, wallet_app_id: AppId) {
        let req = AdminWalletProviderAddRequest {
            wallet_provider,
            wallet_app_id,
        };

        let _result = api::call::notify(self.canister_id, "admin_wallet_provider_add", (req,));
    }

    fn admin_wallet_cycle_recharge(&self, wallet_id: Principal, cycle: u128, comment: String) {
        let req = AdminWalletCycleRechargeRequest {
            wallet_id,
            cycle,
            comment,
        };

        let _result = api::call::notify(self.canister_id, "admin_wallet_cycle_recharge", (req,));
    }

    fn admin_wallet_order_new(&self, amount: f32) {
        let req = WalletOrderNewRequest { amount };

        let _result = api::call::notify(self.canister_id, "wallet_order_new", (req,));
    }

    fn admin_wallet_main_register(&self, user_id: Principal) {
        let req = WalletMainRegisterRequest { user_id };

        let _result = api::call::notify(self.canister_id, "wallet_main_register", (req,));
    }
}
