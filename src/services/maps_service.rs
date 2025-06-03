// src/services/Maps_service.rs
use reqwest::Client;
use serde::Deserialize;
use crate::models::Coordinate; // Dari models/route.rs atau models/mod.rs
use crate::error::{AppError, AppResult};

#[derive(Debug, Deserialize)]
struct GoogleDirectionsResponse {
    routes: Vec<GoogleRoute>,
    status: String,
    error_message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GoogleRoute {
    overview_polyline: GooglePolyline,
    // legs: Vec<GoogleRouteLeg>, // Jika butuh info lebih detail per leg/step
}

#[derive(Debug, Deserialize)]
struct GooglePolyline {
    points: String, // Encoded polyline
}

// Anda mungkin memerlukan library untuk decode polyline seperti `polyline` crate
// atau implementasi decoding sederhana jika formatnya sederhana.

pub async fn get_route_polyline(
    http_client: &Client,
    api_key: &str,
    origin: Coordinate,
    destination: Coordinate,
) -> AppResult<String> { // Mengembalikan encoded polyline string
    let url = format!(
        "https://maps.googleapis.com/maps/api/directions/json?origin={},{}&destination={},{}&key={}",
        origin.lat, origin.lng, destination.lat, destination.lng, api_key
    );

    let response = http_client.get(&url).send().await.map_err(AppError::ReqwestError)?;

    if !response.status().is_success() {
        let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        tracing::error!("Google Maps API HTTP error: {}", error_body);
        return Err(AppError::ExternalServiceError(format!(
            "Google Maps API request failed: {}", error_body
        )));
    }

    let directions_response = response.json::<GoogleDirectionsResponse>().await.map_err(|e| {
        tracing::error!("Failed to parse Google Maps API response: {:?}", e);
        AppError::ExternalServiceError("Failed to parse Google Maps API response.".to_string())
    })?;


    if directions_response.status != "OK" {
        let error_msg = directions_response.error_message.unwrap_or_else(|| directions_response.status.clone());
        tracing::error!("Google Maps API error: {}", error_msg);
        return Err(AppError::ExternalServiceError(format!(
            "Google Maps API error: {}", error_msg
        )));
    }

    directions_response
        .routes
        .first()
        .map(|route| route.overview_polyline.points.clone())
        .ok_or_else(|| AppError::NotFoundError("No route found by Google Maps API.".to_string()))
}

// Fungsi untuk decode polyline (contoh, mungkin perlu crate `polyline`)
// Untuk sekarang kita asumsikan ada fungsi ini, atau kita bisa dapatkan step points
// jika overview_polyline terlalu kompleks untuk didecode tanpa library tambahan.
// Jika menggunakan step points, modifikasi GoogleDirectionsResponse untuk parse steps.
pub fn decode_polyline(encoded_polyline: &str) -> AppResult<Vec<Coordinate>> {
    // Implementasi atau panggil library polyline decoding
    // Contoh dengan crate `polyline`:
    // polyline::decode_polyline(encoded_polyline, 5) // Presisi 5 untuk Google
    //     .map(|line_string| {
    //         line_string.into_iter().map(|coord| Coordinate { lat: coord.y, lng: coord.x }).collect()
    //     })
    //     .map_err(|e| AppError::InternalServerError(format!("Failed to decode polyline: {}",e)))
    // Untuk sementara, kita akan mengembalikan Vec kosong sebagai placeholder jika tidak ada library.
    // Idealnya, Anda akan mengimplementasikan ini atau menggunakan crate.
    if encoded_polyline.is_empty() { return Ok(vec![]); }
    tracing::warn!("Polyline decoding is not fully implemented without a library. Returning placeholder for: {}", encoded_polyline);
    // Sebagai alternatif, dapatkan poin-poin dari 'steps' di respons Google API.
    Ok(vec![]) // Placeholder
}

