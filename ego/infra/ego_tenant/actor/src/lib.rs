use ic_cdk_macros::{init, update};
use candid::candid_method;
use ic_cdk::caller;
use ego_tenant_mod::c2c::ego_file::EgoFile;
use ego_tenant_mod::c2c::ic_management::IcManagement;
use ego_tenant_mod::service::EgoTenantService;
use ego_tenant_mod::types::{AppMainInstallRequest, AppMainInstallResponse, AppMainUpgradeRequest, AppMainUpgradeResponse};
use ego_types::ego_error::EgoError;

#[init]
#[candid_method(init, rename = "init")]
fn canister_init() {
    ic_cdk::println!("ego_tenant: init, caller is {}", caller());
}


#[update(name = "app_main_install")]
#[candid_method(update, rename = "app_main_install")]
async fn app_main_install(req: AppMainInstallRequest) -> Result<AppMainInstallResponse, EgoError> {
    ic_cdk::println!("ego_tenant: app_main_install");
    let management = IcManagement::new();
    let ego_file = EgoFile::new();

    let canister_id = EgoTenantService::app_main_install(req.wallet_id, ego_file, management, req.wasm).await?;
    Ok(AppMainInstallResponse{canister_id})
}

#[update(name = "app_main_upgrade")]
#[candid_method(update, rename = "app_main_upgrade")]
async fn app_main_upgrade(req: AppMainUpgradeRequest) -> Result<AppMainUpgradeResponse, EgoError> {
    ic_cdk::println!("ego_tenant: app_main_upgrade");
    let management = IcManagement::new();
    let ego_file = EgoFile::new();

    let ret = EgoTenantService::app_main_upgrade(req.wallet_id, req.canister_id, ego_file, management, req.wasm).await?;
    Ok(AppMainUpgradeResponse{ret})
}
