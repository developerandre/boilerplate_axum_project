use axum::{
    http::{HeaderMap, StatusCode},
    middleware,
    routing::post,
    Json, Router,
};
use serde_json::Value;

use super::protected::protected_guard;

async fn login_handler(Json(payload): Json<Value>) -> (StatusCode, &'static str) {
    println!("{payload:?}");
    (StatusCode::OK, "LOGIN HANDLER")
}

async fn register_handler(Json(payload): Json<Value>) -> (StatusCode, &'static str) {
    println!("{payload:?}");
    (StatusCode::OK, "REGISTER HANDLER")
}

async fn get_account_handler(header: HeaderMap) -> (StatusCode, &'static str) {
    println!("{header:?}");
    (StatusCode::OK, "GET ACCOUNT HANDLER")
}

pub fn login_routes() -> Router {
    Router::new().nest(
        "/auth",
        Router::new()
            .route("/", post(login_handler))
            .route("/register", post(register_handler))
            .route(
                "/account",
                post(get_account_handler).route_layer(middleware::from_fn(protected_guard)),
            ),
    )
}
