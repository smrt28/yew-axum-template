use crate::client_pool::{PollableClientFactory};
use crate::config::RedisConfig;

pub struct RedisClientFactory {
    pub redis_uri: String,
}

impl PollableClientFactory<redis::Client> for RedisClientFactory {
    fn build_client(&self) -> redis::Client {
        redis::Client::open(self.redis_uri.as_str()).unwrap()
    }
}

impl RedisClientFactory {
    pub fn new(config: &RedisConfig) -> Self {
        Self {
            redis_uri: config.uri.clone(),
        }
    }
}
