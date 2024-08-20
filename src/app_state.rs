use sqlx::PgPool;
use tower_sessions_redis_store::fred::prelude::RedisPool;

#[derive(Clone)]
pub struct AppState {
    pub redis_pool: RedisPool,
    pub db_pool: PgPool,
}
