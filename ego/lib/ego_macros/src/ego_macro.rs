#[macro_export]
macro_rules! inject_ego_macros {
    () => {
        // for user management
        #[update(name = "ego_owner_add", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_owner_add")]
        pub fn ego_owner_add(principal: Principal) -> Result<(), String> {
            owner_add(principal);
            Ok(())
        }

        #[inline(always)]
        pub fn owner_guard() -> Result<(), String> {
            let caller = ic_cdk::api::caller();
            let ret = is_owner(caller);
            if ret {
                Ok(())
            } else {
                ic_cdk::api::trap(&format!("{} unauthorized", caller));
            }
        }

        #[update(name = "ego_user_add", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_user_add")]
        pub fn ego_user_add(principal: Principal) -> Result<(), String> {
            user_add(principal);
            Ok(())
        }

        #[inline(always)]
        pub fn user_guard() -> Result<(), String> {
            let caller = ic_cdk::api::caller();
            let ret = USER.with(|b| b.borrow().is_user(caller));
            if ret {
                Ok(())
            } else {
                ic_cdk::api::trap(&format!("{} unauthorized", caller));
            }
        }

        #[inline(always)]
        pub fn op_guard() -> Result<(), String> {
            let caller = caller();
            let ret = USER.with(|b| b.borrow().is_op(caller));
            if ret {
                Ok(())
            } else {
                ic_cdk::api::trap(&format!("{} unauthorized", caller));
            }
        }

        // for canister management
        #[update(name = "ego_canister_add", guard = "op_guard")]
        #[candid_method(update, rename = "ego_canister_add")]
        pub fn ego_canister_add(name: String, canister_id: Principal) -> Result<(), String> {
            canister_add(name, canister_id);
            Ok(())
        }

        #[query(name = "ego_canister_list", guard = "op_guard")]
        #[candid_method(query, rename = "ego_canister_list")]
        pub fn ego_canister_list() -> Result<BTreeMap<String, Vec<Principal>>, String> {
            Ok(canister_list())
        }

        // for log
        #[query(name = "ego_log_list", guard = "op_guard")]
        #[candid_method(query, rename = "ego_log_list")]
        pub fn ego_log_list(after_ts: u64) -> Result<Vec<LogEntry>, String> {
            Ok(log_list_after(after_ts))
        }

        // balance
        #[query(name = "balance_get")]
        #[candid_method(update, rename = "balance_get")]
        pub fn balance_get() -> u128 {
            ic_cdk::api::canister_balance128()
        }
    };
}

#[macro_export]
macro_rules! inject_log {
    () => {
        pub fn ego_log(message: &str) {
            ic_cdk::println!("{}", message.to_string());

            {
                // TODO: need a way to call time()
                // log_add(ic_cdk::api::time(), message.to_string());
                log_add(0, message.to_string());
            }

        }
    }
}
