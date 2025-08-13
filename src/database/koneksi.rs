use welds_connections::any::AnyClient;
use std::{
    error::Error,
    collections::HashMap,
    sync::Arc,
    env
};

pub struct KredentialDatabase {
    url: String,
    pengguna: String,
    kata_kunci: String
}

// implementasi fungsi buat untuk struct KredentialDatabase
impl KredentialDatabase {
    pub fn buat(
        url: String,
        pengguna: String,
        kata_kunci: String
    ) -> Self {
        Self {
            url,
            pengguna,
            kata_kunci
        }
    }
}

pub async fn db_klien() -> Result<HashMap<String, Arc<AnyClient>>, Box<dyn Error>> {
    // seluruh mod ini akan dipanggil setelah proses populasi variabel lingkungan
    // oleh baca_dan_muat_rahasia_env() selesai, sehingga variabel lingkungan
    // sudah tersedia untuk digunakan
    // populasi kredensial_database dari variabel lingkungan
    let kredensial_database = KredentialDatabase::buat(
        env::var("DB_URL").expect("DB_URL tidak ditemukan"),
        env::var("DB_USER").expect("DB_USER tidak ditemukan"),
        env::var("DB_PASS").expect("DB_PASS tidak ditemukan")
    );

    // populasi vektor_db dengan nama-nama database yang akan dihubungkan
    let vektor_db: Vec<String> = vec![
        env::var("DB_SOSCO").expect("DB_SOSCO tidak ditemukan"),
        env::var("DB_AIR").expect("DB_AIR tidak ditemukan"),
        env::var("DB_OMNI").expect("DB_OMNI tidak ditemukan"),
        env::var("DB_OMNI_S").expect("DB_OMNI_S tidak ditemukan"),
        env::var("DB_WIR").expect("DB_WIR tidak ditemukan"),
        env::var("DB_ORBIT").expect("DB_ORBIT tidak ditemukan"),
        env::var("DB_ORBIT_S").expect("DB_ORBIT_S tidak ditemukan"),
        env::var("DB_WIP").expect("DB_WIP tidak ditemukan"),
        env::var("DB_AIM").expect("DB_AIM tidak ditemukan")
    ];

    // buat variabel mutable HashMap baru untuk menyimpan koneksi database
    // dengan format nama database sebagai kunci dan Arc<AnyClient> sebagai
    // nilai
    let mut peta_db = HashMap::new();

    // iterasi vektor_db, membuat koneksi untuk setiap database
    // dan menyimpannya dalam peta_db
    for db in vektor_db {
        let koneksi = format!(
            "server={};user id={};password={};Database={};TrustServerCertificate=true;",
            kredensial_database.url,
            kredensial_database.pengguna,
            kredensial_database.kata_kunci,
            db
        );

        let klien = welds::connections::connect(koneksi).await?;

        // simpan koneksi dalam peta_db dengan nama database sebagai kunci
        peta_db.insert(db, Arc::new(klien));
    }
    
    Ok(peta_db)
}