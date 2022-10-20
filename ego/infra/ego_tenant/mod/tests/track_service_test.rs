use async_trait::async_trait;
use candid::Nat;
use ic_cdk::export::Principal;
use mockall::mock;

use ego_tenant_mod::c2c::ic_management::TIcManagement;
use ego_tenant_mod::c2c::ego_store::TEgoStore;
use ego_tenant_mod::service::EgoTenantService;
use ego_tenant_mod::state::EGO_TENANT;
use ego_tenant_mod::task::Task;
use ego_types::ego_error::EgoError;
use ego_utils::ic_management::Cycles;

use ic_cdk::api::management_canister::main::{CanisterStatusResponse, CanisterStatusType};

static EXISTS_WALLET_ID: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";
static EXISTS_CANISTER_ID: &str = "223xb-saaaa-aaaaf-arlqa-cai";

static TEST_WALLET_ID: &str = "22aq5-waaaa-aaaaf-aobwq-cai";
static TEST_CANISTER_ID: &str = "224jh-lqaaa-aaaad-qaxda-cai";


pub fn set_up() {
  let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();
  let canister_principal = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();

  EGO_TENANT.with(|ego_tenant|
    ego_tenant.borrow_mut().tasks.insert(canister_principal,Task::new(wallet_principal, canister_principal))
  );
}

mock! {
  Management {}

  #[async_trait]
  impl TIcManagement for Management {
    async fn canister_main_create(&self, cycles_to_use: Cycles) -> Result<Principal, EgoError>;
    async fn canister_code_install(&self, canister_id: Principal, wasm_module: Vec<u8>) -> Result<(), EgoError>;
    async fn canister_code_upgrade(&self, canister_id: Principal, wasm_module: Vec<u8>) -> Result<(), EgoError>;

    async fn canister_controller_set(&self, canister_id: Principal, user_ids: Vec<Principal>) -> Result<(), EgoError>;

    async fn canister_owner_set(&self, canister_id: Principal, user_id: Principal) -> Result<(), EgoError>;

    async fn canister_status_get(&self, canister_id: Principal) -> Result<CanisterStatusResponse, EgoError>;
    async fn canister_cycle_top_up(&self, canister_id: Principal, cycles_to_use: Cycles) -> Result<(), EgoError>;
  }
}

mock! {
  Store {}

  #[async_trait]
  impl TEgoStore for Store {
    async fn wallet_cycle_charge(&self, store_id: Principal, wallet_id: Principal, cycle: u128) -> Result<bool, EgoError>;
  }
}

#[test]
fn canister_main_track() {
  set_up();

  let wallet_principal = Principal::from_text(TEST_WALLET_ID.to_string()).unwrap();
  let canister_principal = Principal::from_text(TEST_CANISTER_ID.to_string()).unwrap();

  let ret = EgoTenantService::canister_main_track(wallet_principal, canister_principal);
  assert!(ret.is_ok())
}

#[tokio::test]
async fn canister_cycles_check_first_time(){
  set_up();

  let canister_principal = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();

  let sentinel = 10u64;
  let cycle = 1_000_000u128;

  let task = EGO_TENANT.with(|ego_tenant|
    ego_tenant.borrow().tasks.get(&canister_principal).unwrap().clone()
  );

  assert_eq!(0, task.last_check_time);
  assert_eq!(0, task.last_cycle);

  let mut management = MockManagement::new();
  let ego_store = MockStore::new();

  management.expect_canister_status_get().times(1).returning(move |canister_id| {
    assert_eq!(canister_principal, canister_id);
    let cycles = Nat::from(cycle);

    let status = CanisterStatusResponse{
      status: CanisterStatusType::Running,
      settings: Default::default(),
      module_hash: None,
      memory_size: Default::default(),
      cycles,
      idle_cycles_burned_per_day: Default::default()
    };
    Ok(status)
  });

  let _result = EgoTenantService::canister_cycles_check(management, ego_store, sentinel).await;

  let task = EGO_TENANT.with(|ego_tenant|
    ego_tenant.borrow().tasks.get(&canister_principal).unwrap().clone()
  );

  assert_eq!(sentinel + 1000 * 60 * 30, task.next_check_time);
  assert_eq!(0, task.last_check_time);
  assert_eq!(1_000_000, task.last_cycle);
}

