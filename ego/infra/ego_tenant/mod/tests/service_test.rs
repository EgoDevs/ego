use async_trait::async_trait;
use ic_types::Principal;
use mockall::mock;
use ego_store_mod::app::App;

use ego_tenant_mod::service::EgoTenantService;
use ego_utils::types::{Category, Cycles, EgoError, Version, Wasm, WasmId};
use ego_tenant_mod::state::EGO_TENANT;
use ego_tenant_mod::wallet::Wallet;
use ego_tenant_mod::c2c::ic_management::TIcManagement;
use ego_tenant_mod::c2c::ego_file::TEgoFile;
use ego_utils::types::CanisterType::{ASSET, BACKEND};

static FILE_CANISTER_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";

static EXISTS_WALLET_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";

static TEST_WALLET_ID: &str = "227wz-liaaa-aaaaa-qaara-cai";

static EXISTS_APP_ID: &str = "app_test";
static EXISTS_APP_NAME: &str = "test app";
static EXISTS_APP_LOGO: &str = "logo";
static EXISTS_APP_DESCRIPTION: &str = "test is app description";

pub fn set_up() {
  let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();

  EGO_TENANT.with(|ego_tenant| {
    // add tenant
    ego_tenant.borrow_mut().wallets.insert(wallet_principal, Wallet::new(wallet_principal));
  });
}

mock! {
  Management {}

  #[async_trait]
  impl TIcManagement for Management {
    async fn canister_main_create(&self, cycles_to_use: Cycles) -> Result<Principal, EgoError>;
    async fn canister_code_install(&self, canister_id: Principal, wasm_module: Vec<u8>) -> Result<(), EgoError>;
    async fn canister_code_upgrade(&self, canister_id: Principal, wasm_module: Vec<u8>) -> Result<(), EgoError>;

    async fn canister_controller_set(&self, canister_id: Principal, user_ids: Vec<Principal>) -> Result<(), EgoError>;
  }

}

mock! {
  EgoFile {}

  #[async_trait]
  impl TEgoFile for EgoFile {
    async fn file_main_read(&self, canister_id: Principal, fid: WasmId) -> Result<Vec<u8>, EgoError>;
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
  set_up();

  let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();
  let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();

  let mut mock_management = MockManagement::new();
  let mut mock_ego_file = MockEgoFile::new();

  let version = Version::new(1, 0, 1);
  let wasms = vec![Wasm::new(EXISTS_APP_ID.to_string(), version, ASSET, None), Wasm::new(EXISTS_APP_ID.to_string(), version, BACKEND, Some(file_canister))];
  let app = App::new(EXISTS_APP_ID.to_string(), EXISTS_APP_NAME.to_string(), Category::Vault, EXISTS_APP_LOGO.to_string(), EXISTS_APP_DESCRIPTION.to_string(), version, wasms, 1.2f32);

  let created_canister_id = Principal::from_text("qvhpv-4qaaa-aaaaa-aaagq-cai").unwrap();
  let fake_wasm_module = vec![1,0,1,0,0,1,0,1];

  mock_management.expect_canister_main_create().returning(move |_cycles_to_use| Ok(created_canister_id.clone()));
  mock_ego_file.expect_file_main_read().returning(move |_canister_id, _fid| Ok(fake_wasm_module.clone()));

  mock_management.expect_canister_code_install().returning(move |canister_id, _wasm_module| {
    assert_eq!(&canister_id, &created_canister_id);
    Ok(())
  });
  mock_management.expect_canister_controller_set().returning(|_canister_id, _user_ids| Ok(()));

  match EgoTenantService::wallet_app_install(wallet_principal, mock_ego_file, mock_management, app).await {
    Ok(principals) => {
      let key = "app_test|BACKEND";
      match principals.get(key) {
        None => panic!("should not go here"),
        Some(principal) => {
          assert_eq!(*principal, created_canister_id);
        }
      }
    },
    Err(_e) => {
      panic!("should not go here");
    }
  }
}

#[tokio::test]
async fn wallet_app_install_failed_with_not_exists_wallet() {
  set_up();

  let wallet_principal = Principal::from_text(TEST_WALLET_ID.to_string()).unwrap();
  let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();

  let mock_management = MockManagement::new();
  let mock_ego_file = MockEgoFile::new();

  let version = Version::new(1, 0, 1);
  let wasms = vec![Wasm::new(EXISTS_APP_ID.to_string(), version, ASSET, None), Wasm::new(EXISTS_APP_ID.to_string(), version, BACKEND, Some(file_canister))];
  let app = App::new(EXISTS_APP_ID.to_string(), EXISTS_APP_NAME.to_string(), Category::Vault, EXISTS_APP_LOGO.to_string(), EXISTS_APP_DESCRIPTION.to_string(), version, wasms, 1.2f32);

  match EgoTenantService::wallet_app_install(wallet_principal, mock_ego_file, mock_management, app).await {
    Ok(_principal) => panic!("should not go here"),
    Err(e) => assert_eq!(4002, e.code),
  }
}

#[tokio::test]
async fn wallet_app_install_failed() {
  set_up();

  let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();
  let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();

  let mut mock_management = MockManagement::new();
  let mock_ego_file = MockEgoFile::new();

  let version = Version::new(1, 0, 1);
  let wasms = vec![Wasm::new(EXISTS_APP_ID.to_string(), version, ASSET, None), Wasm::new(EXISTS_APP_ID.to_string(), version, BACKEND, Some(file_canister))];
  let app = App::new(EXISTS_APP_ID.to_string(), EXISTS_APP_NAME.to_string(), Category::Vault, EXISTS_APP_LOGO.to_string(), EXISTS_APP_DESCRIPTION.to_string(), version, wasms, 1.2f32);

  mock_management.expect_canister_main_create().returning(move |_cycles_to_use| Err(EgoError::from("error".to_string())));

  match EgoTenantService::wallet_app_install(wallet_principal, mock_ego_file, mock_management, app).await {
    Ok(_principal) => panic!("should not go here"),
    Err(e) => {
      println!("{:?}", e);
      assert_eq!(255, e.code)},
  }
}

// #[tokio::test]
// async fn wallet_app_upgrade() {
//   let mock_management = MockManagement::new();
//   let mock_ego_file = MockEgoFile::new();
//
//   mock_management.expect_canister_main_create().returning(|_cycles_to_use| Ok(Principal::from_text("qvhpv-4qaaa-aaaaa-aaagq-cai").unwrap()));
//   mock_ego_file.expect_canister_main_create().returning(|_cycles_to_use| Ok(Principal::from_text("qvhpv-4qaaa-aaaaa-aaagq-cai").unwrap()));
//   match EgoTenantService::wallet_app_install(service, "app_1").await {
//     Ok(principal) => assert_eq!(principal.to_text(), "qvhpv-4qaaa-aaaaa-aaagq-cai".to_string()),
//     Err(_e) => panic!("should not go here"),
//   }
// }
