use ic_cdk_macros::{init, update, query};
use candid::candid_method;
use ic_cdk::caller;
use ego_tenant_mod::c2c::ego_file::EgoFile;
use ego_tenant_mod::c2c::ic_management::IcManagement;
use ego_tenant_mod::service::EgoTenantService;
use ego_tenant_mod::types::{WalletAppInstallRequest, WalletAppInstallResponse, WalletAppUpgradeRequest, WalletAppUpgradeResponse, WalletMainAddRequest, WalletMainAddResponse, WalletMainGetRequest, WalletMainGetResponse, WalletMainRemoveRequest, WalletMainRemoveResponse};
use ego_types::ego_error::EgoError;

#[init]
#[candid_method(init, rename = "init")]
fn canister_init() {
    ic_cdk::println!("ego_tenant: init, caller is {}", caller());
}

/********************  ego-store methods  ********************/
#[update(name = "wallet_main_add")]
fn wallet_main_add(req: WalletMainAddRequest) -> Result<WalletMainAddResponse, EgoError> {
    ic_cdk::println!("ego_tenant: wallet_app_install");

    match EgoTenantService::wallet_main_add(req.wallet_id) {
        Ok(ret) => {Ok(WalletMainAddResponse{ret})},
        Err(e) => {Err(e)}
    }
}

#[update(name = "wallet_main_remove")]
fn wallet_main_remove(req: WalletMainRemoveRequest) -> Result<WalletMainRemoveResponse, EgoError> {
    ic_cdk::println!("ego_tenant: wallet_main_remove");

    match EgoTenantService::wallet_main_remove(&req.wallet_id) {
        Ok(ret) => {Ok(WalletMainRemoveResponse{ret})},
        Err(e) => {Err(e)}
    }
}

#[query(name = "wallet_main_get")]
fn wallet_main_get(req: WalletMainGetRequest) -> Result<WalletMainGetResponse, EgoError> {
    ic_cdk::println!("ego_tenant: wallet_main_get");

    match EgoTenantService::wallet_main_get(&req.wallet_id) {
        Ok(wallet) => {Ok(WalletMainGetResponse{wallet})},
        Err(e) => {Err(e)}
    }
}


#[update(name = "wallet_app_install")]
async fn wallet_app_install(req: WalletAppInstallRequest) -> Result<WalletAppInstallResponse, EgoError> {
    ic_cdk::println!("ego_tenant: wallet_app_install");
    let management = IcManagement::new();
    let ego_file = EgoFile::new();

    let canisters = EgoTenantService::wallet_app_install(caller(), ego_file, management, req.app).await?;
    Ok(WalletAppInstallResponse{canisters})
}

#[update(name = "wallet_app_upgrade")]
async fn wallet_app_upgrade(req: WalletAppUpgradeRequest) -> Result<WalletAppUpgradeResponse, EgoError> {
    ic_cdk::println!("ego_tenant: wallet_app_install");
    let management = IcManagement::new();
    let ego_file = EgoFile::new();

    let ret = EgoTenantService::wallet_app_upgrade(caller(), ego_file, management, req.app).await?;
    Ok(WalletAppUpgradeResponse{ret})
}
