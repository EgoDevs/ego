use candid::Principal;

use ego_store_mod::types::tenant::Tenant;

static TENANT_ID1: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";
static TENANT_ID2: &str = "223xb-saaaa-aaaaf-arlqa-cai";

pub fn set_up() {
  let tenant_id1 = Principal::from_text(TENANT_ID1.to_string()).unwrap();
  let tenant1 = Tenant::new(&tenant_id1);
  tenant1.save();
}

#[test]
pub fn new() {
  set_up();

  assert_eq!(1, Tenant::len());

  let tenant_id2 = Principal::from_text(TENANT_ID2.to_string()).unwrap();
  let tenant2 = Tenant::new(&tenant_id2);
  tenant2.save();

  assert_eq!(2, Tenant::len());
}

#[test]
pub fn len() {
  set_up();
  assert_eq!(1, Tenant::len());
}

#[test]
pub fn list() {
  set_up();

  let tenant_id1 = Principal::from_text(TENANT_ID1.to_string()).unwrap();

  let tenants = Tenant::list(0, 100);

  assert_eq!(1, tenants.len());
  assert_eq!(tenant_id1, tenants.get(0).unwrap().canister_id);
}

#[test]
pub fn get() {
  set_up();

  let tenant_id1 = Principal::from_text(TENANT_ID1.to_string()).unwrap();
  let tenant1 = Tenant::get(&tenant_id1);
  assert!(tenant1.is_some());

  let tenant_id2 = Principal::from_text(TENANT_ID2.to_string()).unwrap();
  let tenant2 = Tenant::get(&tenant_id2);
  assert!(tenant2.is_none());
}