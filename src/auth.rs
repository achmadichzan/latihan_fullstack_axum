use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    http::StatusCode,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use crate::{models::Claims, error::AppError};

pub const KUNCI_RAHASIA: &[u8] = b"RAHASIA_NEGARA_SANGAT_KUAT";

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers.get("Authorization")
            .and_then(|value| value.to_str().ok());

        if let Some(auth_header) = auth_header {
            if auth_header.starts_with("Bearer ") {
                let token = auth_header.trim_start_matches("Bearer ");

                let token_data = decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret(KUNCI_RAHASIA.as_ref()),
                    &Validation::new(Algorithm::HS256),
                ).map_err(|_| AppError::baru(StatusCode::UNAUTHORIZED, "Token tidak valid atau sudah kedaluwarsa"))?;

                return Ok(token_data.claims);
            }
        }

        Err(AppError::baru(
            StatusCode::UNAUTHORIZED,
            "Akses ditolak: Anda harus menyertakan Bearer Token"
        ))
    }
}