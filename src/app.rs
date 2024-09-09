use self::{country::country_routes, login::login_routes};
use axum::{extract::DefaultBodyLimit, http::StatusCode, Json, Router};
use serde_json::json;
use tower_http::{compression::CompressionLayer, validate_request::ValidateRequestHeaderLayer};

mod country;
mod login;
mod model;
mod protected;

pub fn app_axum() -> Router {
    let routes = Router::new()
        .nest("/auth", login_routes())
        .nest("/country", country_routes())
        .layer(DefaultBodyLimit::max(1024))
        .layer(CompressionLayer::new())
        .layer(ValidateRequestHeaderLayer::accept("application/json"));

    Router::new().nest("/api", routes).fallback(fallback)
}

async fn fallback() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "error":"Lien invalide"
        })),
    )
}
