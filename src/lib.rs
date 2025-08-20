use std::sync::Arc;

use axum::Router;
use tokio::sync::Mutex;

use crate::auth::client::AuthClient;
use crate::auth::routes::create_auth_router;

pub mod auth;

pub mod authentication_proto {
    tonic::include_proto!("authenticator");
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AppConfig {
    pub listen_address: &'static str,
    pub authentication_address: &'static str,
}

pub async fn create_app(config: AppConfig) -> anyhow::Result<axum::Router> {
    let app_state = AppState::init(config.authentication_address).await?;
    let authentication_routes = create_auth_router().await?;

    let app = Router::new()
        .merge(authentication_routes)
        .with_state(app_state);

    Ok(app)
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub authentication_client: Arc<Mutex<AuthClient>>,
}

impl AppState {
    pub async fn init(authentication_address: &'static str) -> anyhow::Result<Self> {
        let authentication_client = AuthClient::new(authentication_address).await?;

        Ok(Self {
            authentication_client: Arc::new(Mutex::new(authentication_client)),
        })
    }
}
