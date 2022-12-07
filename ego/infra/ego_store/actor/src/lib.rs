use candid::candid_method;
use ic_cdk::api::time;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::{storage};
use ic_cdk_macros::*;
use serde::Serialize;

use ego_macros::inject_balance_get;
use ego_store_mod::c2c::ego_ledger::EgoLedger;
use ego_store_mod::c2c::ego_tenant::EgoTenant;
use ego_store_mod::ego_store::EgoStore;
use ego_store_mod::service::*;
use ego_store_mod::state::EGO_STORE;
use ego_store_mod::types::*;
use ego_types::app::{App, DeployMode};
use ego_types::app::DeployMode::DEDICATED;
use ego_types::ego_error::EgoError;
use ego_macros::inject_ego_macros;
use ego_store_mod::order::Order;

use astrox_macros::inject_canister_registry;
use astrox_macros::inject_canister_users;

inject_canister_users!();
inject_canister_registry!();

inject_ego_macros!();
inject_balance_get!();


#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct InitArg {
    init_caller: Option<Principal>,
}

#[init]
#[candid_method(init)]
pub fn init(arg: InitArg) {
    let caller = arg.init_caller.unwrap_or(caller());
    ic_cdk::println!("ego_store: init, caller is {}", caller.clone());

    ic_cdk::println!("==> add caller as the owner");
    owner_add(caller.clone());
}

#[derive(CandidType, Deserialize, Serialize)]
struct PersistState {
    pub ego_store: EgoStore,
    pub user: User,
    pub registry: Registry,
}

#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("ego_store: pre_upgrade");

    let ego_store = EGO_STORE.with(|ego_store| ego_store.borrow().clone());
    let user = users_pre_upgrade();
    let registry = registry_pre_upgrade();

    let state = PersistState { ego_store, user, registry };
    storage::stable_save((state,)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("ego_store: post_upgrade");

    let (state,): (PersistState,) = storage::stable_restore().unwrap();
    EGO_STORE.with(|ego_store| *ego_store.borrow_mut() = state.ego_store);

    users_post_upgrade(state.user);
    registry_post_upgrade(state.registry)
}

/********************  methods for ego_registry   ********************/
fn on_canister_added(name: &str, canister_id: Principal) {
    ego_log(&format!("on_canister_added name: {}, canister_id: {}", name, canister_id));
    let _ = match name {
        "ego_dev" => user_add(canister_id),
        "ego_ledger" => user_add(canister_id),
        "ego_tenant" => {
            user_add(canister_id);
            EgoStoreService::admin_ego_tenant_add(canister_id);
        },
        _ => {}
    };
}

/********************  methods for wallet   ********************/
#[query(name = "app_main_list")]
#[candid_method(query, rename = "app_main_list")]
pub fn app_main_list(request: AppMainListRequest) -> Result<AppMainListResponse, EgoError> {
    ego_log("ego_store: app_main_list");
    match EgoStoreService::app_main_list(request.query_param) {
        Ok(apps) => Ok(AppMainListResponse { apps }),
        Err(e) => Err(e),
    }
}

#[query(name = "app_main_get")]
#[candid_method(query, rename = "app_main_get")]
pub fn app_main_get(request: AppMainGetRequest) -> Result<AppMainGetResponse, EgoError> {
    ego_log("ego_store: app_main_get");
    match EgoStoreService::app_main_get(request.app_id) {
        Ok(app) => Ok(AppMainGetResponse {
            app: App::from(app),
        }),
        Err(e) => Err(e),
    }
}

#[update(name = "wallet_main_register")]
#[candid_method(update, rename = "wallet_main_register")]
pub fn wallet_main_register(
    req: WalletMainRegisterRequest,
) -> Result<WalletMainRegisterResponse, EgoError> {
    ego_log("ego_store: wallet_main_register");
    let tenant_id = EgoStoreService::wallet_main_register(ic_cdk::caller(), req.user_id)?;

    Ok(WalletMainRegisterResponse { tenant_id })
}

#[query(name = "wallet_tenant_get")]
#[candid_method(query, rename = "wallet_tenant_get")]
pub fn wallet_tenant_get() -> Result<WalletTenantGetResponse, EgoError> {
    ego_log("ego_store: wallet_tenant_get");
    match EgoStoreService::wallet_tenant_get(ic_cdk::caller()) {
        Ok(tenant_id) => Ok(WalletTenantGetResponse { tenant_id }),
        Err(e) => Err(e),
    }
}

#[query(name = "wallet_app_list")]
#[candid_method(query, rename = "wallet_app_list")]
pub fn wallet_app_list() -> Result<WalletAppListResponse, EgoError> {
    ego_log("ego_store: wallet_app_list");
    let wallet_id = ic_cdk::caller();
    match EgoStoreService::wallet_app_list(&wallet_id) {
        Ok(apps) => Ok(WalletAppListResponse { apps }),
        Err(e) => Err(e),
    }
}

