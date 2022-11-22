use async_trait::async_trait;
use ic_cdk::export::Principal;
use mockall::mock;
use ego_ledger_mod::service::EgoLedgerService;
use ego_macros::ego_log::TEgoLogCanister;
use ego_ledger_mod::c2c::ic_ledger::TIcLedger;
use ego_ledger_mod::c2c::ego_store::TEgoStore;
use ic_ledger_types::{AccountIdentifier, Block, BlockIndex, Memo, Subaccount, Timestamp, Tokens, Transaction};
use ic_ledger_types::Operation::Transfer;
use ego_ledger_mod::payment::Payment;
use ego_ledger_mod::state::EGO_LEDGER;
use ego_types::ego_error::EgoError;

static FROM_ACCOUNT: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";
static EGO_STORE_ID: &str = "225da-yaaaa-aaaah-qahrq-cai";
static TEST_ACCOUNT_ID: &str = "223xb-saaaa-aaaaf-arlqa-cai";
static EXIST_MEMO: u64 = 1u64;

pub fn set_up() {
  let from_canister = Principal::from_text(FROM_ACCOUNT.to_string()).unwrap();
  let store_canister = Principal::from_text(EGO_STORE_ID.to_string()).unwrap();

  let mut bytes = [0u8; 32];
  let mut subaccount = Subaccount(bytes);
  let from = AccountIdentifier::new(&from_canister, &subaccount);

  bytes
    .split_at_mut(8)
    .0
    .copy_from_slice(EXIST_MEMO.to_le_bytes().as_slice());
  subaccount = Subaccount(bytes);
  let to = AccountIdentifier::new(&store_canister, &subaccount);

  let payment = Payment::new(from, to, Tokens::from_e8s(1), Memo(1));
  EGO_LEDGER.with(|ego_ledger| {
    ego_ledger.borrow_mut().payments.entry(to).or_insert(payment);
  });
}

mock! {
  Store {}

  impl TEgoStore for Store {
    fn wallet_order_notify(&self, memo: Memo);
  }
}

mock! {
  Log {}

  impl TEgoLogCanister for Log {
    fn canister_log_add(&self, message: &str);
  }
}

mock! {
  Ledger {}

  #[async_trait]
  impl TIcLedger for Ledger {
    async fn query_blocks(&self, start: BlockIndex) -> Result<Vec<Block>, EgoError>;
  }
}

#[test]
fn ledger_main_init() {
  set_up();

  let start:BlockIndex = 1;
  EgoLedgerService::ledger_main_init(start);

  EGO_LEDGER.with(|ego_ledger| {
    assert_eq!(1, ego_ledger.borrow().start)
  });
}

#[test]
fn ledger_payment_add() {
  set_up();

  let test_account_canister = Principal::from_text(TEST_ACCOUNT_ID.to_string()).unwrap();
  let store_canister = Principal::from_text(EGO_STORE_ID.to_string()).unwrap();
  let bytes = [0u8; 32];
  let subaccount = Subaccount(bytes);

  let from = AccountIdentifier::new(&test_account_canister, &subaccount);
  let to = AccountIdentifier::new(&store_canister, &subaccount);
  EgoLedgerService::ledger_payment_add(from, to, Tokens::from_e8s(1), Memo(1));

  EGO_LEDGER.with(|ego_ledger| {
    assert_eq!(2, ego_ledger.borrow().payments.len());
    assert!(ego_ledger.borrow().payments.contains_key(&to));
    match ego_ledger.borrow().payments.get(&to) {
      None => {
        panic!("should not go here")
      }
      Some(r) => {
        assert_eq!(Memo(1), r.memo);
      }
    }
  });
}

#[tokio::test]
async fn ledger_payment_match() {
  set_up();

  let mut ego_store = MockStore::new();
  let mut ego_log = MockLog::new();
  let mut ic_ledger = MockLedger::new();

  ego_log.expect_canister_log_add().returning(|_msg| ());

  ic_ledger.expect_query_blocks().returning(|_idx| {
    let from_canister = Principal::from_text(FROM_ACCOUNT.to_string()).unwrap();
    let store_canister = Principal::from_text(EGO_STORE_ID.to_string()).unwrap();

    let mut bytes = [0u8; 32];
    let mut subaccount = Subaccount(bytes);
    let from = AccountIdentifier::new(&from_canister, &subaccount);

    bytes
      .split_at_mut(8)
      .0
      .copy_from_slice(EXIST_MEMO.to_le_bytes().as_slice());
    subaccount = Subaccount(bytes);
    let to = AccountIdentifier::new(&store_canister, &subaccount);

    let mut blocks = vec![];

    let transaction = Transaction{
      memo: Memo(1),
      operation: Some(Transfer {
        from,
        to,
        amount: Tokens::from_e8s(1),
        fee: Tokens::from_e8s(0),
      }),
      created_at_time: Timestamp { timestamp_nanos: 0 }
    };

    let block = Block{
      parent_hash: None,
      transaction,
      timestamp: Timestamp { timestamp_nanos: 0 }
    };

    blocks.push(block);

    Ok(blocks)
  });

  ego_store.expect_wallet_order_notify().returning(|_memo| ());

  EGO_LEDGER.with(|ego_ledger| assert_eq!(1, ego_ledger.borrow().payments.len()));

  match EgoLedgerService::ledger_payment_match(ego_store, ic_ledger, ego_log).await{
    Ok(_) => {}
    Err(_) => {}
  };

  EGO_LEDGER.with(|ego_ledger| assert_eq!(0, ego_ledger.borrow().payments.len()));
}

#[tokio::test]
async fn ledger_payment_match_ic_ledger_error() {
  set_up();

  let mut ego_store = MockStore::new();
  let mut ego_log = MockLog::new();
  let mut ic_ledger = MockLedger::new();

  ego_log.expect_canister_log_add().returning(|_msg| ());

  ic_ledger.expect_query_blocks().returning(|_block| Err(EgoError::from("ic ledger error".to_string())));

  ego_store.expect_wallet_order_notify().returning(|_memo| ());

  EGO_LEDGER.with(|ego_ledger| assert_eq!(1, ego_ledger.borrow().payments.len()));

  match EgoLedgerService::ledger_payment_match(ego_store, ic_ledger, ego_log).await{
    Ok(_) => {}
    Err(e) => {
      assert_eq!(255, e.code);}
  };

  EGO_LEDGER.with(|ego_ledger| assert_eq!(1, ego_ledger.borrow().payments.len()));
}
