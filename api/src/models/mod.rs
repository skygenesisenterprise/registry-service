use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageRequest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub maintainer: String,
    pub architecture: String,
    pub size: i64,
    pub dependencies: Vec<DependencyRequest>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyRequest {
    pub name: String,
    pub version: String,
    pub dependency_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageResponse {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub maintainer: String,
    pub architecture: String,
    pub size: i64,
    pub checksum: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub author: String,
    pub dependencies: Vec<DependencyResponse>,
    pub tags: Vec<TagResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyResponse {
    pub name: String,
    pub version: String,
    pub dependency_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagResponse {
    pub name: String,
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}