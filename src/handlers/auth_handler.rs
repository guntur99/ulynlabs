// src/handlers/auth_handler.rs
use axum::{
    routing::post,
    Router, Json, extract::State,
};
use std::sync::Arc;
use crate::app_state::AppState;
use crate::models::{RegisterRequest, LoginRequest, UserResponse};
use crate::db::user_repository;
use crate::auth::{generate_jwt, hash_password, verify_password};
use crate::error::{AppError, AppResult};

pub fn auth_routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/register", post(register_handler))
        .route("/auth/login", post(login_handler))
        .with_state(state) // State diteruskan ke router ini
}

async fn register_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<Json<UserResponse>> {
    if payload.password.len() < 8 {
        return Err(AppError::ValidationError("Password must be at least 8 characters long.".to_string()));
    }

    // Cek apakah username sudah ada
    if user_repository::find_user_by_username(&state.db_pool, &payload.username).await?.is_some() {
        return Err(AppError::ValidationError("Username already exists.".to_string()));
    }

    let hashed_password = hash_password(&payload.password)?;
    let user_role = &payload.role; // Default role

    let new_user = user_repository::create_user(&state.db_pool, &payload.username, &hashed_password, user_role).await?;
    let token = generate_jwt(new_user.id, &new_user.role, &state.jwt_secret)?;

    Ok(Json(UserResponse {
        id: new_user.id,
        username: new_user.username,
        role: new_user.role,
        token,
    }))
}

async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<UserResponse>> {
    let user = user_repository::find_user_by_username(&state.db_pool, &payload.username)
        .await?
        .ok_or_else(|| AppError::AuthenticationError("Invalid username or password.".to_string()))?;

    if !verify_password(&user.password_hash, &payload.password)? {
        return Err(AppError::AuthenticationError("Invalid username or password.".to_string()));
    }

    let token = generate_jwt(user.id, &user.role, &state.jwt_secret)?;
    Ok(Json(UserResponse {
        id: user.id,
        username: user.username,
        role: user.role,
        token,
    }))
}