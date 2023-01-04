use ic_cdk::export::Principal;
use ic_ledger_types::Memo;

use ego_lib::ego_canister::TEgoCanister;
use ego_types::app::{App, AppId, Canister, UserApp};
use ego_types::app::EgoError;

use crate::app::EgoStoreApp;
use crate::c2c::ego_ledger::TEgoLedger;
use crate::c2c::ego_tenant::TEgoTenant;
use crate::cash_flow::CashFlow;
use crate::order::Order;
use crate::state::{EGO_STORE, log_add};

pub struct EgoStoreService {}

impl EgoStoreService {
  pub fn app_main_list() -> Result<Vec<App>, EgoError> {
    EGO_STORE.with(|ego_store| ego_store.borrow().app_main_list())
  }

  pub fn app_main_get(app_id: &AppId) -> Result<EgoStoreApp, EgoError> {
    EGO_STORE.with(|ego_store| ego_store.borrow().app_main_get(&app_id))
  }

  pub fn wallet_main_register(
    wallet_id: Principal,
    user_id: Principal,
  ) -> Result<Principal, EgoError> {
    EGO_STORE.with(|ego_store| {
      ego_store
        .borrow_mut()
        .wallet_main_register(wallet_id, user_id)
    })
  }

  pub fn wallet_tenant_get(wallet_id: Principal) -> Result<Principal, EgoError> {
    EGO_STORE.with(|ego_store| ego_store.borrow().wallet_tenant_get(&wallet_id))
  }

  pub fn wallet_app_list(wallet_id: &Principal) -> Result<Vec<UserApp>, EgoError> {
    EGO_STORE.with(|ego_store| ego_store.borrow().wallet_app_list(&wallet_id))
  }

  pub fn wallet_app_get(wallet_id: &Principal, canister_id: &Principal) -> Result<UserApp, EgoError> {
    EGO_STORE.with(|ego_store| ego_store.borrow().wallet_app_get(wallet_id, canister_id))
  }

  pub async fn wallet_app_install<T: TEgoTenant, EC: TEgoCanister>(
    ego_tenant: T,
    ego_canister: EC,
    wallet_id: Principal,
    ego_store_app: EgoStoreApp,
  ) -> Result<UserApp, EgoError> {
    log_add("3 get ego_tenant_id relative to wallet");
    let ego_tenant_id = EGO_STORE.with(|ego_store| ego_store.borrow().wallet_tenant_get(&wallet_id).clone())?;

    log_add("4 get wallet");
    let wallet = EGO_STORE.with(|ego_store| ego_store.borrow().wallet_main_get(wallet_id))?;

    log_add("5 call ego tenant to install wasm");

    let canister_id = ego_tenant
      .app_main_install(
        ego_tenant_id,
        wallet_id,
        wallet.user_id,
        &ego_store_app.wasm,
      )
      .await?;

    let user_app = UserApp::new(
      &ego_store_app.app,
      Canister::new(canister_id, ego_store_app.wasm.canister_type),
    );

    EGO_STORE.with(|ego_store| {
      ego_store
        .borrow_mut()
        .wallet_app_install(&wallet_id, &user_app);
    });

    log_add("6 set app info");
    ego_canister.app_info_update(canister_id, wallet_id, ego_store_app.app.app_id, ego_store_app.app.current_version).await?;

    Ok(user_app)
  }

  pub async fn wallet_app_upgrade<T: TEgoTenant>(
    ego_tenant: T,
    wallet_id: &Principal,
    canister_id: &Principal,
  ) -> Result<(), EgoError> {
    log_add("1 get user_app to be upgrade");

    let user_app = EgoStoreService::wallet_app_get(wallet_id, canister_id)?;

    log_add("2 get app to be upgrade");
    let ego_store_app = EGO_STORE.with(|ego_store| ego_store.borrow().app_main_get(&user_app.app.app_id).clone())?;

    log_add("3 get ego tenant id relative to wallet");
    let ego_tenant_id =
      EGO_STORE.with(|ego_store| ego_store.borrow().wallet_tenant_get(&wallet_id).clone())?;


    log_add("4 call ego tenant to upgrade backend");
    ego_tenant
      .app_main_upgrade(
        ego_tenant_id,
        user_app.canister.canister_id,
        &ego_store_app.wasm,
      )
      .await?;


    EGO_STORE.with(|ego_store| {
      ego_store
        .borrow_mut()
        .wallet_app_upgrade(&wallet_id, &user_app, &ego_store_app);
    });

    Ok(())
  }

  pub fn wallet_app_remove(wallet_id: &Principal, canister_id: &Principal) -> Result<(), EgoError> {
    log_add("1 check user_app exists");
    let _user_app = EgoStoreService::wallet_app_get(&wallet_id, &canister_id)?;

    EGO_STORE.with(|ego_store| {
      ego_store
        .borrow_mut()
        .wallet_app_remove(&wallet_id, &canister_id)
    })
  }

  pub fn wallet_canister_track<T: TEgoTenant>(
    ego_tenant: T,
    wallet_id: &Principal,
    canister_id: &Principal,
  ) -> Result<(), EgoError> {
    log_add("1 get ego tenant id");
    let ego_tenant_id =
      EGO_STORE.with(|ego_store| ego_store.borrow().wallet_tenant_get(&wallet_id).clone())?;

    log_add("2 get user app");
    // confirm user app exists
    let _user_app =
      EGO_STORE.with(|ego_store| ego_store.borrow().wallet_app_get(&wallet_id, &canister_id))?;

    log_add("3 track canister");
    ego_tenant
      .canister_main_track(
        ego_tenant_id,
        wallet_id,
        canister_id,
      );

    Ok(())
  }

