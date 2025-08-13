use infisical::{
    Client, 
    AuthMethod,
    resources::secrets::ListSecretsRequest
};
use std::{
    env,
    error::Error
};

pub async fn bind_rahasia_env(
    url_klien: String,
    id_klien: String,
    rahasia_klien: String,
    id_proyek: String,
    mode_env: String 
) -> Result<(), Box<dyn Error>> {
    // 1. Bentuk klien
    let mut klien = Client::builder()
        .base_url(url_klien)
        .build()
        .await?;

    // 2. Siapkan metode autentikasi dan masuk
    let metode_autentikasi = AuthMethod::new_universal_auth(id_klien, rahasia_klien);
    klien.login(metode_autentikasi).await?;

    // 3. Lakukan rekues ke secrets manager
    let rekues = ListSecretsRequest::builder(id_proyek, mode_env)
        .path("/")
        .recursive(true)
        .attach_to_process_env(true)
        .build();

    // 4. Lakukan panggilan API untuk mendapatkan semua rahasia dan memuatnya ke lingkungan proses
    let rahasia = klien.secrets().list(rekues).await?;

    // 5. Jika dalam mode pengembangan, cetak rahasia yang diambil pada konsol untuk debugging
    if env::var("INF_ENV_MODE").unwrap() == "dev" {
        println!("Fetched secret key: {:?}", rahasia)
    }

    // 6. Kembalikan void jika semuanya berjalan lancar
    Ok(())
}