use serde::{Serialize, Deserialize};
use sqlx::FromRow; // Jika menggunakan sqlx untuk mapping dari row DB
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)] // Clone jika diperlukan
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip_serializing)] // Jangan kirim password_hash ke client
    pub password_hash: String,
    pub role: String, // "admin" atau "user"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub role: String, // "admin" atau "user"
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub role: String,
    pub token: String, // JWT token
}

// Claims untuk JWT sudah bisa diletakkan di auth/jwt.rs