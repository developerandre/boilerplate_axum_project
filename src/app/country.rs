use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    middleware,
    routing::{get, put},
    Json, Router,
};
use serde_json::Value;

use super::protected::protected_guard;

async fn get_handler(Query(params): Query<HashMap<String, String>>) -> (StatusCode, &'static str) {
    println!("{params:?}");
    (StatusCode::OK, "COUNTRY GET HANDLER")
}

async fn post_handler(Json(payload): Json<Value>) -> (StatusCode, &'static str) {
    println!("{payload:?}");
    (StatusCode::OK, "COUNTRY POST HANDLER")
}

async fn put_handler(
    Path(country): Path<u64>,
    Json(payload): Json<Value>,
) -> (StatusCode, &'static str) {
    println!("{country} {payload:?}");
    (StatusCode::OK, "COUNTRY PUT HANDLER")
}

async fn delete_handler(Path(country): Path<u64>) -> (StatusCode, &'static str) {
    println!("{country}");
    (StatusCode::OK, "COUNTRY DELETE HANDLER")
}

pub fn country_routes() -> Router {
    Router::new()
        .nest(
            "/country",
            Router::new()
                .route("/", get(get_handler).post(post_handler))
                .route("/:country", put(put_handler).delete(delete_handler)),
        )
        .route_layer(middleware::from_fn(protected_guard))
}
