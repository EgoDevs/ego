use std::borrow::{Borrow, BorrowMut};
use std::collections::BTreeMap;

use candid::CandidType;
use ic_cdk::export::candid::Deserialize;
use ic_cdk::export::Principal;
use serde::Serialize;

use crate::user::Role::{OP, OWNER, USER};

#[derive(Debug)]
enum Role {
  OWNER,
  USER,
  OP,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct User {
  owners: Option<BTreeMap<Principal, String>>,
  users: Option<BTreeMap<Principal, String>>,
  ops: Option<BTreeMap<Principal, String>>,
}

impl Default for User {
  fn default() -> Self {
    User {
      owners: Some(BTreeMap::default()),
      users: Some(BTreeMap::default()),
      ops: Some(BTreeMap::default()),
    }
  }
}

impl User {
  pub fn owners_set(&mut self, users: BTreeMap<Principal, String>) {
    self.role_users_set(OWNER, users);
  }

  pub fn owners(&self) -> Option<BTreeMap<Principal, String>> {
    self.role_users(OWNER).clone().map(|e| e)
  }

  pub fn owner_add(&mut self, name: String, user_id: Principal) {
    self.role_user_add(OWNER, name, user_id);
  }

  pub fn owner_remove(&mut self, user_id: Principal) {
    self.role_user_remove(OWNER, user_id);
  }

  pub fn is_owner(&self, who: Principal) -> bool {
    self.is_a(OWNER, who)
  }

  pub fn users_set(&mut self, users: BTreeMap<Principal, String>) {
    self.role_users_set(USER, users);
  }

  pub fn users(&self) -> Option<BTreeMap<Principal, String>> {
    self.role_users(USER).clone().map(|e| e)
  }

  pub fn user_add(&mut self, name: String, user_id: Principal) {
    self.role_user_add(USER, name, user_id);
  }

  pub fn user_remove(&mut self, user_id: Principal) {
    self.role_user_remove(USER, user_id);
  }

  pub fn is_user(&self, who: Principal) -> bool {
    self.is_a(USER, who) || self.is_a(OWNER, who)
  }

  pub fn ops_set(&mut self, users: BTreeMap<Principal, String>) {
    self.role_users_set(OP, users);
  }

  pub fn ops(&self) -> Option<BTreeMap<Principal, String>> {
    self.role_users(OP).clone().map(|e| e)
  }

  pub fn op_add(&mut self, name: String, user_id: Principal) {
    self.role_user_add(OP, name, user_id);
  }

  pub fn op_remove(&mut self, user_id: Principal) {
    self.role_user_remove(OP, user_id);
  }

  pub fn is_op(&self, who: Principal) -> bool {
    self.is_a(OP, who) || self.is_a(OWNER, who)
  }

  fn role_users_set(&mut self, role: Role, users: BTreeMap<Principal, String>) {
    match role {
      OWNER => self.owners = Some(users),
      USER => self.users = Some(users),
      OP => self.ops = Some(users),
    }
  }

  fn role_users(&self, role: Role) -> &Option<BTreeMap<Principal, String>> {
    match role {
      OWNER => self.owners.borrow(),
      USER => self.users.borrow(),
      OP => self.ops.borrow(),
    }
  }

  fn role_users_mut(&mut self, role: Role) -> &mut Option<BTreeMap<Principal, String>> {
    match role {
      OWNER => self.owners.borrow_mut(),
      USER => self.users.borrow_mut(),
      OP => self.ops.borrow_mut(),
    }
  }

  fn role_user_add(&mut self, role: Role, name: String, user_id: Principal) {
    let users = self.role_users_mut(role).as_mut().unwrap();

    if !users.contains_key(&user_id) {
      users.insert(user_id, name);
    }
  }

  fn role_user_remove(&mut self, role: Role, user_id: Principal) {
    let users = self.role_users_mut(role).as_mut().unwrap();

    if users.contains_key(&user_id) {
      users.remove(&user_id);
    }
  }

  fn is_a(&self, role: Role, who: Principal) -> bool {
    let users = self.role_users(role).as_ref().unwrap();
    users.contains_key(&who)
  }
}
