#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct ClientPoolConfig {
    pub max_clients_count: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RedisConfig {
    pub uri: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Http {
    pub port: u16,
    pub static_www: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub http: Http,
    pub redis: RedisConfig,
    pub client_pool: ClientPoolConfig,
}


impl Config {
    pub fn read(path: PathBuf) -> Result<Config, anyhow::Error> {
        let contents = fs::read_to_string(path)?;
        Ok(toml::from_str::<Config>(&contents)?)
    }

    pub fn sanitize(self) -> Result<Self, anyhow::Error> {
        if self.client_pool.max_clients_count >= 1000 {
            return Err(anyhow::anyhow!("Max clients count must be less than 1000"));
        }

        Ok(self)
    }
}