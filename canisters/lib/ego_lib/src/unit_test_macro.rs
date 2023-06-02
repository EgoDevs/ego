#[macro_export]
macro_rules! inject_mock_ego_canister {
  () => {
    use ego_types::cycle_info::CycleInfo;
    use ego_types::cycle_info::CycleRecord;
    use ego_types::log::LogEntry;

    mock! {
      Canister {}

      #[async_trait]
      impl TEgoCanister for Canister {
        fn ego_owner_set(&self, target_canister_id: Principal, principals: Vec<Principal>);
        fn ego_owner_add(&self, target_canister_id: Principal, principal: Principal);
        fn ego_owner_remove(&self, target_canister_id: Principal, principal: Principal);
        async fn ego_owner_list(&self, target_canister_id: Principal) -> Result<Option<std::collections::BTreeMap<Principal, String>>, String>;

        fn ego_user_set(&self, target_canister_id: Principal, user_ids: Vec<Principal>);
        fn ego_user_add(&self, target_canister_id: Principal, principal: Principal);
        fn ego_user_remove(&self, target_canister_id: Principal, principal: Principal);
        async fn ego_user_list(&self, target_canister_id: Principal) -> Result<Option<std::collections::BTreeMap<Principal, String>>, String>;

        fn ego_op_add(&self, target_canister_id: Principal, user_id: Principal);
        fn ego_op_remove(&self, target_canister_id: Principal, principal: Principal);
        async fn ego_op_list(&self, target_canister_id: Principal) -> Result<Option<std::collections::BTreeMap<Principal, String>>, String>;

        fn ego_canister_add(&self, target_canister_id: Principal, name: String, principal: Principal);
        fn ego_canister_remove(&self, target_canister_id: Principal, name: String, principal: Principal);
        async fn ego_canister_list(&self, target_canister_id: Principal) -> Result<std::collections::BTreeMap<String, Vec<Principal>>, String>;

        async fn ego_controller_set(&self, target_canister_id: Principal, principals: Vec<Principal>);
        async fn ego_controller_add(&self, target_canister_id: Principal, principal: Principal);
        fn ego_controller_remove(&self, target_canister_id: Principal, principal: Principal);

        async fn balance_get(&self, target_canister_id: Principal) -> Result<u128, String>;

        // app info
        fn ego_app_info_update(&self, target_canister_id: Principal, wallet_id: Option<Principal>, app_id: AppId, version: Version);
        async fn ego_app_info_get(&self, target_canister_id: Principal) -> Result<AppInfo, String>;
        async fn ego_app_version_check(&self, target_canister_id: Principal) -> Result<App, String>;

        // canister relative
        fn ego_canister_upgrade(&self, target_canister_id: Principal);
        fn ego_canister_delete(&self, target_canister_id: Principal);

        // canister cycle info
        fn ego_cycle_check(&self, target_canister_id: Principal);
        async fn ego_cycle_history(&self, target_canister_id: Principal) -> Result<Vec<CycleRecord>, String>;
        async fn ego_cycle_info(&self, target_canister_id: Principal) -> Result<CycleInfo, String>;
        fn ego_cycle_estimate_set(&self, target_canister_id: Principal, estimate: u64);
        async fn ego_cycle_threshold_get(&self, target_canister_id: Principal) -> Result<u128, String>;
        async fn ego_runtime_cycle_threshold_get(&self, target_canister_id: Principal) -> Result<u128, String>;
        async fn ego_cycle_recharge(&self, target_canister_id: Principal, cycles: u128) -> Result<(), String>;

        async fn ego_log_list(&self, target_canister_id: Principal, amount: usize) -> Result<Vec<LogEntry>, String>;
      }
    }
  }
}

#[macro_export]
macro_rules! inject_mock_ego_store {
    () => {
        mock! {
          Store {}

          #[async_trait]
          impl TEgoStore for Store {
            async fn wallet_main_register(&self, user_id: Principal) -> Result<Principal, EgoError>;

            async fn wallet_main_new(&self, user_id: Principal) -> Result<UserApp, EgoError>;

            async fn app_main_list(&self) -> Result<Vec<App>, EgoError>;
            async fn app_main_get(&self, app_id: AppId) -> Result<App, EgoError>;

            async fn wallet_app_install(&self, app_id: AppId) -> Result<UserApp, EgoError>;
            async fn wallet_app_upgrade(&self, wallet_id: Principal);
            async fn wallet_app_upgrade_by_wallet(&self, canister_id: Principal);

            fn wallet_app_remove(&self, wallet_id: Principal);
            async fn wallet_app_list(&self) -> Result<Vec<UserApp>, EgoError>;

            async fn wallet_cycle_balance(&self) -> Result<u128, EgoError>;
            async fn wallet_cycle_list(&self) -> Result<Vec<ego_types::app::CashFlow>, EgoError>;

            // canister track
            fn wallet_canister_track(&self, canister_id: Principal);
            fn wallet_canister_untrack(&self, canister_id: Principal);
          }
        }
    };
}

#[macro_export]
macro_rules! inject_mock_ego_tenant {
    () => {
        mock! {
          Tenant {}

          #[async_trait]
          pub trait TEgoTenant {
            fn ego_cycle_check_cb(&self, records: Vec<CycleRecord>, threshold: u128);
            async fn wallet_cycle_recharge(&self, cycles: u128) -> Result<(), EgoError>;
          }
        }
    };
}
