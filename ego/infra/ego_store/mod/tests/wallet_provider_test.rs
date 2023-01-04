use async_trait::async_trait;
use ic_cdk::export::Principal;
use ic_ledger_types::Memo;
use mockall::mock;

use ego_lib::ego_canister::TEgoCanister;
use ego_store_mod::app::EgoStoreApp;
use ego_store_mod::c2c::ego_tenant::TEgoTenant;
use ego_store_mod::order::Order;
use ego_store_mod::service::EgoStoreService;
use ego_store_mod::state::EGO_STORE;
use ego_store_mod::tenant::Tenant;
use ego_store_mod::wallet::Wallet;
use ego_types::app::{App, AppId, Category, Wasm};
use ego_types::app::CanisterType::BACKEND;
use ego_types::app::EgoError;
use ego_types::app::Version;
use ego_types::app_info::AppInfo;

mock! {
  Tenant {}

  #[async_trait]
  impl TEgoTenant for Tenant {
    async fn app_main_install(
        &self,
        ego_tenant_id: Principal,
        wallet_id: Principal,
        user_id: Principal,
        wasm: &Wasm,
    ) -> Result<Principal, EgoError>;
    async fn app_main_upgrade(
        &self,
        ego_tenant_id: Principal,
        canister_id: Principal,
        wasm: &Wasm,
    ) -> Result<bool, EgoError>;
    fn canister_main_track(
        &self,
        ego_tenant_id: Principal,
        wallet_id: &Principal,
        canister_id: &Principal,
    );
    fn canister_main_untrack(
        &self,
        ego_tenant_id: Principal,
        wallet_id: &Principal,
        canister_id: &Principal,
    );
  }
}

mock! {
  Canister {}

  #[async_trait]
  impl TEgoCanister for Canister {
    fn ego_owner_set(&self, target_canister_id: Principal, principals: Vec<Principal>);
    fn ego_owner_add(&self, target_canister_id: Principal, principal: Principal);
    fn ego_owner_remove(&self, target_canister_id: Principal, principal: Principal);

    fn ego_user_set(&self, target_canister_id: Principal, user_ids: Vec<Principal>);
    fn ego_user_add(&self, target_canister_id: Principal, principal: Principal);
    fn ego_user_remove(&self, target_canister_id: Principal, principal: Principal);

    fn ego_op_add(&self, target_canister_id: Principal, user_id: Principal);

    fn ego_canister_add(&self, target_canister_id: Principal, name: String, principal: Principal);

    fn ego_controller_set(&self, target_canister_id: Principal, principals: Vec<Principal>);
    async fn ego_controller_add(&self, target_canister_id: Principal, principal: Principal) -> Result<(), String>;
    fn ego_controller_remove(&self, target_canister_id: Principal, principal: Principal);

    async fn balance_get(&self, target_canister_id: Principal) -> Result<u128, String>;

    // app info
    async fn app_info_update(&self, target_canister_id: Principal, wallet_id: Principal, app_id: AppId, version: Version) -> Result<(), String>;
    async fn app_info_get(&self, target_canister_id: Principal) -> Result<AppInfo, String>;
    async fn app_version_check(&self, target_canister_id: Principal) -> Result<App, String>;

    // canister upgrade
    fn ego_canister_upgrade(&self, target_canister_id: Principal);
  }
}

static FILE_CANISTER_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";
static WALLET_PROVIDER_ID: &str = "2265i-mqaaa-aaaad-qbsga-cai";
static WALLET_APP_ID: &str = "app_exists";
static EXISTS_WALLET_ID: &str = "23vqh-waaaa-aaaai-qhcya-cai";
static TENANT_ID: &str = "2avdy-paaaa-aaaaf-abcga-cai";
static STORE_ID: &str = "22cl3-kqaaa-aaaaf-add7q-cai";
static EXISTS_USER_ID: &str = "o2ivq-5dsz3-nba5d-pwbk2-hdd3i-vybeq-qfz35-rqg27-lyesf-xghzc-3ae";

