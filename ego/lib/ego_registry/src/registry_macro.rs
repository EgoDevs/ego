#[macro_export]
macro_rules! inject_ego_registry {
    () => {
        thread_local! {
          pub static REGISTRY: RefCell<Registry> = RefCell::new(Registry::default());
        }

        use ego_registry::registry::CanisterTrait;
        use ego_registry::registry::Registry;

        #[update(name = "canister_add", guard = "owner_guard")]
        #[candid_method(update, rename = "canister_add")]
        pub fn canister_add(name: String, canister_id: Principal) -> Result<(), String> {
            REGISTRY.with(|s| s.borrow_mut().canister_add(name, canister_id));
            Ok(())
        }

        #[update(name = "canister_remove", guard = "owner_guard")]
        #[candid_method(update, rename = "canister_remove")]
        pub fn canister_remove(name: String, canister_id: Principal) -> Result<(), String> {
            REGISTRY.with(|s| s.borrow_mut().canister_remove(name, canister_id));
            Ok(())
        }

        pub fn registry_pre_upgrade() -> Registry {
            REGISTRY.with(|s| s.take().into())
        }

        pub fn registry_post_upgrade(stable_state: Registry) {
            REGISTRY.with(|s| s.replace(stable_state));
        }
    };
}
