use candid::Principal;

use ego_store_mod::types::cash_flow::CashFlow;
use ego_types::app::CashFlowType;
use ego_utils::util::time;

static CANISTER_ID1: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";
static CANISTER_ID2: &str = "223xb-saaaa-aaaaf-arlqa-cai";
static CANISTER_ID3: &str = "wtb37-uyaaa-aaaai-qa3zq-cai";

static OPERATOR: &str = "225da-yaaaa-aaaah-qahrq-cai";

pub fn set_up() {
  let operator = Principal::from_text(OPERATOR.to_string()).unwrap();

  let wallet1 = Principal::from_text(CANISTER_ID1.to_string()).unwrap();
  let mut cash_flow1 = CashFlow::new(&wallet1, CashFlowType::RECHARGE, 1, 1, &operator, "recharge".to_string());
  cash_flow1.save();

  let mut cash_flow2 = CashFlow::new(&wallet1, CashFlowType::CHARGE, 1, 0, &operator, "charge".to_string());
  cash_flow2.save();

  let wallet2 = Principal::from_text(CANISTER_ID2.to_string()).unwrap();
  let mut cash_flow3 = CashFlow::new(&wallet2, CashFlowType::RECHARGE, 1, 1, &operator, "recharge".to_string());
  cash_flow3.save();
}

#[test]
pub fn new() {
  set_up();

  assert_eq!(3, CashFlow::len());

  let operator = Principal::from_text(OPERATOR.to_string()).unwrap();

  let wallet3 = Principal::from_text(CANISTER_ID3.to_string()).unwrap();
  let mut cash_flow = CashFlow::new(&wallet3, CashFlowType::RECHARGE, 1, 1, &operator, "recharge".to_string());
  cash_flow.save();

  assert_eq!(4, CashFlow::len());
}

#[test]
pub fn len() {
  set_up();
  assert_eq!(3, CashFlow::len());
}

#[test]
pub fn by_last_update() {
  set_up();

  let now = time();

  assert_eq!(3, CashFlow::by_last_update(now).len());
}

#[test]
pub fn by_wallet_id() {
  set_up();

  let wallet1 = Principal::from_text(CANISTER_ID1.to_string()).unwrap();

  assert_eq!(2, CashFlow::by_wallet_id(&wallet1).len());
}

#[test]
pub fn list() {
  set_up();

  let wallet1 = Principal::from_text(CANISTER_ID1.to_string()).unwrap();

  let cash_flows = CashFlow::list();

  assert_eq!(3, cash_flows.len());
  assert_eq!(wallet1, cash_flows.get(0).unwrap().wallet_id);
}

#[test]
pub fn into() {
  set_up();

  let cash_flows = CashFlow::list();

  let cash_flow = cash_flows.get(0).unwrap();
  let c_f: ego_types::app::CashFlow = cash_flow.clone().into();

  assert_eq!(cash_flow.cash_flow_type, c_f.cash_flow_type);
  assert_eq!(cash_flow.cycles, c_f.cycles);
  assert_eq!(cash_flow.balance, c_f.balance);
}
