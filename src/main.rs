mod models;
mod repository;
mod handlers;
mod error;
mod auth;

use axum::http::{header, Method};
use axum::routing::{get, post};
use axum::Router;
use dotenvy::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")
        .expect("Variabel DATABASE_URL tidak ditemukan");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Gagal terhubung ke database");

    let state = AppState { db: pool };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/api/login", post(handlers::login))
        .route("/api/menu", get(handlers::get_menu).post(handlers::tambah_menu))
        .route(
            "/api/menu/{id}",
            get(handlers::get_menu_by_id)
                .put(handlers::update_menu)
                .delete(handlers::delete_menu),
        )
        .layer(cors)
        .with_state(state)
        .fallback_service(ServeDir::new("frontend"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}