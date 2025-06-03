// src/auth/middleware.rs
use axum::{
    async_trait,
    extract::{
        FromRequestParts, 
        // State
    },
    http::{
        request::Parts, 
        // StatusCode, 
        // HeaderMap, 
        Request
    },
    response::{Response, IntoResponse},
    middleware::Next,
    // Json,
    body::Body,
};
use std::sync::Arc;
use crate::app_state::AppState;
use crate::auth::jwt::{
    validate_jwt, 
    // Claims
};
use crate::error::{
    AppError, 
    // AppResult
}; // Menggunakan AppError dan AppResult
use crate::models::User; // Untuk menyimpan User yang sudah di-resolve
use crate::db::user_repository;
use axum::extract::FromRef;


// Extractor untuk mendapatkan user yang terautentikasi
pub struct AuthUser(pub User); // Menyimpan struct User lengkap, bukan hanya Claims

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    Arc<AppState>: FromRef<S>, // Memastikan AppState bisa di-extract dari S
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state: Arc<AppState> = Arc::from_ref(state);

        let token = parts.headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|auth_header| auth_header.to_str().ok())
            .and_then(|auth_value| {
                auth_value.strip_prefix("Bearer ")
            })
            .ok_or_else(|| AppError::AuthenticationError("Missing Bearer token.".to_string()))?;

        let claims = validate_jwt(token, &app_state.jwt_secret)?;

        // Ambil detail user dari database berdasarkan claims.sub (user_id)
        let user = user_repository::find_user_by_id(&app_state.db_pool, claims.sub)
            .await? // Propagate sqlx::Error atau NotFoundError
            .ok_or_else(|| AppError::AuthenticationError(format!("User {} not found.", claims.sub)))?;

        // Cek apakah role di token masih sesuai dengan di DB (opsional, tapi baik)
        if user.role.to_lowercase() != claims.role.to_lowercase() { // Added .to_lowercase() for robustness
             return Err(AppError::AuthenticationError("User role mismatch.".to_string()));
        }

        Ok(AuthUser(user))
    }
}

// Middleware untuk rute yang hanya bisa diakses user terautentikasi
pub async fn user_required( // ADDED: Trait bound for B
    AuthUser(_user): AuthUser, // Cukup pastikan user terautentikasi
    request: Request<Body>,
    next: Next,
) -> Response
where
    Body: Send + 'static, // Ensure Body is Axum's Body type and meets Send/Static requirements
{
    next.run(request).await
}

// Middleware untuk rute yang hanya bisa diakses admin
pub async fn admin_required( // ADDED: Trait bound for B
    AuthUser(user): AuthUser, // Ekstrak User dari AuthUser
    request: Request<Body>,
    next: Next,
) -> Response {
    if user.role.to_lowercase() != "admin" {
        AppError::AuthorizationError("Admin privileges required.".to_string()).into_response()
    } else {
        next.run(request).await
    }
}