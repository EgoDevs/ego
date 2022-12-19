use ic_cdk::api;
use ic_cdk::export::Principal;

use crate::app::{AppVersion, EgoDevApp};
use crate::c2c::c2c_types::{AppMainReleaseRequest, EgoStoreApp};

pub trait TEgoStore {
  fn app_main_release(
    &self,
    app: EgoDevApp,
    app_version: AppVersion,
  );
}

pub struct EgoStore {
  pub canister_id: Principal,
}

impl EgoStore {
  pub fn new(canister_id: Principal) -> Self {
    EgoStore { canister_id }
  }
}


impl TEgoStore for EgoStore {
  fn app_main_release(
    &self,
    app: EgoDevApp,
    released_version: AppVersion,
  ) {
    let req = AppMainReleaseRequest {
      app: EgoStoreApp {
        app_id: app.app_id,
        name: app.name,
        category: app.category,
        logo: app.logo,
        description: app.description,
        current_version: app.release_version.unwrap(),
        frontend: released_version.frontend,
        backend: released_version.backend,
        price: app.price,
        deploy_mode: app.deploy_mode,
      },
    };

    let _result = api::call::notify(self.canister_id, "app_main_release", (req, ));
  }
}
