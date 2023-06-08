// for user management

#[macro_export]
macro_rules! inject_ego_api {
    () => {
        #[update(name = "ego_owner_set", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_owner_set")]
        pub fn ego_owner_set(principals: Vec<Principal>) -> Result<(), String> {
            owners_set(BTreeMap::default());
            for &principal in &principals {
                info_log_add(format!("ego_owner_add {}", principal).as_str());
                owner_add(principal);
            }
            Ok(())
        }

        #[update(name = "ego_owner_add_with_name", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_owner_add_with_name")]
        pub fn ego_owner_add_with_name(name: String, principal: Principal) -> Result<(), String> {
            info_log_add(
                format!(
                    "ego_owner_add_with_name name:{}, principal:{}",
                    name, principal
                )
                .as_str(),
            );
            owner_add_with_name(name, principal);
            Ok(())
        }

        #[update(name = "ego_owner_add", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_owner_add")]
        pub fn ego_owner_add(principal: Principal) -> Result<(), String> {
            info_log_add(format!("ego_owner_add {}", principal).as_str());
            owner_add(principal);
            Ok(())
        }

        #[update(name = "ego_owner_remove", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_owner_remove")]
        pub fn ego_owner_remove(principal: Principal) -> Result<(), String> {
            info_log_add(format!("ego_owner_remove {}", principal).as_str());
            owner_remove(principal);
            Ok(())
        }

        #[query(name = "ego_is_owner")]
        #[candid_method(query, rename = "ego_is_owner")]
        pub fn ego_is_owner() -> Result<bool, String> {
            let ret = is_owner(caller());
            Ok(ret)
        }

        #[update(name = "ego_owner_list")]
        #[candid_method(update, rename = "ego_owner_list")]
        pub fn ego_owner_list() -> Result<Option<BTreeMap<Principal, String>>, String> {
            Ok(owners())
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
            info_log_add(format!("ego_user_add {}", principal).as_str());
            user_add(principal);
            Ok(())
        }

        #[update(name = "ego_user_remove", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_user_remove")]
        pub fn ego_user_remove(principal: Principal) -> Result<(), String> {
            info_log_add(format!("ego_user_remove {}", principal).as_str());
            user_remove(principal);
            Ok(())
        }

        #[query(name = "ego_is_user")]
        #[candid_method(query, rename = "ego_is_user")]
        pub fn ego_is_user() -> Result<bool, String> {
            let ret = is_user(caller());
            Ok(ret)
        }

        #[update(name = "ego_user_list")]
        #[candid_method(update, rename = "ego_user_list")]
        pub fn ego_user_list() -> Result<Option<BTreeMap<Principal, String>>, String> {
            Ok(users())
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
            info_log_add(format!("ego_op_add {}", principal).as_str());
            op_add(principal);
            Ok(())
        }

        #[update(name = "ego_op_remove", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_op_remove")]
        pub fn ego_op_remove(principal: Principal) -> Result<(), String> {
            info_log_add(format!("ego_op_remove {}", principal).as_str());
            op_remove(principal);
            Ok(())
        }

        #[query(name = "ego_is_op")]
        #[candid_method(query, rename = "ego_is_op")]
        pub fn ego_is_op() -> Result<bool, String> {
            let ret = is_op(caller());
            Ok(ret)
        }

        #[update(name = "ego_op_list")]
        #[candid_method(update, rename = "ego_op_list")]
        pub fn ego_op_list() -> Result<Option<BTreeMap<Principal, String>>, String> {
            Ok(ops())
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

        #[update(name = "ego_canister_remove", guard = "op_guard")]
        #[candid_method(update, rename = "ego_canister_remove")]
        pub fn ego_canister_remove(name: String, canister_id: Principal) -> Result<(), String> {
            canister_remove(name, canister_id);
            Ok(())
        }

        #[update(name = "ego_canister_list", guard = "op_guard")]
        #[candid_method(update, rename = "ego_canister_list")]
        pub fn ego_canister_list() -> Result<BTreeMap<String, Vec<Principal>>, String> {
            Ok(canister_list())
        }

        use ego_lib::ic_management::{controller_add, controller_remove, controller_set};

        #[update(name = "ego_controller_set", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_controller_set")]
        pub async fn ego_controller_set(principals: Vec<Principal>) -> Result<(), String> {
            match controller_set(ic_cdk::api::id(), principals).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e.msg),
            }
        }

        #[update(name = "ego_controller_add", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_controller_add")]
        pub async fn ego_controller_add(principal: Principal) -> Result<(), String> {
            match controller_add(ic_cdk::api::id(), principal).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e.msg),
            }
        }

        #[update(name = "ego_controller_remove", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_controller_remove")]
        pub async fn ego_controller_remove(principal: Principal) -> Result<(), String> {
            match controller_remove(ic_cdk::api::id(), principal).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e.msg),
            }
        }



        // for log
        #[query(name = "ego_log_list", guard = "op_guard")]
        #[candid_method(query, rename = "ego_log_list")]
        pub fn ego_log_list(amount: usize) -> Result<Vec<ego_types::log::LogEntry>, String> {
            Ok(log_list(amount))
        }
    };
}

