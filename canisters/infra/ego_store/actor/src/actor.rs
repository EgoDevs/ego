use std::collections::BTreeMap;

use candid::candid_method;
use candid::{Principal};
use ic_cdk::{caller, id};
use ic_cdk_macros::*;
use ic_ledger_types::Memo;

use ego_lib::ego_canister::EgoCanister;
use ego_macros::{inject_cycle_info_api, inject_ego_api};
use ego_store_mod::c2c::ego_ledger::EgoLedger;
use ego_store_mod::c2c::ego_tenant::EgoTenant as EgoTenantInner;
use ego_store_mod::service::*;
use ego_store_mod::state::*;
use ego_store_mod::types::*;
use ego_store_mod::types::ego_store_app::EgoStoreApp;
use ego_store_mod::types::order::Order;
use ego_store_mod::types::wallet::Wallet;
use ego_store_mod::types::wallet_provider::WalletProvider;
use ego_types::app::CashFlow;
use ego_types::app::EgoError;
use ego_types::app::UserApp;
use ego_types::app::{App, AppId};
use ego_types::types::{AppInstallRequest, AppReInstallRequest, AppUpgradeRequest, WalletUpgradeAppRequest};

inject_ego_api!();
inject_cycle_info_api!();

pub const GIFT_CYCLES_AMOUNT: u128 = 1_000_000_000_000;

#[init]
#[candid_method(init)]
fn init() {
    let caller = caller();
    info_log_add(format!("init, caller is {}", caller.clone()).as_str());

    info_log_add("==> add caller as the owner");
    owner_add(caller.clone());
}

#[pre_upgrade]
fn pre_upgrade() {
    info_log_add("pre_upgrade");

    ego_store_mod::state::pre_upgrade();
}

#[post_upgrade]
fn post_upgrade() {
    info_log_add("post_upgrade");

    ego_store_mod::state::post_upgrade();
}

/********************  methods for wallet   ********************/
#[update(name = "app_main_list")]
#[candid_method(update, rename = "app_main_list")]
pub fn app_main_list() -> Result<Vec<App>, EgoError> {
    info_log_add("app_main_list");
    match EgoStoreService::app_main_list() {
        Ok(apps) => Ok(apps),
        Err(e) => Err(e),
    }
}

#[update(name = "app_main_get")]
#[candid_method(update, rename = "app_main_get")]
pub fn app_main_get(app_id: AppId) -> Result<App, EgoError> {
    info_log_add("app_main_get");
    match EgoStoreService::app_main_get(&app_id) {
        Ok(ego_store_app) => Ok(ego_store_app.app),
        Err(e) => Err(e),
    }
}

#[update(name = "wallet_main_register")]
#[candid_method(update, rename = "wallet_main_register")]
pub fn wallet_main_register(user_id: Principal) -> Result<Principal, EgoError> {
    info_log_add("wallet_main_register");
    let wallet_id = caller();
    let tenant_id = EgoStoreService::wallet_main_register(&wallet_id, &user_id)?;

    Ok(tenant_id)
}

#[update(name = "wallet_tenant_get")]
#[candid_method(update, rename = "wallet_tenant_get")]
pub fn wallet_tenant_get() -> Result<Principal, EgoError> {
    info_log_add("wallet_tenant_get");
    let wallet_id = caller();
    match EgoStoreService::wallet_main_get(&wallet_id) {
        Ok(wallet) => Ok(wallet.tenant_id),
        Err(e) => Err(e),
    }
}

#[update(name = "wallet_app_list")]
#[candid_method(update, rename = "wallet_app_list")]
pub fn wallet_app_list() -> Result<Vec<UserApp>, EgoError> {
    info_log_add("wallet_app_list");
    let wallet_id = ic_cdk::caller();

    let user_apps = EgoStoreService::wallet_app_list(&wallet_id).iter().map(|user_app| {
        user_app.into_ego_user_app()
    }).collect();
    Ok(user_apps)
}

#[update(name = "wallet_app_install")]
#[candid_method(update, rename = "wallet_app_install")]
pub async fn wallet_app_install(app_id: AppId) -> Result<UserApp, EgoError> {
    info_log_add("wallet_app_install");

    info_log_add(format!("1 get app [{}] to be install", app_id).as_str());
    let app = EgoStoreApp::get(&app_id).expect("app not exists");

    let wallet_id = caller();
    info_log_add(format!("2 get wallet_id {}", wallet_id).as_str());

    let ego_tenant = EgoTenantInner::new();
    let ego_canister = EgoCanister::new();

    let user_app =
        EgoStoreService::wallet_app_install(ego_tenant, ego_canister, &wallet_id, &app).await?;

    Ok(user_app.into_ego_user_app())
}

