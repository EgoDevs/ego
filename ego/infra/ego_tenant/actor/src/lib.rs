use ic_cdk_macros::{init, update};
use candid::candid_method;
use ic_cdk::caller;
use ego_tenant_mod::service::TenantService;
use ego_tenant_mod::types::{WalletAppInstallRequest, WalletAppInstallResponse};
use ego_utils::management::IcpManagement;
use ego_utils::types::EgoError;

#[init]
#[candid_method(init, rename = "init")]
fn canister_init() {
    ic_cdk::println!("in ego_tenant init, caller is {}", caller());
}

#[update(name = "wallet_app_install")]
#[candid_method(update, rename = "wallet_app_install")]
async fn wallet_app_install(req: WalletAppInstallRequest) -> Result<WalletAppInstallResponse, EgoError> {
    let service = IcpManagement{};
    let canister_id = TenantService::wallet_app_install(service, &req.app_id).await?;
    Ok(WalletAppInstallResponse{canister_id})
}
