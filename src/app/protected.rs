use axum::{
    extract::Request,
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde_json::{json, Value};

pub async fn protected_guard(
    req: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<Value>)> {
    let auth = req.headers().get(AUTHORIZATION);
    if let Some(a) = auth {
        let token = a.to_str().unwrap_or_default().replace("Bearer", "");
        let result = decode::<serde_json::Value>(
            token.trim(),
            &DecodingKey::from_secret("".as_ref()),
            &Validation::new(Algorithm::HS256),
        );
        match result {
            Ok(r) => {
                println!("{:?}", r);
                return Ok(next.run(req).await);
            }
            Err(e) => {
                println!("{e:?}");
            }
        }
    }
    Err((
        StatusCode::UNAUTHORIZED,
        Json(json!({
            "code": "INVALID_TOKEN",
                    "message": "Invalid token, please Log in first"
        })),
    ))
}
