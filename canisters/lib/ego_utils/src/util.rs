use candid::Principal;

pub fn time() -> u64 {
  #[cfg(not(target_arch = "wasm32"))]
  {
    std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .expect("Failed to get timestamp")
      .as_secs()
  }

  #[cfg(target_arch = "wasm32")]
  {
    ic_cdk::api::time()
  }
}

pub fn caller() -> Principal {
  #[cfg(not(target_arch = "wasm32"))]
  {
    Principal::from_text("wzmn5-x5ep5-jbbjy-uexkb-aj7gf-kq2mo-ujdws-iizyz-qbdq3-wywpm-4qe".to_string()).unwrap()
  }

  #[cfg(target_arch = "wasm32")]
  {
    ic_cdk::api::caller()
  }
}
