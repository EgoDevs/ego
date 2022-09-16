use async_trait::async_trait;
use ic_types::Principal;
use mockall::mock;

use ego_tenant_mod::service::EgoTenantService;
use ego_utils::types::{Cycles, EgoError, Management};
use ic_cdk::api::management_canister::main::{CanisterStatusResponse};
use ego_tenant_mod::state::EGO_TENANT;
use ego_tenant_mod::wallet::Wallet;

static EXISTS_WALLET_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";

static TEST_WALLET_ID: &str = "227wz-liaaa-aaaaa-qaara-cai";

pub fn set_up() {
  let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();

  EGO_TENANT.with(|ego_tenant| {
    // add tenant
    ego_tenant.borrow_mut().wallets.insert(wallet_principal, Wallet::new(wallet_principal));
  });
}

mock! {
  Dump {}


  #[async_trait]
  impl Management for Dump {
    // canister relative methods
    async fn canister_main_create(&self, cycles_to_use: Cycles) -> Result<Principal, EgoError>;
    async fn canister_code_install(&self, canister_id: Principal, wasm_module: Vec<u8>) -> Result<(), EgoError>;
    async fn canister_code_upgrade(&self, canister_id: Principal, wasm_module: Vec<u8>) -> Result<(), EgoError>;

    async fn canister_status_get(&self, canister_id: Principal) -> Result<CanisterStatusResponse, EgoError>;
    async fn canister_controller_add(&self, canister_id: Principal, user_id: Principal) -> Result<(), EgoError>;
    async fn canister_controller_remove(&self, canister_id: Principal, user_id: Principal) -> Result<(), EgoError>;

    // cycle relative methods
    async fn canister_cycle_top_up(&self, canister_id: Principal, cycles_to_use: Cycles) -> Result<(), EgoError>;
  }
}

#[test]
fn wallet_main_add(){
  set_up();

  let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();
  let result = EgoTenantService::wallet_main_add(wallet_id);
  assert!(result.is_ok());
}

#[test]
fn wallet_main_add_multi_time(){
  set_up();

  let wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
  let result = EgoTenantService::wallet_main_add(wallet_id);
  assert!(result.is_err());
  assert_eq!(4001, result.unwrap_err().code);
}

#[test]
fn wallet_main_remove(){
  set_up();

  let wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
  let result = EgoTenantService::wallet_main_remove(&wallet_id);
  assert!(result.is_ok());

  // after remove, it can be added again
  let result = EgoTenantService::wallet_main_add(wallet_id);
  assert!(result.is_ok());
}

#[test]
fn wallet_main_remove_not_exists(){
  set_up();

  let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();
  let result = EgoTenantService::wallet_main_remove(&wallet_id);
  assert!(result.is_err());
  assert_eq!(4002, result.unwrap_err().code);
}

#[test]
fn wallet_main_get_not_exists(){
  set_up();

  let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();
  let result = EgoTenantService::wallet_main_get(&wallet_id);
  assert!(result.is_err());
  assert_eq!(4002, result.unwrap_err().code);
}

#[test]
fn wallet_main_get(){
  set_up();

  let wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
  let result = EgoTenantService::wallet_main_get(&wallet_id);
  assert!(result.is_ok());
  assert_eq!(EXISTS_WALLET_ID, result.unwrap().wallet_id.to_string());
}


#[tokio::test]
async fn wallet_app_install() {
  let mut service = MockDump::new();
  service.expect_canister_main_create().returning(|_cycles_to_use| Ok(Principal::from_text("qvhpv-4qaaa-aaaaa-aaagq-cai").unwrap()));
  match EgoTenantService::wallet_app_install(service, "app_1").await {
    Ok(principal) => assert_eq!(principal.to_text(), "qvhpv-4qaaa-aaaaa-aaagq-cai".to_string()),
    Err(_e) => panic!("should not go here"),
  }
}

#[tokio::test]
async fn wallet_app_install_failed() {
  let mut service = MockDump::new();
  service.expect_canister_main_create().returning(|_cycles_to_use| Err(EgoError::from("error".to_string())));
  match EgoTenantService::wallet_app_install(service, "app_1").await {
    Ok(_principal) => panic!("should not go here"),
    Err(e) => assert_eq!("error".to_string(), e.msg),
  }
}

#[tokio::test]
async fn wallet_app_upgrade() {
  let mut service = MockDump::new();
  service.expect_canister_main_create().returning(|_cycles_to_use| Ok(Principal::from_text("qvhpv-4qaaa-aaaaa-aaagq-cai").unwrap()));
  match EgoTenantService::wallet_app_install(service, "app_1").await {
    Ok(principal) => assert_eq!(principal.to_text(), "qvhpv-4qaaa-aaaaa-aaagq-cai".to_string()),
    Err(_e) => panic!("should not go here"),
  }
}
