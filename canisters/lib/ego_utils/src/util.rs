use candid::Principal;

pub fn time() -> u64 {
  #[cfg(feature = "test_mode")]
  {
    std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .expect("Failed to get timestamp")
      .as_secs()
  }

  #[cfg(not(feature = "test_mode"))]
  {
    ic_cdk::api::time()
  }
}

pub fn caller() -> Principal {
  #[cfg(feature = "test_mode")]
  {
    Principal::from_text("wzmn5-x5ep5-jbbjy-uexkb-aj7gf-kq2mo-ujdws-iizyz-qbdq3-wywpm-4qe".to_string()).unwrap()
  }

  #[cfg(not(feature = "test_mode"))]
  {
    ic_cdk::api::caller()
  }
}
