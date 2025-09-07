use crate::client_pool::{ClientPoolConfig, PollableClientFactory};

struct RedisClientFactory {
    redis_uri: String,
    pool_config: ClientPoolConfig,
}

impl PollableClientFactory<redis::Client> for RedisClientFactory {
    fn build_client(&self) -> redis::Client {
        redis::Client::open(self.redis_uri.as_str()).unwrap()
    }
    fn get_config(&self) -> &ClientPoolConfig {
        &self.pool_config
    }
}
