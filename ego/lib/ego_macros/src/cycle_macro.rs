#[macro_export]
macro_rules! inject_balance_get {
    () => {
        #[query(name = "balance_get")]
        #[candid_method(update, rename = "balance_get")]
        pub fn balance_get() -> u128 {
            ic_cdk::api::canister_balance128()
        }
    };
}