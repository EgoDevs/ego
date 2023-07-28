use candid::Principal;
use ic_ledger_types::Memo;

use ego_lib::ego_canister::TEgoCanister;
use ego_types::app::{App, AppId, Canister};
use ego_types::app::EgoError;

use crate::c2c::ego_ledger::TEgoLedger;
use crate::c2c::ego_tenant::TEgoTenant;
use crate::state::{error_log_add, info_log_add};
use crate::types::EgoStoreErr;
use crate::types::cash_flow::CashFlow;
use crate::types::ego_store_app::EgoStoreApp;
use crate::types::order::{Order, OrderStatus};
use crate::types::tenant::Tenant;
use crate::types::user_app::UserApp;
use crate::types::wallet::Wallet;

pub struct EgoStoreService {}

impl EgoStoreService {
  pub fn app_main_list() -> Result<Vec<App>, EgoError> {
    let apps = EgoStoreApp::list().iter().map(|ego_store_app| ego_store_app.app.clone()).collect();
    Ok(apps)
  }

  pub fn app_main_get(app_id: &AppId) -> Result<EgoStoreApp, EgoError> {
    EgoStoreApp::get(app_id).ok_or(EgoError::from(EgoStoreErr::AppNotExists))
  }

  pub fn wallet_main_get(
    wallet_id: &Principal,
  ) -> Result<Wallet, EgoError> {
    Wallet::get(&wallet_id).ok_or(EgoError::from(EgoStoreErr::WalletNotExists))
  }

  pub fn wallet_main_register(
    wallet_id: &Principal,
    user_id: &Principal,
  ) -> Result<Principal, EgoError> {
    let ego_tenant_id = EgoStoreService::tenant_get()?;

    let mut wallet = Wallet::new(&ego_tenant_id, wallet_id, user_id);
    wallet.save();
    Ok(ego_tenant_id)
  }

  pub fn wallet_app_list(wallet_id: &Principal) -> Vec<UserApp> {
    UserApp::by_wallet_id(wallet_id)
  }

  pub fn wallet_app_get(
    wallet_id: &Principal,
    canister_id: &Principal,
  ) -> Result<UserApp, EgoError> {
    let user_app = UserApp::get(canister_id).ok_or(EgoError::from(EgoStoreErr::AppNotExists))?;
    match user_app.wallet_id.is_some() && user_app.wallet_id.unwrap() == *wallet_id {
      true => {
        Ok(user_app)
      }
      false => {
        error_log_add("wallet_app_get: app not exists");
        Err(EgoStoreErr::AppNotExists.into())
      }
    }
  }

  pub async fn wallet_app_install<T: TEgoTenant, EC: TEgoCanister>(
    ego_tenant: T,
    ego_canister: EC,
    wallet_id: &Principal,
    ego_store_app: &EgoStoreApp,
  ) -> Result<UserApp, EgoError> {
    info_log_add("3 get wallet");
    let mut wallet = EgoStoreService::wallet_main_get(wallet_id)?;

    info_log_add("4 get ego_tenant_id relative to wallet");
    let ego_tenant_id = wallet.tenant_id;

    info_log_add("5 call ego tenant to install wasm");

    let canister_id = ego_tenant
      .app_main_install(
        ego_tenant_id,
        wallet_id.clone(),
        wallet.user_id,
        &ego_store_app.wasm,
      )
      .await?;

    let mut user_app = UserApp::new(
        &ego_store_app.app,
        Canister::new(canister_id, ego_store_app.wasm.canister_type.clone()),
        Some(wallet_id.clone()),
    );

    wallet.app_install(&mut user_app);

    info_log_add("6 track canister");
    ego_tenant.canister_main_track(ego_tenant_id, &wallet_id, &canister_id);

    info_log_add("7 set app info");
    ego_canister.ego_app_info_update(
      canister_id,
      Some(wallet_id.clone()),
      ego_store_app.app.app_id.clone(),
      ego_store_app.app.current_version,
    );

    Ok(user_app)
  }

  pub async fn wallet_app_upgrade<T: TEgoTenant, EC: TEgoCanister>(
    ego_tenant: T,
    ego_canister: EC,
    wallet_id: &Principal,
    canister_id: &Principal,
  ) -> Result<(), EgoError> {
    info_log_add("1 get user_app to be upgrade");

    let mut user_app = EgoStoreService::wallet_app_get(wallet_id, canister_id)?;

    info_log_add("2 get app to be upgrade");
    let ego_store_app = EgoStoreApp::get(&user_app.app.app_id).ok_or(EgoError::from(EgoStoreErr::AppNotExists))?;

    info_log_add(
      format!(
        "3 current version is {:?}, next version is {:?}",
        user_app.app.current_version, ego_store_app.app.current_version
      )
        .as_str(),
    );

    info_log_add("4 get ego tenant id relative to wallet");
    let mut wallet = EgoStoreService::wallet_main_get(wallet_id)?;
    let ego_tenant_id = wallet.tenant_id;

    info_log_add("5 call ego tenant to upgrade canister");
    ego_tenant
      .app_main_upgrade(
        ego_tenant_id,
        user_app.canister.canister_id,
        &ego_store_app.wasm,
      )
      .await?;

    wallet.app_upgrade(&mut user_app, &ego_store_app.app.current_version);

    info_log_add("6 set app info");
    ego_canister.ego_app_info_update(
      canister_id.clone(),
      Some(wallet_id.clone()),
      ego_store_app.app.app_id,
      ego_store_app.app.current_version,
    );

    Ok(())
  }

