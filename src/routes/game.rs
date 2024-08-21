use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_macros::debug_handler;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::AuthSession;

#[derive(Deserialize, Serialize, Debug)]
pub struct GamePayload {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub occurred_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct UserStats {
    pub total_games_played: i64,
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    id: Uuid,
    email: String,
    pub stats: UserStats,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseBody {
    pub user: UserResponse,
}

#[debug_handler]
pub async fn game_event_handler(
    auth_session: AuthSession,
    State(db_pool): State<PgPool>,
    Json(game_payload): Json<GamePayload>,
) -> impl IntoResponse {
    match auth_session.user {
        Some(user) => {
            let game_id = Uuid::new_v4();
            let result = sqlx::query!(
                r#"
                INSERT INTO games (game_id, user_id, type, occurred_at)
                VALUES ($1, $2, $3, $4)
                "#,
                game_id,
                user.user_id,
                "COMPLETED",
                game_payload.occurred_at.naive_utc(),
            )
            .execute(&db_pool)
            .await;

            match result {
                Ok(_) => {
                    tracing::debug!("inserted game event for {}", user.email);
                    return StatusCode::OK.into_response();
                }
                Err(e) => {
                    tracing::debug!("Error inserting game: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
        None => StatusCode::FORBIDDEN.into_response(),
    }
}

#[debug_handler]
pub async fn game_summary_handler(
    auth_session: AuthSession,
    State(db_pool): State<PgPool>,
) -> impl IntoResponse {
    match auth_session.user {
        Some(user) => {
            let result = sqlx::query!(
                "SELECT COUNT(game_id) FROM games WHERE user_id = $1",
                user.user_id
            )
            .fetch_optional(&db_pool)
            .await;

            let game_count: i64 = match result {
                Ok(Some(row)) => row.count.unwrap_or(0), // Default to 0 if count is NULL
                Ok(None) => 0,                           // No row found, default to 0
                Err(e) => {
                    tracing::debug!("Error fetching game summary: {:?}", e);
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
            };

            let response = ResponseBody {
                user: UserResponse {
                    id: user.user_id,
                    email: user.email,
                    stats: UserStats {
                        total_games_played: game_count,
                    },
                },
            };

            return Json(response).into_response();
        }
        None => StatusCode::FORBIDDEN.into_response(),
    }
}
