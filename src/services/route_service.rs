// src/services/route_service.rs
use sqlx::PgPool;
use reqwest::Client;
use crate::models::{
    // Place, 
    NearbyPlaceResponse, Coordinate};
use crate::db::place_repository;
use crate::services::maps_service::{get_route_polyline, decode_polyline};
use crate::utils::distance_calculator::calculate_distance_km;
use crate::error::{
    // AppError, 
    AppResult};

pub async fn find_places_near_route(
    db_pool: &PgPool,
    http_client: &Client,
    api_key: &str,
    origin_lat: f64,
    origin_lng: f64,
    dest_lat: f64,
    dest_lng: f64,
    radius_km: f64,
) -> AppResult<Vec<NearbyPlaceResponse>> {
    let origin = Coordinate { lat: origin_lat, lng: origin_lng };
    let destination = Coordinate { lat: dest_lat, lng: dest_lng };

    // 1. Dapatkan encoded polyline dari Google Maps
    let encoded_polyline = get_route_polyline(http_client, api_key, origin, destination).await?;

    // 2. Decode polyline menjadi serangkaian titik koordinat
    //    PENTING: Implementasi decode_polyline() atau gunakan poin dari steps jika ini tidak diimplementasikan.
    let route_points = decode_polyline(&encoded_polyline)?;
    if route_points.is_empty() {
        // Jika polyline tidak bisa di-decode atau rute tidak punya poin (misal, origin=destination)
        // Anda mungkin ingin mencoba strategi lain atau mengembalikan hasil kosong.
        // Untuk saat ini, kita asumsikan jika tidak ada poin, tidak ada tempat sekitar yang relevan dengan rute.
        tracing::warn!("No route points decoded from polyline. Potentially an issue with polyline decoding or an empty route.");
        // Sebagai alternatif, jika Anda tidak bisa decode polyline,
        // Anda bisa mendapatkan titik-titik dari `steps` di Google Directions API.
        // Ini akan lebih kasar tapi bisa jadi fallback.
        // Untuk saat ini kita biarkan kosong jika tidak ada poin.
        // return Ok(vec![]);
    }


    // 3. Ambil semua tempat dari database
    let all_places = place_repository::get_all_places(db_pool).await?;
    if all_places.is_empty() {
        return Ok(vec![]); // Tidak ada tempat di DB, tidak ada yang bisa ditemukan
    }

    // 4. Filter tempat yang berada dalam radius dari salah satu titik rute
    let mut nearby_places_responses: Vec<NearbyPlaceResponse> = Vec::new();

    for place in all_places {
        let mut min_distance_to_route: Option<f64> = None;
        let mut is_near = false;

        // Jika tidak ada titik rute (misal polyline gagal decode),
        // kita bisa cek jarak dari origin dan destination saja sebagai fallback kasar.
        if route_points.is_empty() {
            let dist_to_origin = calculate_distance_km(place.latitude, place.longitude, origin_lat, origin_lng);
            let dist_to_dest = calculate_distance_km(place.latitude, place.longitude, dest_lat, dest_lng);
            if dist_to_origin <= radius_km || dist_to_dest <= radius_km {
                is_near = true;
                min_distance_to_route = Some(dist_to_origin.min(dist_to_dest));
            }
        } else {
            for route_point in &route_points {
                let distance = calculate_distance_km(
                    place.latitude,
                    place.longitude,
                    route_point.lat,
                    route_point.lng,
                );

                if distance <= radius_km {
                    is_near = true;
                    min_distance_to_route = Some(min_distance_to_route.map_or(distance, |d| d.min(distance)));
                    // Tidak perlu break, agar bisa mencari jarak terdekat sebenarnya ke rute jika diperlukan
                }
            }
        }


        if is_near {
            nearby_places_responses.push(NearbyPlaceResponse {
                place: place.clone(), // Clone karena 'place' masih dipakai di loop
                jarak_dari_rute_km: min_distance_to_route,
            });
        }
    }

    // Hapus duplikat jika sebuah tempat dekat dengan beberapa titik di rute (sudah ditangani dengan hanya push sekali)
    // Sortir berdasarkan nama atau jarak jika perlu
    nearby_places_responses.sort_by(|a, b| a.place.nama.cmp(&b.place.nama));

    Ok(nearby_places_responses)
}

