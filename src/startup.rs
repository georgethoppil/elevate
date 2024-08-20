use crate::AppState;
use crate::{health_check, Configuration};
use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;

use tower_sessions::cookie::time::Duration;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_redis_store::fred::prelude::{ClientLike, RedisPool};
use tower_sessions_redis_store::fred::types::RedisConfig;
use tower_sessions_redis_store::RedisStore;

pub struct Application {
    config: Configuration,
}

impl Application {
    pub fn build(configuration: Configuration) -> Self {
        Application {
            config: configuration,
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        // set up db pool
        let db_pool = PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .connect_lazy_with(self.config.database.with_db());

        // redis
        let redis_config = RedisConfig::from_url(&self.config.redis.connection_string())?;
        let redis_pool = RedisPool::new(redis_config, None, None, None, 6)?;
        redis_pool.init().await?;
        let redis_store = RedisStore::new(redis_pool.clone());

        // sessions
        let session_layer = SessionManagerLayer::new(redis_store)
            .with_secure(self.config.redis.secure)
            .with_expiry(Expiry::OnInactivity(Duration::seconds(
                self.config.redis.expiry_duration,
            )));

        // app_state
        let app_state = AppState {
            redis_pool,
            db_pool,
        };

        // tcp listener
        let listener = tokio::net::TcpListener::bind(format!(
            "{}:{}",
            self.config.application.host, self.config.application.port
        ))
        .await?;

        // router
        let app = Router::new()
            .route("/health_check", get(health_check))
            .with_state(app_state);

        // serve
        axum::serve(listener, app).await?;
        Ok(())
    }
}
