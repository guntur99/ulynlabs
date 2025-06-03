// src/handlers/admin_handler.rs
use axum::{
    extract::{Path, State}, middleware, routing::{delete, get, post, put}, Json, Router
};
use std::sync::Arc;
use uuid::Uuid;
use crate::app_state::AppState;
use crate::models::{CreatePlaceRequest, 
    // Place, 
    PlaceResponse, 
    UpdatePlaceRequest,
    // User
};
use crate::db::place_repository;
use crate::auth::{admin_required, AuthUser}; // AuthUser untuk mendapatkan info admin jika perlu
use crate::error::{AppError, AppResult};
// use axum::extract::Json;

pub fn admin_routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/admin/places", post(add_place_handler).get(get_all_places_handler))
        .route("/admin/places/:id", get(get_place_by_id_handler))
        .route("/admin/places/:id", put(update_place_handler)) // Ganti dengan put jika sesuai
        .route("/admin/places/:id", delete(delete_place_handler)) // Ganti dengan delete jika sesuai
        // Tambahkan .put() .delete() jika perlu
        .route_layer(middleware::from_fn_with_state(state.clone(), admin_required)) // Proteksi semua rute admin
        .with_state(state)
}

async fn add_place_handler(
    State(state): State<Arc<AppState>>,
    AuthUser(admin_user): AuthUser, // Opsional, jika butuh info admin yang menambahkan
    Json(payload): Json<CreatePlaceRequest>,
) -> AppResult<Json<PlaceResponse>> {
    tracing::info!("Admin {} adding place: {:?}", admin_user.username, payload.nama);
    // Anda bisa menyimpan admin_user.id ke kolom 'ditambahkan_oleh_user_id'
    let new_place = place_repository::add_place(&state.db_pool, &payload /*, admin_user.id*/).await?;
    Ok(Json(PlaceResponse { // Mapping dari Place ke PlaceResponse jika berbeda
        id: new_place.id,
        nama: new_place.nama,
        deskripsi: new_place.deskripsi,
        latitude: new_place.latitude,
        longitude: new_place.longitude,
        ditambahkan_pada: new_place.ditambahkan_pada,
    }))
}


// Tambahkan handler untuk update dan delete jika perlu
async fn update_place_handler(
    State(state): State<Arc<AppState>>,
    AuthUser(_admin_user): AuthUser,
    Path(place_id): Path<Uuid>,
    Json(payload): Json<UpdatePlaceRequest>,
) -> AppResult<Json<PlaceResponse>> {
    let updated_place = place_repository::update_place(&state.db_pool, place_id, &payload)
        .await?;
    // dbg!(&payload); // Debugging untuk melihat data yang diupdate
    Ok(Json(PlaceResponse {
        id: updated_place.id,
        nama: updated_place.nama,
        deskripsi: updated_place.deskripsi,
        latitude: updated_place.latitude,
        longitude: updated_place.longitude,
        ditambahkan_pada: updated_place.ditambahkan_pada,
    }))
}

async fn get_all_places_handler(
    State(state): State<Arc<AppState>>,
    AuthUser(_admin_user): AuthUser,
) -> AppResult<Json<Vec<PlaceResponse>>> {
    let places = place_repository::get_all_places(&state.db_pool).await?;
    let place_responses = places.into_iter().map(|p| PlaceResponse {
         id: p.id,
        nama: p.nama,
        deskripsi: p.deskripsi,
        latitude: p.latitude,
        longitude: p.longitude,
        ditambahkan_pada: p.ditambahkan_pada,
    }).collect();
    Ok(Json(place_responses))
}

async fn get_place_by_id_handler(
    State(state): State<Arc<AppState>>,
    AuthUser(_admin_user): AuthUser,
    Path(place_id): Path<Uuid>,
    ) -> AppResult<Json<PlaceResponse>> 
{
        let place = place_repository::get_place_by_id(&state.db_pool, place_id)
            .await?
            .ok_or_else(|| AppError::NotFoundError(format!("Place with id {} not found", place_id)))?;
        Ok(Json(PlaceResponse {
            id: place.id,
            nama: place.nama,
            deskripsi: place.deskripsi,
            latitude: place.latitude,
            longitude: place.longitude,
            ditambahkan_pada: place.ditambahkan_pada,
        })
    )
}

pub async fn delete_place_handler(
    State(state): State<Arc<AppState>>,
    AuthUser(_admin_user): AuthUser, // Assuming this route requires an authenticated admin
    Path(place_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> { // Changed return type to AppResult<Json<serde_json::Value>>
    // Call the repository to delete the place
    let deleted_count = place_repository::delete_place(&state.db_pool, place_id).await?;

    // Check if any rows were actually deleted
    if deleted_count == 0 {
        // If no rows were deleted, it means the place was not found
        return Err(AppError::NotFoundError(format!("Place with ID {} not found", place_id)));
    }

    // Return a JSON success message
    Ok(Json(serde_json::json!({"message": format!("Place {} deleted successfully", place_id)})))
}



