use candid::Principal;

use ego_store_mod::backup::{job_list, record_export};
use ego_store_mod::state::{canister_add, owner_add};
use ego_store_mod::types::cash_flow::CashFlow;
use ego_store_mod::types::ego_store_app::EgoStoreApp;
use ego_store_mod::types::order::Order;
use ego_store_mod::types::stable_state::StableState;
use ego_store_mod::types::tenant::Tenant;
use ego_store_mod::types::user_app::UserApp;
use ego_store_mod::types::wallet::Wallet;
use ego_store_mod::types::wallet_provider::WalletProvider;
use ego_types::app::{App, Canister, CashFlowType, Category, Version, Wasm};
use ego_types::app::CanisterType::BACKEND;
use ego_utils::util::time;

static EXISTS_CANISTER_ID: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";
static EXISTS_USER_ID: &str = "225da-yaaaa-aaaah-qahrq-cai";

static FILE_CANISTER_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";
static EXISTS_APP_ID: &str = "app_exists";
static APP_NAME: &str = "app1";
static APP_LOGO: &str = "logo";
static APP_DESCRIPTION: &str = "test is app description";

static TENANT_ID1: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";

static WALLET_PROVIDER_ID: &str = "2265i-mqaaa-aaaad-qbsga-cai";
static WALLET_ID1: &str = "23vqh-waaaa-aaaai-qhcya-cai";
static USER_ID1: &str = "o2ivq-5dsz3-nba5d-pwbk2-hdd3i-vybeq-qfz35-rqg27-lyesf-xghzc-3ae";

static USER_APP_CANISTER_ID1: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";

static STORE: &str = "225da-yaaaa-aaaah-qahrq-cai";

static OPERATOR: &str = "wtb37-uyaaa-aaaai-qa3zq-cai";

fn set_up() {
  owner_add(Principal::from_text(EXISTS_USER_ID.to_string()).unwrap());
  canister_add("test".to_string(), Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap());

  // add exists app
  let file_canister = Principal::from_text(FILE_CANISTER_ID.to_string()).unwrap();
  let version = Version::new(1, 0, 1);

  let wasm = Wasm::new(EXISTS_APP_ID.to_string(), version, BACKEND, file_canister);

  let app = App {
    app_id: EXISTS_APP_ID.to_string(),
    name: APP_NAME.to_string(),
    category: Category::Vault,
    logo: APP_LOGO.to_string(),
    description: APP_DESCRIPTION.to_string(),
    current_version: version,
    price: 0.0,
    app_hash: "".to_string(),
  };

  let mut ego_store_app = EgoStoreApp::new(&app, &wasm);
  ego_store_app.save();

  // add tenant
  let tenant_id = Principal::from_text(TENANT_ID1.to_string()).unwrap();
  let tenant = Tenant::new(&tenant_id);
  tenant.save();

  // add wallet provider
  let wallet_provider_id = Principal::from_text(WALLET_PROVIDER_ID.to_string()).unwrap();
  let wallet_provider = WalletProvider::new(&wallet_provider_id, &APP_NAME.to_string());
  wallet_provider.save();

  // add wallet
  let wallet_id = Principal::from_text(WALLET_ID1).unwrap();
  let user_id = Principal::from_text(USER_ID1).unwrap();

  let mut wallet = Wallet::new(&tenant_id, &wallet_id, &user_id);
  wallet.save();

  // add user app
  let user_app_canister_id1 = Principal::from_text(USER_APP_CANISTER_ID1.to_string()).unwrap();
  let canister = Canister::new(user_app_canister_id1, BACKEND);
  let mut user_app = UserApp::new(&app, &canister, Some(wallet_id));
  user_app.save();

  // add order
  let store = Principal::from_text(STORE.to_string()).unwrap();
  let mut order = Order::new(&wallet_id, &store, 10 as f32);
  order.save();

  // add cash flow
  let operator = Principal::from_text(OPERATOR.to_string()).unwrap();
  let mut cash_flow1 = CashFlow::new(&wallet_id, CashFlowType::RECHARGE, 1, 1, &operator, "recharge".to_string());
  cash_flow1.save();
}

#[test]
fn test_job_list() {
  set_up();

  let jobs = job_list();
  assert_eq!(8, jobs.len());

  assert_eq!("config", jobs.get(0).unwrap().name);
  assert_eq!(1, jobs.get(0).unwrap().amount);

  assert_eq!("ego_store_apps", jobs.get(1).unwrap().name);
  assert_eq!(1, jobs.get(1).unwrap().amount);

  assert_eq!("tenants", jobs.get(2).unwrap().name);
  assert_eq!(1, jobs.get(2).unwrap().amount);

  assert_eq!("wallet_providers", jobs.get(3).unwrap().name);
  assert_eq!(1, jobs.get(3).unwrap().amount);

  assert_eq!("wallets", jobs.get(4).unwrap().name);
  assert_eq!(1, jobs.get(4).unwrap().amount);

  assert_eq!("user_apps", jobs.get(5).unwrap().name);
  assert_eq!(1, jobs.get(5).unwrap().amount);

  assert_eq!("orders", jobs.get(6).unwrap().name);
  assert_eq!(1, jobs.get(6).unwrap().amount);

  assert_eq!("cash_flows", jobs.get(7).unwrap().name);
  assert_eq!(1, jobs.get(7).unwrap().amount);
}