#[macro_export]
macro_rules! inject_app_info_api {
    () => {
        // for canister info
        use ego_lib::ego_store::{EgoStore, TEgoStore};
        use ego_types::app::{AppId, Version};
        use ego_types::app_info::AppInfo;
        use ego_types::types::{AppUpgradeRequest, AppReInstallRequest};

        #[update(name = "ego_app_info_update", guard = "op_guard")]
        #[candid_method(update, rename = "ego_app_info_update")]
        pub fn ego_app_info_update(wallet_id: Option<Principal>, app_id: AppId, version: Version) {
            info_log_add("ego_app_info_update wallet_id");

            app_info_update(wallet_id, app_id, version);
        }

        #[query(name = "ego_app_info_get", guard = "op_guard")]
        #[candid_method(query, rename = "ego_app_info_get")]
        pub fn ego_app_info_get() -> Result<AppInfo, String> {
            Ok(app_info_get())
        }

        #[update(name = "ego_app_version_check", guard = "op_guard")]
        #[candid_method(update, rename = "ego_app_version_check")]
        pub async fn ego_app_version_check() -> Result<AppInfo, String> {
            let app_info = app_info_get();

            info_log_add(format!("app_version_check {}", app_info.app_id).as_str());

            let ego_store_id = canister_get_one("ego_store").unwrap();
            let ego_store = EgoStore::new(ego_store_id);

            let app = match ego_store.app_main_get(app_info.app_id).await {
                Ok(app) => Ok(app),
                Err(e) => Err(e.msg),
            }?;

            app_info_update(app_info.wallet_id, app.app_id, app.current_version);

            Ok(app_info_get())
        }

        // canister app upgrade
        #[update(name = "ego_canister_upgrade", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_canister_upgrade")]
        pub async fn ego_canister_upgrade() -> Result<(), String> {
            info_log_add("ego_canister_upgrade");

            let caller = caller();
            let app_info = app_info_get();

            info_log_add("1 add ego_tenant as controller");
            let ego_tenant_id = canister_get_one("ego_tenant").unwrap();
            let _result = match controller_add(ic_cdk::api::id(), ego_tenant_id).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e.msg),
            };

            info_log_add("2 call ego_store to upgrade");
            let ego_store_id = canister_get_one("ego_store").unwrap();
            let ego_store = EgoStore::new(ego_store_id);

            ego_store
                .wallet_app_upgrade_v2(AppUpgradeRequest{wallet_id: app_info.wallet_id.unwrap()})
                .await;

            Ok(())
        }

        // canister remove
        #[update(name = "ego_canister_delete", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_canister_delete")]
        pub async fn ego_canister_delete() -> Result<(), String> {
            let app_info = app_info_get();

            info_log_add("ego_canister_delete");

            info_log_add("1 add ego_tenant as controller");
            let ego_tenant_id = canister_get_one("ego_tenant").unwrap();
            let _result = match controller_add(ic_cdk::api::id(), ego_tenant_id).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e.msg),
            };

            info_log_add("2 call ego_store to delete");
            let ego_store_id = canister_get_one("ego_store").unwrap();
            let ego_store = EgoStore::new(ego_store_id);

            ego_store.wallet_app_remove(app_info.wallet_id.unwrap());

            Ok(())
        }

        // track canister
        #[update(name = "ego_canister_track", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_canister_track")]
        pub async fn ego_canister_track() -> Result<(), String> {
            let app_info = app_info_get();

            info_log_add("ego_canister_track");

            info_log_add("2 call ego_store to track");
            let ego_store_id = canister_get_one("ego_store").unwrap();
            let ego_store = EgoStore::new(ego_store_id);

            ego_store.wallet_canister_track_self(app_info.wallet_id.unwrap());

            Ok(())
        }

        // untrack canister
        #[update(name = "ego_canister_untrack", guard = "owner_guard")]
        #[candid_method(update, rename = "ego_canister_untrack")]
        pub async fn ego_canister_untrack() -> Result<(), String> {
            let app_info = app_info_get();

            info_log_add("ego_canister_untrack");

            info_log_add("2 call ego_store to untrack");
            let ego_store_id = canister_get_one("ego_store").unwrap();
            let ego_store = EgoStore::new(ego_store_id);

            ego_store.wallet_canister_untrack_self(app_info.wallet_id.unwrap());

            Ok(())
        }
    };
}

