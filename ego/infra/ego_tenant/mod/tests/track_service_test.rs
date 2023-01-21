use async_trait::async_trait;
use ic_cdk::export::Principal;
use mockall::mock;

use ego_lib::ego_canister::TEgoCanister;
use ego_lib::inject_mock_ego_canister;
use ego_tenant_mod::c2c::ego_store::TEgoStore;
use ego_tenant_mod::c2c::ic_management::TIcManagement;
use ego_tenant_mod::service::{EgoTenantService, NEXT_CHECK_DURATION};
use ego_tenant_mod::state::canister_add;
use ego_tenant_mod::state::EGO_TENANT;
use ego_tenant_mod::task::Task;
use ego_types::app::{App, AppId, Version};
use ego_types::app::EgoError;
use ego_types::app_info::AppInfo;
use ego_types::cycle_info::CycleRecord;
use ego_utils::ic_management::Cycles;

static STORE_CANISTER_ID: &str = "qhbym-qaaaa-aaaaa-aaafq-cai";

static EXISTS_WALLET_ID: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";
static EXISTS_CANISTER_ID: &str = "223xb-saaaa-aaaaf-arlqa-cai";

static TEST_WALLET_ID: &str = "22aq5-waaaa-aaaaf-aobwq-cai";
static TEST_CANISTER_ID: &str = "224jh-lqaaa-aaaad-qaxda-cai";

pub fn set_up() {
  let store_canister_id = Principal::from_text(STORE_CANISTER_ID.to_string()).unwrap();
  canister_add("ego_store".to_string(), store_canister_id);

  let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();
  let canister_principal = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();

  EGO_TENANT.with(|ego_tenant| {
    ego_tenant.borrow_mut().tasks.insert(
      canister_principal,
      Task::new(wallet_principal, canister_principal),
    )
  });
}

mock! {
  Management {}

  #[async_trait]
  impl TIcManagement for Management {
    async fn canister_main_create(&self, cycles_to_use: Cycles) -> Result<Principal, EgoError>;

  async fn canister_code_install(
    &self,
    canister_id: Principal,
    wasm_module: Vec<u8>,
  ) -> Result<(), EgoError>;

  async fn canister_code_upgrade(
    &self,
    canister_id: Principal,
    wasm_module: Vec<u8>,
  ) -> Result<(), EgoError>;

  async fn canister_cycle_top_up(
    &self,
    canister_id: Principal,
    cycles_to_use: Cycles,
  ) -> Result<(), EgoError>;

  async fn canister_main_delete(&self, canister_id: Principal) -> Result<(), EgoError>;

  }
}

mock! {
  Store {}

  #[async_trait]
  impl TEgoStore for Store {
    async fn wallet_cycle_charge(
      &self,
      wallet_id: Principal,
      cycle: u128,
      comment: String,
    ) -> Result<bool, EgoError>;
  }
}

inject_mock_ego_canister!();

#[test]
fn canister_main_track() {
  set_up();

  let wallet_principal = Principal::from_text(TEST_WALLET_ID.to_string()).unwrap();
  let canister_principal = Principal::from_text(TEST_CANISTER_ID.to_string()).unwrap();

  let ret = EgoTenantService::canister_main_track(wallet_principal, canister_principal);
  assert!(ret.is_ok())
}

#[test]
fn canister_main_untrack() {
  set_up();

  let canister_principal = Principal::from_text(TEST_CANISTER_ID.to_string()).unwrap();

  let result = EgoTenantService::canister_main_untrack(canister_principal);
  assert!(result.is_ok());
}

