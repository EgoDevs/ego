use async_trait::async_trait;
use ic_cdk::api;
use ic_cdk::export::Principal;

use ego_types::cycle_info::CycleRecord;

#[async_trait]
pub trait TEgoTenant {
  fn ego_cycle_check_cb(&self, records: Vec<CycleRecord>);
}

pub struct EgoTenant {
  pub canister_id: Principal,
}

impl EgoTenant {
  pub fn new(canister_id: Principal) -> Self {
    EgoTenant {
      canister_id
    }
  }
}

#[async_trait]
impl TEgoTenant for EgoTenant {
  fn ego_cycle_check_cb(&self, records: Vec<CycleRecord>) {
    let _result = api::call::notify(self.canister_id, "ego_cycle_check_cb", (records, ));
  }
}