use ic_cdk::api;
use ic_cdk::api::management_canister::main::{
    CanisterIdRecord, CanisterSettings, CanisterStatusResponse, CreateCanisterArgument,
    UpdateSettingsArgument,
};
use ic_cdk::export::candid::CandidType;
use ic_cdk::export::Principal;

use serde::Deserialize;
use tracing::error;

use crate::consts::INSTALL_CANISTER_CYCLES_FEE;
use ego_types::ego_error::EgoError;

pub type Cycles = u128;

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

    let (_,): ((),) = match api::call::call_with_payment128(
        Principal::management_canister(),
        "install_code",
        (install_config,),
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

async fn controllers_update(
    canister_id: Principal,
    controllers: Vec<Principal>,
) -> Result<(), EgoError> {
    let in_arg = UpdateSettingsArgument {
        canister_id,
        settings: CanisterSettings {
            controllers: Some(controllers),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
        },
    };

    let (_,): ((),) = match api::call::call(
        Principal::management_canister(),
        "update_settings",
        (in_arg,),
    )
    .await
    {
        Ok(x) => x,
        Err((code, msg)) => {
            let code = code as u16;
            error!(
                error_code = code,
                error_message = msg.as_str(),
                "Error calling controllers_update"
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

    let (create_result,): (CreateResult,) = match api::call::call_with_payment(
        Principal::management_canister(),
        "create_canister",
        (in_arg,),
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

pub async fn canister_status_get(
    canister_id: Principal,
) -> Result<CanisterStatusResponse, EgoError> {
    ic_cdk::println!("canister_status_get");

    let req = CanisterIdRecord { canister_id };

    let (status_result,): (CanisterStatusResponse,) =
        match api::call::call(Principal::management_canister(), "canister_status", (req,)).await {
            Ok(x) => {
                ic_cdk::println!("canister_status_get success {:?}", x);
                x
            }
            Err((code, msg)) => {
                let code = code as u16;
                error!(
                    error_code = code,
                    error_message = msg.as_str(),
                    "Error calling canister_status_get"
                );
                return Err(EgoError { code, msg });
            }
        };

    Ok(status_result)
}

pub async fn canister_controller_add(
    canister_id: Principal,
    user_id: Principal,
) -> Result<(), EgoError> {
    let status_resp = canister_status_get(canister_id).await?;

    let mut current_controllers = status_resp.settings.controllers;

    if !current_controllers.contains(&user_id) {
        current_controllers.push(user_id);
        controllers_update(canister_id, current_controllers).await
    } else {
        Ok(())
    }
}

pub async fn canister_controller_remove(
    canister_id: Principal,
    user_id: Principal,
) -> Result<(), EgoError> {
    let status_resp = canister_status_get(canister_id).await?;

    let mut current_controllers = status_resp.settings.controllers;

    if current_controllers.contains(&user_id) {
        current_controllers.retain(|p| *p != user_id);
        controllers_update(canister_id, current_controllers).await
    } else {
        Ok(())
    }
}

pub async fn canister_controller_set(
    canister_id: Principal,
    user_ids: Vec<Principal>,
) -> Result<(), EgoError> {
    controllers_update(canister_id, user_ids).await
}

pub async fn canister_cycle_top_up(
    canister_id: Principal,
    cycles_to_use: Cycles,
) -> Result<(), EgoError> {
    match api::call::call_with_payment128(
        Principal::management_canister(),
        "deposit_cycles",
        (DepositCyclesArgs { canister_id },),
        cycles_to_use.try_into().unwrap(),
    )
    .await
    {
        Ok(x) => x,
        Err((code, msg)) => {
            api::call::msg_cycles_refunded128();
            let code = code as u16;
            error!(
                error_code = code,
                error_message = msg.as_str(),
                "Error calling canister_cycle_top_up"
            );
            return Err(EgoError { code, msg });
        }
    };

    Ok(())
}

pub async fn canister_owner_set(
    canister_id: Principal,
    user_ids: Vec<Principal>,
) -> Result<(), EgoError> {
    match api::call::call(canister_id, "role_owner_set", (user_ids,)).await {
        Ok(x) => x,
        Err((code, msg)) => {
            let code = code as u16;
            error!(
                error_code = code,
                error_message = msg.as_str(),
                "Error calling role_owner_set"
            );
            return Err(EgoError { code, msg });
        }
    };

    Ok(())
}
