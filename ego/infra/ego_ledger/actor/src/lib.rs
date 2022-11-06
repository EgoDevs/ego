use candid::candid_method;
use ego_ledger_mod::ego_ledger::EgoLedger;
use ego_ledger_mod::service::EgoLedgerService;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::storage;
use ic_cdk_macros::*;
use ic_ledger_types::{MAINNET_LEDGER_CANISTER_ID};
use serde::Serialize;
use ego_ledger_mod::c2c::ego_cron::{EgoCron, TEgoCron};
use ego_ledger_mod::c2c::ego_store::EgoStore;
use ego_ledger_mod::c2c::ic_ledger::{IcLedger};
use ego_ledger_mod::payment::Payment;

use ego_ledger_mod::state::{EGO_LEDGER};
use ego_ledger_mod::types::{
    LedgerMainInitRequest, LedgerPaymentAddRequest,
};
use ego_types::ego_error::EgoError;

use ego_macros::inject_balance_get;
use ego_macros::inject_ego_log;
use ego_users::inject_ego_users;
use ego_registry::inject_ego_registry;


inject_balance_get!();
inject_ego_users!();
inject_ego_registry!();
inject_ego_log!();

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct InitArg {
    init_caller: Option<Principal>,
}

#[init]
#[candid_method(init)]
pub fn init(arg: InitArg) {
    let caller = arg.init_caller.unwrap_or(caller());
    ic_cdk::println!("ego-ledger: init, caller is {}", caller.clone());

    ic_cdk::println!("==> add caller as the owner");
    users_init(caller.clone());
}

#[derive(CandidType, Deserialize, Serialize)]
struct PersistState {
    pub ego_ledger: EgoLedger,
    pub user: User,
    pub registry: Registry
}

#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("ego-ledger: pre_upgrade");
    let ego_ledger = EGO_LEDGER.with(|ego_ledger| ego_ledger.borrow().clone());
    let user = users_pre_upgrade();
    let registry = registry_pre_upgrade();

    let state = PersistState {
        ego_ledger,
        user,
        registry,
    };
    storage::stable_save((state,)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("ego-ledger: post_upgrade");
    let (state,): (PersistState,) = storage::stable_restore().unwrap();
    EGO_LEDGER.with(|ego_ledger| *ego_ledger.borrow_mut() = state.ego_ledger);

    users_post_upgrade(state.user);
    registry_post_upgrade(state.registry);
}

/********************  methods for ego_registry   ********************/
fn on_canister_added(name: &str, canister_id: Principal) {
    let _ = match name {
        "ego_store" => role_user_add(canister_id).unwrap(),
        "ego_cron" => {
            role_user_add(canister_id).unwrap();

            let ego_cron = EgoCron::new(canister_id);
            ego_cron.task_main_add("message_main_notify");
        },
        _ => {}
    };
}


/********************  user  ********************/
#[update(name = "ledger_payment_add", guard = "user_guard")]
#[candid_method(update, rename = "ledger_payment_add")]
fn ledger_payment_add(req: LedgerPaymentAddRequest) -> Result<(), EgoError> {
    ego_log(&format!("ego-ledger: ledger_payment_add from:{} to:{} memo:{:?}", req.from, req.to, req.memo));

    EgoLedgerService::ledger_payment_add(req.from, req.to, req.amount, req.memo);
    Ok(())
}

/********************  owner  ********************/
#[update(name = "ledger_main_init", guard = "owner_guard")]
#[candid_method(update, rename = "ledger_main_init")]
fn ledger_main_init(req: LedgerMainInitRequest) -> Result<(), EgoError> {
    ego_log("ego-ledger: ledger_main_init");
    EgoLedgerService::ledger_main_init(req.start);
    Ok(())
}

#[update(name = "ledger_payment_list", guard = "owner_guard")]
#[candid_method(update, rename = "ledger_payment_list")]
fn ledger_payment_list() -> Result<Vec<Payment>, EgoError> {
    ego_log("ego-ledger: ledger_payment_list");

    let payments = EGO_LEDGER.with(|ego_ledger| ego_ledger.borrow().payments.values().cloned().collect());

    Ok(payments)
}

/********************  notify  ********************/
#[update(name = "message_main_notify", guard = "user_guard")]
#[candid_method(update, rename = "message_main_notify")]
async fn message_main_notify() -> Result<(), EgoError> {
    ego_log("ego-ledger: message_main_notify");

    let ego_store_id = REGISTRY.with(|r| r.borrow().canister_get_one("ego_store")).unwrap();
    let ego_store = EgoStore::new(ego_store_id);

    let ic_ledger = IcLedger::new(MAINNET_LEDGER_CANISTER_ID);

    match get_ego_log(){
        None => {}
        Some(ego_log) => {
            EgoLedgerService::ledger_payment_match(ego_store, ic_ledger, ego_log).await?;
        }
    }

    Ok(())
}
