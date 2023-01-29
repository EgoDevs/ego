#[macro_export]
macro_rules! inject_app_info {
    () => {
        thread_local! {
          pub static APP_INFO: RefCell<AppInfo> = RefCell::new(AppInfo::default());
        }

        use ego_types::app_info::AppInfo;
        use ego_types::app::{AppId, Version};

        pub fn app_info_get() -> AppInfo {
            APP_INFO.with(|info| {
                info.borrow().clone()
            })
        }

        pub fn app_info_update(wallet_id: Option<Principal>, app_id: AppId, version: Version)  {
            APP_INFO.with(|info| {
                if wallet_id.is_some() {
                    info.borrow_mut().wallet_id = wallet_id;
                }
                info.borrow_mut().app_id = app_id;
                info.borrow_mut().current_version = version;
                info.borrow_mut().latest_version = version;
            });
        }

        pub fn app_info_pre_upgrade() -> AppInfo {
            APP_INFO.with(|s| s.take().into())
        }

        pub fn app_info_post_upgrade(stable_state: AppInfo) {
            APP_INFO.with(|s| s.replace(stable_state));
        }
    };
}

#[macro_export]
macro_rules! inject_cycle_info {
    () => {
        thread_local! {
          pub static CYCLE_INFO: RefCell<CycleInfo> = RefCell::new(CycleInfo::new());
        }

        use ego_types::cycle_info::CycleInfo;
        use ego_types::cycle_info::CycleRecord;

        pub fn cycle_record_add(balance: u128, ts: u64) {
            CYCLE_INFO.with(|cycle_info|{
                cycle_info.borrow_mut().record_add(balance, ts);
            })
        }

        pub fn cycle_record_list() -> Vec<CycleRecord> {
            CYCLE_INFO.with(|cycle_info|{
                cycle_info.borrow().record_list()
            })
        }

        pub fn cycle_info_get() -> CycleInfo {
            CYCLE_INFO.with(|cycle_info|{
                cycle_info.borrow().clone()
            })
        }

        pub fn estimate_remaining_set(estimate: u64) {
            CYCLE_INFO.with(|cycle_info|{
                cycle_info.borrow_mut().estimate_remaining_set(estimate)
            })
        }

        pub fn cycle_info_pre_upgrade() -> CycleInfo {
            CYCLE_INFO.with(|s| s.take().into())
        }

        pub fn cycle_info_post_upgrade(stable_state: CycleInfo) {
            CYCLE_INFO.with(|s| s.replace(stable_state));
        }

        pub fn is_cycle_available(amount: u128) -> bool {
            let balance = ic_cdk::api::canister_balance128();

            if balance > amount {
                true
            } else {
                false
            }
        }
    };
}



