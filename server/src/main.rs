mod router;
mod config;
mod client_pool;
mod app_error;

use std::path::PathBuf;
use anyhow::Result;
use crate::config::Config;

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
    let config_path = get_config_path()?;
    Config::read(config_path)?;
    Ok(())
}