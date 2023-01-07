#[macro_export]
macro_rules! inject_mock_ego_canister {
  () => {
    mock! {
      Canister {}

      #[async_trait]
      impl TEgoCanister for Canister {
        fn ego_owner_set(&self, target_canister_id: Principal, principals: Vec<Principal>);
        fn ego_owner_add(&self, target_canister_id: Principal, principal: Principal);
        fn ego_owner_remove(&self, target_canister_id: Principal, principal: Principal);

        fn ego_user_set(&self, target_canister_id: Principal, user_ids: Vec<Principal>);
        fn ego_user_add(&self, target_canister_id: Principal, principal: Principal);
        fn ego_user_remove(&self, target_canister_id: Principal, principal: Principal);

        fn ego_op_add(&self, target_canister_id: Principal, user_id: Principal);

        fn ego_canister_add(&self, target_canister_id: Principal, name: String, principal: Principal);

        fn ego_controller_set(&self, target_canister_id: Principal, principals: Vec<Principal>);
        async fn ego_controller_add(&self, target_canister_id: Principal, principal: Principal) -> Result<(), String>;
        fn ego_controller_remove(&self, target_canister_id: Principal, principal: Principal);

        async fn balance_get(&self, target_canister_id: Principal) -> Result<u128, String>;

        // app info
        fn ego_app_info_update(&self, target_canister_id: Principal, wallet_id: Option<Principal>, app_id: AppId, version: Version);
        async fn ego_app_info_get(&self, target_canister_id: Principal) -> Result<AppInfo, String>;
        async fn ego_app_version_check(&self, target_canister_id: Principal) -> Result<App, String>;

        // canister upgrade
        fn ego_canister_upgrade(&self, target_canister_id: Principal);
        fn ego_canister_remove(&self, target_canister_id: Principal);
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
        async fn wallet_main_new(&self, user_id: Principal) -> Result<UserApp, EgoError>;

        async fn app_main_list(&self) -> Result<Vec<App>, EgoError>;
        async fn app_main_get(&self, app_id: AppId) -> Result<App, EgoError>;

        async fn wallet_app_install(&self, app_id: AppId) -> Result<UserApp, EgoError>;
        fn wallet_app_upgrade(&self, wallet_id: Principal);
        fn wallet_app_remove(&self, wallet_id: Principal);
        async fn wallet_app_list(&self) -> Result<Vec<UserApp>, EgoError>;
      }
    }
  }
}