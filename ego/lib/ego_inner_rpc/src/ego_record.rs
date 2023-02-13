use async_trait::async_trait;
use ic_cdk::api;
use ic_cdk::export::Principal;
use crate::types::{AppOperationRecord, ConsumeCycleRecord, RechargeCycleRecord, SnapshotCycleBalanceRecord};

#[async_trait]
pub trait TEgoRecord {
  // fn record_add(&self, scope: String, event: String, message: String, created_at: Option<u64>);
  fn record_add(&self, event : EgoEvent);
}

pub struct EgoRecord {
  pub canister_id: Principal,
}

impl EgoRecord {
  pub fn new(canister_id: Principal) -> Self {
    EgoRecord {
      canister_id
    }
  }
}

#[async_trait]
impl TEgoRecord for EgoRecord {
  fn record_add(&self, event : EgoEvent){
    let (scope, event, message) = unpack_event(event);
    let _result = api::call::notify(self.canister_id, "record_add", (scope, event, message, ));
  }
}

fn unpack_event(ev: EgoEvent) -> (String, String, String) {
  match ev {
    EgoEvent::MainInstall(ev) => (
      "EGO".to_string(),
      "CONTROLLER_INSTALL".to_string(),
      serde_json::to_string(&ev).unwrap(),
    ),

    EgoEvent::MainUninstall(ev) => (
      "EGO".to_string(),
      "CONTROLLER_UNINSTALL".to_string(),
      serde_json::to_string(&ev).unwrap(),
    ),

    EgoEvent::MainUpgrade(ev) => (
      "EGO".to_string(),
      "CONTROLLER_UPGRADE".to_string(),
      serde_json::to_string(&ev).unwrap(),
    ),

    EgoEvent::AppInstall(ev) => (
      "EGO".to_string(),
      "APP_INSTALL".to_string(),
      serde_json::to_string(&ev).unwrap(),
    ),
    EgoEvent::AppUninstall(ev) => (
      "EGO".to_string(),
      "APP_UNINSTALL".to_string(),
      serde_json::to_string(&ev).unwrap(),
    ),
    EgoEvent::AppUpgrade(ev) => (
      "EGO".to_string(),
      "APP_UPGRADE".to_string(),
      serde_json::to_string(&ev).unwrap(),
    ),

    EgoEvent::CycleRecharge(ev) => (
      "EGO".to_string(),
      "CYCLE_RECHARGE".to_string(),
      serde_json::to_string(&ev).unwrap(),
    ),

    EgoEvent::SnapshotCycleBalance(ev) => (
      "EGO".to_string(),
      "CYCLE_BALANCE_SNAPSHOT".to_string(),
      serde_json::to_string(&ev).unwrap(),
    ),

  }
}




pub enum EgoEvent {

  //cycle 充值
  CycleRecharge(RechargeCycleRecord),
  SnapshotCycleBalance(SnapshotCycleBalanceRecord),// 用户canister里cycle余额，每隔一小时snapshot
  //app安装，卸载，更新事件
  AppInstall(AppOperationRecord),
  AppUninstall(AppOperationRecord),
  AppUpgrade(AppOperationRecord),

  MainInstall(AppOperationRecord),
  MainUninstall(AppOperationRecord),
  MainUpgrade(AppOperationRecord),



  //canister 方法调用事件
  // CanisterCall(MethodCallRecord),



}





