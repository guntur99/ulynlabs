// src/main.rs
mod app_state;
mod config;
mod db;
mod error;
mod handlers;
mod models;
mod auth;
mod services;
mod utils; // Jika ada

use axum::{Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tower_http::cors::{Any, CorsLayer}; // Untuk CORS
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tokio::net::TcpListener; // ADDED: Import TcpListener

use app_state::AppState;
use config::Config;
use db::create_db_pool; // Fungsi dari db/mod.rs atau db.rs

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "rute_axum_api=debug,tower_http=debug,axum::rejection=trace".into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");

    // Create database pool
    let db_pool = create_db_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");

    // (Opsional) Jalankan migrasi jika menggunakan sqlx-cli
    // sqlx::migrate!("./migrations")
    //     .run(&db_pool)
    //     .await
    //     .expect("Failed to run database migrations");

    // Create HTTP client untuk Google Maps API
    let http_client = reqwest::Client::new();

    // Create shared state
    let shared_state = Arc::new(AppState {
        db_pool,
        jwt_secret: config.jwt_secret.clone(),
        maps_api_key: config.maps_api_key.clone(),
        http_client,
    });

    // CORS Middleware
    let cors = CorsLayer::new()
        .allow_origin(Any) // Sesuaikan dengan kebutuhan (misal: spesifik origin frontend)
        .allow_methods(Any)
        .allow_headers(Any);

    // Define routes
    let app = Router::new()
        .merge(handlers::auth_handler::auth_routes(shared_state.clone()))
        .merge(handlers::admin_handler::admin_routes(shared_state.clone()))
        .merge(handlers::user_handler::user_routes(shared_state.clone()))
        .layer(TraceLayer::new_for_http()) // Middleware untuk logging request/response
        .layer(cors) // Middleware CORS
        .with_state(shared_state); // Menyediakan state ke semua handler

    let addr_str = format!("{}:{}", config.server_address, config.server_port);
    let addr = addr_str.parse::<SocketAddr>()
        .expect("Invalid server address format");

    tracing::info!("🚀 Server listening on {}", addr);

    // CHANGED: Use tokio::net::TcpListener and axum::serve instead of axum::Server
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}