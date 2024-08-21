use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use chrono::{DateTime, Utc};
use elevate::{get_configuration, Application, ResponseBody};

use serde_json::json;
use tower::ServiceExt;

use http_body_util::BodyExt;
mod common;
use common::fake_email;

#[tokio::test]
async fn auth_test_user_with_no_game_entry() {
    let configuration = get_configuration().expect("Failed to read configuration");
    let app = Application::new(configuration).build().await.unwrap();

    let email = fake_email();
    let request_body = json!({
        "email": email,
        "password": "password123"
    });

    // create user
    let request = Request::builder()
        .method("POST")
        .uri("/api/user")
        .header("Content-Type", "application/json")
        .body(Body::from(request_body.to_string()))
        .unwrap();
    app.clone().oneshot(request).await.unwrap();

    // login user
    let request = Request::builder()
        .method("POST")
        .uri("/api/sessions")
        .header("Content-Type", "application/json")
        .body(Body::from(request_body.to_string()))
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();

    let cookies = response.headers().get(axum::http::header::SET_COOKIE);
    assert!(cookies.is_some(), "Set-Cookie header is missing");

    // extract cookie
    let cookie = cookies.unwrap();

    // get user stats
    let request = Request::builder()
        .method("GET")
        .uri("/api/user")
        .header("Content-Type", "application/json")
        .header("Cookie", cookie)
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // should assert to 0
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let response_body: ResponseBody = serde_json::from_slice(&body).unwrap();

    assert_eq!(response_body.user.stats.total_games_played, 0);
}

#[tokio::test]
async fn auth_test_user_with_game_entries() {
    let configuration = get_configuration().expect("Failed to read configuration");
    let app = Application::new(configuration).build().await.unwrap();

    let email = fake_email();
    let request_body = json!({
        "email": email,
        "password": "password123"
    });

    // create user
    let request = Request::builder()
        .method("POST")
        .uri("/api/user")
        .header("Content-Type", "application/json")
        .body(Body::from(request_body.to_string()))
        .unwrap();
    app.clone().oneshot(request).await.unwrap();

    // login user
    let request = Request::builder()
        .method("POST")
        .uri("/api/sessions")
        .header("Content-Type", "application/json")
        .body(Body::from(request_body.to_string()))
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();

    let cookies = response.headers().get(axum::http::header::SET_COOKIE);
    assert!(cookies.is_some(), "Set-Cookie header is missing");

    // extract cookie
    let cookie = cookies.unwrap();
    let datetime: DateTime<Utc> = Utc::now();

    let request_body = json!({
        "occurred_at": datetime.timestamp_millis(),
    });

    // send a completed game
    let request = Request::builder()
        .method("POST")
        .uri("/api/user/game_events")
        .header("Content-Type", "application/json")
        .header("Cookie", cookie)
        .body(Body::from(request_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // get user stats
    let request = Request::builder()
        .method("GET")
        .uri("/api/user")
        .header("Content-Type", "application/json")
        .header("Cookie", cookie)
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // should assert to 1
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let response_body: ResponseBody = serde_json::from_slice(&body).unwrap();

    assert_eq!(response_body.user.stats.total_games_played, 1);
}
