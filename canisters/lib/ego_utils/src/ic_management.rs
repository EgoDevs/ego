use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::main::{CanisterInstallMode, CanisterSettings, create_canister, CreateCanisterArgument, delete_canister, deposit_cycles, install_code, InstallCodeArgument, stop_canister};
use ic_cdk::api::management_canister::provisional::CanisterIdRecord;

use ego_types::app::EgoError;

pub type Cycles = u128;

#[derive(CandidType)]
struct DepositCyclesArgs {
  pub canister_id: Principal,
}

async fn code_install(
  canister_id: Principal,
  mode: CanisterInstallMode,
  wasm_module: Vec<u8>,
) -> Result<(), EgoError> {
  let install_config = InstallCodeArgument {
    mode,
    canister_id,
    wasm_module,
    arg: b" ".to_vec(),
  };

  match install_code(install_config).await {
    Ok(_) => Ok(()),
    Err((code, msg)) => {
      let code = code as u16;
      Err(EgoError { code, msg })
    }
  }
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

  match create_canister(in_arg, cycles_to_use).await {
    Ok(resp) => {
      let canister_id_record = resp.0;
      Ok(canister_id_record.canister_id)
    }
    Err((code, msg)) => {
      let code = code as u16;
      Err(EgoError { code, msg })
    }
  }
}

pub async fn canister_main_delete(canister_id: Principal) -> Result<(), EgoError> {
  // stop the canister
  let _stop_result = match stop_canister(CanisterIdRecord { canister_id }).await {
    Ok(_) => Ok(()),
    Err((code, msg)) => {
      let code = code as u16;
      Err(EgoError { code, msg })
    }
  }?;

  let _delete_result = match delete_canister(CanisterIdRecord { canister_id }).await {
    Ok(_) => Ok(()),
    Err((code, msg)) => {
      let code = code as u16;
      Err(EgoError { code, msg })
    }
  }?;

  Ok(())
}

pub async fn canister_code_reinstall(
  canister_id: Principal,
  wasm_module: Vec<u8>,
) -> Result<(), EgoError> {
  code_install(canister_id, CanisterInstallMode::Reinstall, wasm_module).await
}

pub async fn canister_code_install(
  canister_id: Principal,
  wasm_module: Vec<u8>,
) -> Result<(), EgoError> {
  code_install(canister_id, CanisterInstallMode::Install, wasm_module).await
}

pub async fn canister_code_upgrade(
  canister_id: Principal,
  wasm_module: Vec<u8>,
) -> Result<(), EgoError> {
  code_install(canister_id, CanisterInstallMode::Upgrade, wasm_module).await
}

pub async fn canister_cycle_top_up(
  canister_id: Principal,
  cycles_to_use: Cycles,
) -> Result<(), EgoError> {
  match deposit_cycles(CanisterIdRecord { canister_id }, cycles_to_use).await {
    Ok(_) => Ok(()),
    Err((code, msg)) => {
      let code = code as u16;
      Err(EgoError { code, msg })
    }
  }
}
