use ego_ops_mod::service::EgoOpsService;
use ic_cdk::{id, storage};
use ic_cdk_macros::*;

use candid::candid_method;
use ego_ops_mod::c2c::ego_canister::{EgoCanister, TEgoCanister};
use ego_ops_mod::c2c::ego_dev::EgoDev;
use ego_ops_mod::c2c::ego_store::{EgoStore, TEgoStore};
use ego_ops_mod::c2c::ego_tenant::{EgoTenant, TEgoTenant};
use ego_ops_mod::ego_ops::EgoOps;
use ego_ops_mod::state::EGO_OPS;
use ego_ops_mod::types::{AdminAppCreateRequest, AdminWalletCycleRechargeRequest, AdminWalletProviderAddRequest};
use ego_types::ego_error::EgoError;
use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;

use ego_macros::inject_balance_get;
use ego_users::inject_ego_users;
use ego_registry::inject_ego_registry;

inject_balance_get!();
inject_ego_users!();
inject_ego_registry!();

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct InitArg {
    init_caller: Option<Principal>,
}

#[init]
#[candid_method(init)]
pub fn init(arg: InitArg) {
    let caller = arg.init_caller.unwrap_or(caller());
    ic_cdk::println!("ego-ops: init, caller is {}", caller.clone());

    ic_cdk::println!("==> add caller as the owner");
    users_init(caller.clone());
}

#[derive(CandidType, Deserialize, Serialize)]
struct PersistState {
    pub ego_ops: EgoOps,
    pub user: User,
    pub registry: Registry
}

#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("ego-ops: pre_upgrade");
    let ego_ops = EGO_OPS.with(|ego_ops| ego_ops.borrow().clone());
    let user = users_pre_upgrade();
    let registry = registry_pre_upgrade();

    let state = PersistState { ego_ops, user, registry };
    storage::stable_save((state,)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("ego-ops: post_upgrade");
    let (state,): (PersistState,) = storage::stable_restore().unwrap();
    EGO_OPS.with(|ego_ops| *ego_ops.borrow_mut() = state.ego_ops);
    users_post_upgrade(state.user);
    registry_post_upgrade(state.registry);
}

/********************  methods for ego_registry   ********************/
fn on_canister_added(_name: &str, _canister_id: Principal) {

}

/********************   owner method   ********************/
#[update(name = "canister_relation_update", guard = "owner_guard")]
#[candid_method(update, rename = "canister_relation_update")]
pub async fn canister_relation_update() {
    ic_cdk::println!("ego-ops: canister_relation_update");

    REGISTRY.with(|register| {
        let ego_canister = EgoCanister::new();

        let ego_dev_id = register.borrow().canister_get_one("ego_dev").unwrap();
        let ego_file_ids = register.borrow().canister_get_all("ego_file");

        let ego_store_id = register.borrow().canister_get_one("ego_store").unwrap();
        let ego_tenant_ids = register.borrow().canister_get_all("ego_tenant");

        let ego_cron_id = register.borrow().canister_get_one("ego_cron").unwrap();
        let ego_ledger_id = register.borrow().canister_get_one("ego_ledger").unwrap();
        let ego_log_id = register.borrow().canister_get_one("ego_log").unwrap();

        // ego_dev
        ic_cdk::println!("1 add canister to ego_dev");
        for ego_file_id in ego_file_ids.iter() {
            ego_canister.canister_add(&ego_dev_id, "ego_file".to_string(), ego_file_id);
        }
        ego_canister
          .canister_add(&ego_dev_id, "ego_store".to_string(), &ego_store_id);

        // ego_file
        ic_cdk::println!("2 add canister to ego_file");
        for ego_file_id in ego_file_ids.iter() {
            ego_canister.canister_add(ego_file_id, "ego_dev".to_string(), &ego_dev_id);
            for ego_tenant_id in ego_tenant_ids.iter() {
                ego_canister.canister_add(ego_file_id, "ego_tenant".to_string(), ego_tenant_id);
            }
        }

        // ego_store
        ic_cdk::println!("3 add canister to ego_store");
        ego_canister.canister_add(&ego_store_id, "ego_dev".to_string(), &ego_dev_id);
        ego_canister.canister_add(&ego_store_id, "ego_cron".to_string(), &ego_cron_id);
        for ego_tenant_id in ego_tenant_ids.iter() {
            ego_canister.canister_add(&ego_store_id, "ego_tenant".to_string(), ego_tenant_id);
        }

        // ego_tenant
        ic_cdk::println!("4 add canister to ego_tenant");
        for ego_tenant_id in ego_tenant_ids.iter() {
            ego_canister.canister_add(ego_tenant_id, "ego_store".to_string(), &ego_store_id);
            ego_canister.canister_add(ego_tenant_id, "ego_cron".to_string(), &ego_cron_id);
        }

        // ego_cron
        ic_cdk::println!("5 add canister to ego_cron");
        ego_canister.canister_add(&ego_cron_id, "ego_ledger".to_string(), &ego_ledger_id);
        for ego_tenant_id in ego_tenant_ids.iter() {
            ego_canister.canister_add(&ego_cron_id, "ego_tenant".to_string(), ego_tenant_id);
        }

        // ego_ledger
        ic_cdk::println!("6 add canister to ego_ledger");
        ego_canister.canister_add(&ego_ledger_id, "ego_cron".to_string(), &ego_cron_id);
        ego_canister.canister_add(&ego_ledger_id, "ego_store".to_string(), &ego_store_id);
        ego_canister.canister_add(&ego_ledger_id, "ego_log".to_string(), &ego_log_id);

        // ego_log
        ic_cdk::println!("7 add canister to ego_log");
        ego_canister.canister_add(&ego_log_id, "ego_cron".to_string(), &ego_ledger_id);
    });
}

