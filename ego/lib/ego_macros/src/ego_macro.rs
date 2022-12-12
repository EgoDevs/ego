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

        #[update(name = "ego_user_add", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_user_add")]
        pub fn ego_user_add(principal: Principal) -> Result<(), String> {
            user_add(principal);
            Ok(())
        }

        // for canister management
        #[update(name = "ego_canister_add", guard = "op_guard")]
        #[candid_method(update, rename = "ego_canister_add")]
        pub fn ego_canister_add(name: String, canister_id: Principal) -> Result<(), String> {
            canister_add(name, canister_id);
            Ok(())
        }

        #[update(name = "ego_canister_list", guard = "op_guard")]
        #[candid_method(update, rename = "ego_canister_list")]
        pub fn ego_canister_list() -> Result<BTreeMap<String, Vec<Principal>>, String> {
            Ok(canister_list())
        }

        // for log
        use ego_macros::ego_log::{TEgoLogCanister, EgoLogCanister};

        pub fn ego_log(message: &str) {
            // for development
            ic_cdk::println!("{}", message.to_string());

            // for production
            // ic_cdk::println!("ego-log: message: {}", message.clone());
            // match REGISTRY.with(|r| r.borrow().canister_get_one("ego_log")) {
            //     None => {},
            //     Some(ego_log_id) => {
            //         let ego_log = EgoLogCanister::new(ego_log_id);
            //         ego_log.canister_log_add(message);
            //     }
            // };
        }

        pub fn get_ego_log() -> Option<EgoLogCanister> {
            match REGISTRY.with(|r| r.borrow().canister_get_one("ego_log")) {
                None => {
                    None
                },
                Some(ego_log_id) => {
                    Some(EgoLogCanister::new(ego_log_id))
                }
            }
        }
    };
}
