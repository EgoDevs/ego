#[macro_export]
macro_rules! inject_ego_log {
    () => {
        use ego_macros::ego_log::{TEgoLogCanister, EgoLogCanister};

        pub fn ego_log(message: &str) {
            ic_cdk::println!("ego-log: message: {}", message.clone());
            match REGISTRY.with(|r| r.borrow().canister_get_one("ego_log")) {
                None => {},
                Some(ego_log_id) => {
                    let ego_log = EgoLogCanister::new(ego_log_id);
                    ego_log.canister_log_add(message);
                }
            };
        }

        pub fn get_ego_log() -> Option<EgoLogCanister> {
            match REGISTRY.with(|r| r.borrow().canister_get_one("ego_log")) {
                None => {
                    None
                },
                Some(ego_log_id) => {
                    Some(EgoLogCanister::new(ego_log_id))
                }
            }
        }
    };
}
