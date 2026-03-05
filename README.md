# 🍽️ Dashboard Kasir — Fullstack Rust (Axum)

Aplikasi web fullstack **Dashboard Kasir** untuk mengelola daftar menu restoran. Dibangun menggunakan **Rust (Axum)** sebagai backend dan **HTML/CSS/JavaScript** vanilla sebagai frontend.

## ✨ Fitur Utama

- 🔐 **Autentikasi JWT** — Login dengan username & password, dilindungi oleh JSON Web Token
- 📋 **CRUD Menu** — Tambah, lihat, edit, dan hapus item menu
- ✅ **Validasi Input** — Nama menu tidak boleh kosong, harga tidak boleh negatif
- 🌐 **REST API** — Endpoint API yang rapi dan terstruktur
- 🔒 **Proteksi Route** — Operasi tulis (tambah, edit, hapus) hanya bisa dilakukan setelah login

## 📸 Screenshot

| Belum Login | Sudah Login |
|:-----------:|:-----------:|
| <img width="480" height="665" alt="Screenshot 2026-03-05 101933" src="https://github.com/user-attachments/assets/2e6da475-c38c-4a11-931a-d2de76e71784" /> | <img width="480" height="665" alt="Screenshot 2026-03-05 101958" src="https://github.com/user-attachments/assets/31a11476-1e78-4bc6-87ab-7333a9545f36" /> |

## 🛠️ Teknologi yang Digunakan

| Komponen | Teknologi |
|----------|-----------|
| Backend | [Rust](https://www.rust-lang.org/) + [Axum](https://github.com/tokio-rs/axum) |
| Database | [PostgreSQL](https://www.postgresql.org/) + [SQLx](https://github.com/launchbadge/sqlx) |
| Autentikasi | [jsonwebtoken](https://crates.io/crates/jsonwebtoken) (JWT HS256) |
| Validasi | [validator](https://crates.io/crates/validator) |
| Frontend | HTML, CSS, JavaScript (Vanilla) |
| CORS | [tower-http](https://crates.io/crates/tower-http) |

## 📁 Struktur Project

```
latihan_fullstack_axum/
├── Cargo.toml              # Konfigurasi project & dependensi Rust
├── .env                    # Variabel environment (DATABASE_URL)
├── src/
│   ├── main.rs             # Entry point, setup server & routing
│   ├── handlers.rs         # Handler untuk setiap endpoint API
│   ├── models.rs           # Struct data (Menu, Claims, dll.)
│   ├── repository.rs       # Query database (CRUD)
│   ├── auth.rs             # Middleware autentikasi JWT
│   └── error.rs            # Custom error handling
├── frontend/
    ├── index.html          # Halaman utama
    ├── style.css           # Styling
    └── script.js           # Logika frontend (fetch API)
```

## 🚀 Cara Menjalankan

### Prasyarat

- [Rust](https://rustup.rs/) (edisi 2024)
- [PostgreSQL](https://www.postgresql.org/download/) yang sudah berjalan
- Database dan tabel `menu` yang sudah dibuat

### 1. Siapkan Database

Buat database PostgreSQL dan jalankan query berikut untuk membuat tabel:

```sql
CREATE TABLE menu (
    id SERIAL PRIMARY KEY,
    nama VARCHAR(255) NOT NULL,
    harga INTEGER NOT NULL
);
```

### 2. Konfigurasi Environment

Buat file `.env` di root project (atau edit yang sudah ada):

```env
DATABASE_URL=postgres://username:password@localhost/nama_database
```

### 3. Jalankan Aplikasi

```bash
cargo run
```

Server akan berjalan di **http://127.0.0.1:3000**

### 4. Buka di Browser

Akses **http://127.0.0.1:3000** untuk membuka Dashboard Kasir.

## 🔑 Kredensial Login

| Field    | Nilai   |
|----------|---------|
| Username | `kasir` |
| Password | `123`   |

## 📡 API Endpoints

| Method   | Endpoint          | Deskripsi                 | Auth |
|----------|-------------------|---------------------------|------|
| `POST`   | `/api/login`      | Login dan dapatkan token  | ❌   |
| `GET`    | `/api/menu`       | Ambil semua menu          | ❌   |
| `GET`    | `/api/menu/{id}`  | Ambil menu berdasarkan ID | ❌   |
| `POST`   | `/api/menu`       | Tambah menu baru          | ✅   |
| `PUT`    | `/api/menu/{id}`  | Update menu               | ✅   |
| `DELETE` | `/api/menu/{id}`  | Hapus menu                | ✅   |

> Endpoint yang membutuhkan autentikasi (✅) harus menyertakan header:
> ```
> Authorization: Bearer <token>
> ```

## 📝 Lisensi

Project ini dibuat untuk tujuan pembelajaran fullstack development dengan Rust.
