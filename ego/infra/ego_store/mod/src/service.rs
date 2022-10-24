use ic_cdk::export::Principal;
use ic_ledger_types::Memo;

use ego_types::app::{App, AppId, Canister, CanisterType};
use ego_types::ego_error::EgoError;
use crate::app::EgoStoreApp;

use crate::c2c::ego_tenant::TEgoTenant;
use crate::order::Order;
use crate::state::EGO_STORE;
use crate::types::{EgoStoreErr, QueryParam};
use crate::user_app::{AppInstalled, UserApp};

pub struct EgoStoreService {}

impl EgoStoreService {
  pub fn app_main_list(query_param: QueryParam) -> Result<Vec<App>, EgoError> {
    EGO_STORE.with(|ego_store| ego_store.borrow().app_main_list(&query_param))
  }

  pub fn app_main_get(app_id: AppId) -> Result<EgoStoreApp, EgoError> {
    EGO_STORE.with(
      |ego_store| ego_store.borrow().app_main_get(&app_id)
    )
  }

  pub fn wallet_main_register(
    wallet_id: Principal,
    user_id: Principal
  ) -> Result<Principal, EgoError> {
    EGO_STORE.with(|ego_store| {
      ego_store
        .borrow_mut().wallet_main_register(wallet_id, user_id)
    })
  }

  pub fn wallet_tenant_get(wallet_id: Principal) -> Result<Principal, EgoError> {
    EGO_STORE.with(|ego_store| {
      ego_store
        .borrow().wallet_tenant_get(&wallet_id)
    })
  }

  pub fn wallet_app_list(wallet_id: Principal) -> Result<Vec<AppInstalled>, EgoError> {
    EGO_STORE.with(|ego_store| {
      ego_store
        .borrow().wallet_app_list(&wallet_id)
    })
  }

  pub async fn wallet_app_install<T: TEgoTenant>(ego_tenant: T, wallet_id: Principal, app_id: AppId) -> Result<UserApp, EgoError> {
    EGO_STORE.with(|ego_store| {
      match ego_store.borrow().wallets.get(&wallet_id) {
        None => Err(EgoError::from(EgoStoreErr::WalletNotExists)),
        Some(wallet) => {
          match wallet.apps.get(&app_id) {
            None => Ok(()),
            Some(_user_app) => {
              Err(EgoError::from(EgoStoreErr::AppAlreadyInstall))
            }
          }
        }
      }
    })?;

    ic_cdk::println!("1 get ego tenant id relative to wallet");
    let ego_tenant_id = EGO_STORE.with(|ego_store| {
      ego_store
        .borrow().wallet_tenant_get(&wallet_id).clone()
    })?;

    ic_cdk::println!("2 get app to be install");
    let app = EGO_STORE.with(|ego_store| {
      ego_store
        .borrow().app_main_get(&app_id).clone()
    })?;

    ic_cdk::println!("3 get wallet");
    let wallet = EGO_STORE.with(|ego_store| {
      ego_store
        .borrow().wallet_main_get(wallet_id)
    })?;

    ic_cdk::println!("4 call ego tenant to install frontend");
    let frontend_canister = match app.frontend.is_some() {
      false => {
        None
      }
      true => {
        let frontend_canister_id = ego_tenant.app_main_install(ego_tenant_id, wallet_id, wallet.user_id, app.frontend.unwrap()).await?;
        Some(Canister::new(frontend_canister_id, CanisterType::ASSET))
      }
    };

    ic_cdk::println!("5 call ego tenant to install backend");
    let backend_canister = match app.backend.is_some() {
      false => {
        None
      }
      true => {
        let backend_canister_id = ego_tenant.app_main_install(ego_tenant_id, wallet_id, wallet.user_id, app.backend.unwrap()).await?;
        Some(Canister::new(backend_canister_id, CanisterType::BACKEND))
      }
    };

    let user_app = UserApp::new(&app.app_id, &app.current_version, frontend_canister, backend_canister);

    EGO_STORE.with(|ego_store| {
      ego_store
        .borrow_mut().wallet_app_install(&wallet_id, &app_id, &user_app);
    });

    Ok(user_app)
  }

  pub async fn wallet_app_upgrade<T: TEgoTenant>(ego_tenant: T, wallet_id: Principal, app_id: AppId) -> Result<UserApp, EgoError> {
    ic_cdk::println!("1 get previous installed user app");
    let user_app = EGO_STORE.with(|ego_store| {
      ego_store
        .borrow().user_app_get(&wallet_id, &app_id)
    })?;

    ic_cdk::println!("2 get ego tenant id relative to wallet");
    let ego_tenant_id = EGO_STORE.with(|ego_store| {
      ego_store
        .borrow().wallet_tenant_get(&wallet_id).clone()
    })?;

    ic_cdk::println!("3 get app to be upgrade");
    let app = EGO_STORE.with(|ego_store| {
      ego_store
        .borrow().app_main_get(&app_id).clone()
    })?;

    // TODO: 假设不同版本里面的app wasm一致，例如：不存在原来有前端后来没有了的情况
    ic_cdk::println!("4 call ego tenant to upgrade frontend");
    if app.frontend.is_some() {
      ego_tenant.app_main_upgrade(ego_tenant_id, user_app.frontend.as_ref().unwrap().canister_id, app.frontend.unwrap()).await?;
    }

    ic_cdk::println!("5 call ego tenant to upgrade backend");
    if app.backend.is_some() {
      ego_tenant.app_main_upgrade(ego_tenant_id, user_app.backend.as_ref().unwrap().canister_id, app.backend.unwrap()).await?;
    }

    EGO_STORE.with(|ego_store| {
      ego_store
        .borrow_mut().wallet_app_upgrade(&wallet_id, &app_id, &app.current_version);
    });

    Ok(user_app)
  }

