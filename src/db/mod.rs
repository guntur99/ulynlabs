pub mod user_repository;
pub mod place_repository;

use sqlx::{postgres::PgPoolOptions, PgPool, Error};

pub async fn create_db_pool(database_url: &str) -> Result<PgPool, Error> {
    PgPoolOptions::new()
        .max_connections(10) // Sesuaikan dengan kebutuhan
        .connect(database_url)
        .await
}