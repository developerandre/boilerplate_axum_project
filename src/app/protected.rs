use axum::{
    extract::Request,
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn protected_guard(req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth = req.headers().get(AUTHORIZATION);
    match auth {
        Some(a) => {
            let bearer = a.to_str().unwrap_or_default();
            println!("{bearer}");
            Ok(next.run(req).await)
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}