  pub async fn wallet_app_reinstall<T: TEgoTenant, EC: TEgoCanister>(
    ego_tenant: T,
    ego_canister: EC,
    wallet_id: &Principal,
    canister_id: &Principal,
  ) -> Result<(), EgoError> {
    info_log_add("1 get user_app to be reinstall");

    let mut user_app = EgoStoreService::wallet_app_get(wallet_id, canister_id)?;

    info_log_add("2 get app to be reinstall");

    let ego_store_app = EgoStoreApp::get(&user_app.app.app_id).expect("app not exists");

    info_log_add(
      format!(
        "3 current version is {:?}, next version is {:?}",
        user_app.app.current_version, ego_store_app.app.current_version
      )
        .as_str(),
    );

    info_log_add("4 get ego tenant id relative to wallet");
    let mut wallet = EgoStoreService::wallet_main_get(&wallet_id)?;
    let ego_tenant_id = wallet.tenant_id;

    info_log_add("5 call ego tenant to reinstall canister");
    ego_tenant
      .app_main_reinstall(
        ego_tenant_id,
        user_app.canister.canister_id,
        &ego_store_app.wasm,
      )
      .await?;

    wallet.app_upgrade(&mut user_app, &ego_store_app.app.current_version);

    info_log_add("6 set app info");
    ego_canister.ego_app_info_update(
      canister_id.clone(),
      None,
      ego_store_app.app.app_id,
      ego_store_app.app.current_version,
    );

    Ok(())
  }

  pub fn wallet_app_remove<T: TEgoTenant>(
    ego_tenant: T,
    wallet_id: &Principal,
    canister_id: &Principal,
  ) -> Result<(), EgoError> {
    info_log_add("1 get user_app to be delete");

    let user_app = EgoStoreService::wallet_app_get(wallet_id, canister_id).unwrap();

    info_log_add("2 get ego tenant id relative to wallet");
    let mut wallet = EgoStoreService::wallet_main_get(wallet_id)?;
    let ego_tenant_id = wallet.tenant_id;

    info_log_add("3 call ego tenant to delete canister");
    ego_tenant.app_main_delete(ego_tenant_id, &user_app.canister.canister_id);

    info_log_add("4 remove the user app from wallet");
    wallet.app_remove(&user_app);

    Ok(())
  }

  pub fn wallet_canister_track<T: TEgoTenant>(
    ego_tenant: T,
    wallet_id: &Principal,
    canister_id: &Principal,
  ) -> Result<(), EgoError> {
    info_log_add("1 get ego tenant id");
    let wallet = EgoStoreService::wallet_main_get(&wallet_id)?;
    let ego_tenant_id = wallet.tenant_id;

    info_log_add("2 get user app");
    // confirm user app exists
    let _ = EgoStoreService::wallet_app_get(wallet_id, canister_id)?;

    info_log_add("3 track canister");
    ego_tenant.canister_main_track(ego_tenant_id, wallet_id, canister_id);
    Ok(())
  }

  pub fn wallet_canister_untrack<T: TEgoTenant>(
    ego_tenant: T,
    wallet_id: &Principal,
    canister_id: &Principal,
  ) -> Result<(), EgoError> {
    info_log_add("1 get ego tenant id");

    let wallet = EgoStoreService::wallet_main_get(wallet_id)?;

    let ego_tenant_id = wallet.tenant_id;

    info_log_add("2 get user app");
    // confirm user app exists
    let _ = EgoStoreService::wallet_app_get(wallet_id, canister_id)?;
    info_log_add("3 untrack canister");
    ego_tenant.canister_main_untrack(ego_tenant_id, canister_id);

    Ok(())
  }

  pub fn wallet_order_list(wallet_id: &Principal) -> Vec<Order> {
    Order::by_wallet_id(wallet_id)
  }

  pub fn wallet_order_new<L: TEgoLedger>(
    ego_ledger: L,
    wallet_id: &Principal,
    store_id: &Principal,
    amount: f32,
  ) -> Result<Order, EgoError> {
    let _ = EgoStoreService::wallet_main_get(wallet_id)?;

    let mut order = Order::new(wallet_id, store_id, amount);
    order.save();
    ego_ledger.ledger_payment_add(&order);
    Ok(order)
  }

  pub fn wallet_cash_flow_list(wallet_id: &Principal) -> Vec<CashFlow> {
    CashFlow::by_wallet_id(wallet_id)
  }

  pub fn wallet_cycle_balance(wallet_id: &Principal) -> Result<u128, EgoError> {
    let wallet = EgoStoreService::wallet_main_get(wallet_id)?;
    Ok(wallet.cycles)
  }

