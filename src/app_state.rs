use bb8::Pool;
use bb8_redis::bb8;
use bb8_redis::RedisConnectionManager;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub redis_pool: Pool<RedisConnectionManager>,
    pub db_pool: PgPool,
}
