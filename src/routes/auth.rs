use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{AuthSession, Credentials};

pub async fn login(
    mut auth_session: AuthSession,
    Json(creds): Json<Credentials>,
) -> impl IntoResponse {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            tracing::debug!("invalid creds");
            return StatusCode::UNAUTHORIZED.into_response();
        }
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    tracing::debug!("Successfully logged in as {}", user.email);

    return Redirect::to("/").into_response();
}

pub async fn signup(
    Json(creds): Json<Credentials>,
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    let user_id = Uuid::new_v4();
    let password_hash = password_auth::generate_hash(creds.password);

    // let result = sqlx::query!(
    //     r#"
    //     INSERT INTO users (email, password_hash)
    //     VALUES ($1, $2)
    //     RETURNING id
    //     "#,
    //     creds.email,
    //     password_hash,
    // )
    // .fetch_one(&pool)
    // .await;

    return StatusCode::CREATED;
}
