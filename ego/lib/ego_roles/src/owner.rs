use ic_cdk::caller;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_types::Principal;
use serde::Serialize;
use std::cell::RefCell;

/// OwnerTrait, keep it simple
pub trait OwnerTrait {
    /// set owners to canister
    fn role_owner_set(&mut self, owners: Vec<Principal>);

    /// add owner to canister
    fn role_owner_add(&mut self, user_id: Principal);

    /// remove owner from canister
    fn role_owner_remove(&mut self, user_id: Principal);

    /// check whether `who` is the owner of canister, here we have 3 status
    ///     NotSet,Same,Different
    fn check_owner(&self, who: Principal) -> bool;
}

pub trait UserTrait {
    /// set owners to canister
    fn role_user_set(&mut self, users: Vec<Principal>);

    /// add owner to canister
    fn role_user_add(&mut self, user_id: Principal);

    /// remove owner from canister
    fn role_user_remove(&mut self, user_id: Principal);

    /// check whether `who` is the owner of canister, here we have 3 status
    ///     NotSet,Same,Different
    fn check_user(&self, who: Principal) -> bool;
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    owners: Vec<Principal>,
    users: Vec<Principal>,
}

impl Default for User {
    fn default() -> Self {
        User { owners: vec![], users: vec![] }
    }
}

impl OwnerTrait for User {
    fn role_owner_set(&mut self, owners: Vec<Principal>) {
        ic_cdk::println!("set owner to: {:?}", owners);
        self.owners = owners;
    }

    fn role_owner_add(&mut self, user_id: Principal) {
        ic_cdk::println!("add owner: {}", user_id);
        if !self.owners.contains(&user_id) {
            self.owners.push(user_id);
        }
    }

    fn role_owner_remove(&mut self, user_id: Principal) {
        ic_cdk::println!("remove owner: {}", user_id);
        if self.owners.contains(&user_id) {
            self.owners.retain(|p| *p != user_id);
        }
    }

    fn check_owner(&self, who: Principal) -> bool {
        ic_cdk::println!("check owner: {}", who);
        self.owners.contains(&who)
    }
}

impl UserTrait for User {
    fn role_user_set(&mut self, users: Vec<Principal>) {
        ic_cdk::println!("set user to: {:?}", users);
        self.users = users;
    }

    fn role_user_add(&mut self, user_id: Principal) {
        ic_cdk::println!("add user: {}", user_id);
        if !self.users.contains(&user_id) {
            self.users.push(user_id);
        }
    }

    fn role_user_remove(&mut self, user_id: Principal) {
        ic_cdk::println!("remove user: {}", user_id);
        if self.users.contains(&user_id) {
            self.users.retain(|p| *p != user_id);
        }
    }

    fn check_user(&self, who: Principal) -> bool {
        ic_cdk::println!("check user: {}", who);
        self.users.contains(&who)
    }
}

thread_local! {
    pub static USER: RefCell<User> = RefCell::new(User::default());
}

///
///  Guard methods, use in authorized api function
///  We use OWNER to store state
///
#[inline(always)]
pub fn owner_guard() -> bool {
    USER.with(|b| b.borrow().check_owner(caller()))
}

pub fn role_owner_set(principals: Vec<Principal>) {
    USER.with(|s| s.borrow_mut().role_owner_set(principals))
}

pub fn role_owner_add(principal: Principal) {
    USER.with(|s| s.borrow_mut().role_owner_add(principal))
}

pub fn role_owner_remove(principal: Principal) {
    USER.with(|s| s.borrow_mut().role_owner_remove(principal))
}

#[inline(always)]
pub fn user_guard() -> bool {
    USER.with(|b| b.borrow().check_user(caller()))
}

pub fn role_user_set(principals: Vec<Principal>) {
    USER.with(|s| s.borrow_mut().role_user_set(principals))
}

pub fn role_user_add(principal: Principal) {
    USER.with(|s| s.borrow_mut().role_user_add(principal))
}

pub fn role_user_remove(principal: Principal) {
    USER.with(|s| s.borrow_mut().role_user_remove(principal))
}

/// Lifetime functions, use them in your project
/// You can combine them with other state(s)
/// examples:
/// ```rust
///
/// #[derive(Clone, Debug, CandidType, Deserialize)]
/// struct StableState {
///   my_state: MyState,
///   owner: crate::ego_owner::Owner,
/// }
///
///
/// #[candid_method(init, rename = "init")]
/// #[init]
/// fn canister_init() {
///     crate::ego_owner::init();
/// }
/// ```
/// #[pre_upgrade]
/// pub fn pre_upgrade() {
///   let stable_state = STATE.with(|s| StableState {
///     my_state: s.my_state,
///     owner: crate::ego_owner::role_pre_upgrade(),
///   });
///   ic_cdk::storage::stable_save((stable_state,)).expect("failed to save stable state");
/// }
///
/// #[post_upgrade]
/// pub fn post_upgrade() {
///       let (StableState { owner, my_state },): (StableState,) =
///   ic_cdk::storage::stable_restore().expect("failed to restore stable state");
///   crate::ego_owner::role_post_upgrade(owner);
///   STATE.with(|s| {
///       s.my_state = my_state;
///   };
/// }
///
///
pub fn init() {
    USER.with(|s| {
        let mut s = s.borrow_mut();
        s.role_owner_add(caller())
    });
}

pub fn role_pre_upgrade() -> User {
    USER.with(|s| s.take().into())
}

pub fn role_post_upgrade(stable_state: User) {
    USER.with(|s| s.replace(stable_state));
}
