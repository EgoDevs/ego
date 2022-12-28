use ic_cdk::api;
use ic_cdk::api::management_canister::main::{CanisterSettings, CreateCanisterArgument};
use ic_cdk::export::candid::CandidType;
use ic_cdk::export::Principal;
use serde::Deserialize;
use tracing::error;

use ego_types::app::EgoError;

pub type Cycles = u128;

pub const INSTALL_CANISTER_CYCLES_FEE: Cycles = 1_000_000_000_000; // 1T cycles

#[derive(CandidType, Deserialize)]
struct CreateResult {
  canister_id: Principal,
}

#[derive(CandidType)]
struct DepositCyclesArgs {
  pub canister_id: Principal,
}

#[derive(CandidType, Deserialize)]
enum InstallMode {
  #[serde(rename = "install")]
  Install,
  #[serde(rename = "reinstall")]
  Reinstall,
  #[serde(rename = "upgrade")]
  Upgrade,
}

#[derive(CandidType, Deserialize)]
struct CanisterInstall {
  mode: InstallMode,
  canister_id: Principal,
  #[serde(with = "serde_bytes")]
  wasm_module: Vec<u8>,
  #[serde(with = "serde_bytes")]
  arg: Vec<u8>,
}

async fn code_install(
  canister_id: Principal,
  mode: InstallMode,
  wasm_module: Vec<u8>,
) -> Result<(), EgoError> {
  let install_config = CanisterInstall {
    mode,
    canister_id,
    wasm_module,
    arg: b" ".to_vec(),
  };

  let (_, ): ((), ) = match api::call::call_with_payment128(
    Principal::management_canister(),
    "install_code",
    (install_config, ),
    INSTALL_CANISTER_CYCLES_FEE,
  )
    .await
  {
    Ok(x) => x,
    Err((code, msg)) => {
      let code = code as u16;
      error!(
                error_code = code,
                error_message = msg.as_str(),
                "Error calling install_code"
            );
      return Err(EgoError { code, msg });
    }
  };

  Ok(())
}

pub async fn canister_main_create(cycles_to_use: Cycles) -> Result<Principal, EgoError> {
  let in_arg = CreateCanisterArgument {
    settings: Some(CanisterSettings {
      controllers: Some(vec![ic_cdk::id()]),
      compute_allocation: None,
      memory_allocation: None,
      freezing_threshold: None,
    }),
  };

  let (create_result, ): (CreateResult, ) = match api::call::call_with_payment(
    Principal::management_canister(),
    "create_canister",
    (in_arg, ),
    cycles_to_use.try_into().unwrap(),
  )
    .await
  {
    Ok(x) => x,
    Err((code, msg)) => {
      let code = code as u16;
      error!(
                error_code = code,
                error_message = msg.as_str(),
                "Error calling canister_main_create"
            );
      return Err(EgoError { code, msg });
    }
  };

  Ok(create_result.canister_id)
}

pub async fn canister_code_install(
  canister_id: Principal,
  wasm_module: Vec<u8>,
) -> Result<(), EgoError> {
  code_install(canister_id, InstallMode::Install, wasm_module).await
}

pub async fn canister_code_upgrade(
  canister_id: Principal,
  wasm_module: Vec<u8>,
) -> Result<(), EgoError> {
  code_install(canister_id, InstallMode::Upgrade, wasm_module).await
}


pub fn canister_cycle_top_up(
  canister_id: Principal,
  cycles_to_use: Cycles,
) {
  let _result = api::call::notify_with_payment128(
    Principal::management_canister(),
    "deposit_cycles",
    (DepositCyclesArgs { canister_id }, ),
    cycles_to_use.try_into().unwrap(),
  );
}
