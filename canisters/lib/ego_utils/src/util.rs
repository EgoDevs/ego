use candid::Principal;

// return seconds
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
    ic_cdk::api::time() / 1000_000_000
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

pub fn id() -> Principal {
  #[cfg(not(target_arch = "wasm32"))]
  {
    Principal::from_text("qhbym-qaaaa-aaaaa-aaafq-cai".to_string()).unwrap()
  }

  #[cfg(target_arch = "wasm32")]
  {
    ic_cdk::api::id()
  }
}

pub fn get_md5(data: &Vec<u8>) -> String {
  let digest = md5::compute(data);
  return format!("{:?}", digest);
}