#[update(name = "wallet_app_install_v2")]
#[candid_method(update, rename = "wallet_app_install_v2")]
pub async fn wallet_app_install_v2(req: AppInstallRequest) -> Result<UserApp, EgoError> {
    info_log_add("wallet_app_install_v2");

    let app_id = req.app_id;

    info_log_add(format!("1 get app [{}] to be install", app_id).as_str());
    let app = EgoStoreApp::get(&app_id).expect("app not exists");

    let wallet_id = caller();
    info_log_add(format!("2 get wallet_id {}", wallet_id).as_str());

    let ego_tenant = EgoTenantInner::new();
    let ego_canister = EgoCanister::new();

    let user_app =
      EgoStoreService::wallet_app_install(ego_tenant, ego_canister, &wallet_id, &app).await?;

    Ok(user_app.into_ego_user_app())
}

/// canister自己升级自己
#[update(name = "wallet_app_upgrade")]
#[candid_method(update, rename = "wallet_app_upgrade")]
pub async fn wallet_app_upgrade(wallet_id: Principal) -> Result<(), EgoError> {
    let canister_id = caller();
    let ego_tenant = EgoTenantInner::new();
    let ego_canister = EgoCanister::new();

    info_log_add(
        format!(
            "wallet_app_upgrade wallet_id: {}, canister_id: {}",
            wallet_id, canister_id
        )
        .as_str(),
    );

    EgoStoreService::wallet_app_upgrade(ego_tenant, ego_canister, &wallet_id, &canister_id).await?;
    Ok(())
}

#[update(name = "wallet_app_upgrade_v2")]
#[candid_method(update, rename = "wallet_app_upgrade_v2")]
pub async fn wallet_app_upgrade_v2(req: AppUpgradeRequest) -> Result<(), EgoError> {
    let wallet_id = req.wallet_id;
    let canister_id = caller();
    let ego_tenant = EgoTenantInner::new();
    let ego_canister = EgoCanister::new();

    info_log_add(
        format!(
            "wallet_app_upgrade_v2 wallet_id: {}, canister_id: {}",
            wallet_id, canister_id
        )
          .as_str(),
    );

    EgoStoreService::wallet_app_upgrade(ego_tenant, ego_canister, &wallet_id, &canister_id).await?;
    Ok(())
}

/// wallet升级canister
#[update(name = "wallet_app_upgrade_by_wallet")]
#[candid_method(update, rename = "wallet_app_upgrade_by_wallet")]
pub async fn wallet_app_upgrade_by_wallet(canister_id: Principal) -> Result<(), EgoError> {
    let wallet_id = caller();
    let ego_tenant = EgoTenantInner::new();
    let ego_canister = EgoCanister::new();

    info_log_add(
        format!(
            "wallet_app_upgrade_by_wallet wallet_id: {}, canister_id: {}",
            wallet_id, canister_id
        )
        .as_str(),
    );

    EgoStoreService::wallet_app_upgrade(ego_tenant, ego_canister, &wallet_id, &canister_id).await?;
    Ok(())
}

#[update(name = "wallet_app_upgrade_by_wallet_v2")]
#[candid_method(update, rename = "wallet_app_upgrade_by_wallet_v2")]
pub async fn wallet_app_upgrade_by_wallet_v2(req: WalletUpgradeAppRequest) -> Result<(), EgoError> {
    let wallet_id = caller();
    let canister_id = req.canister_id;
    let ego_tenant = EgoTenantInner::new();
    let ego_canister = EgoCanister::new();

    info_log_add(
        format!(
            "wallet_app_upgrade_by_wallet_v2 wallet_id: {}, canister_id: {}",
            wallet_id, canister_id
        )
          .as_str(),
    );

    EgoStoreService::wallet_app_upgrade(ego_tenant, ego_canister, &wallet_id, &canister_id).await?;
    Ok(())
}

#[update(name = "wallet_app_reinstall_by_wallet_v2")]
#[candid_method(update, rename = "wallet_app_reinstall_by_wallet_v2")]
pub async fn wallet_app_reinstall_by_wallet_v2(req: AppReInstallRequest) -> Result<(), EgoError> {
    let wallet_id = caller();
    let canister_id = req.canister_id;
    let ego_tenant = EgoTenantInner::new();
    let ego_canister = EgoCanister::new();

    info_log_add(
        format!(
            "wallet_app_reinstall_by_wallet_v2 wallet_id: {}, canister_id: {}",
            wallet_id, canister_id
        )
          .as_str(),
    );

    EgoStoreService::wallet_app_reinstall(ego_tenant, ego_canister, &wallet_id, &canister_id).await?;
    Ok(())
}


