use std::collections::BTreeMap;

use ic_cdk::export::Principal;

use ego_types::user::User;

#[test]
fn owner_test() {
  let mut user = User::default();

  let owner_1 = "owner_1";
  let owner_1_principal =
    Principal::from_text("o2ivq-5dsz3-nba5d-pwbk2-hdd3i-vybeq-qfz35-rqg27-lyesf-xghzc-3ae")
      .unwrap();

  assert!(!user.is_owner(owner_1_principal));

  user.owner_add(owner_1.to_string(), owner_1_principal);
  assert!(user.is_owner(owner_1_principal));

  user.owner_remove(owner_1_principal);
  assert!(!user.is_owner(owner_1_principal));

  user.owners_set(BTreeMap::from([(owner_1_principal, owner_1.to_string())]));
  assert!(user.is_owner(owner_1_principal));
}

#[test]
fn user_test() {
  let mut user = User::default();

  let owner_1 = "owner_1";
  let owner_1_principal =
    Principal::from_text("o2ivq-5dsz3-nba5d-pwbk2-hdd3i-vybeq-qfz35-rqg27-lyesf-xghzc-3ae")
      .unwrap();

  let user_1 = "user_1";
  let user_1_principal =
    Principal::from_text("3zjeh-xtbtx-mwebn-37a43-7nbck-qgquk-xtrny-42ujn-gzaxw-ncbzw-kqe")
      .unwrap();

  assert!(!user.is_user(owner_1_principal));
  assert!(!user.is_user(user_1_principal));

  user.owner_add(owner_1.to_string(), owner_1_principal);
  assert!(user.is_user(owner_1_principal));

  user.user_add(user_1.to_string(), user_1_principal);
  assert!(user.is_user(user_1_principal));

  user.user_remove(user_1_principal);
  assert!(!user.is_user(user_1_principal));

  user.users_set(BTreeMap::from([(user_1_principal, user_1.to_string())]));

  // owner_1 still in the owners map
  assert!(user.is_user(owner_1_principal));
  assert!(user.is_user(user_1_principal));
}

#[test]
fn op_test() {
  let mut user = User::default();

  let owner_1 = "owner_1";
  let owner_1_principal =
    Principal::from_text("o2ivq-5dsz3-nba5d-pwbk2-hdd3i-vybeq-qfz35-rqg27-lyesf-xghzc-3ae")
      .unwrap();

  let op_1 = "op_1";
  let op_1_principal =
    Principal::from_text("3zjeh-xtbtx-mwebn-37a43-7nbck-qgquk-xtrny-42ujn-gzaxw-ncbzw-kqe")
      .unwrap();

  assert!(!user.is_op(owner_1_principal));
  assert!(!user.is_op(op_1_principal));

  user.owner_add(owner_1.to_string(), owner_1_principal);
  assert!(user.is_op(owner_1_principal));

  user.op_add(op_1.to_string(), op_1_principal);
  assert!(user.is_op(op_1_principal));

  user.op_remove(op_1_principal);
  assert!(!user.is_op(op_1_principal));

  user.ops_set(BTreeMap::from([(op_1_principal, op_1.to_string())]));
  assert!(user.is_op(op_1_principal));
}
