use sqlx::PgPool;
use uuid::Uuid;
use crate::models::User; // Asumsi User ada di crate::models
use crate::error::AppError; // Atau crate::error::AppResult

pub async fn create_user(pool: &PgPool, username: &str, password_hash: &str, role: &str) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, password_hash, role)
        VALUES ($1, $2, $3)
        RETURNING id, username, password_hash, role, created_at, updated_at
        "#,
        username,
        password_hash,
        role
    )
    .fetch_one(pool)
    .await?;
    Ok(user)
}

pub async fn find_user_by_username(pool: &PgPool, username: &str) -> Result<Option<User>, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password_hash, role, created_at, updated_at
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await?;
    Ok(user)
}

pub async fn find_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password_hash, role, created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(user)
}