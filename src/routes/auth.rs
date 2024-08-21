use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_macros::debug_handler;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{AuthSession, Credentials};

#[debug_handler]
pub async fn login_handler(
    mut auth_session: AuthSession,
    Json(creds): Json<Credentials>,
) -> impl IntoResponse {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            tracing::debug!("invalid creds");
            return StatusCode::UNAUTHORIZED.into_response();
        }
        Err(e) => {
            tracing::debug!("{}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    tracing::debug!("Successfully logged in as {}", user.email);
    StatusCode::OK.into_response()
}
#[debug_handler]
pub async fn signup_handler(
    State(db_pool): State<PgPool>,
    Json(creds): Json<Credentials>,
) -> impl IntoResponse {
    let user_id = Uuid::new_v4();
    let password_hash = password_auth::generate_hash(creds.password);

    let result = sqlx::query!(
        r#"
        INSERT INTO users (user_id, email, password)
        VALUES ($1, $2, $3)
        "#,
        user_id,
        creds.email,
        password_hash,
    )
    .execute(&db_pool)
    .await;

    match result {
        Ok(_) => {
            tracing::debug!("created user {}", creds.email);
            return StatusCode::CREATED.into_response();
        }
        Err(e) => {
            tracing::debug!("error creating user {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