  pub fn wallet_canister_untrack<T: TEgoTenant>(
    ego_tenant: T,
    wallet_id: &Principal,
    canister_id: &Principal,
  ) -> Result<(), EgoError> {
    log_add("1 get ego tenant id");
    let ego_tenant_id =
      EGO_STORE.with(|ego_store| ego_store.borrow().wallet_tenant_get(&wallet_id).clone())?;

    log_add("2 get user app");
    // confirm user app exists
    let _user_app =
      EGO_STORE.with(|ego_store| ego_store.borrow().wallet_app_get(&wallet_id, &canister_id))?;

    log_add("4 untrack canister");

    ego_tenant
      .canister_main_untrack(
        ego_tenant_id,
        wallet_id,
        canister_id,
      );

    Ok(())
  }

  pub fn wallet_order_list(wallet_id: Principal) -> Result<Vec<Order>, EgoError> {
    EGO_STORE.with(|ego_store| ego_store.borrow().wallet_order_list(&wallet_id))
  }

  pub fn wallet_order_list_all() -> Vec<Order> {
    EGO_STORE.with(|ego_store| ego_store.borrow().wallet_order_list_all())
  }

  pub fn wallet_order_new<L: TEgoLedger>(
    ego_ledger: L,
    wallet_id: Principal,
    store_id: Principal,
    amount: f32,
  ) -> Result<Order, EgoError> {
    let order = EGO_STORE.with(|ego_store| {
      ego_store
        .borrow_mut()
        .wallet_order_new(&wallet_id, &store_id, amount)
    })?;
    ego_ledger.ledger_payment_add(&order);
    Ok(order)
  }

  pub fn wallet_cycle_list(wallet_id: Principal) -> Result<Vec<CashFlow>, EgoError> {
    let cash_flows = EGO_STORE.with(|ego_store| {
      ego_store
        .borrow()
        .wallet_cycle_list(&wallet_id)
    })?;
    Ok(cash_flows)
  }

  pub fn wallet_order_notify(memo: Memo, operator: Principal, ts: u64) -> Result<bool, EgoError> {
    EGO_STORE.with(|ego_store| ego_store.borrow_mut().wallet_order_notify(memo, operator, ts))
  }

  pub fn wallet_cycle_charge(
    wallet_id: Principal,
    cycle: u128,
    operator: Principal,
    ts: u64,
    comment: String,
  ) -> Result<bool, EgoError> {
    EGO_STORE.with(|ego_store| {
      ego_store
        .borrow_mut()
        .wallet_cycle_charge(wallet_id, cycle, operator, ts, comment)
    })
  }

  pub fn admin_ego_tenant_add(tenant_id: Principal) {
    EGO_STORE.with(|ego_store| ego_store.borrow_mut().admin_tenant_add(tenant_id))
  }

  pub fn admin_wallet_provider_add(
    wallet_provider: &Principal,
    wallet_id: &AppId,
  ) {
    EGO_STORE.with(|ego_store| {
      ego_store
        .borrow_mut()
        .admin_wallet_provider_add(wallet_provider, wallet_id)
    })
  }

  pub fn admin_wallet_cycle_recharge(
    wallet_id: Principal,
    cycle: u128,
    operator: Principal,
    ts: u64,
    comment: String,
  ) -> Result<bool, EgoError> {
    EGO_STORE.with(|ego_store| {
      ego_store
        .borrow_mut()
        .wallet_cycle_recharge(wallet_id, cycle, operator, ts, comment)
    })
  }

  pub fn app_main_release(app: EgoStoreApp) -> Result<bool, EgoError> {
    EGO_STORE.with(|ego_store| ego_store.borrow_mut().app_main_release(app))
  }

  pub async fn wallet_controller_install<T: TEgoTenant, EC: TEgoCanister>(
    ego_tenant: T,
    ego_canister: EC,
    wallet_provider: Principal,
    user_id: Principal,
    app_id: AppId,
  ) -> Result<UserApp, EgoError> {
    log_add("2 get ego tenant id");
    let ego_tenant_id = EGO_STORE.with(|ego_store| ego_store.borrow_mut().tenant_get())?;

    log_add("3 get app to be install");
    let ego_store_app = EGO_STORE.with(|ego_store| ego_store.borrow().app_main_get(&app_id).clone())?;

    log_add("4 call ego tenant to install code");
    let canister_id = ego_tenant
      .app_main_install(ego_tenant_id, wallet_provider, user_id, &ego_store_app.wasm)
      .await?;

    log_add("5 register wallet to ego_store");
    let _result = EGO_STORE.with(|ego_store| {
      ego_store
        .borrow_mut()
        .wallet_main_register(canister_id, user_id)
    });

    let user_app = UserApp::new(
      &ego_store_app.app,
      Canister::new(canister_id, ego_store_app.wasm.canister_type),
    );

    EGO_STORE.with(|ego_store| {
      ego_store
        .borrow_mut()
        .wallet_app_install(&canister_id, &user_app);
    });

    log_add("6 set app info");
    ego_canister.app_info_update(canister_id, canister_id, ego_store_app.app.app_id, ego_store_app.app.current_version).await?;

    Ok(user_app)
  }
}