pub fn set_up() {
  let tenant_principal = Principal::from_text(TENANT_ID.to_string()).unwrap();
  let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();
  let user_principal = Principal::from_text(EXISTS_USER_ID.to_string()).unwrap();
  let store_principal = Principal::from_text(STORE_ID.to_string()).unwrap();
  let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();

  EGO_STORE.with(|ego_store| {
    // add tenant
    ego_store
      .borrow_mut()
      .tenants
      .insert(tenant_principal, Tenant::new(tenant_principal));

    // add exists app
    let version = Version::new(1, 0, 1);

    let wasm = Wasm::new(WALLET_APP_ID.to_string(), version, BACKEND, file_canister);
    let app = App {
      app_id: WALLET_APP_ID.to_string(),
      name: "".to_string(),
      category: Category::Vault,
      logo: "".to_string(),
      description: "".to_string(),
      current_version: version,
      price: 1.2f32,
    };

    let ego_store_app = EgoStoreApp::new(
      app, wasm,
    );
    ego_store
      .borrow_mut()
      .apps
      .insert(WALLET_APP_ID.to_string(), ego_store_app);

    // add wallet
    let mut wallet = Wallet::new(tenant_principal, wallet_principal, user_principal);

    // add order
    let order = Order::new(wallet_principal, &store_principal, 12.5f32, 10);
    ego_store.borrow_mut().orders.insert(Memo(10), order);

    wallet.orders.push(Memo(10));

    ego_store
      .borrow_mut()
      .wallets
      .insert(wallet_principal, wallet);
  });
}

#[test]
fn admin_wallet_provider_add() {
  let wallet_provider = Principal::from_text(WALLET_PROVIDER_ID.to_string()).unwrap();

  // before add
  EGO_STORE.with(|ego_store| {
    assert_eq!(0, ego_store.borrow().wallet_providers.len());
  });

  EgoStoreService::admin_wallet_provider_add(&wallet_provider, &WALLET_APP_ID.to_string());

  // after add
  EGO_STORE.with(|ego_store| {
    assert_eq!(1, ego_store.borrow().wallet_providers.len());
  });
}

#[test]
fn admin_wallet_cycle_recharge_wallet_not_exists() {
  let wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
  let operator = Principal::from_text(WALLET_PROVIDER_ID).unwrap();
  let result = EgoStoreService::admin_wallet_cycle_recharge(
    wallet_id,
    128,
    operator,
    64,
    "comment".to_string(),
  );
  assert!(result.is_err());
  assert_eq!(3006, result.as_ref().unwrap_err().code);
  assert_eq!(
    "ego-store: wallet not exists",
    result.as_ref().unwrap_err().msg
  );
}

#[test]
fn admin_wallet_cycle_recharge() {
  set_up();
  let wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
  let operator = Principal::from_text(EXISTS_USER_ID).unwrap();

  // before recharge
  let cycle_list = EgoStoreService::wallet_cycle_list(wallet_id);
  assert_eq!(0, cycle_list.unwrap().len());

  let result = EgoStoreService::admin_wallet_cycle_recharge(
    wallet_id,
    128,
    operator,
    64,
    "admin wallet cycle recharge".to_string(),
  );
  assert!(result.is_ok());

  // after recharge
  let cycle_list = EgoStoreService::wallet_cycle_list(wallet_id);
  assert_eq!(1, cycle_list.unwrap().len());
}

#[test]
fn admin_ego_tenant_add() {
  let tenant_id = Principal::from_text(TENANT_ID).unwrap();

  // before add
  EGO_STORE.with(|ego_store| {
    assert_eq!(0, ego_store.borrow().tenants.len());
  });

  EgoStoreService::admin_ego_tenant_add(tenant_id);

  // after add
  EGO_STORE.with(|ego_store| {
    assert_eq!(1, ego_store.borrow().tenants.len());
  });
}

#[tokio::test]
async fn wallet_controller_install() {
  set_up();

  let wallet_provider_principal = Principal::from_text(WALLET_PROVIDER_ID.to_string()).unwrap();
  let user_principal = Principal::from_text(EXISTS_USER_ID.to_string()).unwrap();

  let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();


  let mut ego_tenant = MockTenant::new();
  ego_tenant.expect_app_main_install().returning(move |_, w_id, u_id, _| {
    assert_eq!(wallet_provider_principal, w_id);
    assert_eq!(user_principal, u_id);
    Ok(wallet_principal)
  });

  let mut ego_canister = MockCanister::new();
  ego_canister.expect_app_info_update().returning(|_, _, _, _| {
    Ok(())
  });

  let result = EgoStoreService::wallet_controller_install(ego_tenant, ego_canister, wallet_provider_principal, user_principal, WALLET_APP_ID.to_string()).await;
  assert!(result.is_ok());
  let c_id = result.unwrap().canister.canister_id;
  assert_eq!(wallet_principal, c_id);
}