#[macro_export]
macro_rules! inject_ego_data {
    () => {
        thread_local! {
          pub static LOG: RefCell<Log> = RefCell::new(Log::new());
        }

        use ego_types::log::Log;

        pub fn info_log_add(log: &str)  {
            ic_cdk::println!("{}", log.to_string());
            LOG.with(|s| s.borrow_mut().info_info_log_add(log.to_string()));
        }

        pub fn error_log_add(log: &str)  {
            ic_cdk::println!("{}", log.to_string());
            LOG.with(|s| s.borrow_mut().error_info_log_add(log.to_string()));
        }

        pub fn log_list(amount: usize) -> Vec<String> {
            LOG.with(|s| s.borrow().log_list(amount))
        }

        pub fn log_clear(remain: usize) {
            LOG.with(|s| s.borrow_mut().log_clear(remain));
        }

        thread_local! {
          pub static REGISTRY: RefCell<Registry> = RefCell::new(Registry::default());
        }

        use ego_types::registry::Registry;
        use std::collections::BTreeMap;

        pub fn canister_add(name: String, canister_id: Principal)  {
            REGISTRY.with(|s| s.borrow_mut().canister_add(name.clone(), canister_id));
            on_canister_added(&name, canister_id);
        }

        pub fn canister_remove(name: String, canister_id: Principal)  {
            REGISTRY.with(|s| s.borrow_mut().canister_remove(name, canister_id));
        }

        pub fn canister_remove_all(name: String)  {
            REGISTRY.with(|s| s.borrow_mut().canister_remove_all(name));
        }

        pub fn canister_list() -> BTreeMap<String, Vec<Principal>> {
            REGISTRY.with(|s| s.borrow().canister_list_all())
        }

        pub fn canister_get_one(name: &str) -> Option<Principal> {
            REGISTRY.with(|s| s.borrow().canister_get_one(name))
        }

        pub fn canister_get_all(name: &str) -> Vec<Principal> {
            REGISTRY.with(|s| s.borrow().canister_get_all(name))
        }

        pub fn registry_pre_upgrade() -> Registry {
            REGISTRY.with(|s| s.take().into())
        }

        pub fn registry_post_upgrade(stable_state: Registry) {
            REGISTRY.with(|s| s.replace(stable_state));
        }

        thread_local! {
          pub static USER: RefCell<User> = RefCell::new(User::default());
        }

        use ego_types::user::User;
        use ic_cdk::caller;
        use ic_cdk::export::Principal;

        /* owner relative methods */
        pub fn is_owner(user_id: Principal) -> bool {
            USER.with(|b| b.borrow().is_owner(user_id))
        }

        pub fn owners_set(users: BTreeMap<Principal, String>) {
            USER.with(|s| s.borrow_mut().owners_set(users));
        }

        pub fn owners() -> Option<BTreeMap<Principal, String>> {
            USER.with(|s| s.borrow().owners())
        }


        pub fn owner_add(user_id: Principal) {
            USER.with(|s| s.borrow_mut().owner_add(user_id.to_text(), user_id));
        }


        pub fn owner_add_with_name(name: String, user_id: Principal) {
            USER.with(|s| s.borrow_mut().owner_add(name, user_id));
        }


        pub fn owner_remove(user_id: Principal) {
            USER.with(|s| s.borrow_mut().owner_remove(user_id));
        }

        /* user relative methods */
        pub fn users_set(users: BTreeMap<Principal, String>)  {
            USER.with(|s| s.borrow_mut().users_set(users));
        }


        pub fn users() -> Option<BTreeMap<Principal, String>> {
            USER.with(|s| s.borrow().users())
        }

        pub fn user_add(user_id: Principal) {
            USER.with(|s| s.borrow_mut().user_add(user_id.to_text(), user_id));
        }

        pub fn user_add_with_name(name: String, user_id: Principal) {
            USER.with(|s| s.borrow_mut().user_add(name, user_id));
        }

        pub fn user_remove(user_id: Principal) {
            USER.with(|s| s.borrow_mut().user_remove(user_id));
        }

        pub fn is_user(user_id: Principal) -> bool {
            USER.with(|b| b.borrow().is_user(user_id))
        }

        /* op relative methods */
        pub fn ops_set(users: BTreeMap<Principal, String>)  {
            USER.with(|s| s.borrow_mut().ops_set(users));
        }

        pub fn ops() -> Option<BTreeMap<Principal, String>> {
            USER.with(|s| s.borrow().ops())
        }

        pub fn op_add(user_id: Principal) {
            USER.with(|s| s.borrow_mut().op_add(user_id.to_text(), user_id));
        }

        pub fn op_add_with_name(name: String, user_id: Principal) {
            USER.with(|s| s.borrow_mut().op_add(name, user_id));
        }

        pub fn op_remove(user_id: Principal) {
            USER.with(|s| s.borrow_mut().op_remove(user_id));
        }

        pub fn is_op(user_id: Principal) -> bool {
            USER.with(|b| b.borrow().is_op(user_id))
        }

        pub fn users_pre_upgrade() -> User {
            USER.with(|s| s.take().into())
        }

        pub fn users_post_upgrade(stable_state: User) {
            USER.with(|s| s.replace(stable_state));
        }
    }
}