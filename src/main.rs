mod rahasia;
mod database;
mod model;
mod kondisi;
mod rute;
mod utilitas;

use rahasia::pemuat::baca_dan_muat_rahasia_env;
use database::koneksi::db_klien;
use kondisi::KondisiAplikasi;
use rute::sales::get_sales_data;

use axum::{
    routing::post,
    Router
};
use std::{
    env,
    sync::Arc,
};

#[tokio::main]
async fn main() {
    // baca variabel lingkungan dari file .env dan muat rahasia ke dalam
    // variabel lingkungan saat aplikasi dimulai
    baca_dan_muat_rahasia_env()
        .await
        .expect(
            format!("Gagal memuat rahasia dari {}", env::var("INF_CLIENT_URL").unwrap())
                .as_str()
        );

    // simpan koneksi database dalam HashMap yang dikembalikan oleh
    // db_klien() dalam Arc untuk memastikan bahwa koneksi
    // dapat diakses dibagikan dalam beberapa thread
    let kondisi_aplikasi = KondisiAplikasi {
        peta_database: Arc::new(db_klien().await.unwrap())
    };

    let app = Router::new()
        .route("/sales", post(get_sales_data))
        // tambahkan kondisi_aplikasi sebagai state untuk aplikasi
        // yang dapat diakses oleh handler di berbagai rute
        .with_state(kondisi_aplikasi);
    

    // jalankan api server, bind ke localhost port 3000
    let tokio_bind = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(tokio_bind, app).await.unwrap();
}