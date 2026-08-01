use axum::http::StatusCode;
use serde::Serialize;

/**
Implement this trait to make Kangaroo gracefully handle errors.

```ignore
#[kangarooise]
async fn get_home() -> Result<HomeData, Error> {
    // ...
}

// ...

impl IntoKangarooError for Error {
    fn into_kangaroo_error(self) -> (StatusCode, impl Serialize) {
        // ...
    }
}
*/
pub trait IntoKangarooError {
    fn into_kangaroo_error(self) -> (StatusCode, impl Serialize);
}
