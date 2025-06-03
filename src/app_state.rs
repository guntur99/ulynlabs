// src/app_state.rs
use sqlx::PgPool;
use reqwest::Client;

#[derive(Clone)] // Penting agar bisa di-clone untuk Arc
pub struct AppState {
    pub db_pool: PgPool,
    pub jwt_secret: String,
    pub maps_api_key: String,
    pub http_client: Client, // Untuk memanggil Google Maps API
}