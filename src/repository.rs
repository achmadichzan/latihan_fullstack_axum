use crate::models::{Menu, MenuBaru};
use sqlx::postgres::PgPool;

pub async fn get_all(pool: &PgPool) -> Result<Vec<Menu>, sqlx::Error> {
    sqlx::query_as("SELECT id, nama, harga FROM menu ORDER BY id")
        .fetch_all(pool)
        .await
}

pub async fn create(pool: &PgPool, payload: &MenuBaru) -> Result<Menu, sqlx::Error> {
    sqlx::query_as(
        "INSERT INTO menu (nama, harga) VALUES ($1, $2) RETURNING id, nama, harga"
    )
        .bind(&payload.nama)
        .bind(payload.harga)
        .fetch_one(pool)
        .await
}


pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Menu>, sqlx::Error> {
    sqlx::query_as("SELECT id, nama, harga FROM menu WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn update(pool: &PgPool, id: i32, payload: &MenuBaru) -> Result<Option<Menu>, sqlx::Error> {
    sqlx::query_as(
        "UPDATE menu SET nama = $1, harga = $2 WHERE id = $3 RETURNING id, nama, harga"
    )
        .bind(&payload.nama)
        .bind(payload.harga)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM menu WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}