#[update(name = "canister_main_track", guard = "owner_guard")]
#[candid_method(update, rename = "canister_main_track")]
pub async fn canister_main_track() {
    ic_cdk::println!("ego-ops: canister_main_track");

    let wallet_id = id();
    let ego_tenant = EgoTenant::new();

    REGISTRY.with(|register| {
        let tracker_ego_tenant_id = register.borrow().canister_get_one("ego_tenant").unwrap();

        // ego_dev
        ic_cdk::println!("1 track ego_dev");
        let ego_dev_id = register.borrow().canister_get_one("ego_dev").unwrap();
        ego_tenant.canister_main_track(tracker_ego_tenant_id, wallet_id, ego_dev_id);

        // ego_file
        ic_cdk::println!("2 track ego_file");
        for ego_file_id in register.borrow().canister_get_all("ego_file") {
            ego_tenant.canister_main_track(tracker_ego_tenant_id, wallet_id, ego_file_id);
        }

        // ego_store
        ic_cdk::println!("3 track ego_store");
        let ego_store_id = register.borrow().canister_get_one("ego_store").unwrap();
        ego_tenant
          .canister_main_track(tracker_ego_tenant_id, wallet_id, ego_store_id);

        // ego_tenant
        ic_cdk::println!("4 track ego_tenant");
        for ego_tenant_id in register.borrow().canister_get_all("ego_tenant") {
            ego_tenant
              .canister_main_track(tracker_ego_tenant_id, wallet_id, ego_tenant_id);
        }

        // ego_cron
        ic_cdk::println!("5 track ego_cron");
        let ego_cron_id = register.borrow().canister_get_one("ego_cron").unwrap();
        ego_tenant
          .canister_main_track(tracker_ego_tenant_id, wallet_id, ego_cron_id);

        // ego_ledger
        ic_cdk::println!("6 track ego_ledger");
        let ego_ledger_id = register.borrow().canister_get_one("ego_ledger").unwrap();
        ego_tenant
          .canister_main_track(tracker_ego_tenant_id, wallet_id, ego_ledger_id);

        // ego_ops
        ic_cdk::println!("7 track ego_ops");
        ego_tenant
          .canister_main_track(tracker_ego_tenant_id, wallet_id, wallet_id);

        // ego_log
        let ego_log_id = register.borrow().canister_get_one("ego_log").unwrap();
        ic_cdk::println!("7 track ego_log");
        ego_tenant
          .canister_main_track(tracker_ego_tenant_id, wallet_id, ego_log_id);

    });
}

#[update(name = "admin_app_create", guard = "owner_guard")]
#[candid_method(update, rename = "admin_app_create")]
pub fn admin_app_create(
    req: AdminAppCreateRequest,
) -> Result<(), EgoError> {
    ic_cdk::println!("ego-ops: admin_app_create");

    let ego_dev = EgoDev::new();
    let ego_dev_id = REGISTRY.with(|r| r.borrow().canister_get_one("ego_dev")).unwrap();


    EgoOpsService::admin_app_create(
        ego_dev,
        ego_dev_id,
        req.app_id,
        req.name,
        req.version,
        req.category,
        req.logo,
        req.description,
        req.backend_data,
        req.backend_hash,
        req.frontend,
        req.deploy_mode,
    )?;
    Ok(())
}

#[update(name = "admin_wallet_provider_add", guard = "owner_guard")]
#[candid_method(update, rename = "admin_wallet_provider_add")]
pub fn admin_wallet_provider_add(req: AdminWalletProviderAddRequest) -> Result<(), EgoError> {
    ic_cdk::println!("ego_ops: admin_wallet_provider_add");

    let ego_store_id = REGISTRY.with(|r| r.borrow().canister_get_one("ego_store")).unwrap();
    let ego_store = EgoStore::new(ego_store_id);

    ego_store
        .admin_wallet_provider_add(req.wallet_provider, req.wallet_app_id);

    Ok(())
}

#[update(name = "admin_wallet_cycle_recharge", guard = "owner_guard")]
#[candid_method(update, rename = "admin_wallet_cycle_recharge")]
pub fn admin_wallet_cycle_recharge(
    req: AdminWalletCycleRechargeRequest,
) -> Result<(), EgoError> {
    ic_cdk::println!("ego_ops: admin_wallet_cycle_recharge");

    let ego_store_id = REGISTRY.with(|r| r.borrow().canister_get_one("ego_store")).unwrap();
    let ego_store = EgoStore::new(ego_store_id);

    ego_store
        .admin_wallet_cycle_recharge(req.wallet_id, req.cycle, req.comment);

    Ok(())
}

#[update(name = "admin_wallet_order_new", guard = "owner_guard")]
#[candid_method(update, rename = "admin_wallet_order_new")]
pub fn admin_wallet_order_new(
    amount: f32,
) -> Result<(), EgoError> {
    ic_cdk::println!("ego_ops: admin_wallet_order_new");

    let ego_store_id = REGISTRY.with(|r| r.borrow().canister_get_one("ego_store")).unwrap();
    let ego_store = EgoStore::new(ego_store_id);

    ego_store
      .admin_wallet_order_new(amount);

    Ok(())
}