// zero cycle consumption between two check times
#[tokio::test]
async fn canister_cycles_check_second_time_zero_cycle_consumption(){
  set_up();

  let canister_principal = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();

  let sentinel = 10u64;
  let cycle = 1_000_000u128;

  EGO_TENANT.with(|ego_tenant|
    ego_tenant.borrow_mut().task_update(canister_principal, cycle, sentinel)
  );

  let task = EGO_TENANT.with(|ego_tenant|
    ego_tenant.borrow().tasks.get(&canister_principal).unwrap().clone()
  );

  assert_eq!(0, task.last_check_time);
  assert_eq!(sentinel, task.next_check_time);
  assert_eq!(cycle, task.last_cycle);

  let mut management = MockManagement::new();
  let ego_store = MockStore::new();

  management.expect_canister_status_get().times(1).returning(move |canister_id| {
    assert_eq!(canister_principal, canister_id);
    let cycles = Nat::from(cycle);

    let status = CanisterStatusResponse{
      status: CanisterStatusType::Running,
      settings: Default::default(),
      module_hash: None,
      memory_size: Default::default(),
      cycles,
      idle_cycles_burned_per_day: Default::default()
    };
    Ok(status)
  });

  let _result = EgoTenantService::canister_cycles_check(management, ego_store, sentinel).await;

  let task = EGO_TENANT.with(|ego_tenant|
    ego_tenant.borrow().tasks.get(&canister_principal).unwrap().clone()
  );

  assert_eq!(sentinel, task.last_check_time);
  assert_eq!(sentinel + 1000 * 60 * 30, task.next_check_time);
  assert_eq!(1_000_000, task.last_cycle);
}

// zero cycle consumption between two check times
#[tokio::test]
async fn canister_cycles_check_second_time_none_zero_cycle_consumption(){
  set_up();

  let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();
  let canister_principal = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();

  let sentinel = 10u64;
  let cycle = 1_000_000u128;

  EGO_TENANT.with(|ego_tenant|
    ego_tenant.borrow_mut().task_update(canister_principal, 2 * cycle, sentinel)
  );

  let task = EGO_TENANT.with(|ego_tenant|
    ego_tenant.borrow().tasks.get(&canister_principal).unwrap().clone()
  );

  assert_eq!(0, task.last_check_time);
  assert_eq!(sentinel, task.next_check_time);
  assert_eq!(2 * cycle, task.last_cycle);

  let mut management = MockManagement::new();
  let mut ego_store = MockStore::new();

  management.expect_canister_status_get().times(1).returning(move |canister_id| {
    assert_eq!(canister_principal, canister_id);
    let cycles = Nat::from(cycle);

    let status = CanisterStatusResponse{
      status: CanisterStatusType::Running,
      settings: Default::default(),
      module_hash: None,
      memory_size: Default::default(),
      cycles,
      idle_cycles_burned_per_day: Default::default()
    };
    Ok(status)
  });

  ego_store.expect_wallet_cycle_charge().returning(move |_store_id, wallet_id, cycle| {
    assert_eq!(wallet_principal, wallet_id);
    assert_eq!(180000000000, cycle);
    Ok(true)
  });

  management.expect_canister_cycle_top_up().returning(move |canister_id, cycle| {
    assert_eq!(canister_principal, canister_id);
    assert_eq!(180000000000, cycle);
    Ok(())
  });

  let _result = EgoTenantService::canister_cycles_check(management, ego_store, sentinel).await;

  let task = EGO_TENANT.with(|ego_tenant|
    ego_tenant.borrow().tasks.get(&canister_principal).unwrap().clone()
  );

  assert_eq!(sentinel, task.last_check_time);
  assert_eq!(sentinel + 1800000, task.next_check_time);
  assert_eq!(180_000_000_000 + 1_000_000, task.last_cycle);
}