#[update(name = "wallet_app_remove")]
#[candid_method(update, rename = "wallet_app_remove")]
pub fn wallet_app_remove(wallet_id: Principal) -> Result<(), EgoError> {
    let canister_id = caller();
    let ego_tenant = EgoTenantInner::new();

    info_log_add(
        format!(
            "wallet_app_remove wallet_id: {}, canister_id: {}",
            wallet_id, canister_id
        )
        .as_str(),
    );

    match EgoStoreService::wallet_app_remove(ego_tenant, &wallet_id, &canister_id) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

#[update(name = "wallet_canister_track")]
#[candid_method(update, rename = "wallet_canister_track")]
pub fn wallet_canister_track(canister_id: Principal) -> Result<(), EgoError> {
    info_log_add("canister_main_track");

    let ego_tenant = EgoTenantInner::new();
    let wallet_id = caller();

    EgoStoreService::wallet_canister_track(ego_tenant, &wallet_id, &canister_id)
}

#[update(name = "wallet_canister_track_self")]
#[candid_method(update, rename = "wallet_canister_track_self")]
pub fn wallet_canister_track_self(wallet_id: Principal) -> Result<(), EgoError> {
    info_log_add("wallet_canister_track_self");

    let ego_tenant = EgoTenantInner::new();
    let canister_id = caller();

    EgoStoreService::wallet_canister_track(ego_tenant, &wallet_id, &canister_id)
}

#[update(name = "wallet_canister_untrack")]
#[candid_method(update, rename = "wallet_canister_untrack")]
pub fn wallet_canister_untrack(canister_id: Principal) -> Result<(), EgoError> {
    info_log_add("canister_main_untrack");

    let ego_tenant = EgoTenantInner::new();
    let wallet_id = caller();

    EgoStoreService::wallet_canister_untrack(ego_tenant, &wallet_id, &canister_id)
}

#[update(name = "wallet_canister_untrack_self")]
#[candid_method(update, rename = "wallet_canister_untrack_self")]
pub fn wallet_canister_untrack_self(wallet_id: Principal) -> Result<(), EgoError> {
    info_log_add("wallet_canister_untrack_self");

    let ego_tenant = EgoTenantInner::new();
    let canister_id = caller();

    EgoStoreService::wallet_canister_untrack(ego_tenant, &wallet_id, &canister_id)
}

#[update(name = "wallet_order_list")]
#[candid_method(update, rename = "wallet_order_list")]
pub fn wallet_order_list() -> Result<Vec<Order>, EgoError> {
    info_log_add("wallet_order_list");

    let wallet_id = caller();

    Ok(EgoStoreService::wallet_order_list(&wallet_id))
}

#[update(name = "wallet_order_new")]
#[candid_method(update, rename = "wallet_order_new")]
pub async fn wallet_order_new(amount: f32) -> Result<Memo, EgoError> {
    info_log_add("wallet_order_new");

    let ego_ledger_id = canister_get_one("ego_ledger").unwrap();
    let ego_ledger = EgoLedger::new(ego_ledger_id);

    let wallet_id = caller();
    let store_id = id();

    match EgoStoreService::wallet_order_new(ego_ledger, &wallet_id, &store_id, amount) {
        Ok(order) => Ok(order.memo),
        Err(e) => {
            info_log_add(&format!("wallet_order_new {:?}", e));
            Err(e)
        }
    }
}

#[update(name = "wallet_cycle_balance")]
#[candid_method(update, rename = "wallet_cycle_balance")]
pub fn wallet_cycle_balance() -> Result<u128, EgoError> {
    info_log_add("wallet_cycle_balance");

    let wallet_id = caller();

    match EgoStoreService::wallet_cycle_balance(&wallet_id) {
        Ok(balance) => Ok(balance),
        Err(e) => {
            info_log_add(&format!("wallet_cycle_list {:?}", e));
            Err(e)
        }
    }
}

#[update(name = "wallet_cycle_list")]
#[candid_method(update, rename = "wallet_cycle_list")]
pub fn wallet_cycle_list() -> Result<Vec<CashFlow>, EgoError> {
    info_log_add("wallet_cycle_list");

    let wallet_id = caller();

    let cash_flows = EgoStoreService::wallet_cash_flow_list(&wallet_id);

    Ok(cash_flows.iter().map(|cash_flow| CashFlow{
        cash_flow_type: cash_flow.cash_flow_type.clone(),
        cycles: cash_flow.cycles,
        balance: cash_flow.balance,
        created_at: cash_flow.created_at,
        operator: cash_flow.operator.clone(),
        comment: cash_flow.comment.clone(),
    }).collect())
}

/********************  methods for ego_tenant  ********************/
#[update(name = "wallet_cycle_charge", guard = "user_guard")]
#[candid_method(update, rename = "wallet_cycle_charge")]
pub fn wallet_cycle_charge(
    request: WalletCycleChargeRequest,
) -> Result<WalletCycleChargeResponse, EgoError> {
    info_log_add("wallet_cycle_charge");

    // the tenant id or something else
    let operator = caller();

    match EgoStoreService::wallet_cycle_charge(
        &request.wallet_id,
        request.cycle,
        &operator,
        request.comment,
    ) {
        Ok(_) => Ok(WalletCycleChargeResponse { ret: true }),
        Err(e) => Err(e),
    }
}

/********************  methods for ego_dev  ********************/
#[update(name = "app_main_release", guard = "user_guard")]
#[candid_method(update, rename = "app_main_release")]
pub async fn app_main_release(app: EgoStoreApp) -> Result<bool, EgoError> {
    info_log_add(format!("app_main_release, app_id {}", app.app.app_id).as_str());

    match EgoStoreService::app_main_release(app) {
        Ok(ret) => Ok(ret),
        Err(e) => Err(e),
    }
}

/********************  methods for ego-ledger callback  ********************/
#[update(name = "wallet_order_notify", guard = "user_guard")]
#[candid_method(update, rename = "wallet_order_notify")]
pub fn wallet_order_notify(memo: Memo) -> Result<bool, EgoError> {
    info_log_add("wallet_order_notify");

    // the ego_ledger id
    let operator = caller();

    match EgoStoreService::wallet_order_notify(memo, &operator) {
        Ok(_) => Ok(true),
        Err(e) => Err(e),
    }
}

/******************** owner methods  ********************/
#[update(name = "admin_wallet_cycle_recharge", guard = "owner_guard")]
#[candid_method(update, rename = "admin_wallet_cycle_recharge")]
pub fn admin_wallet_cycle_recharge(req: AdminWalletCycleRechargeRequest) -> Result<bool, EgoError> {
    info_log_add("admin_wallet_cycle_recharge");

    // the ego_ops id
    let operator = caller();

    let wallet_id = req.wallet_id;

    let _result = EgoStoreService::admin_wallet_cycle_recharge(
        &wallet_id,
        req.cycle,
        &operator,
        req.comment,
    )?;

    let ego_tenant = EgoTenantInner::new();
    let _track_result = EgoStoreService::wallet_user_apps_track(ego_tenant, &wallet_id);
    Ok(true)
}

/********************  methods for wallet provider  ********************/
#[update(name = "wallet_main_new")]
#[candid_method(update, rename = "wallet_main_new")]
pub async fn wallet_main_new(user_id: Principal) -> Result<UserApp, EgoError> {
    info_log_add("wallet_main_new");

    let wallet_provider_id = caller();

    info_log_add(&format!("wallet_provider is {}", wallet_provider_id));

    let wallet_provider = WalletProvider::get(&wallet_provider_id).ok_or(EgoStoreErr::WalletProviderNotExists).unwrap();

    let app_id = wallet_provider.app_id;

    info_log_add(&format!("1 get controller app_id {}", app_id));

    let ego_tenant = EgoTenantInner::new();
    let ego_canister = EgoCanister::new();
    let user_app = EgoStoreService::wallet_controller_install(
        ego_tenant,
        ego_canister,
        wallet_provider_id,
        user_id,
        app_id,
    )
    .await?;

    info_log_add("9 send gift cycle to wallet");
    let canister_id = user_app.canister.canister_id;
    let _result = EgoStoreService::admin_wallet_cycle_recharge(
        &canister_id,
        GIFT_CYCLES_AMOUNT,
        &id(),
        "Register Account".to_string(),
    );

    Ok(user_app.into_ego_user_app())
}

/********************  methods for astro_deployer   ********************/
///
/// 返回wallet_provider列表
///
#[update(name = "admin_wallet_provider_list", guard = "owner_guard")]
#[candid_method(update, rename = "admin_wallet_provider_list")]
pub fn admin_wallet_provider_list() -> Result<Vec<WalletProvider>, EgoError> {
    info_log_add("admin_wallet_provider_list");

    Ok(WalletProvider::list())
}

///
/// 添加wallet_provider
///
#[update(name = "admin_wallet_provider_add", guard = "owner_guard")]
#[candid_method(update, rename = "admin_wallet_provider_add")]
pub fn admin_wallet_provider_add(req: AdminWalletProviderAddRequest) -> Result<(), EgoError> {
    info_log_add("admin_wallet_provider_add");

    let wallet_provider = WalletProvider::new(&req.wallet_provider, &req.wallet_app_id);

    wallet_provider.save();
    Ok(())
}
///
/// 删除wallet_provider
///
#[update(name = "admin_wallet_provider_delete", guard = "owner_guard")]
#[candid_method(update, rename = "admin_wallet_provider_delete")]
pub fn admin_wallet_provider_delete(wallet_provider_id: Principal) -> Result<(), EgoError> {
    info_log_add("admin_wallet_provider_delete");

    let wallet_provider = WalletProvider::get(&wallet_provider_id).unwrap();
    wallet_provider.remove();
    Ok(())
}

///
/// 返回Wallet列表
///
#[update(name = "admin_wallet_list", guard = "owner_guard")]
#[candid_method(update, rename = "admin_wallet_list")]
pub fn admin_wallet_list(last_update: u64) -> Result<Vec<Wallet>, EgoError> {
    info_log_add(format!("admin_wallet_list after {}", last_update).as_str());

    Ok(Wallet::by_last_update(last_update))
}

///
/// 添加用户钱包
///
#[update(name = "admin_wallet_add", guard = "owner_guard")]
#[candid_method(update, rename = "admin_wallet_add")]
pub fn admin_wallet_add(wallets: Vec<WalletImport>) {
    info_log_add("admin_wallet_add");

    wallets.iter().for_each(|wallet| {
        let mut w = Wallet::new(&wallet.tenant_id, &wallet.wallet_id, &wallet.user_id);
        w.cycles = wallet.cycles;
        w.save();

        wallet.user_apps.iter().for_each(|app| {
            info_log_add(format!("admin_wallet_app_add app_id:{}", app.app.app_id).as_str());
            let mut user_app = ego_store_mod::types::user_app::UserApp::new(&app.app, app.canister.clone(), Some(wallet.wallet_id));
            user_app.latest_version = app.latest_version;
            user_app.save();
        });

        wallet.cash_flows.iter().for_each(|cash_flow| {
            let mut c_f = cash_flow::CashFlow::new(wallet.wallet_id, cash_flow.cash_flow_type.clone(), cash_flow.cycles, cash_flow.balance, &cash_flow.operator, cash_flow.comment.clone());
            c_f.created_at = cash_flow.created_at / 1000000000;
            c_f.save();
        })
    });
}


///
/// 返回某个Wallet已经安装的应用列表
///
#[update(name = "admin_wallet_app_list", guard = "owner_guard")]
#[candid_method(update, rename = "admin_wallet_app_list")]
pub fn admin_wallet_app_list(last_update: u64) -> Result<Vec<user_app::UserApp>, EgoError> {
    info_log_add(format!("admin_wallet_app_list after {}", last_update).as_str());

    let user_apps = user_app::UserApp::by_last_update(last_update);
    Ok(user_apps)
}

///
/// 返回某个Wallet已经安装的应用列表
///
#[update(name = "admin_wallet_cash_flow_list", guard = "owner_guard")]
#[candid_method(update, rename = "admin_wallet_cash_flow_list")]
pub fn admin_wallet_cash_flow_list(last_update: u64) -> Result<Vec<cash_flow::CashFlow>, EgoError> {
    info_log_add(format!("admin_wallet_cash_flow_list after {}", last_update).as_str());

    let cash_flows = cash_flow::CashFlow::by_last_update(last_update);
    Ok(cash_flows)
}

/********************  数据导出   ********************/
#[update(name = "admin_export", guard = "owner_guard")]
#[candid_method(update, rename = "admin_export")]
fn admin_export() -> Vec<u8> {
    info_log_add("admin_export");
    vec![]
}

/********************  methods for ego_cycle_threshold_get   ********************/
pub fn cycle_threshold_get() -> u128 {
    1_000_000_000_000
}

pub fn runtime_cycle_threshold_get() -> u128 {
    1_000_000_000_000
}