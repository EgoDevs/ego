// for user management

#[macro_export]
macro_rules! inject_ego_api {
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
            log_add(format!("ego_owner_add {}", principal).as_str());
            owner_add(principal);
            Ok(())
        }


        #[update(name = "ego_owner_remove", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_owner_remove")]
        pub fn ego_owner_remove(principal: Principal) -> Result<(), String> {
            log_add(format!("ego_owner_remove {}", principal).as_str());
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
            log_add(format!("ego_user_add {}", principal).as_str());
            user_add(principal);
            Ok(())
        }

        #[update(name = "ego_user_remove", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_user_remove")]
        pub fn ego_user_remove(principal: Principal) -> Result<(), String> {
            log_add(format!("ego_user_remove {}", principal).as_str());
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
            log_add(format!("ego_op_add {}", principal).as_str());
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

        #[update(name = "ego_canister_add", guard = "op_guard")]
        #[candid_method(update, rename = "ego_canister_add")]
        pub fn ego_canister_add(name: String, canister_id: Principal) -> Result<(), String> {
            canister_add(name, canister_id);
            Ok(())
        }

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

        // for log
        #[query(name = "ego_log_list", guard = "op_guard")]
        #[candid_method(query, rename = "ego_log_list")]
        pub fn ego_log_list(amount: usize) -> Result<Vec<String>, String> {
            Ok(log_list(amount))
        }

        // for balance
        #[query(name = "balance_get", guard = "op_guard")]
        #[candid_method(query, rename = "balance_get")]
        pub fn balance_get() -> Result<u128, String>  {
            Ok(ic_cdk::api::canister_balance128())
        }
    }
}

#[macro_export]
macro_rules! inject_app_info_api {
    () => {
        // for canister info
        use ego_types::app_info::AppInfo;
        use ego_types::app::{AppId, Version};
        use ego_lib::ego_store::{TEgoStore, EgoStore};

        thread_local! {
          pub static CANISTER_INFO: std::cell::RefCell<AppInfo> = std::cell::RefCell::new(AppInfo::default());
        }

        #[update(name = "app_info_update", guard = "op_guard")]
        #[candid_method(update, rename = "app_info_update")]
        pub fn app_info_update(wallet_id: Principal, app_id: AppId, version: Version) -> Result<(), String> {
            log_add(format!("app_info_update {}", app_id.clone()).as_str());

            CANISTER_INFO.with(|c_i| {
                c_i.borrow_mut().wallet_id = Some(wallet_id);
                c_i.borrow_mut().app_id = app_id;
                c_i.borrow_mut().current_version = version;
                c_i.borrow_mut().latest_version = version;
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

        #[update(name = "app_version_check", guard = "op_guard")]
        #[candid_method(update, rename = "app_version_check")]
        pub async fn app_version_check() -> Result<AppInfo, String> {
            let app_id = CANISTER_INFO.with(|c_i| {
                c_i.borrow().app_id.clone()
            });

            log_add(format!("app_version_check {}", app_id.clone()).as_str());

            let ego_store_id = canister_get_one("ego_store").unwrap();
            let ego_store = EgoStore::new(ego_store_id);

            let app = match ego_store.app_main_get(app_id).await{
              Ok(app) => Ok(app),
              Err(e) => Err(e.msg)
            }?;

            CANISTER_INFO.with(|c_i| {
                c_i.borrow_mut().latest_version = app.current_version;
            });

            let app_info = CANISTER_INFO.with(|c_i| {
                c_i.borrow_mut().clone()
            });
            Ok(app_info)
        }

        // canister app upgrade
        #[update(name = "ego_canister_upgrade", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_canister_upgrade")]
        pub async fn ego_canister_upgrade() -> Result<(), String> {
            let wallet_id = CANISTER_INFO.with(|c_i| {
                c_i.borrow().wallet_id.unwrap().clone()
            });

            log_add("ego_canister_upgrade");

            log_add("1 add ego_tenant as controller");
            let ego_tenant_id = canister_get_one("ego_tenant").unwrap();
            let _result = match controller_add(ic_cdk::api::id(), ego_tenant_id).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e.msg)
            };

            log_add("2 call ego_store to upgrade");
            let ego_store_id = canister_get_one("ego_store").unwrap();
            let ego_store = EgoStore::new(ego_store_id);

            ego_store.wallet_app_upgrade(wallet_id);

            Ok(())
        }
    };
}