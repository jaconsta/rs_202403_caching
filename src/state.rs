use fred::clients::RedisPool;
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
}
