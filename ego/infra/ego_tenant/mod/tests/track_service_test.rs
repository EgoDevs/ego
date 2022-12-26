use async_trait::async_trait;
use ego_lib::ego_canister::TEgoCanister;
use ic_cdk::export::Principal;
use mockall::mock;

use ego_tenant_mod::c2c::ego_store::TEgoStore;
use ego_tenant_mod::c2c::ic_management::TIcManagement;
use ego_tenant_mod::service::EgoTenantService;
use ego_tenant_mod::state::EGO_TENANT;
use ego_tenant_mod::task::Task;
use ego_types::ego_error::EgoError;
use ego_utils::ic_management::Cycles;

static EXISTS_WALLET_ID: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";
static EXISTS_CANISTER_ID: &str = "223xb-saaaa-aaaaf-arlqa-cai";

static TEST_WALLET_ID: &str = "22aq5-waaaa-aaaaf-aobwq-cai";
static TEST_CANISTER_ID: &str = "224jh-lqaaa-aaaad-qaxda-cai";

pub fn set_up() {
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

    fn canister_cycle_top_up(
        &self,
        canister_id: Principal,
        cycles_to_use: Cycles,
    ) ;

    async fn canister_controller_set(
        &self,
        canister_id: Principal,
        principals: Vec<Principal>,
    ) -> Result<(), EgoError>;

  }
}

mock! {
  Store {}

  #[async_trait]
  impl TEgoStore for Store {
    async fn wallet_cycle_charge(
      &self,
      canister_id: Principal,
      wallet_id: Principal,
      cycle: u128,
      comment: String,
    ) -> Result<bool, EgoError>;
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

#[test]
fn canister_main_untrack() {
  set_up();
  let wallet_principal = Principal::from_text(TEST_WALLET_ID.to_string()).unwrap();
  let canister_principal = Principal::from_text(TEST_CANISTER_ID.to_string()).unwrap();

  let result = EgoTenantService::canister_main_untrack(wallet_principal, canister_principal);
  assert!(result.is_ok());
}

#[tokio::test]
async fn canister_cycles_check_first_time() {
  set_up();

  let canister_principal = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();

  let sentinel = 10u64;
  let cycle = 1_000_000u128;

  let task = EGO_TENANT.with(|ego_tenant| {
    ego_tenant
      .borrow()
      .tasks
      .get(&canister_principal)
      .unwrap()
      .clone()
  });

  assert_eq!(0, task.last_check_time);
  assert_eq!(0, task.last_cycle);

  let management = MockManagement::new();
  let ego_store = MockStore::new();
  let mut ego_canister = MockCanister::new();

  ego_canister
    .expect_balance_get()
    .times(1)
    .returning(move |canister_id| {
      assert_eq!(canister_principal, canister_id);

      Ok(cycle)
    });

  let _result = EgoTenantService::canister_cycles_check(
    management,
    ego_store,
    ego_canister,
    sentinel,
    task,
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

  assert_eq!(sentinel + 1000 * 60 * 30, task.next_check_time);
  assert_eq!(0, task.last_check_time);
  assert_eq!(1_000_000, task.last_cycle);
}

// zero cycle consumption between two check times
#[tokio::test]
async fn canister_cycles_check_second_time_zero_cycle_consumption() {
  set_up();

  let canister_principal = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();

  let sentinel = 10u64;
  let cycle = 1_000_000u128;

  EGO_TENANT.with(|ego_tenant| {
    ego_tenant
      .borrow_mut()
      .task_update(canister_principal, cycle, sentinel)
  });

  let task = EGO_TENANT.with(|ego_tenant| {
    ego_tenant
      .borrow()
      .tasks
      .get(&canister_principal)
      .unwrap()
      .clone()
  });

  assert_eq!(0, task.last_check_time);
  assert_eq!(sentinel, task.next_check_time);
  assert_eq!(cycle, task.last_cycle);

  let management = MockManagement::new();
  let ego_store = MockStore::new();
  let mut ego_canister = MockCanister::new();

  ego_canister
    .expect_balance_get()
    .times(1)
    .returning(move |canister_id| {
      assert_eq!(canister_principal, canister_id);

      Ok(cycle)
    });

  let _result = EgoTenantService::canister_cycles_check(
    management,
    ego_store,
    ego_canister,
    sentinel,
    task,
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

  assert_eq!(sentinel, task.last_check_time);
  assert_eq!(sentinel + 1000 * 60 * 30, task.next_check_time);
  assert_eq!(1_000_000, task.last_cycle);
}

// zero cycle consumption between two check times
#[tokio::test]
async fn canister_cycles_check_second_time_none_zero_cycle_consumption() {
  set_up();

  let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();
  let canister_principal = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();

  let sentinel = 10u64;
  let cycle = 1_000_000u128;

  EGO_TENANT.with(|ego_tenant| {
    ego_tenant
      .borrow_mut()
      .task_update(canister_principal, 2 * cycle, sentinel)
  });

  let task = EGO_TENANT.with(|ego_tenant| {
    ego_tenant
      .borrow()
      .tasks
      .get(&canister_principal)
      .unwrap()
      .clone()
  });

  assert_eq!(0, task.last_check_time);
  assert_eq!(sentinel, task.next_check_time);
  assert_eq!(2 * cycle, task.last_cycle);

  let mut management = MockManagement::new();
  let mut ego_store = MockStore::new();
  let mut ego_canister = MockCanister::new();

  ego_canister
    .expect_balance_get()
    .times(1)
    .returning(move |canister_id| {
      assert_eq!(canister_principal, canister_id);

      Ok(cycle)
    });

  ego_store
    .expect_wallet_cycle_charge()
    .returning(move |_canister_id,wallet_id, cycle, _comment| {
      assert_eq!(wallet_principal, wallet_id);
      assert_eq!(180000000000, cycle);
      Ok(true)
    });

  management
    .expect_canister_cycle_top_up()
    .returning(move |canister_id, cycle| {
      assert_eq!(canister_principal, canister_id);
      assert_eq!(180000000000, cycle);
    });

  let _result = EgoTenantService::canister_cycles_check(
    management,
    ego_store,
    ego_canister,
    sentinel,
    task,
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

  assert_eq!(sentinel, task.last_check_time);
  assert_eq!(sentinel + 1800000, task.next_check_time);
  assert_eq!(180_000_000_000 + 1_000_000, task.last_cycle);
}
