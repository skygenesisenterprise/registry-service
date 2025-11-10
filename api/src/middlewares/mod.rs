use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use tower_http::cors::{Any, CorsLayer};

pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}

pub async fn auth_middleware<B>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = req.headers().get("authorization");
    
    match auth_header {
        Some(header) if header.to_str().unwrap_or("").starts_with("Bearer ") => {
            Ok(next.run(req).await)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}