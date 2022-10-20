use ic_cdk::export::Principal;

use async_trait::async_trait;
use ic_cdk::api::management_canister::main::CanisterStatusResponse;
use ego_types::ego_error::EgoError;
use ego_utils::ic_management::{canister_code_install, canister_code_upgrade, canister_controller_set, canister_cycle_top_up, canister_main_create, canister_owner_set, canister_status_get, Cycles};

#[async_trait]
pub trait TIcManagement {
  async fn canister_main_create(&self, cycles_to_use: Cycles) -> Result<Principal, EgoError>;
  async fn canister_code_install(&self, canister_id: Principal, wasm_module: Vec<u8>) -> Result<(), EgoError>;
  async fn canister_code_upgrade(&self, canister_id: Principal, wasm_module: Vec<u8>) -> Result<(), EgoError>;

  async fn canister_controller_set(&self, canister_id: Principal, user_ids: Vec<Principal>) -> Result<(), EgoError>;
  async fn canister_owner_set(&self, canister_id: Principal, user_id: Principal) -> Result<(), EgoError>;

  async fn canister_status_get(&self, canister_id: Principal) -> Result<CanisterStatusResponse, EgoError>;
  async fn canister_cycle_top_up(&self, canister_id: Principal, cycles_to_use: Cycles) -> Result<(), EgoError>;
}

pub struct IcManagement {
}

impl IcManagement{
  pub fn new() -> Self {
    IcManagement{}
  }
}

#[async_trait]
impl TIcManagement for IcManagement {
  async fn canister_main_create(&self, cycles_to_use: Cycles) -> Result<Principal, EgoError>{
    canister_main_create(cycles_to_use).await
  }
  async fn canister_code_install(&self, canister_id: Principal, wasm_module: Vec<u8>) -> Result<(), EgoError>{
    canister_code_install(canister_id, wasm_module).await
  }
  async fn canister_code_upgrade(&self, canister_id: Principal, wasm_module: Vec<u8>) -> Result<(), EgoError>{
    canister_code_upgrade(canister_id, wasm_module).await
  }

  async fn canister_controller_set(&self, canister_id: Principal, user_ids: Vec<Principal>) -> Result<(), EgoError>{
    canister_controller_set(canister_id, user_ids).await
  }

  async fn canister_owner_set(&self, canister_id: Principal, user_id: Principal) -> Result<(), EgoError>{
    canister_owner_set(canister_id, vec![user_id]).await
  }

  async fn canister_status_get(&self, canister_id: Principal) -> Result<CanisterStatusResponse, EgoError> {
    canister_status_get(canister_id).await
  }

  async fn canister_cycle_top_up(&self, canister_id: Principal, cycles_to_use: Cycles) -> Result<(), EgoError> {
    canister_cycle_top_up(canister_id, cycles_to_use).await
  }
}