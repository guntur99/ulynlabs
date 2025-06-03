// src/models/place.rs
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)] // Clone jika diperlukan
pub struct Place { // Mengganti nama "Tempat" menjadi "Place" agar konsisten Inggris
    pub id: Uuid,
    pub nama: String,
    pub deskripsi: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    // pub ditambahkan_oleh_user_id: Option<Uuid>, // Opsional
    pub ditambahkan_pada: DateTime<Utc>,
    pub diperbarui_pada: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePlaceRequest {
    pub nama: String,
    pub deskripsi: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    // pub ditambahkan_pada: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePlaceRequest {
    pub nama: Option<String>,
    pub deskripsi: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    // pub ditambahkan_pada: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PlaceResponse { // Bisa jadi sama dengan Place, atau disesuaikan
    pub id: Uuid,
    pub nama: String,
    pub deskripsi: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub ditambahkan_pada: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct NearbyPlaceResponse {
    #[serde(flatten)] // Menggabungkan field dari Place
    pub place: Place,
    pub jarak_dari_rute_km: Option<f64>,
}