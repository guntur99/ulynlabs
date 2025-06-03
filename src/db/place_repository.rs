use sqlx::PgPool;
use uuid::Uuid;
use crate::models::{Place, CreatePlaceRequest, UpdatePlaceRequest};
use crate::error::AppError;

pub async fn add_place(pool: &PgPool, new_place: &CreatePlaceRequest /*, user_id: Uuid*/) -> Result<Place, AppError> {
    let place = sqlx::query_as!(
        Place,
        r#"
        INSERT INTO places (nama, deskripsi, latitude, longitude /*, ditambahkan_oleh_user_id */)
        VALUES ($1, $2, $3, $4 /*, $5 */)
        RETURNING id, nama, deskripsi, latitude, longitude, ditambahkan_pada, diperbarui_pada
        "#,
        new_place.nama,
        new_place.deskripsi,
        new_place.latitude,
        new_place.longitude,
        // user_id // Jika ada kolom ditambahkan_oleh_user_id
    )
    .fetch_one(pool)
    .await?;
    Ok(place)
}

pub async fn get_all_places(pool: &PgPool) -> Result<Vec<Place>, AppError> {
    let places = sqlx::query_as!(
        Place,
        r#"
        SELECT id, nama, deskripsi, latitude, longitude, ditambahkan_pada, diperbarui_pada
        FROM places
        WHERE deleted_at IS NULL
        ORDER BY ditambahkan_pada DESC
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(places)
}

pub async fn get_place_by_id(pool: &PgPool, place_id: Uuid) -> Result<Option<Place>, AppError> {
    let place = sqlx::query_as!(
        Place,
        r#"
        SELECT id, nama, deskripsi, latitude, longitude, ditambahkan_pada, diperbarui_pada
        FROM places
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        place_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(place)
}
// Tambahkan fungsi update dan delete jika perlu

pub async fn update_place(pool: &PgPool, place_id: Uuid, updated_place: &UpdatePlaceRequest) -> Result<Place, AppError> {
    let place = sqlx::query_as!(
        Place,
        r#"
        UPDATE places
        SET nama = $1, deskripsi = $2, latitude = $3, longitude = $4, diperbarui_pada = NOW()
        WHERE id = $5
        RETURNING id, nama, deskripsi, latitude, longitude, ditambahkan_pada, diperbarui_pada
        "#,
        updated_place.nama,
        updated_place.deskripsi,
        updated_place.latitude,
        updated_place.longitude,
        place_id
    )
    .fetch_one(pool)
    .await?;
    Ok(place)
}
pub async fn delete_place(pool: &sqlx::PgPool, place_id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE places SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
        place_id
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected()) // Return the number of rows affected
}

// Tambahkan fungsi lain sesuai kebutuhan, misalnya untuk mencari tempat berdasarkan kriteria tertentu
// Jika ada fungsi lain yang perlu ditambahkan, seperti mencari tempat berdasarkan nama atau koordinat, tambahkan di sini.
// Pastikan untuk menangani error dengan baik, misalnya jika tempat tidak ditemukan atau terjadi kesalahan pada database.