  pub fn wallet_order_notify(memo: Memo, operator: &Principal) -> Result<(), EgoError> {
    match Order::get(memo) {
      None => {
        error_log_add("wallet_order_notify: order not exists");
        Err(EgoStoreErr::OrderNotExists.into())
      }
      Some(mut order) => {
        order.status = OrderStatus::SUCCESS;
        order.save();

        match Wallet::get(&order.wallet_id) {
          None => {
            error_log_add("wallet_order_notify: wallet not exists");
            Err(EgoStoreErr::WalletNotExists.into())
          }
          Some(mut wallet) => {
            let cycle = (order.amount.clone() * 1_000_000f32) as u128;
            wallet.cycle_recharge(
                cycle,
                operator,
                format!("wallet cycle recharge, order memo {}", memo.0),
            )
          }
        }
      }
    }
  }

  pub fn wallet_cycle_charge(
    wallet_id: &Principal,
    cycle: u128,
    operator: &Principal,
    comment: String,
  ) -> Result<(), EgoError> {
    if cycle > 0 {
      let mut wallet = EgoStoreService::wallet_main_get(wallet_id)?;
      wallet.cycle_charge(cycle, operator, comment)
    } else {
      Err(EgoStoreErr::CyclesNotEnouth.into())
    }
  }

  pub fn admin_wallet_cycle_recharge(
    wallet_id: &Principal,
    cycle: u128,
    operator: &Principal,
    comment: String,
  ) -> Result<(), EgoError> {
    info_log_add(
      format!(
        "admin_wallet_cycle_recharge operator:{}, cycle:{}",
        operator, cycle
      )
        .as_str(),
    );
    if cycle > 0 {
      let mut wallet = EgoStoreService::wallet_main_get(wallet_id)?;
      wallet.cycle_recharge(cycle, operator, comment)
    } else {
      Ok(())
    }
  }

  pub fn app_main_release(ego_store_app: &mut EgoStoreApp) -> Result<bool, EgoError> {
    ego_store_app.save();
    Ok(true)
  }

  pub async fn wallet_controller_install<T: TEgoTenant, EC: TEgoCanister>(
    ego_tenant: T,
    ego_canister: EC,
    wallet_provider: Principal,
    user_id: Principal,
    app_id: AppId,
  ) -> Result<UserApp, EgoError> {
    info_log_add("2 get ego tenant id");
    let ego_tenant_id = EgoStoreService::tenant_get()?;

    info_log_add("3 get app to be install");
    let ego_store_app = EgoStoreApp::get(&app_id).ok_or(EgoError::from(EgoStoreErr::AppNotExists))?;

    info_log_add(format!("4 call ego tenant {} to install code", ego_tenant_id).as_str());
    let canister_id = ego_tenant
      .app_main_install(ego_tenant_id, wallet_provider, user_id, &ego_store_app.wasm)
      .await?;

    info_log_add(format!("5 register wallet {}, to ego_store", canister_id).as_str());
    let _result = EgoStoreService::wallet_main_register(&canister_id, &user_id);

    let mut user_app = UserApp::new(
        &ego_store_app.app,
        Canister::new(canister_id, ego_store_app.wasm.canister_type), Some(wallet_provider),
    );

    let mut wallet = EgoStoreService::wallet_main_get(&canister_id)?;
    wallet.app_install(&mut user_app);

    info_log_add("7 track canister");
    ego_tenant.canister_main_track(ego_tenant_id, &canister_id, &canister_id);

    info_log_add("8 set app info");
    ego_canister.ego_app_info_update(
      canister_id,
      Some(canister_id),
      ego_store_app.app.app_id,
      ego_store_app.app.current_version,
    );

    Ok(user_app)
  }

  pub fn wallet_user_apps_track<T: TEgoTenant>(
    ego_tenant: T,
    wallet_id: &Principal,
  ) -> Result<(), EgoError> {
    info_log_add("1 get ego tenant id");
    let wallet = EgoStoreService::wallet_main_get(wallet_id)?;

    let ego_tenant_id = wallet.tenant_id;

    info_log_add("2 track user_apps");
    let user_apps = EgoStoreService::wallet_app_list(&wallet_id);

    user_apps.iter().for_each(|user_app| {
      ego_tenant.canister_main_track(
        ego_tenant_id,
        &wallet_id,
        &user_app.canister.canister_id,
      );
    });

    Ok(())
  }

  pub fn admin_wallet_app_transfer(
    new_wallet_id: &Principal,
    canister_id: &Principal,
  ) -> Result<(), EgoError> {
    match UserApp::get(canister_id) {
      None => Err(EgoError::from(format!("user app {} not exists", canister_id))),
      Some(mut user_app) => {
        user_app.wallet_id = Some(new_wallet_id.clone());
        user_app.save();
        Ok(())
      }
    }
  }

  pub fn tenant_get() -> Result<Principal, EgoError> {
    let tenants = Tenant::list();

    if tenants.len() == 0 {
      error_log_add("tenant_get: no tenant");
      Err(EgoStoreErr::NoTenant.into())
    } else {
      let mut tenant = tenants.iter().min().unwrap().clone();
      tenant.wallet_count += 1;
      tenant.save();
      Ok(tenant.canister_id)
    }
  }
}
