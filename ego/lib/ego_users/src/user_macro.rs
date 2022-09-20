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