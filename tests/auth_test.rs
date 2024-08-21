use axum::{
    body::Body,
    http::{Request, StatusCode},
};

mod common;
use common::fake_email;
use elevate::{get_configuration, Application};
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn auth_test_login_non_existing_user() {
    let configuration = get_configuration().expect("Failed to read configuration");
    let app = Application::new(configuration).build().await.unwrap();

    let request_body = json!({
        "email": fake_email(),
        "password": "password123"
    });

    // try to login with a non existing user
    let request = Request::builder()
        .method("POST")
        .uri("/api/sessions")
        .header("Content-Type", "application/json")
        .body(Body::from(request_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn auth_test_create_user() {
    let configuration = get_configuration().expect("Failed to read configuration");
    let app = Application::new(configuration).build().await.unwrap();

    let request_body = json!({
        "email": fake_email(),
        "password": "password123"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/user")
        .header("Content-Type", "application/json")
        .body(Body::from(request_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn auth_test_create_session() {
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
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // login user
    let request = Request::builder()
        .method("POST")
        .uri("/api/sessions")
        .header("Content-Type", "application/json")
        .body(Body::from(request_body.to_string()))
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Check if the "Set-Cookie" header is present
    let cookies = response.headers().get(axum::http::header::SET_COOKIE);
    assert!(cookies.is_some(), "Set-Cookie header is missing");
}
