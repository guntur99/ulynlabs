// src/models/route.rs
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RouteRequest {
    pub origin_lat: f64,
    pub origin_lng: f64,
    pub destination_lat: f64,
    pub destination_lng: f64,
}

// Struct untuk response dari Google Maps API bisa diletakkan di services/Maps_service.rs
// atau di sini jika lebih general.
#[derive(Debug, Deserialize, Clone)]
pub struct Coordinate {
    pub lat: f64,
    pub lng: f64,
}

