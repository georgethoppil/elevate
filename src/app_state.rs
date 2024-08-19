use std::sync::Arc;

use sqlx::PgPool;

use crate::RedisDatabase;

#[derive(Clone)]

pub struct AppState {
    pub redis_database: RedisDatabase,
    pub db_pool: Arc<PgPool>,
}
