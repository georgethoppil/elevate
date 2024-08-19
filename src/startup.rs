use std::sync::Arc;

use crate::AppState;
use crate::RedisDatabase;
use crate::{health_check, Configuration};
use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;

pub struct Application {
    config: Configuration,
}

impl Application {
    pub fn build(configuration: Configuration) -> Self {
        Application {
            config: configuration,
        }
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        // set up db pool
        let connection_pool = PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .connect_lazy_with(self.config.database.with_db());

        // redis
        let redis_database = RedisDatabase::new(self.config.redis.connection_string()).await;

        // app_state
        let app_state = AppState {
            redis_database,
            db_pool: Arc::new(connection_pool),
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
