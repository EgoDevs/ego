use async_trait::async_trait;
use ego_lib::ic_management::controllers_update;
use candid::{Principal};

use ego_types::app::EgoError;
use ego_utils::ic_management::{canister_code_install, canister_code_reinstall, canister_code_upgrade, canister_cycle_top_up, canister_main_create, canister_main_delete, Cycles};

#[async_trait]
pub trait TIcManagement {
    async fn canister_main_create(&self, cycles_to_use: Cycles) -> Result<Principal, EgoError>;

    async fn canister_code_reinstall(
        &self,
        canister_id: Principal,
        wasm_module: Vec<u8>,
    ) -> Result<(), EgoError>;
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

    async fn canister_cycle_top_up(
        &self,
        canister_id: Principal,
        cycles_to_use: Cycles,
    ) -> Result<(), EgoError>;

    async fn canister_main_delete(&self, canister_id: Principal) -> Result<(), EgoError>;

    async fn controllers_update(
        &self,
        canister_id: Principal,
        controllers: Vec<Principal>,
    ) -> Result<(), EgoError>;
}

#[derive(Clone)]
pub struct IcManagement {}

impl IcManagement {
    pub fn new() -> Self {
        IcManagement {}
    }
}

#[async_trait]
impl TIcManagement for IcManagement {
    async fn canister_main_create(&self, cycles_to_use: Cycles) -> Result<Principal, EgoError> {
        canister_main_create(cycles_to_use).await
    }
    async fn canister_code_reinstall(
        &self,
        canister_id: Principal,
        wasm_module: Vec<u8>,
    ) -> Result<(), EgoError> {
        canister_code_reinstall(canister_id, wasm_module).await
    }

    async fn canister_code_install(
        &self,
        canister_id: Principal,
        wasm_module: Vec<u8>,
    ) -> Result<(), EgoError> {
        canister_code_install(canister_id, wasm_module).await
    }

    async fn canister_code_upgrade(
        &self,
        canister_id: Principal,
        wasm_module: Vec<u8>,
    ) -> Result<(), EgoError> {
        canister_code_upgrade(canister_id, wasm_module).await
    }

    async fn canister_cycle_top_up(
        &self,
        canister_id: Principal,
        cycles_to_use: Cycles,
    ) -> Result<(), EgoError> {
        canister_cycle_top_up(canister_id, cycles_to_use).await
    }

    async fn canister_main_delete(&self, canister_id: Principal) -> Result<(), EgoError> {
        canister_main_delete(canister_id).await
    }

    async fn controllers_update(
        &self,
        canister_id: Principal,
        controllers: Vec<Principal>,
    ) -> Result<(), EgoError> {
        controllers_update(canister_id, controllers).await
    }
}