#[macro_export]
macro_rules! inject_cycle_info_api {
    () => {
        // for canister cycle info
        use ego_lib::ego_tenant::{EgoTenant, TEgoTenant};
        use ego_types::cycle_info::{CycleInfo, CycleRecord};

        use std::ops::Div;

        #[update(name = "ego_cycle_check", guard = "op_guard")]
        #[candid_method(update, rename = "ego_cycle_check")]
        pub fn ego_cycle_check() -> Result<(), String> {
            info_log_add("ego_cycle_check");

            let balance = ic_cdk::api::canister_balance128();
            let ts = ic_cdk::api::time().div(1e9 as u64);

            cycle_record_add(balance, ts);

            let ego_tenant_id = canister_get_one("ego_tenant").unwrap();
            let ego_tenant = EgoTenant::new(ego_tenant_id);

            ego_tenant.ego_cycle_check_cb(cycle_record_list(), runtime_cycle_threshold_get());

            Ok(())
        }

        #[update(name = "ego_cycle_history", guard = "op_guard")]
        #[candid_method(update, rename = "ego_cycle_history")]
        pub fn ego_cycle_history() -> Result<Vec<CycleRecord>, String> {
            Ok(cycle_record_list())
        }

        #[update(name = "ego_cycle_info", guard = "op_guard")]
        #[candid_method(update, rename = "ego_cycle_info")]
        pub async fn ego_cycle_info() -> Result<CycleInfo, String> {
            Ok(cycle_info_get())
        }

        #[update(name = "ego_cycle_estimate_set", guard = "op_guard")]
        #[candid_method(update, rename = "ego_cycle_estimate_set")]
        pub fn ego_cycle_estimate_set(estimate: u64) -> Result<(), String> {
            info_log_add(format!("ego_cycle_estimate_set {}", estimate).as_str());
            estimate_remaining_set(estimate);
            Ok(())
        }

        #[update(name = "ego_cycle_threshold_get", guard = "op_guard")]
        #[candid_method(update, rename = "ego_cycle_threshold_get")]
        pub fn ego_cycle_threshold_get() -> Result<u128, String> {
            Ok(cycle_threshold_get())
        }

        #[update(name = "ego_runtime_cycle_threshold_get", guard = "op_guard")]
        #[candid_method(update, rename = "ego_runtime_cycle_threshold_get")]
        pub fn ego_runtime_cycle_threshold_get() -> Result<u128, String> {
            Ok(runtime_cycle_threshold_get())
        }

        #[update(name = "ego_cycle_recharge", guard = "op_guard")]
        #[candid_method(update, rename = "ego_cycle_recharge")]
        pub async fn ego_cycle_recharge(cycles: u128) -> Result<(), String> {
            let ego_tenant_id = canister_get_one("ego_tenant").unwrap();
            let ego_tenant = EgoTenant::new(ego_tenant_id);

            match ego_tenant.wallet_cycle_recharge(cycles).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e.msg),
            }
        }

        #[query(name = "balance_get", guard = "op_guard")]
        #[candid_method(query, rename = "balance_get")]
        pub fn balance_get() -> Result<u128, String> {
            Ok(ic_cdk::api::canister_balance128())
        }
    };
}
