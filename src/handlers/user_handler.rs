// src/handlers/user_handler.rs
use axum::{
    routing::post,
    Router, Json, extract::State,
    middleware,
};
use std::sync::Arc;
use crate::app_state::AppState;
use crate::models::{RouteRequest, NearbyPlaceResponse, 
    // User
};
use crate::auth::{user_required, AuthUser};
use crate::services::route_service; // Logika inti ada di sini
use crate::error::AppResult;

pub fn user_routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/user/routes/nearby-places", post(find_nearby_places_handler))
        .route_layer(middleware::from_fn_with_state(state.clone(), user_required)) // Proteksi rute user
        .with_state(state)
}

async fn find_nearby_places_handler(
    State(state): State<Arc<AppState>>,
    AuthUser(user): AuthUser, // Info user yang melakukan request
    Json(payload): Json<RouteRequest>,
) -> AppResult<Json<Vec<NearbyPlaceResponse>>> {
    tracing::info!("User {} requesting nearby places for route: {:?} to {:?}",
        user.username,
        (payload.origin_lat, payload.origin_lng),
        (payload.destination_lat, payload.destination_lng)
    );

    let nearby_places = route_service::find_places_near_route(
        &state.db_pool,
        &state.http_client,
        &state.maps_api_key,
        payload.origin_lat,
        payload.origin_lng,
        payload.destination_lat,
        payload.destination_lng,
        1.0, // Radius 1 km
    )
    .await?;

    Ok(Json(nearby_places))
}