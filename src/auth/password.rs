use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use crate::error::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| {
            tracing::error!("Password hashing error: {:?}", e);
            AppError::InternalServerError // Atau error spesifik
        })
}

pub fn verify_password(hashed_password: &str, password_to_verify: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hashed_password).map_err(|e| {
        tracing::error!("Password hash parsing error: {:?}", e);
        AppError::AuthenticationError("Invalid stored password hash format.".to_string())
    })?;
    Ok(Argon2::default()
        .verify_password(password_to_verify.as_bytes(), &parsed_hash)
        .is_ok())
}