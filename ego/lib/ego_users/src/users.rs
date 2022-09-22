use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_types::Principal;
use serde::Serialize;

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

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
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
        let ret = self.owners.contains(&who);
        ic_cdk::println!("check owner: {}, result: {}", who, ret);
        ret
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
        let ret = self.users.contains(&who) || self.owners.contains(&who);
        ic_cdk::println!("check user: {}, result: {}", who, ret);
        ret
    }
}
