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

static EXISTS_WALLET_ID: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";
static EXISTS_CANISTER_ID: &str = "223xb-saaaa-aaaaf-arlqa-cai";


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

    async fn canister_owner_set(&self, canister_id: Principal, user_id: Principal) -> Result<(), EgoError>;
  }
}

#[test]
fn canister_main_track() {
  set_up();

  let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();
  let canister_principal = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();

  let ret = EgoTenantService::canister_main_track(wallet_principal, canister_principal);
  assert!(ret)
}