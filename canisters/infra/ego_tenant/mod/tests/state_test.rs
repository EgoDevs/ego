use candid::Principal;
use ego_tenant_mod::memory::CONFIG;
use ego_tenant_mod::state::{canister_add, canister_list, canister_remove_all, owner_add, owner_remove, owners, post_upgrade, pre_upgrade};

static EXISTS_CANISTER_ID: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";
static EXISTS_USER_ID: &str = "225da-yaaaa-aaaah-qahrq-cai";

#[test]
fn test_upgrade(){
  // assert is empty at the beginning
  CONFIG.with(|cell| {
    assert!(cell.borrow().get().users.is_none());
    assert!(cell.borrow().get().registry.is_none());
  });

  owner_add(Principal::from_text(EXISTS_USER_ID.to_string()).unwrap());
  canister_add("test".to_string(), Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap());

  // assert is empty before pre_upgrade
  CONFIG.with(|cell| {
    assert!(cell.borrow().get().users.is_none());
    assert!(cell.borrow().get().registry.is_none());
  });

  pre_upgrade();

  CONFIG.with(|cell| {
    assert!(cell.borrow().get().users.is_some());
    assert!(cell.borrow().get().registry.is_some());
  });

  // remove data
  owner_remove(Principal::from_text(EXISTS_USER_ID.to_string()).unwrap());
  canister_remove_all("test".to_string());

  // assert is empty before post_upgrade
  assert!(owners().unwrap().is_empty());
  assert!(canister_list().is_empty());

  post_upgrade();
  assert!(!owners().unwrap().is_empty());
  assert!(!canister_list().is_empty());
}