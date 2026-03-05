use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Serialize, sqlx::FromRow)]
pub struct Menu {
    pub id: i32,
    pub nama: String,
    pub harga: i32
}

#[derive(Deserialize, Validate)]
pub struct MenuBaru {
    #[validate(length(min = 1, message = "Nama menu tidak boleh kosong"))]
    pub nama: String,
    #[validate(range(min = 0, message = "Harga tidak boleh bernilai negatif"))]
    pub harga: i32
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}