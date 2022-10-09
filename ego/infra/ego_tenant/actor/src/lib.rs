use ic_cdk_macros::*;
use candid::candid_method;
use ic_cdk::storage;
use ego_tenant_mod::c2c::ego_file::EgoFile;
use ego_tenant_mod::c2c::ic_management::IcManagement;
use ego_tenant_mod::ego_tenant::EgoTenant;
use ego_tenant_mod::service::EgoTenantService;
use ego_tenant_mod::types::{AppMainInstallRequest, AppMainInstallResponse, AppMainUpgradeRequest, AppMainUpgradeResponse};
use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;
use ego_tenant_mod::state::EGO_TENANT;
use ego_types::ego_error::EgoError;
use ego_users::inject_ego_users;

inject_ego_users!();


#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct InitArg {
    init_caller: Option<Principal>,
}

#[init]
#[candid_method(init)]
pub fn init(arg: InitArg) {
    let caller = arg.init_caller.unwrap_or(caller());
    ic_cdk::println!("ego-tenant: init, caller is {}", caller.clone());

    ic_cdk::println!("==> add caller as the owner");
    users_init(caller.clone());
}

#[derive(CandidType, Deserialize, Serialize)]
struct PersistState{
    pub ego_tenant: EgoTenant,
    pub user: User
}

#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("ego-tenant: pre_upgrade");
    let ego_tenant = EGO_TENANT.with(|ego_tenant| ego_tenant.borrow().clone());
    let user = users_pre_upgrade();

    let state = PersistState{ego_tenant, user};
    storage::stable_save((state, )).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("ego-tenant: post_upgrade");
    let (state, ): (PersistState, ) = storage::stable_restore().unwrap();
    EGO_TENANT.with(|ego_tenant|
      *ego_tenant.borrow_mut() = state.ego_tenant
    );

    users_post_upgrade(state.user);
}

#[update(name = "app_main_install", guard = "user_guard")]
#[candid_method(update, rename = "app_main_install")]
async fn app_main_install(req: AppMainInstallRequest) -> Result<AppMainInstallResponse, EgoError> {
    ic_cdk::println!("ego_tenant: app_main_install");
    let management = IcManagement::new();
    let ego_file = EgoFile::new();

    let canister_id = EgoTenantService::app_main_install(ego_file, management, req.wallet_id, req.user_id, req.wasm).await?;
    Ok(AppMainInstallResponse{canister_id})
}

#[update(name = "app_main_upgrade", guard = "user_guard")]
#[candid_method(update, rename = "app_main_upgrade")]
async fn app_main_upgrade(req: AppMainUpgradeRequest) -> Result<AppMainUpgradeResponse, EgoError> {
    ic_cdk::println!("ego_tenant: app_main_upgrade");
    let management = IcManagement::new();
    let ego_file = EgoFile::new();

    let ret = EgoTenantService::app_main_upgrade(ego_file, management, req.canister_id, req.wasm).await?;
    Ok(AppMainUpgradeResponse{ret})
}

/********************  notify  ********************/
#[update(name = "message_main_notify", guard = "user_guard")]
#[candid_method(update, rename = "message_main_notify")]
async fn message_main_notify() -> Result<(), EgoError> {
    ic_cdk::println!("ego-tenant: message_main_notify");

    Ok(())
}
