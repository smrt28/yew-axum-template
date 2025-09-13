#![allow(unused_imports)]
#![allow(dead_code)]
use std::fmt;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Deserializer};
use crate::app_error::AppError;

fn default_max_clients_count() -> usize {  16 }

#[derive(Debug, Clone)]
pub struct RelativePath {
    path: PathBuf,
}

impl<'de> Deserialize<'de> for RelativePath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let path_str = String::deserialize(deserializer)?;
        Ok(RelativePath {
            path: PathBuf::from(path_str),
        })
    }
}

impl RelativePath {
    pub fn to_path(&self, config: &Config) -> Result<PathBuf, AppError> {
        if let Some(s) = self.path.to_str() {
            if !s.is_empty() && s.starts_with('/') {
                return Ok(self.path.clone());
            }
        }
        if let Some(root) = &config.root {
            Ok(PathBuf::from(root).join(&self.path))
        } else {
            Ok(self.path.clone())
        }
    }
}

impl fmt::Display for RelativePath {
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
    pub path: RelativePath,
    pub target: String,
    pub fallback: Option<String>,
}

impl Mapping {
    pub fn check(&self, _config: &Config) -> Result<(), AppError> {
        if self.target.is_empty() {
            return Err(AppError::GenerateError("Target path is empty".to_string()));
        }
        if !self.target.starts_with('/') {
            return Err(AppError::GenerateError("Target path must start with /".to_string()));
        }
        Ok(())
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Http {
    pub port: u16,

    #[serde(default = "default_max_clients_count")]
    pub max_clients_count: usize,
    pub mappings: Option<Vec<Mapping>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub root: Option<String>,
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