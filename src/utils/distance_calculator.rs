use haversine::{Location, distance};

// const EARTH_RADIUS_KM: f64 = 6371.0;

// Implementasi Haversine manual jika tidak ingin dependency
// pub fn calculate_distance_km_manual(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
//     let d_lat = (lat2 - lat1).to_radians();
//     let d_lon = (lon2 - lon1).to_radians();

//     let lat1_rad = lat1.to_radians();
//     let lat2_rad = lat2.to_radians();

//     let a = (d_lat / 2.0).sin().powi(2) + lat1_rad.cos() * lat2_rad.cos() * (d_lon / 2.0).sin().powi(2);
//     let c = 2.0 * a.sqrt().asin();

//     EARTH_RADIUS_KM * c
// }

// Menggunakan crate haversine
pub fn calculate_distance_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let loc1 = Location { latitude: lat1, longitude: lon1 };
    let loc2 = Location { latitude: lat2, longitude: lon2 };
    distance(loc1, loc2, haversine::Units::Kilometers)
}

