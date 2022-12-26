use async_trait::async_trait;
use ego_lib::ego_canister::TEgoCanister;
use ic_cdk::export::Principal;
use mockall::mock;

use ego_tenant_mod::c2c::ego_file::TEgoFile;
use ego_tenant_mod::c2c::ic_management::TIcManagement;
use ego_tenant_mod::service::EgoTenantService;
use ego_types::app::CanisterType::BACKEND;
use ego_types::app::{Wasm, WasmId};
use ego_types::ego_error::EgoError;
use ego_types::version::Version;
use ego_utils::ic_management::Cycles;

// static STORE_CANISTER_ID: &str = "qhbym-qaaaa-aaaaa-aaafq-cai";
static TENANT_CANISTER_ID: &str = "rdmx6-jaaaa-aaaaa-aaadq-cai";
static FILE_CANISTER_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";

static EXISTS_WALLET_ID: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";
static EXISTS_USER_ID: &str = "225da-yaaaa-aaaah-qahrq-cai";
static TEST_USER_ID: &str = "hjpa3-qyaaa-aaaan-qagva-cai";
static EXISTS_CANISTER_ID: &str = "223xb-saaaa-aaaaf-arlqa-cai";
static TEST_WALLET_ID: &str = "wtb37-uyaaa-aaaai-qa3zq-cai";
static EXISTS_APP_ID: &str = "app_test";

pub fn set_up() {}

mock! {
  Management {}

  #[async_trait]
  impl TIcManagement for Management {
    async fn canister_main_create(&self, cycles_to_use: Cycles) -> Result<Principal, EgoError>;

    async fn canister_code_install(
        &self,
        canister_id: Principal,
        wasm_module: Vec<u8>,
    ) -> Result<(), EgoError>;

    async fn canister_code_upgrade(
        &self,
        canister_id: Principal,
        wasm_module: Vec<u8>,
    ) -> Result<(), EgoError>;

    fn canister_cycle_top_up(
        &self,
        canister_id: Principal,
        cycles_to_use: Cycles,
    );

    async fn canister_controller_set(
      &self,
      canister_id: Principal,
      principals: Vec<Principal>,
    ) -> Result<(), EgoError>;
  }
}

mock! {
  EgoFile {}

  #[async_trait]
  impl TEgoFile for EgoFile {
    async fn file_main_read(&self, canister_id: Principal, fid: WasmId) -> Result<Vec<u8>, EgoError>;
  }
}

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
  }
}

