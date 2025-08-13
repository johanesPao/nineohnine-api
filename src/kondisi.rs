use std::{
    collections::HashMap,
    sync::Arc,
};
use welds_connections::any::AnyClient;

// struct untuk menyimpan kondisi aplikasi (NineOhNine-API)
#[derive(Clone)]
pub struct KondisiAplikasi {
    // peta database yang berisi koneksi ke berbagai database
    pub peta_database: Arc<HashMap<String, Arc<AnyClient>>>,
}