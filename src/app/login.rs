use axum::{
    http::{HeaderMap, StatusCode},
    middleware,
    response::Response,
    routing::post,
    Json, Router,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::Value;

use crate::app::model::Claims;

use super::protected::protected_guard;

async fn login_handler(Json(payload): Json<Value>) -> Response<String> {
    println!("{payload:?}");
    let claims = Claims {
        id: 0,
        exp: (1738368000000 / 1000),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    );
    let token = token.unwrap_or_default();
    Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .header("X-Token", token)
        .body("LOGIN HANDLER".to_string())
        .unwrap()
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
    Router::new()
        .route("/", post(login_handler))
        .route("/register", post(register_handler))
        .route(
            "/account",
            post(get_account_handler).route_layer(middleware::from_fn(protected_guard)),
        )
}