#[tokio::test]
async fn app_main_install() {
    set_up();

    // let store_canister_id = Principal::from_text(STORE_CANISTER_ID.to_string()).unwrap();
    let tenant_canister_id = Principal::from_text(TENANT_CANISTER_ID.to_string()).unwrap();
    let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();
    let user_principal = Principal::from_text(EXISTS_USER_ID.to_string()).unwrap();
    let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();

    let mut mock_management = MockManagement::new();
    let mut mock_ego_file = MockEgoFile::new();
    let mut mock_ego_canister = MockCanister::new();

    let version = Version {
        major: 1,
        minor: 0,
        patch: 0,
    };
    let backend = Wasm::new(EXISTS_APP_ID.to_string(), version, BACKEND, file_canister);

    let created_canister_id = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();
    let fake_wasm_module = vec![1, 0, 1, 0, 0, 1, 0, 1];

    mock_management
        .expect_canister_main_create()
        .returning(move |_cycles_to_use| Ok(created_canister_id.clone()));
    mock_ego_file
        .expect_file_main_read()
        .returning(move |_canister_id, _fid| Ok(fake_wasm_module.clone()));

    mock_management
        .expect_canister_code_install()
        .returning(move |canister_id, _wasm_module| {
            assert_eq!(&canister_id, &created_canister_id);
            Ok(())
        });
    mock_management
        .expect_canister_controller_set()
        .returning(|_, _| Ok(()));

    mock_management
        .expect_canister_controller_set()
        .returning(move |_canister_id, _principal| Ok(()));

    mock_ego_canister
        .expect_ego_controller_set()
        .returning(move |canister_id, user_ids| {
            assert_eq!(created_canister_id, canister_id);
            assert_eq!(wallet_principal, *user_ids.get(0).unwrap());
            assert_eq!(user_principal, *user_ids.get(1).unwrap());
            ()
        });
    mock_ego_canister
        .expect_ego_owner_set()
        .returning(move |canister_id, user_ids| {
            assert_eq!(created_canister_id, canister_id);
            assert_eq!(wallet_principal, *user_ids.get(0).unwrap());
            assert_eq!(user_principal, *user_ids.get(1).unwrap());
            ()
        });

    mock_ego_canister
        .expect_ego_op_add()
        .returning(|_target_canister_id, _principal| ());

    mock_ego_canister
        .expect_ego_canister_add()
        .returning(|_, _, _| ());

    match EgoTenantService::app_main_install(
        tenant_canister_id,
        mock_ego_file,
        mock_management,
        mock_ego_canister,
        wallet_principal,
        user_principal,
        backend,
    )
    .await
    {
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

    // let store_canister_id = Principal::from_text(STORE_CANISTER_ID.to_string()).unwrap();
    let tenant_canister_id = Principal::from_text(TENANT_CANISTER_ID.to_string()).unwrap();
    let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();
    let user_principal = Principal::from_text(EXISTS_USER_ID.to_string()).unwrap();
    let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();

    let mut mock_management = MockManagement::new();
    let mock_ego_file = MockEgoFile::new();
    let mock_ego_canister = MockCanister::new();

    let version = Version {
        major: 1,
        minor: 0,
        patch: 0,
    };
    let backend = Wasm::new(EXISTS_APP_ID.to_string(), version, BACKEND, file_canister);

    mock_management
        .expect_canister_main_create()
        .returning(move |_cycles_to_use| Err(EgoError::from("error".to_string())));

    match EgoTenantService::app_main_install(
        tenant_canister_id,
        mock_ego_file,
        mock_management,
        mock_ego_canister,
        wallet_principal,
        user_principal,
        backend, // backend,
    )
    .await
    {
        Ok(_principal) => panic!("should not go here"),
        Err(e) => {
            assert_eq!(255, e.code)
        }
    }
}

#[tokio::test]
async fn app_main_install_canister_code_install_fail() {
    set_up();

    // let store_canister_id = Principal::from_text(STORE_CANISTER_ID.to_string()).unwrap();
    let tenant_canister_id = Principal::from_text(TENANT_CANISTER_ID.to_string()).unwrap();
    let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();
    let user_principal = Principal::from_text(EXISTS_USER_ID.to_string()).unwrap();
    let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();

    let mut mock_management = MockManagement::new();
    let mut mock_ego_file = MockEgoFile::new();
    let mut mock_ego_canister = MockCanister::new();

    let version = Version {
        major: 1,
        minor: 0,
        patch: 0,
    };
    let backend = Wasm::new(EXISTS_APP_ID.to_string(), version, BACKEND, file_canister);

    let created_canister_id = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();
    let fake_wasm_module = vec![1, 0, 1, 0, 0, 1, 0, 1];

    mock_management
        .expect_canister_main_create()
        .returning(move |_cycles_to_use| Ok(created_canister_id.clone()));
    mock_ego_file
        .expect_file_main_read()
        .returning(move |_canister_id, _fid| Ok(fake_wasm_module.clone()));

    mock_management
        .expect_canister_code_install()
        .returning(move |_canister_id, _wasm_module| {
            Err(EgoError::from("canister code install error".to_string()))
        });

    mock_ego_canister
        .expect_ego_controller_set()
        .returning(move |canister_id, user_ids| {
            assert_eq!(created_canister_id, canister_id);
            assert_eq!(wallet_principal, *user_ids.get(0).unwrap());
            assert_eq!(user_principal, *user_ids.get(1).unwrap());
            ()
        });
    mock_ego_canister
        .expect_ego_owner_set()
        .returning(move |canister_id, user_ids| {
            assert_eq!(created_canister_id, canister_id);
            assert_eq!(wallet_principal, *user_ids.get(0).unwrap());
            assert_eq!(user_principal, *user_ids.get(1).unwrap());
            ()
        });

    match EgoTenantService::app_main_install(
        tenant_canister_id,
        mock_ego_file,
        mock_management,
        mock_ego_canister,
        wallet_principal,
        user_principal,
        backend, // backend,
    )
    .await
    {
        Ok(_principal) => panic!("should not go here"),

        Err(_e) => {
            assert_eq!(255, _e.code);
        }
    }
}

#[tokio::test]
async fn app_main_install_ego_faile_fail() {
    set_up();

    // let store_canister_id = Principal::from_text(STORE_CANISTER_ID.to_string()).unwrap();
    let tenant_canister_id = Principal::from_text(TENANT_CANISTER_ID.to_string()).unwrap();
    let wallet_principal = Principal::from_text(TEST_WALLET_ID.to_string()).unwrap();
    let user_principal = Principal::from_text(TEST_USER_ID.to_string()).unwrap();
    let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();

    let mut mock_management = MockManagement::new();
    let mut mock_ego_file = MockEgoFile::new();
    let mut mock_ego_canister = MockCanister::new();

    let version = Version {
        major: 1,
        minor: 0,
        patch: 0,
    };

    let created_canister_id = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();
    let backend = Wasm::new(EXISTS_APP_ID.to_string(), version, BACKEND, file_canister);
    mock_management
        .expect_canister_main_create()
        .returning(move |_cycles_to_use| Ok(created_canister_id.clone()));
    mock_ego_file
        .expect_file_main_read()
        .returning(move |_canister_id, _fid| Err(EgoError::from("error".to_string())));

    mock_management
        .expect_canister_code_install()
        .returning(move |canister_id, _wasm_module| {
            assert_eq!(&canister_id, &created_canister_id);
            Ok(())
        });

    mock_ego_canister
        .expect_ego_controller_set()
        .returning(move |canister_id, user_ids| {
            assert_eq!(created_canister_id, canister_id);
            assert_eq!(wallet_principal, *user_ids.get(0).unwrap());
            ()
        });
    mock_ego_canister
        .expect_ego_owner_set()
        .returning(move |canister_id, user_ids| {
            assert_eq!(created_canister_id, canister_id);
            assert_eq!(user_principal, *user_ids.get(1).unwrap());
            ()
        });

    match EgoTenantService::app_main_install(
        tenant_canister_id,
        mock_ego_file,
        mock_management,
        mock_ego_canister,
        wallet_principal,
        user_principal,
        backend, // backend,
    )
    .await
    {
        Ok(_principal) => panic!("should not go here"),
        Err(e) => {
            assert_eq!(255, e.code)
        }
    }
}

#[tokio::test]
async fn app_main_upgrade() {
    set_up();

    let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();

    let mut mock_management = MockManagement::new();
    let mut mock_ego_file = MockEgoFile::new();
    let mut mock_ego_canister = MockCanister::new();

    let version = Version {
        major: 1,
        minor: 0,
        patch: 0,
    };
    let backend = Wasm::new(EXISTS_APP_ID.to_string(), version, BACKEND, file_canister);

    let exists_canister_id = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();
    let fake_wasm_module = vec![1, 0, 1, 0, 0, 1, 0, 1];

    mock_ego_file
        .expect_file_main_read()
        .returning(move |_canister_id, _fid| Ok(fake_wasm_module.clone()));

    mock_management
        .expect_canister_code_upgrade()
        .returning(move |canister_id, _wasm_module| {
            assert_eq!(&canister_id, &exists_canister_id);
            Ok(())
        });

    mock_ego_canister
        .expect_ego_controller_set()
        .returning(|_canister_id, _user_ids| ());

    match EgoTenantService::app_main_upgrade(
        mock_ego_file,
        mock_management,
        exists_canister_id,
        backend,
    )
    .await
    {
        Ok(ret) => {
            assert!(ret);
        }
        Err(e) => {
            println!("{:?}", e);
            panic!("should not go here");
        }
    }
}

#[tokio::test]
async fn app_main_upgrade_ego_file_failed() {
    set_up();
    let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();

    let mut mock_management = MockManagement::new();
    let mut mock_ego_file = MockEgoFile::new();

    let version = Version {
        major: 1,
        minor: 0,
        patch: 0,
    };
    let backend = Wasm::new(EXISTS_APP_ID.to_string(), version, BACKEND, file_canister);
    let exists_canister_id = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();

    mock_ego_file
        .expect_file_main_read()
        .returning(move |_canister_id, _fid| Err(EgoError::from("ego file error".to_string())));
    mock_management
        .expect_canister_code_upgrade()
        .returning(move |canister_id, _wasm_module| {
            assert_eq!(&canister_id, &exists_canister_id);
            Ok(())
        });

    match EgoTenantService::app_main_upgrade(
        mock_ego_file,
        mock_management,
        exists_canister_id,
        backend,
    )
    .await
    {
        Ok(ret) => {
            panic!("should not go here: {:?}", ret);
        }
        Err(e) => {
            println!("{:?}", e);
            assert_eq!(255, e.code)
        }
    }
}

#[tokio::test]
async fn app_main_upgrade_ego_management_failed() {
    set_up();
    let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();
    let mut mock_management = MockManagement::new();
    let mut mock_ego_file = MockEgoFile::new();
    let version = Version {
        major: 1,
        minor: 0,
        patch: 0,
    };
    let backend = Wasm::new(EXISTS_APP_ID.to_string(), version, BACKEND, file_canister);
    let fake_wasm_module = vec![1, 0, 1, 0, 0, 1, 0, 1];
    let exist_canister_id = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();
    mock_ego_file
        .expect_file_main_read()
        .returning(move |_canister_id, _fid| Ok(fake_wasm_module.clone()));
    mock_management
        .expect_canister_code_upgrade()
        .returning(move |_canister_id, _wasm_module| {
            Err(EgoError::from("management error".to_string()))
        });

    match EgoTenantService::app_main_upgrade(
        mock_ego_file,
        mock_management,
        exist_canister_id,
        backend,
    )
    .await
    {
        Ok(ret) => {
            panic!("should not go here: {:?}", ret);
        }
        Err(e) => {
            println!("{:?}", e);
            assert_eq!(255, e.code);
        }
    }
}
