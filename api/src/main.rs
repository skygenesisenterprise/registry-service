mod config;
mod db;
mod models;
mod routes;
mod middlewares;

use axum::Router;
use db::PrismaClient;
use config::create_app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let client = PrismaClient::_builder()
        .build()
        .await?;

    let app = create_app(client);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Server running on http://0.0.0.0:3000");
    
    axum::serve(listener, app).await?;

    Ok(())
}