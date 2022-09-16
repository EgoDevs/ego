use ic_cdk_macros::{init, update};
use candid::candid_method;
use ic_cdk::caller;
use ego_tenant_mod::service::EgoTenantService;
use ego_tenant_mod::types::{WalletAppInstallRequest, WalletAppInstallResponse, WalletMainAddRequest, WalletMainAddResponse};
use ego_utils::management::ICPManagement;
use ego_utils::types::EgoError;

#[init]
#[candid_method(init, rename = "init")]
fn canister_init() {
    ic_cdk::println!("ego_tenant: init, caller is {}", caller());
}

/********************  ego-store methods  ********************/
#[update(name = "wallet_main_add")]
#[candid_method(update, rename = "wallet_main_add")]
fn wallet_main_add(req: WalletMainAddRequest) -> Result<WalletMainAddResponse, EgoError> {
    ic_cdk::println!("ego_tenant: wallet_app_install");

    match EgoTenantService::wallet_main_add(req.wallet_id) {
        Ok(ret) => {Ok(WalletMainAddResponse{ret})},
        Err(e) => {Err(e)}
    }
}


// #[update(name = "wallet_app_install")]
// #[candid_method(update, rename = "wallet_app_install")]
// async fn wallet_app_install(req: WalletAppInstallRequest) -> Result<WalletAppInstallResponse, EgoError> {
//     ic_cdk::println!("ego_tenant: wallet_app_install");
//     let service = ICPManagement {};
//     let canister_id = EgoTenantService::wallet_app_install(service, &req.app_id).await?;
//     Ok(WalletAppInstallResponse{canister_id})
// }
