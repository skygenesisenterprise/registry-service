use axum::{
    middleware,
    routing::get,
    Router,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::{
    db::PrismaClient,
    routes::{package_routes, user_routes, auth_routes},
    middlewares::{cors_layer, auth_middleware},
};

pub fn create_app(client: PrismaClient) -> Router {
    Router::new()
        .route("/", get(health_check))
        .nest("/api/packages", package_routes())
        .nest("/api/users", user_routes())
        .nest("/api/auth", auth_routes())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors_layer())
        )
        .layer(middleware::from_fn(auth_middleware))
        .with_state(client)
}

async fn health_check() -> &'static str {
    "Registry Service is running!"
}