  pub fn wallet_app_remove(wallet_id: Principal, app_id: AppId) -> Result<(), EgoError> {
    EGO_STORE.with(|ego_store| {
      match ego_store.borrow().wallets.get(&wallet_id) {
        None => Err(EgoError::from(EgoStoreErr::WalletNotExists)),
        Some(wallet) => {
          match wallet.apps.get(&app_id) {
            None => Err(EgoError::from(EgoStoreErr::AppNotInstall)),
            Some(_user_app) => Ok(())
          }
        }
      }
    })?;

    EGO_STORE.with(|ego_store| {
      ego_store
        .borrow_mut().wallet_app_remove(&wallet_id, &app_id)
    })
  }

  pub fn wallet_order_list(wallet_id: Principal) -> Result<Vec<Order>, EgoError> {
    EGO_STORE.with(|ego_store| {
      ego_store
        .borrow().wallet_order_list(&wallet_id)
    })
  }

  pub fn wallet_order_new(wallet_id: Principal, store_id: Principal, amount: f32) -> Result<Order, EgoError> {
    EGO_STORE.with(|ego_store| {
      ego_store
        .borrow_mut().wallet_order_new(&wallet_id, &store_id, amount)
    })
  }

  pub fn wallet_order_notify(memo: Memo, operator: Principal) -> Result<bool, EgoError> {
    EGO_STORE.with(|ego_store| {
      ego_store.borrow_mut().wallet_order_notify(memo, operator)
    })
  }

  pub fn wallet_cycle_charge(wallet_id: Principal, cycle: u128, operator: Principal, comment: String) -> Result<bool, EgoError> {
    EGO_STORE.with(|ego_store| {
      ego_store.borrow_mut().wallet_cycle_charge(wallet_id, cycle, operator, comment)
    })
  }

  pub fn admin_ego_tenant_add(tenant_id: Principal) -> Result<bool, EgoError> {
    EGO_STORE.with(|ego_store| {
      ego_store
        .borrow_mut().admin_tenant_add(&tenant_id)
    })
  }

  pub fn admin_wallet_provider_add(wallet_provider: &Principal, wallet_id: &AppId) -> Result<bool, EgoError> {
    EGO_STORE.with(|ego_store| {
      ego_store
        .borrow_mut().admin_wallet_provider_add(wallet_provider, wallet_id)
    })
  }

  pub fn admin_wallet_cycle_recharge(wallet_id: Principal, cycle: u128, operator: Principal, comment: String) -> Result<bool, EgoError> {
    EGO_STORE.with(|ego_store| {
      ego_store.borrow_mut().wallet_cycle_recharge(wallet_id, cycle, operator, comment)
    })
  }

  pub fn app_main_release(app: EgoStoreApp) -> Result<bool, EgoError> {
    EGO_STORE.with(|ego_store| {
      ego_store
        .borrow_mut().app_main_release(app)
    })
  }

  pub async fn wallet_controller_install<T: TEgoTenant>(ego_tenant: T, user_id: Principal, app_id: AppId) -> Result<UserApp, EgoError> {
    ic_cdk::println!("1 get ego tenant id");
    let ego_tenant_id = EGO_STORE.with(|ego_store| {
      ego_store.borrow_mut().tenant_get()
    })?;

    ic_cdk::println!("2 get app to be install");
    let app = EGO_STORE.with(|ego_store| {
      ego_store
        .borrow().app_main_get(&app_id).clone()
    })?;

    ic_cdk::println!("3 call ego tenant to install frontend");
    let frontend_canister = match app.frontend.is_some() {
      false => {
        None
      }
      true => {
        let frontend_canister_id = ego_tenant.app_main_install(ego_tenant_id, user_id, user_id, app.frontend.unwrap()).await?;
        Some(Canister::new(frontend_canister_id, CanisterType::ASSET))
      }
    };

    ic_cdk::println!("4 call ego tenant to install backend");
    let backend_canister = match app.backend.is_some() {
      false => {
        None
      }
      true => {
        let backend_canister_id = ego_tenant.app_main_install(ego_tenant_id, user_id, user_id, app.backend.unwrap()).await?;
        Some(Canister::new(backend_canister_id, CanisterType::BACKEND))
      }
    };

    let user_app = UserApp::new(&app.app_id, &app.current_version, frontend_canister, backend_canister);

    Ok(user_app)
  }
}