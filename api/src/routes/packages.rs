use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::Deserialize;
use std::collections::HashMap;
use crate::{
    db::PrismaClient,
    models::{PackageRequest, PackageResponse},
    prisma::{package, tag, user, package_dependency, package_tag, dependency_type},
};

#[derive(Deserialize)]
pub struct PackageQuery {
    pub search: Option<String>,
    pub tag: Option<String>,
    pub maintainer: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub fn package_routes() -> Router<PrismaClient> {
    Router::new()
        .route("/", get(get_packages).post(create_package))
        .route("/:id", get(get_package).put(update_package).delete(delete_package))
        .route("/:id/download", get(download_package))
        .route("/search/:query", get(search_packages))
}

async fn get_packages(
    State(client): State<PrismaClient>,
    Query(params): Query<PackageQuery>,
) -> Result<Json<Vec<PackageResponse>>, StatusCode> {
    let mut query = client.package().find_many();

    if let Some(search) = params.search {
        query = query.with(package::name::contains(search));
    }

    if let Some(maintainer) = params.maintainer {
        query = query.with(package::maintainer::equals(maintainer));
    }

    let packages = query
        .with(package::author::fetch())
        .with(package::dependencies::fetch(vec![]))
        .with(package::tags::fetch(vec![]))
        .exec()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let package_responses: Vec<PackageResponse> = packages
        .into_iter()
        .map(|p| PackageResponse {
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

async fn get_package(
    State(client): State<PrismaClient>,
    Path(id): Path<String>,
) -> Result<Json<PackageResponse>, StatusCode> {
    let package = client
        .package()
        .find_unique(package::id::equals(id))
        .with(package::author::fetch())
        .with(package::dependencies::fetch(vec![]))
        .with(package::tags::fetch(vec![]))
        .exec()
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    match package {
        Some(p) => Ok(Json(PackageResponse {
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
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_package(
    State(client): State<PrismaClient>,
    Json(payload): Json<PackageRequest>,
) -> Result<Json<PackageResponse>, StatusCode> {
    let checksum = format!("{:x}", sha2::Sha256::digest(&payload.name));
    
    let package = client
        .package()
        .create(
            payload.name.clone(),
            payload.version.clone(),
            payload.maintainer,
            payload.architecture,
            payload.size,
            checksum,
            payload.description,
            format!("/packages/{}/{}-{}.deb", payload.name, payload.name, payload.version),
            user::id::equals("default_user".to_string()),
        )
        .exec()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(PackageResponse {
        id: package.id,
        name: package.name,
        version: package.version,
        description: package.description,
        maintainer: package.maintainer,
        architecture: package.architecture,
        size: package.size,
        checksum: package.checksum,
        created_at: package.created_at.into(),
        updated_at: package.updated_at.into(),
        author: package.author_id,
        dependencies: vec![],
        tags: vec![],
    }))
}

async fn update_package(
    State(client): State<PrismaClient>,
    Path(id): Path<String>,
    Json(payload): Json<PackageRequest>,
) -> Result<Json<PackageResponse>, StatusCode> {
    let package = client
        .package()
        .update(
            package::id::equals(id),
            vec![
                package::name::set(payload.name),
                package::version::set(payload.version),
                package::description::set(payload.description),
                package::maintainer::set(payload.maintainer),
                package::architecture::set(payload.architecture),
                package::size::set(payload.size),
            ],
        )
        .exec()
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(PackageResponse {
        id: package.id,
        name: package.name,
        version: package.version,
        description: package.description,
        maintainer: package.maintainer,
        architecture: package.architecture,
        size: package.size,
        checksum: package.checksum,
        created_at: package.created_at.into(),
        updated_at: package.updated_at.into(),
        author: package.author_id,
        dependencies: vec![],
        tags: vec![],
    }))
}

async fn delete_package(
    State(client): State<PrismaClient>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    client
        .package()
        .delete(package::id::equals(id))
        .exec()
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(StatusCode::NO_CONTENT)
}

async fn download_package(
    State(_client): State<PrismaClient>,
    Path(id): Path<String>,
) -> Result<String, StatusCode> {
    Ok(format!("Download package with id: {}", id))
}

async fn search_packages(
    State(client): State<PrismaClient>,
    Path(query): Path<String>,
) -> Result<Json<Vec<PackageResponse>>, StatusCode> {
    let packages = client
        .package()
        .find_many(vec![
            package::name::contains(query.clone()),
            package::description::contains(query),
        ])
        .exec()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let package_responses: Vec<PackageResponse> = packages
        .into_iter()
        .map(|p| PackageResponse {
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