use anyhow::Result;
use redis::{AsyncCommands, Client};

pub struct RedisClient {
    client: Client,
}

impl RedisClient {
    pub fn new(url: &str) -> Result<Self> {
        let client = Client::open(url)?;
        Ok(Self { client })
    }

    pub async fn set(&self, key: &str, value: &str) -> Result<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let _: () = conn.set(key, value).await?;
        Ok(())
    }

    pub async fn hset(&self, key: &str, status: &str, value: &str) -> Result<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let _: () = conn.hset(&key, &status, &value).await?;
        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let value = conn.get(key).await?;
        Ok(value)
    }
}
