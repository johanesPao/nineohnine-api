use crate::kondisi::KondisiAplikasi;
use crate::model::transact::SalesNonWholesale;

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use welds::prelude::*;

#[derive(Deserialize)]
pub struct GetSalesDataBody {
    nama_database: String,
    brand: Option<Vec<String>>,
    start_date: String,
    end_date: String
}

pub async fn get_sales_data(
    State(kondisi_aplikasi): State<KondisiAplikasi>,
    Json(payload): Json<GetSalesDataBody>
) -> Result<Json<Vec<SalesNonWholesale>>, (StatusCode, String)> {
    // muat koneksi database dari kondisi aplikasi berdasarkan
    // nama database yang diberikan dalam payload JSON post
    let klien = kondisi_aplikasi.peta_database
        .get(&payload.nama_database)
        .ok_or((StatusCode::NOT_FOUND, "Database tidak ditemukan".into()))?;

    let kueri = SalesNonWholesale::all()
        .limit(10)
        .where_col(|kolom| kolom.ITEM_CODE.equal("38WBFFLTF18/323775130722"))
        .run(klien.as_ref())
        .await
        .map_err(|kesalahan| {
            (StatusCode::INTERNAL_SERVER_ERROR, kesalahan.to_string())
        })?
        .into_inners();

    // kembalikan hasil kueri yang sudah diserialisasi ke dalam JSON
    Ok(Json(kueri))
}