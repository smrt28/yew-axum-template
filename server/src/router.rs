

use crate::config::Config;
use axum::{routing::{get}, extract::{State, FromRef}, Router, Json};
use tokio::{net::TcpListener};
use std::net::SocketAddr;
use anyhow::Context;
use log::info;
use tower::{ServiceBuilder};
use tower_http::services::{ServeDir, ServeFile};
use crate::app_error::AppError;
use redis::AsyncCommands;

#[derive(Clone)]
pub struct ApiState {
   redis: redis::Client,
    
   #[allow(dead_code)]
   http: reqwest::Client,
}

impl ApiState {

}

#[derive(Clone)]
pub struct AppState {
    pub api_state: ApiState,
}

impl FromRef<AppState> for ApiState  {
    fn from_ref(app_state: &AppState) -> ApiState {
        app_state.api_state.clone()
    }
}

impl AppState {
    fn new(config: Config) -> Result<Self, AppError> {
        Ok(Self {
            api_state: ApiState {
                redis: redis::Client::open(config.redis.uri)?,
                http: reqwest::ClientBuilder::new()
                    .pool_max_idle_per_host(config.http.max_clients_count)
                    .build()?,
            }
        })
    }
}

pub async fn run_server(config: &Config) -> Result<(), AppError> {

    let app_state = AppState::new(config.clone())?;

    let mut app = Router::new()
        .route("/version", get(version));

    info!("Starting server on port {}", config.http.port);
    info!("Root: {}", config.root.as_ref().unwrap_or(&"N/A".to_string()));

    for mapping in config.http.mappings.as_ref().unwrap_or(&vec![]) {
        mapping.check(&config)?;
        let path = mapping.path.to_path(config)?;
        if !(path.try_exists()?) {
            return Err(AppError::GenerateError(
                format!("Path {} does not exist", path.to_string_lossy())));
        }
        info!("mapping: {}", &mapping);
        let server_dir = ServeDir::new(&path)
            .append_index_html_on_directories(true)
            .precompressed_br()
            .precompressed_gzip();
        if let Some(fallback) = &mapping.fallback {
            let fallback_path = path.join(fallback);
            if !fallback_path.try_exists()? {
                return Err(AppError::GenerateError(format!("Fallback is missing: {}", &fallback)));
            }
            let server_dir = server_dir.not_found_service(ServeFile::new(fallback_path));
            let static_svc = ServiceBuilder::new().service(server_dir);
            app = app.nest_service(&mapping.target, static_svc);
        } else {
            let static_svc = ServiceBuilder::new().service(server_dir);
            app = app.nest_service(&mapping.target, static_svc);
        }
    }

    let app = app.with_state(app_state);
    let addr = SocketAddr::from(([127, 0, 0, 1], config.http.port));
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
    pub value: i32,
    pub version: String,
}


pub async fn version(State(state): State<ApiState>) -> Result<Json<Version>, AppError> {

    let mut con = state.redis.get_multiplexed_async_connection().await?;
    let val: i32 = con.incr("b", 1).await?;

    Ok(Json(Version {
        value: val,
        version: "0.0.0".to_string(),
    }))
}