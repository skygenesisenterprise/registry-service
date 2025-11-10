use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::post,
    Router,
};
use crate::{
    db::PrismaClient,
    models::{LoginRequest, AuthResponse},
    prisma::{user},
};

pub fn auth_routes() -> Router<PrismaClient> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/logout", post(logout))
}

async fn login(
    State(client): State<PrismaClient>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let user = client
        .user()
        .find_first(vec![user::username::equals(payload.username)])
        .exec()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match user {
        Some(u) if u.password == payload.password => {
            let token = format!("token_for_{}", u.username);
            Ok(Json(AuthResponse {
                token,
                user: crate::models::UserResponse {
                    id: u.id,
                    username: u.username,
                    email: u.email,
                    role: format!("{:?}", u.role),
                    created_at: u.created_at.into(),
                },
            }))
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

async fn register(
    State(client): State<PrismaClient>,
    Json(payload): Json<crate::models::UserRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let user = client
        .user()
        .create(
            payload.username.clone(),
            payload.email,
            payload.password,
            crate::prisma::user_role::USER,
            vec![],
        )
        .exec()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let token = format!("token_for_{}", user.username);
    Ok(Json(AuthResponse {
        token,
        user: crate::models::UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            role: format!("{:?}", user.role),
            created_at: user.created_at.into(),
        },
    }))
}

async fn logout() -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::OK)
}