use bb8::Pool;
use bb8_redis::bb8;
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;

#[derive(Clone)]
pub struct RedisDatabase {
    redis_pool: Pool<RedisConnectionManager>,
}

impl RedisDatabase {
    pub async fn new(addr: String) -> Self {
        //redis
        tracing::debug!("connecting to redis");
        let manager = RedisConnectionManager::new(addr).unwrap();
        let redis_pool = bb8::Pool::builder().build(manager).await.unwrap();

        {
            // ping the database before starting
            let mut conn = redis_pool.get().await.unwrap();
            conn.set::<&str, &str, ()>("foo", "bar").await.unwrap();
            let result: String = conn.get("foo").await.unwrap();
            assert_eq!(result, "bar");
        }

        tracing::debug!("successfully connected to redis and pinged it");
        RedisDatabase { redis_pool }
    }
}