#[update(name = "wallet_app_install")]
#[candid_method(update, rename = "wallet_app_install")]
pub async fn wallet_app_install(
    req: WalletAppInstallRequest,
) -> Result<WalletAppInstallResponse, EgoError> {
    ego_log("ego_store: wallet_app_install");

    ego_log("1 get app to be install");
    let app = EGO_STORE.with(|ego_store| ego_store.borrow().app_main_get(&req.app_id).clone())?;

    ego_log("2 get wallet_id");
    let wallet_id = match app.deploy_mode {
        DeployMode::SHARED => {
            REGISTRY.with(|registry| registry.borrow().canister_get_one("ego_ops")).unwrap()
        },
        DEDICATED => {
            caller()
        }
    };

    let result = EgoStoreService::wallet_app_get(&wallet_id, req.app_id.clone());

    match result {
        Ok(app_installed) => {
            Ok(WalletAppInstallResponse{user_app: app_installed})
        }
        Err(_e) => {
            let ego_tenant = EgoTenant::new();
            let user_app =
              EgoStoreService::wallet_app_install(ego_tenant, wallet_id, app).await?;
            Ok(WalletAppInstallResponse { user_app })
        }
    }
}

#[update(name = "wallet_app_upgrade")]
#[candid_method(update, rename = "wallet_app_upgrade")]
pub async fn wallet_app_upgrade(
    req: WalletAppUpgradeRequest,
) -> Result<WalletAppUpgradeResponse, EgoError> {
    ego_log("ego_store: wallet_app_upgrade");

    ego_log("1 get app to be upgrade");
    let app = EGO_STORE.with(|ego_store| ego_store.borrow().app_main_get(&req.app_id).clone())?;

    ego_log("2 get wallet_id");
    let wallet_id = match app.deploy_mode {
        DeployMode::SHARED => {
            let ops_wallet_id = REGISTRY.with(|registry| registry.borrow().canister_get_one("ego_ops")).unwrap();
            // for shared mode dapp, only the ego_ops can upgraded
            if ops_wallet_id != caller() {
                Err(EgoStoreErr::UnAuthorized)
            } else {
                Ok(ops_wallet_id)
            }
        },
        DEDICATED => {
            Ok(caller())
        }
    }?;

    let ego_tenant = EgoTenant::new();

    let user_app =
        EgoStoreService::wallet_app_upgrade(ego_tenant, wallet_id, app).await?;
    Ok(WalletAppUpgradeResponse { user_app })
}

#[update(name = "wallet_app_remove")]
#[candid_method(update, rename = "wallet_app_remove")]
pub fn wallet_app_remove(req: WalletAppRemoveRequest) -> Result<WalletAppRemoveResponse, EgoError> {
    ego_log("ego_store: wallet_app_remove");
    match EgoStoreService::wallet_app_remove(ic_cdk::caller(), req.app_id) {
        Ok(_) => Ok(WalletAppRemoveResponse {}),
        Err(e) => Err(e),
    }
}

#[update(name = "wallet_canister_track", guard = "user_guard")]
#[candid_method(update, rename = "wallet_canister_track")]
pub async fn wallet_canister_track(req: WalletCanisterTrackRequest) -> Result<(), EgoError> {
    ego_log("ego_store: canister_main_track");

    let ego_tenant = EgoTenant::new();
    let wallet_id = caller();

    EgoStoreService::wallet_canister_track(ego_tenant, wallet_id, req.app_id).await?;
    Ok(())
}

#[update(name = "wallet_canister_untrack", guard = "user_guard")]
#[candid_method(update, rename = "wallet_canister_untrack")]
pub async fn wallet_canister_untrack(req: WalletCanisterUnTrackRequest) -> Result<(), EgoError> {
    ego_log("ego_store: canister_main_untrack");

    let ego_tenant = EgoTenant::new();
    let wallet_id = caller();

    EgoStoreService::wallet_canister_untrack(ego_tenant, wallet_id, req.app_id).await?;
    Ok(())
}

#[update(name = "wallet_order_list")]
#[candid_method(update, rename = "wallet_order_list")]
pub fn wallet_order_list() -> Result<WalletOrderListResponse, EgoError> {
    ego_log("ego_store: wallet_order_list");

    match EgoStoreService::wallet_order_list(ic_cdk::caller()) {
        Ok(orders) => Ok(WalletOrderListResponse { orders }),
        Err(e) => Err(e),
    }
}

#[update(name = "wallet_order_new")]
#[candid_method(update, rename = "wallet_order_new")]
pub async fn wallet_order_new(
    request: WalletOrderNewRequest,
) -> Result<WalletOrderNewResponse, EgoError> {
    ego_log("ego_store: wallet_order_new");

    let ego_ledger_id = REGISTRY.with(|r| r.borrow().canister_get_one("ego_ledger")).unwrap();
    let ego_ledger = EgoLedger::new(ego_ledger_id);

    match EgoStoreService::wallet_order_new(ego_ledger, ic_cdk::caller(), ic_cdk::id(), request.amount) {
        Ok(order) => Ok(WalletOrderNewResponse { memo: order.memo }),
        Err(e) => {
            ego_log(&format!("ego_store: wallet_order_new {:?}", e));
            Err(e)
        },
    }
}

