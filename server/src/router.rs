
use crate::config::Config;
use std::sync::Arc;
use axum::{routing::{get}, extract::{State, FromRef}, Router, Json};
use tokio::{net::TcpListener, sync::Mutex};
use std::net::SocketAddr;
use anyhow::Context;

#[derive(Clone, Default, Debug)]
pub struct ApiStateData {
    i: i32,
}

type ApiState = Arc<Mutex<ApiStateData>>;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub api_state: ApiState,
}

impl FromRef<AppState> for ApiState  {
    fn from_ref(app_state: &AppState) -> ApiState {
        app_state.api_state.clone()
    }
}

impl AppState {
    fn new(config: Config) -> Self {
        Self {
            config: config.into(),
            api_state: ApiState::new(ApiStateData::default().into())
        }
    }
}

pub async fn run_server(config: &Config) -> anyhow::Result<()> {
    let app_state = AppState::new(config.clone());

    let app = Router::new()
        .route("/version", get(version))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    let listener = TcpListener::bind(addr).await.context("Failed to bind to address")?;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .context("Server error")?;

    Ok(())
}

#[derive(serde::Serialize)]
pub struct Version {
    pub version: String,
}


pub async fn version(State(state): State<ApiState>) -> Json<Version> {
    Json(Version {
        version: "0.0.0".to_string(),
    })
}