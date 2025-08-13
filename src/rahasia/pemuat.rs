use dotenv::dotenv;
use super::manajer::bind_rahasia_env;
use std::env;
use std::error::Error;

pub async fn baca_dan_muat_rahasia_env() -> Result<(), Box<dyn Error>> {
    // muat file .env untuk mendapatkan variabel lingkungan
    // yang diperlukan untuk menghubungkan ke layanan rahasia
    dotenv().ok();

    // bind rahasia ke dalam variabel lingkungan proses tempat aplikasi berjalan
    bind_rahasia_env(
        env::var("INF_CLIENT_URL").expect("INF_CLIENT_URL tidak dapat ditemukan"),
        env::var("INF_CLIENT_ID").expect("INF_CLIENT_URL tidak dapat ditemukan"),
        env::var("INF_CLIENT_SECRET").expect("INF_CLIENT_URL tidak dapat ditemukan"),
        env::var("INF_PROJECT_ID").expect("INF_CLIENT_URL tidak dapat ditemukan"),
        env::var("INF_ENV_MODE").expect("INF_CLIENT_URL tidak dapat ditemukan")
    ).await.unwrap();

    // kembalikan void jika berhasil
    Ok(())
}