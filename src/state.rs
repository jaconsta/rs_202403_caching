use fred::{
    clients::RedisPool,
    interfaces::{ClientLike, KeysInterface},
    prelude::*,
    types::{Expiration, SetOptions},
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use simple_error::simple_error;
use sqlx::PgPool;

// App adapters
pub struct StateInternal {
    pub database: PgPool,
    pub cache: Cache,
}

impl StateInternal {
    pub fn new(db: PgPool, redis: RedisPool) -> Self {
        StateInternal {
            database: db,
            cache: Cache::new(redis),
        }
    }
}

pub struct Cache {
    redis: RedisPool,
}

impl Cache {
    pub fn new(redis: RedisPool) -> Self {
        Cache { redis }
    }

    fn id_into_key(id: i64) -> String {
        format!("contact:{}", id)
    }

    pub async fn get<T>(&self, id: i64) -> Result<Option<T>, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned,
    {
        if !self.redis.is_connected() {
            return Err(Box::new(simple_error!("Redis is not connected.")));
        }
        let value: Option<Value> = self.redis.get(Self::id_into_key(id)).await?;
        let res = match value {
            Some(x) => match serde_json::from_value(x) {
                Ok(x) => Some(x),
                Err(_) => None,
            },
            None => None,
        };
        Ok(res)
    }

    // Alias for `SET contact:1 '{...}'`
    pub async fn set<T>(
        &self,
        id: i64,
        obj: T,
        expiration: Option<Expiration>,
        opts: Option<SetOptions>,
        get: bool,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Serialize,
    {
        if !self.redis.is_connected() {
            return Err(Box::new(simple_error!("Redis is not connected.")));
        }

        let value: Value = serde_json::to_value(obj)?;
        let key = Self::id_into_key(id);
        self.redis
            .set(key, value.to_string(), expiration, opts, get)
            .await?;
        Ok(())
    }

    pub async fn del(&self, id: i64) -> Result<(), Box<dyn std::error::Error>> {
        if !self.redis.is_connected() {
            return Err(Box::new(simple_error!("Redis is not connected.")));
        }

        let key = Self::id_into_key(id);

        self.redis.del(key).await?;
        Ok(())
    }
}

pub type AppState = std::sync::Arc<StateInternal>;
