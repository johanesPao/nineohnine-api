use serde::Serializer;
use std::fmt::Display;

/// fungsi serialisasi_opsi digunakan untuk men-serialisasi
/// tipe data Option<T> yang berisi nilai generik T atau None
/// untuk digunakan dalam serialiasi JSON dengan serde
pub fn serialisasi_opsi<T, S>(
    nilai: &Option<T>,
    serializer: S
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Display,
{
    match nilai {
        Some(konten) => serializer.serialize_str(&konten.to_string()),
        None => serializer.serialize_none(),
    }
}