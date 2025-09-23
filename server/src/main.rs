mod router;
mod config;
mod app_error;

use boa_engine::{
    Context, JsResult, JsValue, Source, js_string,
    value::{TryFromJs, TryIntoJs},
};

use std::path::PathBuf;
use anyhow::Result;
use log::{error, info};
//use tracing::instrument::WithSubscriber;
use crate::config::Config;
use crate::router::run_server;

fn get_config_path() -> Result<PathBuf> {
    #[cfg(not(debug_assertions))]
    {
        let args: Vec<String> = env::args().collect();
        let config_path = args.get(1).ok_or_else(|| {
            anyhow::anyhow!("Config file path must be passed as first argument")
        })?;
        Ok(PathBuf::from(config_path))
    }
    #[cfg(debug_assertions)]
    {
        let config_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("assets")
            .join("config.toml");
        let config_path = config_file.to_str().ok_or_else(|| {
            anyhow::anyhow!("Invalid UTF-8 in config path")
        })?;
        Ok(PathBuf::from(config_path))
    }
}


#[tokio::main]
async fn main() -> Result<()> {
/*
    let js_code = r#"
function hello() {
    return false;
}
hello();
"#;
    let mut context = Context::default();
    if let Ok(res) = context.eval(Source::from_bytes(js_code)) {
        println!("{:#?}", res);
    }

*/


    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .init();

    info!("starting server");
    let config_path = get_config_path()?;
    let config = Config::read(config_path)?.sanitize()?;
    if let Err(e) = run_server(&config).await {
        error!("Server exited with error: {}", e);
    }

    Ok(())
}