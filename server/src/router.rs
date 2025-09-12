
//use bitflags::traits::Flags;
use crate::config::Config;
use std::sync::Arc;
use axum::{routing::{get}, extract::{State, FromRef}, Router, Json};
use tokio::{net::TcpListener};
use std::net::SocketAddr;
use anyhow::Context;
use tower::{ServiceBuilder};
use tower_http::services::ServeDir;
use crate::app_error::AppError;
use crate::client_pool::{ClientsPool};
use crate::redis::RedisClientFactory;
use redis::AsyncCommands;

#[derive(Clone)]
pub struct ApiState {
   redis_client_pool: Arc<ClientsPool<redis::Client>>
}


impl ApiState {

}

#[derive(Clone)]
pub struct AppState {
    //pub config: Arc<Config>,
    pub api_state: ApiState,
}

impl FromRef<AppState> for ApiState  {
    fn from_ref(app_state: &AppState) -> ApiState {
        app_state.api_state.clone()
    }
}

impl AppState {
    fn new(config: Config) -> Self {
        let redis_client_pool =
            Arc::new(ClientsPool::new(&config.client_pool,
                                      Arc::new(RedisClientFactory::new(&config.redis))));
        Self {
            //config: config.into(),
            api_state: ApiState {
                redis_client_pool: redis_client_pool.clone()
            }
        }
    }
}

pub async fn run_server(config: &Config) -> anyhow::Result<()> {

    let app_state = AppState::new(config.clone());

    let mut app = Router::new()
        .route("/version", get(version));

    if let Some(static_dir) = &config.http.static_www {
        let static_svc = ServiceBuilder::new()
            .service(
                ServeDir::new(&static_dir)
                    .append_index_html_on_directories(true)
                    .precompressed_br()
                    .precompressed_gzip(),
            );
        app = app.nest_service("/www", static_svc);
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
    let client_guard = state.redis_client_pool.pop();
    let mut con = client_guard.get_multiplexed_async_connection().await?;
    let val: i32 = con.incr("b", 1).await?;


    Ok(Json(Version {
        value: val,
        version: "0.0.0".to_string(),
    }))
}