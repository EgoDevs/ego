use ic_types::Principal;
use mockall::{mock};
use ego_tenant_mod::service::TenantService;
use ego_utils::types::{Management, Cycles, EgoError};
use async_trait::async_trait;

mock! {
  Dump {}

  #[async_trait]
  impl Management for Dump {
    async fn canister_main_create(&self, cycles_to_use: Cycles) -> Result<Principal, EgoError>;
  }
}

#[tokio::test]
async fn test_wallet_app_install_success(){
  let mut service = MockDump::new();
  service.expect_canister_main_create().returning(|_cycles_to_use| Ok(Principal::from_text("qvhpv-4qaaa-aaaaa-aaagq-cai").unwrap()));
  match TenantService::wallet_app_install(service, "app_1").await{
    Ok(principal) => assert_eq!(principal.to_text(), "qvhpv-4qaaa-aaaaa-aaagq-cai".to_string()),
    Err(_e) => panic!("should not go here"),
  }
}

#[tokio::test]
async fn test_wallet_app_install_failed(){
  let mut service = MockDump::new();
  service.expect_canister_main_create().returning(|_cycles_to_use| Err(EgoError::from("error".to_string())));
  match TenantService::wallet_app_install(service, "app_1").await{
    Ok(_principal) => panic!("should not go here"),
    Err(e) => assert_eq!("error".to_string(), e.msg),
  }
}
