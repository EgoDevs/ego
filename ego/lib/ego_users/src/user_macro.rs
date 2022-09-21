/// Lifetime functions, use them in your project
/// You can combine them with other state(s)
/// examples:
/// ```rust
///
/// #[derive(Clone, Debug, CandidType, Deserialize)]
/// struct StableState {
///   my_state: MyState,
///   users: crate::ego_users::User,
/// }
///
///
/// #[candid_method(init, rename = "init")]
/// #[init]
/// fn canister_init() {
///     crate::ego_users::init();
/// }
/// ```
/// #[pre_upgrade]
/// pub fn pre_upgrade() {
///   let stable_state = STATE.with(|s| StableState {
///     my_state: s.my_state,
///     owner: crate::ego_users::role_pre_upgrade(),
///   });
///   ic_cdk::storage::stable_save((stable_state,)).expect("failed to save stable state");
/// }
///
/// #[post_upgrade]
/// pub fn post_upgrade() {
///       let (StableState { users, my_state },): (StableState,) =
///   ic_cdk::storage::stable_restore().expect("failed to restore stable state");
///   crate::ego_users::role_post_upgrade(users);
///   STATE.with(|s| {
///       s.my_state = my_state;
///   };
/// }
///
///
#[macro_export]
macro_rules! inject_ego_users {
    () => {
        thread_local! {
          pub static USER: RefCell<User> = RefCell::new(User::default());
        }

        use ic_types::Principal;
        use ego_users::users::User;
        use std::cell::RefCell;
        use ego_users::users::OwnerTrait;
        use ego_users::users::UserTrait;
        use ic_cdk::caller;
        use ic_cdk::trap;

        #[inline(always)]
        pub fn owner_guard() -> Result<(), String> {
            if USER.with(|b| b.borrow().check_owner(caller())) {
                Ok(())
            } else {
              trap(&format!("{} unauthorized", caller()));
            }
        }

        #[update(name = "role_owner_set", guard="owner_guard")]
        #[candid_method(update, rename = "role_owner_set")]
        pub fn role_owner_set(principals: Vec<Principal>) -> Result<(), String> {
            USER.with(|s| s.borrow_mut().role_owner_set(principals));
            Ok(())
        }

        #[update(name = "role_owner_add", guard="owner_guard")]
        #[candid_method(update, rename = "role_owner_add")]
        pub fn role_owner_add(principal: Principal) -> Result<(), String>  {
            USER.with(|s| s.borrow_mut().role_owner_add(principal));
            Ok(())
        }

        #[update(name = "role_owner_remove", guard="owner_guard")]
        #[candid_method(update, rename = "role_owner_remove")]
        pub fn role_owner_remove(principal: Principal) -> Result<(), String> {
            USER.with(|s| s.borrow_mut().role_owner_remove(principal));
            Ok(())
        }

        #[inline(always)]
        pub fn user_guard() -> Result<(), String> {
            if USER.with(|b| b.borrow().check_user(caller())) {
                Ok(())
            } else {
              trap(&format!("{} unauthorized", caller()));
            }
        }

        #[update(name = "role_user_set", guard="owner_guard")]
        #[candid_method(update, rename = "role_user_set")]
        pub fn role_user_set(principals: Vec<Principal>) -> Result<(), String> {
            USER.with(|s| s.borrow_mut().role_user_set(principals));
            Ok(())
        }

        #[update(name = "role_user_add", guard="owner_guard")]
        #[candid_method(update, rename = "role_user_add")]
        pub fn role_user_add(principal: Principal) -> Result<(), String> {
            USER.with(|s| s.borrow_mut().role_user_add(principal));
            Ok(())
        }

        #[update(name = "role_user_remove", guard="owner_guard")]
        #[candid_method(update, rename = "role_user_remove")]
        pub fn role_user_remove(principal: Principal) -> Result<(), String> {
            USER.with(|s| s.borrow_mut().role_user_remove(principal));
            Ok(())
        }

        pub fn users_init() {
            USER.with(|s| {
                let mut s = s.borrow_mut();
                s.role_owner_add(caller())
            });
        }

        pub fn users_pre_upgrade() -> User {
            USER.with(|s| s.take().into())
        }

        pub fn users_post_upgrade(stable_state: User) {
            USER.with(|s| s.replace(stable_state));
        }
    };
}