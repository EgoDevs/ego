use async_trait::async_trait;
use ic_cdk::export::Principal;
use ic_ledger_types::Memo;
use mockall::mock;

use ego_store_mod::order::Order;
use ego_store_mod::service::EgoStoreService;
use ego_store_mod::state::EGO_STORE;
use ego_store_mod::wallet::Wallet;
use ego_types::app::Wasm;
use ego_types::ego_error::EgoError;
use ego_store_mod::c2c::ego_tenant::TEgoTenant;

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
    async fn canister_main_track(
        &self,
        ego_tenant_id: Principal,
        wallet_id: Principal,
        canister_id: Principal,
    ) -> Result<(), EgoError>;
    async fn canister_main_untrack(
        &self,
        ego_tenant_id: Principal,
        wallet_id: Principal,
        canister_id: Principal,
    ) -> Result<(), EgoError>;
  }
}


static WALLET_PROVIDER_ID: &str = "2265i-mqaaa-aaaad-qbsga-cai";
static WALLET_APP_ID: &str = "app_exists";
static EXISTS_WALLET_ID: &str = "23vqh-waaaa-aaaai-qhcya-cai";
static TENANT_ID: &str = "2avdy-paaaa-aaaaf-abcga-cai";
static STORE_ID: &str = "22cl3-kqaaa-aaaaf-add7q-cai";
static EXISTS_USER_ID: &str = "225da-yaaaa-aaaah-qahrq-cai";

pub fn set_up() {
  let tenant_principal = Principal::from_text(TENANT_ID.to_string()).unwrap();
  let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();
  let user_principal = Principal::from_text(EXISTS_USER_ID.to_string()).unwrap();
  let store_principal = Principal::from_text(STORE_ID.to_string()).unwrap();

  EGO_STORE.with(|ego_store| {
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
  let mut ego_tenant = MockTenant::new();
}
