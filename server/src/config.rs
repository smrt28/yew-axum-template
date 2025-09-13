#![allow(unused_imports)]
#![allow(dead_code)]
use std::fmt;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Deserializer};

fn default_max_clients_count() -> usize {  16 }
fn default_root() -> String {  "/".into() }

#[derive(Debug, Clone)]
pub struct ConfigPath {
    pub path: PathBuf,
}

impl<'de> Deserialize<'de> for ConfigPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let path_str = String::deserialize(deserializer)?;
        Ok(ConfigPath {
            path: PathBuf::from(path_str),
        })
    }
}

impl fmt::Display for ConfigPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}]", self.path.to_str().unwrap_or("/"))
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct RedisConfig {
    pub uri: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Mapping {
    pub path: ConfigPath,
    pub target: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Http {
    pub port: u16,
    pub static_www: Option<String>,

    #[serde(default = "default_max_clients_count")]
    pub max_clients_count: usize,

    pub mappings: Option<Vec<Mapping>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default = "default_root")]
    pub root: String,
    pub http: Http,
    pub redis: RedisConfig,
}


impl fmt::Display for Mapping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " [path: {}, target: {}]", self.path, self.target)
    }
}

impl Config {
    pub fn read(path: PathBuf) -> Result<Config, anyhow::Error> {
        let contents = fs::read_to_string(path)?;
        Ok(toml::from_str::<Config>(&contents)?)
    }

    pub fn sanitize(self) -> Result<Self, anyhow::Error> {
        if self.http.max_clients_count >= 1000 {
            return Err(anyhow::anyhow!("Max clients count must be less than 1000"));
        }

        Ok(self)
    }
}