use crate::{
    game_event_handler, game_summary_handler, login_handler, signup_handler, Configuration,
};
use crate::{AppState, Backend};
use axum::routing::post;
use axum::{routing::get, Router};
use axum_login::{login_required, AuthManagerLayerBuilder};
use sqlx::postgres::PgPoolOptions;

use tower_sessions::cookie::time::Duration;
use tower_sessions::cookie::Key;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_redis_store::fred::prelude::{ClientLike, RedisPool};
use tower_sessions_redis_store::fred::types::RedisConfig;
use tower_sessions_redis_store::RedisStore;

pub struct Application {
    config: Configuration,
}

impl Application {
    pub fn new(configuration: Configuration) -> Self {
        Application {
            config: configuration,
        }
    }

    pub async fn build(self) -> Result<Router, Box<dyn std::error::Error>> {
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
        let key = Key::generate();
        let session_layer = SessionManagerLayer::new(redis_store)
            .with_secure(self.config.redis.secure)
            .with_expiry(Expiry::OnInactivity(Duration::seconds(
                self.config.redis.expiry_duration,
            )))
            .with_signed(key);

        // auth service
        let backend = Backend::new(db_pool.clone());
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

        // app_state
        let app_state = AppState {
            redis_pool,
            db_pool,
        };

        // router
        let app = Router::new()
            .route("/api/user", get(game_summary_handler)) // Game summary
            .route("/api/user/game_events", post(game_event_handler)) // Game event route
            .route_layer(login_required!(Backend))
            .route("/api/user", post(signup_handler)) // Signup route
            .route("/api/sessions", post(login_handler)) // Login route
            .layer(auth_layer)
            .with_state(app_state.db_pool);

        Ok(app)
    }
}
