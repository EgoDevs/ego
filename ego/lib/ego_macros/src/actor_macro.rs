// for user management

#[macro_export]
macro_rules! inject_ego_user {
    () => {
        #[update(name = "ego_owner_set", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_owner_set")]
        pub fn ego_owner_set(principals: Vec<Principal>) -> Result<(), String> {
            owners_set(BTreeMap::default());
            for &principal in &principals {
                owner_add(principal);
            }
            Ok(())
        }

        #[update(name = "ego_owner_add", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_owner_add")]
        pub fn ego_owner_add(principal: Principal) -> Result<(), String> {
            owner_add(principal);
            Ok(())
        }


        #[update(name = "ego_owner_remove", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_owner_remove")]
        pub fn ego_owner_remove(principal: Principal) -> Result<(), String> {
            owner_remove(principal);
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

        #[update(name = "ego_user_set", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_user_set")]
        pub fn ego_user_set(principals: Vec<Principal>) -> Result<(), String> {
            users_set(BTreeMap::default());
            for &principal in &principals {
                user_add(principal);
            }
            Ok(())
        }

        #[update(name = "ego_user_add", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_user_add")]
        pub fn ego_user_add(principal: Principal) -> Result<(), String> {
            user_add(principal);
            Ok(())
        }

        #[update(name = "ego_user_remove", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_user_remove")]
        pub fn ego_user_remove(principal: Principal) -> Result<(), String> {
            user_remove(principal);
            Ok(())
        }

        #[inline(always)]
        pub fn user_guard() -> Result<(), String> {
            let caller = ic_cdk::api::caller();
            let ret = is_user(caller);
            if ret {
                Ok(())
            } else {
                ic_cdk::api::trap(&format!("{} unauthorized", caller));
            }
        }

        #[update(name = "ego_op_add", guard = "op_guard")]
        #[candid_method(update, rename = "ego_op_add")]
        pub fn ego_op_add(principal: Principal) -> Result<(), String> {
            op_add(principal);
            Ok(())
        }

        #[inline(always)]
        pub fn op_guard() -> Result<(), String> {
            let caller = ic_cdk::api::caller();
            let ret = is_op(caller);
            if ret {
                Ok(())
            } else {
                ic_cdk::api::trap(&format!("{} unauthorized", caller));
            }
        }
    }
}

#[macro_export]
macro_rules! inject_ego_registry {
    () => {
        #[update(name = "ego_canister_add", guard = "op_guard")]
        #[candid_method(update, rename = "ego_canister_add")]
        pub fn ego_canister_add(name: String, canister_id: Principal) -> Result<(), String> {
            canister_add(name, canister_id);
            Ok(())
        }
    }
}

// for controller management
#[macro_export]
macro_rules! inject_ego_controller {
    () => {
        use ego_lib::ic_management::{controller_set, controller_add, controller_remove};

        #[update(name = "ego_controller_set", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_controller_set")]
        pub async fn ego_controller_set(principals: Vec<Principal>) -> Result<(), String> {
            match controller_set(ic_cdk::api::id(), principals).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e.msg)
            }
        }

        #[update(name = "ego_controller_add", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_controller_add")]
        pub async fn ego_controller_add(principal: Principal) -> Result<(), String> {
            match controller_add(ic_cdk::api::id(), principal).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e.msg)
            }
        }

        #[update(name = "ego_controller_remove", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_controller_remove")]
        pub async fn ego_controller_remove(principal: Principal) -> Result<(), String> {
            match controller_remove(ic_cdk::api::id(), principal).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e.msg)
            }
        }
    }
}

#[macro_export]
macro_rules! inject_ego_log {
    () => {
        // for log
        #[query(name = "ego_log_list", guard = "op_guard")]
        #[candid_method(query, rename = "ego_log_list")]
        pub fn ego_log_list(amount: usize) -> Result<Vec<String>, String> {
            Ok(log_list(amount))
        }
    }
}

#[macro_export]
macro_rules! inject_ego_app_info {
    () => {
        #[query(name = "balance_get", guard = "op_guard")]
        #[candid_method(query, rename = "balance_get")]
        pub fn balance_get() -> Result<u128, String>  {
            Ok(ic_cdk::api::canister_balance128())
        }

        // for canister info
        use astrox_macros::app_info::AppInfo;
        use astrox_macros::ego_types::{AppId, Version};
        use ego_lib::ego_store::{TEgoStore, EgoStore};

        thread_local! {
          pub static CANISTER_INFO: std::cell::RefCell<AppInfo> = std::cell::RefCell::new(AppInfo::default());
        }

        #[update(name = "app_info_update", guard = "op_guard")]
        #[candid_method(update, rename = "app_info_update")]
        pub fn app_info_update(app_id: AppId, version: Version) -> Result<(), String> {
            CANISTER_INFO.with(|c_i| {
                c_i.borrow_mut().app_id = app_id;
                c_i.borrow_mut().current_version = version;
            });
            Ok(())
        }

        #[query(name = "app_info_get", guard = "op_guard")]
        #[candid_method(query, rename = "app_info_get")]
        pub fn app_info_get() -> Result<AppInfo, String>  {
            let app_info = CANISTER_INFO.with(|c_i| {
                c_i.borrow_mut().clone()
            });
            Ok(app_info)
        }


        #[query(name = "app_version_check", guard = "op_guard")]
        #[candid_method(query, rename = "app_version_check")]
        pub async fn app_version_check() -> Result<App, String> {
            let ego_store_id = canister_get_one("ego_store").unwrap();
            let ego_store = EgoStore::new(ego_store_id);
            let app_id = CANISTER_INFO.with(|c_i| {
                c_i.borrow().app_id.clone()
            });
            match ego_store.app_main_get(app_id).await{
              Ok(app) => Ok(app),
              Err(e) => Err(e.msg)
            }
        }
    };
}

#[macro_export]
macro_rules! inject_ego_all {
    () => {
        use ego_lib::{inject_ego_user, inject_ego_registry, inject_ego_controller, inject_ego_log, inject_ego_app_info};
        inject_ego_user!();
        inject_ego_registry!();
        inject_ego_controller!();
        inject_ego_log!();
        inject_ego_app_info!();

    }
}