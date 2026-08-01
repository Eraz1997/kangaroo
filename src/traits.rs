use axum::http::StatusCode;
use serde::Serialize;

pub trait IntoKangarooError {
    fn into_kangaroo_error(self) -> (StatusCode, impl Serialize);
}
