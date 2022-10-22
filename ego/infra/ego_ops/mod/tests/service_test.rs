// use ic_cdk::export::Principal;
// use async_trait::async_trait;
// use mockall::mock;
// use ego_ops_mod::service::EgoOpsService;
// use ego_ops_mod::state::EGO_OPS;
// use ego_types::ego_error::EgoError;
//
// use ego_ops_mod::c2c::ego_cron::TEgoCron;
// use ego_ops_mod::c2c::ego_dev::TEgoDev;
// use ego_ops_mod::c2c::ego_store::TEgoStore;
// use ego_ops_mod::c2c::ego_user::TEgoUser;
// use ego_ops_mod::c2c::ego_tenant::TEgoTenant;
// use ego_ops_mod::c2c::c2c_types::{CronInterval};
// use ego_types::version::Version;
// use ego_types::app::{AppId, Category, DeployMode};
//
// mock! {
//   User {}
//
//   #[async_trait]
//   impl TEgoUser for User {
//     async fn role_user_add(&self, canister_id: Principal, principal: Principal) -> Result<bool, EgoError>;
//   }
// }
//
// mock! {
//   Dev {}
//
//   #[async_trait]
//   impl TEgoDev for Dev {
//     async fn admin_ego_file_add(&self, canister_id: Principal, ego_file_id: Principal) -> Result<bool, EgoError>;
//     async fn admin_ego_store_set(&self, canister_id: Principal, ego_store_id: Principal) -> Result<bool, EgoError>;
//     async fn admin_app_create(&self, canister_id: Principal, app_id: AppId, name: String, version: Version, category: Category, logo: String, description: String, backend_data: Vec<u8>, backend_data_hash: String, frontend: Option<Principal>, deploy_mode: DeployMode) -> Result<bool, EgoError>;
//   }
// }
//
// mock! {
//   Store {}
//
//   #[async_trait]
//   impl TEgoStore for Store {
//     async fn admin_ego_tenant_add(&self, canister_id: Principal, ego_tenant_id: Principal) -> Result<bool, EgoError>;
//     async fn ego_store_setup(&self, ego_store_id: Principal, ego_dev_id: Principal, ego_cron_id: Principal) -> Result<bool, EgoError>;
//   }
// }
//
// mock! {
//   Cron {}
//
//   #[async_trait]
//   impl TEgoCron for Cron {
//     async fn task_main_add(&self, ego_cron_id: Principal, canister_id: Principal, method: String, interval: CronInterval) -> Result<bool, EgoError>;
//   }
// }
//
//
// mock! {
//   Tenant {}
//
//   #[async_trait]
//   impl TEgoTenant for Tenant {
//     async fn ego_tenant_setup(&self, ego_tenant_id: Principal, ego_store_id: Principal, ego_cron_id: Principal) -> Result<bool, EgoError>;
//   }
// }
//
// static TEST_FILE_CANISTER_ID: &str = "22axz-3yaaa-aaaai-qklaa-cai";
//
// static DEV_CANISTER_ID: &str = "223xb-saaaa-aaaaf-arlqa-cai";
// static FILE_CANISTER_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";
//
// static STORE_CANISTER_ID: &str = "224jh-lqaaa-aaaad-qaxda-cai";
// static TENANT_CANISTER_ID: &str = "2265i-mqaaa-aaaad-qbsga-cai";
//
// static CRON_CANISTER_ID: &str = "225cg-4iaaa-aaaaj-adouq-cai";
// static LEDGER_CANISTER_ID: &str = "225da-yaaaa-aaaah-qahrq-cai";
//
//
// pub fn set_up() {
//   let dev_canister = Principal::from_text(DEV_CANISTER_ID.to_string()).unwrap();
//   EgoOpsService::canister_main_register("ego_dev".to_string(), dev_canister);
//
//   let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();
//   EgoOpsService::canister_main_register("ego_file".to_string(), file_canister);
//
//   let store_canister = Principal::from_text(STORE_CANISTER_ID.to_string()).unwrap();
//   EgoOpsService::canister_main_register("ego_store".to_string(), store_canister);
//
//   let tenant_canister = Principal::from_text(TENANT_CANISTER_ID.to_string()).unwrap();
//   EgoOpsService::canister_main_register("ego_tenant".to_string(), tenant_canister);
//
//   let cron_canister = Principal::from_text(CRON_CANISTER_ID.to_string()).unwrap();
//   EgoOpsService::canister_main_register("ego_cron".to_string(), cron_canister);
//
//   let ledger_canister = Principal::from_text(LEDGER_CANISTER_ID.to_string()).unwrap();
//   EgoOpsService::canister_main_register("ego_ledger".to_string(), ledger_canister);
// }
//
// #[test]
// fn canister_main_register() {
//   EGO_OPS.with(|ego_ops| {
//     assert!(!ego_ops.borrow().canisters.contains_key("ego_file"));
//   });
//
//   let test_file_canister = Principal::from_text(TEST_FILE_CANISTER_ID.to_string()).unwrap();
//   EgoOpsService::canister_main_register("ego_file".to_string(), test_file_canister);
//
//   EGO_OPS.with(|ego_ops| {
//     assert_eq!(1, ego_ops.borrow().canisters.get("ego_file").unwrap().len());
//   });
// }
//
// #[tokio::test]
// async fn canister_relation_update() {
//   set_up();
//
//   let dev_canister = Principal::from_text(DEV_CANISTER_ID.to_string()).unwrap();
//   let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();
//   let store_canister = Principal::from_text(STORE_CANISTER_ID.to_string()).unwrap();
//   let tenant_canister = Principal::from_text(TENANT_CANISTER_ID.to_string()).unwrap();
//   let cron_canister = Principal::from_text(CRON_CANISTER_ID.to_string()).unwrap();
//   let ledger_canister = Principal::from_text(LEDGER_CANISTER_ID.to_string()).unwrap();
//
//   let mut ego_user = MockUser::new();
//   let mut ego_dev = MockDev::new();
//   let mut ego_store = MockStore::new();
//   let mut ego_cron = MockCron::new();
//   let mut ego_tenant = MockTenant::new();
//
//   // ego_dev
//   ego_dev.expect_admin_ego_file_add().times(1).returning(move |_dev, _file| {
//     assert_eq!(dev_canister, _dev);
//     assert_eq!(file_canister, _file);
//     Ok(true)
//   });
//   ego_dev.expect_admin_ego_store_set().times(1).returning(move |_dev, _store| {
//     assert_eq!(dev_canister, _dev);
//     assert_eq!(store_canister, _store);
//     Ok(true)
//   });
//
//   // ego_file
//   ego_user.expect_role_user_add().times(1).returning(move |_file, _dev| {
//     assert_eq!(file_canister, _file);
//     assert_eq!(dev_canister, _dev);
//     Ok(true)
//   });
//   ego_user.expect_role_user_add().times(1).returning(move |_file, _tenant| {
//     assert_eq!(file_canister, _file);
//     assert_eq!(tenant_canister, _tenant);
//     Ok(true)
//   });
//
//   // ego_store
//   ego_store.expect_ego_store_setup().returning(move |_store, _dev, _cron| {
//     assert_eq!(store_canister, _store);
//     assert_eq!(dev_canister, _dev);
//     assert_eq!(cron_canister, _cron);
//     Ok(true)
//   });
//   ego_store.expect_admin_ego_tenant_add().times(1).returning(move |_store, _tenant| {
//     assert_eq!(store_canister, _store);
//     assert_eq!(tenant_canister, _tenant);
//     Ok(true)
//   });
//
//   // ego_tenant
//   ego_tenant.expect_ego_tenant_setup().times(1).returning(move |_tenant, _store, _cron| {
//     assert_eq!(tenant_canister, _tenant);
//     assert_eq!(store_canister, _store);
//     assert_eq!(cron_canister, _cron);
//     Ok(true)
//   });
//   ego_cron.expect_task_main_add().times(1).returning(move |_cron, _tenant, _, _| {
//     assert_eq!(cron_canister, _cron);
//     assert_eq!(tenant_canister, _tenant);
//     Ok(true)
//   });
//
//   // ego_ledger
//   ego_user.expect_role_user_add().times(1).returning(move |_ledger, _cron| {
//     assert_eq!(ledger_canister, _ledger);
//     assert_eq!(cron_canister, _cron);
//     Ok(true)
//   });
//   ego_cron.expect_task_main_add().times(1).returning(move |_cron, _ledger, _, _| {
//     assert_eq!(cron_canister, _cron);
//     assert_eq!(ledger_canister, _ledger);
//     Ok(true)
//   });
//
//
//   let resp = EgoOpsService::canister_relation_update(ego_user, ego_dev, ego_store, ego_cron, ego_tenant).await;
//   assert!(resp.is_ok())
// }