#[update(name = "wallet_cycle_list")]
#[candid_method(update, rename = "wallet_cycle_list")]
pub async fn wallet_cycle_list() -> Result<WalletCycleListResponse, EgoError> {
    ego_log("ego_store: wallet_cycle_list");

    let wallet_id = caller();

    match EgoStoreService::wallet_cycle_list(wallet_id) {
        Ok(cash_flows) => Ok(WalletCycleListResponse { cash_flows }),
        Err(e) => {
            ego_log(&format!("ego_store: wallet_cycle_list {:?}", e));
            Err(e)
        },
    }
}

/********************  methods for ego_tenant  ********************/
#[update(name = "wallet_cycle_charge", guard = "user_guard")]
#[candid_method(update, rename = "wallet_cycle_charge")]
pub fn wallet_cycle_charge(
    request: WalletCycleChargeRequest,
) -> Result<WalletCycleChargeResponse, EgoError> {
    ego_log("ego_store: wallet_cycle_charge");

    // the tenant id or something else
    let operator = caller();

    match EgoStoreService::wallet_cycle_charge(
        request.wallet_id,
        request.cycle,
        operator,
        time(),
        request.comment,
    ) {
        Ok(ret) => Ok(WalletCycleChargeResponse { ret }),
        Err(e) => Err(e),
    }
}

/********************  methods for ego_dev  ********************/
#[update(name = "app_main_release", guard = "user_guard")]
#[candid_method(update, rename = "app_main_release")]
pub async fn app_main_release(
    request: AppMainReleaseRequest,
) -> Result<AppMainReleaseResponse, EgoError> {
    ego_log("ego_store: app_main_release");

    match EgoStoreService::app_main_release(request.app) {
        Ok(ret) => Ok(AppMainReleaseResponse { ret }),
        Err(e) => Err(e),
    }
}

/********************  methods for ego-ledger callback  ********************/
#[update(name = "wallet_order_notify", guard = "user_guard")]
#[candid_method(update, rename = "wallet_order_notify")]
pub fn wallet_order_notify(
    request: WalletOrderNotifyRequest,
) -> Result<WalletOrderNotifyResponse, EgoError> {
    ego_log("ego_store: wallet_order_notify");

    // the ego_ledger id
    let operator = caller();

    match EgoStoreService::wallet_order_notify(request.memo, operator, ic_cdk::api::time()) {
        Ok(ret) => Ok(WalletOrderNotifyResponse { ret }),
        Err(e) => Err(e),
    }
}

/******************** owner methods  ********************/
#[update(name = "admin_wallet_provider_add")]
#[candid_method(update, rename = "admin_wallet_provider_add")]
pub fn admin_wallet_provider_add(
    req: AdminWalletProviderAddRequest,
) -> Result<AdminWalletProviderAddResponse, EgoError> {
    ego_log("ego_store: admin_wallet_provider_add");

    ego_log(&format!("wallet_provider: {}, app_id: {}",
                    req.wallet_provider,
                    req.wallet_app_id));

    match EgoStoreService::admin_wallet_provider_add(&req.wallet_provider, &req.wallet_app_id) {
        Ok(ret) => Ok(AdminWalletProviderAddResponse { ret }),
        Err(e) => Err(e),
    }
}


#[update(name = "admin_wallet_cycle_recharge", guard = "owner_guard")]
#[candid_method(update, rename = "admin_wallet_cycle_recharge")]
pub fn admin_wallet_cycle_recharge(req: AdminWalletCycleRechargeRequest) -> Result<bool, EgoError> {
    ego_log("ego_store: admin_wallet_cycle_recharge");

    // the ego_ops id
    let operator = caller();

    EgoStoreService::admin_wallet_cycle_recharge(req.wallet_id, req.cycle, operator,  time(), req.comment)
}

#[update(name = "admin_wallet_order_list", guard = "owner_guard")]
#[candid_method(update, rename = "admin_wallet_order_list")]
pub fn admin_wallet_order_list() -> Result<Vec<Order>, EgoError> {
    ego_log("ego_store: admin_wallet_order_list");

    Ok(EgoStoreService::wallet_order_list_all())
}

/********************  methods for wallet provider  ********************/
#[update(name = "wallet_main_new")]
#[candid_method(update, rename = "wallet_main_new")]
pub async fn wallet_main_new(req: WalletMainNewRequest) -> Result<WalletMainNewResponse, EgoError> {
    ego_log("ego_store: wallet_main_new");

    let wallet_provider = caller();

    ego_log(&format!("wallet_provider is {}", wallet_provider));

    let app_id = EGO_STORE.with(|ego_store| {
        match ego_store.borrow().wallet_providers.get(&wallet_provider) {
            None => Err(EgoError::from(EgoStoreErr::WalletProviderNotExists)),
            Some(provider) => Ok(provider.app_id.clone()),
        }
    })?;

    let ego_tenant = EgoTenant::new();
    let user_app =
        EgoStoreService::wallet_controller_install(ego_tenant, req.user_id, app_id).await?;

    Ok(WalletMainNewResponse { user_app })
}