#[tokio::test]
async fn canister_cycles_check_first_time() {
  set_up();

  let canister_principal = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();

  let ts = 10u64;
  let cycle = 1_000_000u128;

  let records = vec![CycleRecord { balance: cycle, ts }];

  let task = EGO_TENANT.with(|ego_tenant| {
    ego_tenant
      .borrow()
      .tasks
      .get(&canister_principal)
      .unwrap()
      .clone()
  });

  let management = MockManagement::new();
  let ego_store = MockStore::new();
  let mut ego_canister = MockCanister::new();

  ego_canister
    .expect_ego_cycle_estimate_set()
    .returning(move |canister_id, estimate| {
      assert_eq!(canister_principal, canister_id);
      assert_eq!(0, estimate);
    });

  let _result = EgoTenantService::ego_cycle_check_cb(
    management,
    ego_store,
    ego_canister,
    &task,
    &canister_principal,
    &records,
    100_000,
  )
    .await;

  let task = EGO_TENANT.with(|ego_tenant| {
    ego_tenant
      .borrow()
      .tasks
      .get(&canister_principal)
      .unwrap()
      .clone()
  });

  assert_eq!(ts + NEXT_CHECK_DURATION, task.next_check_time);
}


// zero cycle consumption between two check times
#[tokio::test]
async fn canister_cycles_check_second_time_zero_cycle_consumption() {
  set_up();

  let canister_principal = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();

  let ts1 = 10u64;
  let ts2 = 20u64;
  let cycle = 1_000_000u128;

  let records = vec![CycleRecord { balance: cycle, ts: ts2 }, CycleRecord { balance: cycle, ts: ts1 }];


  let task = EGO_TENANT.with(|ego_tenant| {
    ego_tenant
      .borrow()
      .tasks
      .get(&canister_principal)
      .unwrap()
      .clone()
  });

  let management = MockManagement::new();
  let ego_store = MockStore::new();
  let mut ego_canister = MockCanister::new();

  ego_canister
    .expect_ego_cycle_estimate_set()
    .returning(move |canister_id, estimate| {
      assert_eq!(canister_principal, canister_id);
      assert_eq!(0, estimate);
    });

  let _result = EgoTenantService::ego_cycle_check_cb(
    management,
    ego_store,
    ego_canister,
    &task,
    &canister_principal,
    &records,
    100_000,
  )
    .await;

  let task = EGO_TENANT.with(|ego_tenant| {
    ego_tenant
      .borrow()
      .tasks
      .get(&canister_principal)
      .unwrap()
      .clone()
  });


  assert_eq!(ts2 + NEXT_CHECK_DURATION, task.next_check_time);
}

// zero cycle consumption between two check times
#[tokio::test]
async fn canister_cycles_check_second_time_none_zero_cycle_consumption() {
  set_up();

  let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();
  let canister_principal = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();

  let ts1 = 10u64;
  let ts2 = 20u64;
  let cycle1 = 1_000_000u128;
  let cycle2 = 500_000u128;

  let records = vec![CycleRecord { balance: cycle2, ts: ts2 }, CycleRecord { balance: cycle1, ts: ts1 }];

  let task = EGO_TENANT.with(|ego_tenant| {
    ego_tenant
      .borrow()
      .tasks
      .get(&canister_principal)
      .unwrap()
      .clone()
  });

  let mut management = MockManagement::new();
  let mut ego_store = MockStore::new();
  let mut ego_canister = MockCanister::new();


  ego_store
    .expect_wallet_cycle_charge()
    .returning(move |wallet_id, cycle, _comment| {
      assert_eq!(wallet_principal, wallet_id);
      assert_eq!(500_000, cycle);
      Ok(true)
    });

  management
    .expect_canister_cycle_top_up()
    .returning(move |canister_id, cycle| {
      assert_eq!(canister_principal, canister_id);
      assert_eq!(500_000, cycle);
      Ok(())
    });

  ego_canister
    .expect_ego_cycle_estimate_set()
    .returning(move |canister_id, estimate| {
      assert_eq!(canister_principal, canister_id);
      assert_eq!(10, estimate);
    });

  let _result = EgoTenantService::ego_cycle_check_cb(
    management,
    ego_store,
    ego_canister,
    &task,
    &canister_principal,
    &records,
    1_000_000,
  )
    .await;

  let task = EGO_TENANT.with(|ego_tenant| {
    ego_tenant
      .borrow()
      .tasks
      .get(&canister_principal)
      .unwrap()
      .clone()
  });

  assert_eq!(ts2 + NEXT_CHECK_DURATION, task.next_check_time);
}