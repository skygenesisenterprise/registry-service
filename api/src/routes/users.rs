use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use crate::{
    db::PrismaClient,
    models::{UserRequest, UserResponse},
    prisma::{user, user_role},
};

pub fn user_routes() -> Router<PrismaClient> {
    Router::new()
        .route("/", get(get_users).post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
        .route("/:id/packages", get(get_user_packages))
}

async fn get_users(
    State(client): State<PrismaClient>,
) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    let users = client
        .user()
        .find_many()
        .exec()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_responses: Vec<UserResponse> = users
        .into_iter()
        .map(|u| UserResponse {
            id: u.id,
            username: u.username,
            email: u.email,
            role: format!("{:?}", u.role),
            created_at: u.created_at.into(),
        })
        .collect();

    Ok(Json(user_responses))
}

async fn get_user(
    State(client): State<PrismaClient>,
    Path(id): Path<String>,
) -> Result<Json<UserResponse>, StatusCode> {
    let user = client
        .user()
        .find_unique(user::id::equals(id))
        .exec()
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    match user {
        Some(u) => Ok(Json(UserResponse {
            id: u.id,
            username: u.username,
            email: u.email,
            role: format!("{:?}", u.role),
            created_at: u.created_at.into(),
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_user(
    State(client): State<PrismaClient>,
    Json(payload): Json<UserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    let user = client
        .user()
        .create(
            payload.username,
            payload.email,
            payload.password,
            user_role::USER,
            vec![],
        )
        .exec()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(UserResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        role: format!("{:?}", user.role),
        created_at: user.created_at.into(),
    }))
}

async fn update_user(
    State(client): State<PrismaClient>,
    Path(id): Path<String>,
    Json(payload): Json<UserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    let user = client
        .user()
        .update(
            user::id::equals(id),
            vec![
                user::username::set(payload.username),
                user::email::set(payload.email),
                user::password::set(payload.password),
            ],
        )
        .exec()
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(UserResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        role: format!("{:?}", user.role),
        created_at: user.created_at.into(),
    }))
}

async fn delete_user(
    State(client): State<PrismaClient>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    client
        .user()
        .delete(user::id::equals(id))
        .exec()
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(StatusCode::NO_CONTENT)
}

async fn get_user_packages(
    State(client): State<PrismaClient>,
    Path(id): Path<String>,
) -> Result<Json<Vec<crate::models::PackageResponse>>, StatusCode> {
    let packages = client
        .package()
        .find_many(vec![crate::prisma::package::author_id::equals(id)])
        .exec()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let package_responses: Vec<crate::models::PackageResponse> = packages
        .into_iter()
        .map(|p| crate::models::PackageResponse {
            id: p.id,
            name: p.name,
            version: p.version,
            description: p.description,
            maintainer: p.maintainer,
            architecture: p.architecture,
            size: p.size,
            checksum: p.checksum,
            created_at: p.created_at.into(),
            updated_at: p.updated_at.into(),
            author: p.author_id,
            dependencies: vec![],
            tags: vec![],
        })
        .collect();

    Ok(Json(package_responses))
}