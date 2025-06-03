// src/auth/jwt.rs
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use uuid::Uuid;
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid, // User ID
    pub role: String, // "admin" atau "user"
    pub exp: i64,   // Expiration time (timestamp)
    pub iat: i64,   // Issued at (timestamp)
}

pub fn generate_jwt(user_id: Uuid, role: &str, secret: &str) -> Result<String, AppError> {
    let now = Utc::now();
    let expiration = now + Duration::hours(24); // Token berlaku 24 jam

    let claims = Claims {
        sub: user_id,
        role: role.to_owned(),
        exp: expiration.timestamp(),
        iat: now.timestamp(),
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
        .map_err(AppError::from)
}

pub fn validate_jwt(token: &str, secret: &str) -> Result<Claims, AppError> {
    let mut validation = Validation::new(Algorithm::HS256); // Sesuaikan algoritma jika berbeda
    validation.validate_exp = true; // Validasi expiration time
    // validation.leeway = 60; // Toleransi waktu (detik)

    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &validation)
        .map(|data| data.claims)
        .map_err(AppError::from)
}