#[test]
fn test_export_config() {
  set_up();

  let result = record_export("config".to_string(), 0, 1000, 0).expect("record not founded");
  assert_eq!("config", result.name);
  let stable_state: StableState = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(1, stable_state.seq.clone().unwrap().get_number("order").unwrap());
  assert_eq!(1, stable_state.seq.clone().unwrap().get_number("cash_flow").unwrap());
}

#[test]
fn test_export() {
  set_up();

  let result = record_export("ego_store_apps".to_string(), 0, 1000, 0).expect("record not founded");
  assert_eq!("ego_store_apps", result.name);
  let ego_store_apps: Vec<EgoStoreApp> = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(1, ego_store_apps.len());
  assert_eq!(EXISTS_APP_ID, ego_store_apps.get(0).unwrap().app.app_id);

  let result = record_export("tenants".to_string(), 0, 1000, 0).expect("record not founded");
  assert_eq!("tenants", result.name);
  let tenants: Vec<Tenant> = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(1, tenants.len());
  assert_eq!(TENANT_ID1, tenants.get(0).unwrap().canister_id.to_string());

  let result = record_export("wallet_providers".to_string(), 0, 1000, 0).expect("record not founded");
  assert_eq!("wallet_providers", result.name);
  let wallet_providers: Vec<WalletProvider> = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(1, wallet_providers.len());
  assert_eq!(WALLET_PROVIDER_ID, wallet_providers.get(0).unwrap().wallet_provider.to_string());

  let result = record_export("wallets".to_string(), 0, 1000, 0).expect("record not founded");
  assert_eq!("wallets", result.name);
  let wallets: Vec<Wallet> = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(1, wallets.len());
  assert_eq!(WALLET_ID1, wallets.get(0).unwrap().wallet_id.to_string());

  let result = record_export("user_apps".to_string(), 0, 1000, 0).expect("record not founded");
  assert_eq!("user_apps", result.name);
  let user_apps: Vec<UserApp> = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(1, user_apps.len());
  assert_eq!(WALLET_ID1, user_apps.get(0).unwrap().wallet_id.unwrap().to_string());

  let result = record_export("orders".to_string(), 0, 1000, 0).expect("record not founded");
  assert_eq!("orders", result.name);
  let orders: Vec<Order> = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(1, orders.len());
  assert_eq!(WALLET_ID1, orders.get(0).unwrap().wallet_id.to_string());

  let result = record_export("cash_flows".to_string(), 0, 1000, 0).expect("record not founded");
  assert_eq!("cash_flows", result.name);
  let cash_flows: Vec<CashFlow> = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(1, cash_flows.len());
  assert_eq!(WALLET_ID1, cash_flows.get(0).unwrap().wallet_id.to_string());
}

#[test]
fn test_export_with_last_update() {
  set_up();

  let last_update = time() + 100;

  let result = record_export("ego_store_apps".to_string(), 0, 1000, last_update).expect("record not founded");
  assert_eq!("ego_store_apps", result.name);
  let ego_store_apps: Vec<EgoStoreApp> = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(0, ego_store_apps.len());

  let result = record_export("tenants".to_string(), 0, 1000, last_update).expect("record not founded");
  assert_eq!("tenants", result.name);
  let tenants: Vec<Tenant> = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(1, tenants.len());
  assert_eq!(TENANT_ID1, tenants.get(0).unwrap().canister_id.to_string());

  let result = record_export("wallet_providers".to_string(), 0, 1000, last_update).expect("record not founded");
  assert_eq!("wallet_providers", result.name);
  let wallet_providers: Vec<WalletProvider> = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(1, wallet_providers.len());
  assert_eq!(WALLET_PROVIDER_ID, wallet_providers.get(0).unwrap().wallet_provider.to_string());

  let result = record_export("wallets".to_string(), 0, 1000, last_update).expect("record not founded");
  assert_eq!("wallets", result.name);
  let wallets: Vec<Wallet> = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(0, wallets.len());

  let result = record_export("user_apps".to_string(), 0, 1000, last_update).expect("record not founded");
  assert_eq!("user_apps", result.name);
  let user_apps: Vec<UserApp> = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(0, user_apps.len());

  let result = record_export("orders".to_string(), 0, 1000, last_update).expect("record not founded");
  assert_eq!("orders", result.name);
  let orders: Vec<Order> = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(0, orders.len());

  let result = record_export("cash_flows".to_string(), 0, 1000, last_update).expect("record not founded");
  assert_eq!("cash_flows", result.name);
  let cash_flows: Vec<CashFlow> = serde_json::from_slice(&result.data).unwrap();
  assert_eq!(0, cash_flows.len());
}