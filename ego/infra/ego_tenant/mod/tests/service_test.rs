use async_trait::async_trait;
use ic_cdk::export::Principal;
use mockall::mock;

use ego_tenant_mod::c2c::ego_file::TEgoFile;
use ego_tenant_mod::c2c::ic_management::TIcManagement;
use ego_tenant_mod::service::EgoTenantService;
use ego_types::app::{Wasm, WasmId};
use ego_types::app::CanisterType::{BACKEND};
use ego_types::ego_error::EgoError;
use ego_types::version::Version;
use ego_utils::ic_management::Cycles;

static FILE_CANISTER_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";

static EXISTS_WALLET_ID: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";
static EXISTS_CANISTER_ID: &str = "223xb-saaaa-aaaaf-arlqa-cai";

static EXISTS_APP_ID: &str = "app_test";


pub fn set_up() {
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

#[tokio::test]
async fn app_main_install() {
  set_up();

  let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();
  let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();

  let mut mock_management = MockManagement::new();
  let mut mock_ego_file = MockEgoFile::new();

  let version = Version{
    major: 1,
    minor: 0,
    patch: 0
  };
  let backend = Wasm::new(EXISTS_APP_ID.to_string(), version, BACKEND, Some(file_canister));

  let created_canister_id = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();
  let fake_wasm_module = vec![1, 0, 1, 0, 0, 1, 0, 1];

  mock_management.expect_canister_main_create().returning(move |_cycles_to_use| Ok(created_canister_id.clone()));
  mock_ego_file.expect_file_main_read().returning(move |_canister_id, _fid| Ok(fake_wasm_module.clone()));

  mock_management.expect_canister_code_install().returning(move |canister_id, _wasm_module| {
    assert_eq!(&canister_id, &created_canister_id);
    Ok(())
  });
  mock_management.expect_canister_controller_set().returning(|_canister_id, _user_ids| Ok(()));

  match EgoTenantService::app_main_install(wallet_principal, mock_ego_file, mock_management, backend).await {
    Ok(principal) => {
      assert_eq!(principal, created_canister_id);
    }
    Err(_e) => {
      panic!("should not go here");
    }
  }
}

#[tokio::test]
async fn app_main_install_failed() {
  set_up();

  let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();
  let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();

  let mut mock_management = MockManagement::new();
  let mock_ego_file = MockEgoFile::new();

  let version = Version{
    major: 1,
    minor: 0,
    patch: 0
  };
  let backend = Wasm::new(EXISTS_APP_ID.to_string(), version, BACKEND, Some(file_canister));

  mock_management.expect_canister_main_create().returning(move |_cycles_to_use| Err(EgoError::from("error".to_string())));

  match EgoTenantService::app_main_install(wallet_principal, mock_ego_file, mock_management, backend).await {
    Ok(_principal) => panic!("should not go here"),
    Err(e) => {
      assert_eq!(255, e.code)
    }
  }
}

#[tokio::test]
async fn app_main_upgrade() {
  set_up();

  let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();
  let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();

  let mut mock_management = MockManagement::new();
  let mut mock_ego_file = MockEgoFile::new();


  let version = Version{
    major: 1,
    minor: 0,
    patch: 0
  };
  let backend = Wasm::new(EXISTS_APP_ID.to_string(), version, BACKEND, Some(file_canister));

  let exists_canister_id = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();
  let fake_wasm_module = vec![1, 0, 1, 0, 0, 1, 0, 1];

  mock_ego_file.expect_file_main_read().returning(move |_canister_id, _fid| Ok(fake_wasm_module.clone()));

  mock_management.expect_canister_code_upgrade().returning(move |canister_id, _wasm_module| {
    assert_eq!(&canister_id, &exists_canister_id);
    Ok(())
  });
  mock_management.expect_canister_controller_set().returning(|_canister_id, _user_ids| Ok(()));

  match EgoTenantService::app_main_upgrade(wallet_principal, exists_canister_id, mock_ego_file, mock_management, backend).await {
    Ok(ret) => {
      assert!(ret);
    }
    Err(e) => {
      println!("{:?}", e);
      panic!("should not go here");
    }
  }
}
