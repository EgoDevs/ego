use std::cmp::Ordering;
use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;
use ic_types::Principal;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Tenant{
  pub wallet_count: u16,
  pub tenant_id: Principal
}

impl Eq for Tenant {}

impl PartialEq<Self> for Tenant {
  fn eq(&self, other: &Self) -> bool {
    self.tenant_id == other.tenant_id
  }
}

impl PartialOrd<Self> for Tenant {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.wallet_count.cmp(&other.wallet_count))
  }
}

impl Ord for Tenant {
  fn cmp(&self, other: &Self) -> Ordering {
    self.wallet_count.cmp(&other.wallet_count)
  }
}

impl Tenant {
  pub fn new(tenant_id: Principal) -> Self{
    Tenant{tenant_id, wallet_count: 0}
  }
}