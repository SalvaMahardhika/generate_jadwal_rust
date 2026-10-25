pub mod models;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use std::env;

#[get("/health")]
async fn health_check(pool: web::Data<MySqlPool>) -> impl Responder {
    match sqlx::query("SELECT 1").execute(pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("Koneksi ke database MySQL berhasil!"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Gagal query database: {}", err)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL belum diatur pada file .env");

    println!("Menghubungkan ke database MySQL...");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Gagal terhubung ke database MySQL. Pastikan MySQL di Laragon sudah Start!");

    println!("Berhasil terhubung ke database!");
    println!("Server berjalan di http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(health_check)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}