use ego_types::app::{App, AppId, Category, DeployMode, Wasm};
use ego_types::version::Version;
use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;

/********************  app  ********************/
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct EgoStoreApp {
    pub app_id: AppId,
    pub name: String,
    pub category: Category,
    pub logo: String,
    pub description: String,
    pub current_version: Version,
    pub frontend: Option<Wasm>,
    pub backend: Option<Wasm>,
    pub price: f32,
    pub deploy_mode: DeployMode,
}

impl EgoStoreApp {
    pub fn to_string(&self) -> String {
        format!(
            "app_id: {:?},category:{:?},current_version:{:?},",
            self.app_id, self.category, self.current_version
        )
    }
}

impl EgoStoreApp {
    pub fn new(
        app_id: AppId,
        name: String,
        category: Category,
        logo: String,
        description: String,
        current_version: Version,
        frontend: Option<Wasm>,
        backend: Option<Wasm>,
        price: f32,
        deploy_mode: DeployMode,
    ) -> Self {
        EgoStoreApp {
            app_id,
            name,
            category,
            logo,
            description,
            current_version,
            frontend,
            backend,
            price,
            deploy_mode,
        }
    }
}

impl From<EgoStoreApp> for App {
    fn from(store_app: EgoStoreApp) -> Self {
        App::new(
            store_app.app_id,
            store_app.name,
            store_app.category,
            store_app.logo,
            store_app.description,
            store_app.current_version,
            store_app.price,
        )
    }
}
