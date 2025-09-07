#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct RedisConfig {
    pub uri: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct HttpConfig {
    pub port: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub redis: Option<RedisConfig>,
}


impl Config {
    pub fn read(path: PathBuf) -> Result<Config, anyhow::Error> {
        let contents = fs::read_to_string(path)?;
        Ok(toml::from_str::<Config>(&contents)?)
    }
}