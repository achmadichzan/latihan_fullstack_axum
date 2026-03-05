use crate::{models::*, repository, AppState};
use crate::error::AppError;
use crate::auth::KUNCI_RAHASIA;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json
};
use validator::Validate;
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};

pub async fn get_menu(State(state): State<AppState>) -> Result<Json<Vec<Menu>>, AppError> {
    let menu = repository::get_all(&state.db)
        .await
        .map_err(|err| {
            eprintln!("Database Error: {}", err);
            AppError::baru(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Gagal mengambil data menu dari server"
            )
        })?;
    Ok(Json(menu))
}

pub async fn tambah_menu(
    claims: Claims,
    State(state): State<AppState>,
    Json(payload): Json<MenuBaru>,
) -> Result<(StatusCode, Json<Menu>), AppError> {
    println!("👤 User '{}' sedang menambah menu: {}", claims.sub, payload.nama);

    if let Err(_) = payload.validate() {
        return Err(AppError::baru(
            StatusCode::BAD_REQUEST,
            "Validasi gagal: Pastikan nama tidak kosong dan harga tidak minus",
        ));
    }

    let menu = repository::create(&state.db, &payload).await
        .map_err(|err| {
            eprintln!("Gagal insert data: {}", err);
            AppError::baru(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Gagal menyimpan menu baru"
            )
        })?;
    Ok((StatusCode::CREATED, Json(menu)))
}

pub async fn get_menu_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Menu>, AppError> {
    let menu = repository::get_by_id(&state.db, id)
        .await
        .map_err(|_| AppError::baru(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Gangguan pada database"
        ))?;

    match menu {
        Some(m) => Ok(Json(m)),
        None => Err(AppError::baru(
            StatusCode::NOT_FOUND,
            "Menu dengan ID tersebut tidak ditemukan"
        )),
    }
}

pub async fn login(Json(payload): Json<LoginPayload>) -> Result<Json<AuthResponse>, AppError> {
    if payload.username != "kasir" || payload.password != "123" {
        return Err(AppError::baru(
            StatusCode::UNAUTHORIZED,
            "Username atau password salah"
        ));
    }

    let claims = Claims {
        sub: payload.username.clone(),
        exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(KUNCI_RAHASIA.as_ref()),
    ).map_err(|_| AppError::baru(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Gagal membuat token"
    ))?;

    Ok(Json(AuthResponse { token }))
}

pub async fn update_menu(
    _claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<MenuBaru>,
) -> Result<Json<Menu>, AppError> {
    if let Err(_) = payload.validate() {
        return Err(AppError::baru(
            StatusCode::BAD_REQUEST,
            "Validasi gagal: Pastikan nama tidak kosong dan harga tidak minus",
        ));
    }

    let menu = repository::update(&state.db, id, &payload)
        .await
        .map_err(|_| AppError::baru(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Gagal memperbarui data"
        ))?;

    match menu {
        Some(m) => Ok(Json(m)),
        None => Err(AppError::baru(
            StatusCode::NOT_FOUND,
            "Gagal update: Menu tidak ditemukan"
        )),
    }
}

pub async fn delete_menu(
    _claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let baris_terhapus = repository::delete(&state.db, id)
        .await
        .map_err(|_| AppError::baru(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Gagal menghapus data"
        ))?;

    if baris_terhapus > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::baru(
            StatusCode::NOT_FOUND,
            "Gagal hapus: Menu tidak ditemukan"
        ))
    }
}