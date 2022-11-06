// macro user should implement the on_canister_added method
#[macro_export]
macro_rules! inject_ego_registry {
    () => {
        thread_local! {
          pub static REGISTRY: RefCell<Registry> = RefCell::new(Registry::default());
        }

        use ego_registry::registry::CanisterTrait;
        use ego_registry::registry::Registry;
        use std::collections::BTreeMap;

        #[update(name = "canister_add", guard = "owner_guard")]
        #[candid_method(update, rename = "canister_add")]
        pub fn canister_add(name: String, canister_id: Principal) -> Result<(), String> {
            REGISTRY.with(|s| s.borrow_mut().canister_add(name.clone(), canister_id));
            on_canister_added(&name, canister_id);
            Ok(())
        }

        #[update(name = "canister_remove", guard = "owner_guard")]
        #[candid_method(update, rename = "canister_remove")]
        pub fn canister_remove(name: String, canister_id: Principal) -> Result<(), String> {
            REGISTRY.with(|s| s.borrow_mut().canister_remove(name, canister_id));
            Ok(())
        }

        #[update(name = "canister_list", guard = "owner_guard")]
        #[candid_method(update, rename = "canister_list")]
        pub fn canister_list() -> Result<BTreeMap<String, Vec<Principal>>, String> {
            REGISTRY.with(|s| Ok(s.borrow().canister_list_all()))
        }

        pub fn registry_pre_upgrade() -> Registry {
            REGISTRY.with(|s| s.take().into())
        }

        pub fn registry_post_upgrade(stable_state: Registry) {
            REGISTRY.with(|s| s.replace(stable_state));
        }
    };
}
