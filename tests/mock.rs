use axum::{Router, extract::Path, http::StatusCode, routing::get};
use kangaroo_axum::{IntoKangarooError, KangarooConfig, KangarooRouterExtension, kangarooise};
use serde::Serialize;

#[derive(Serialize)]
struct ExampleData {
    pub resource: String,
}

struct Error {
    pub code: StatusCode,
}

impl IntoKangarooError for Error {
    fn into_kangaroo_error(self) -> (StatusCode, impl Serialize) {
        (self.code, "example error")
    }
}

pub fn create_mock_app() -> Router {
    Router::new()
        .route("/", get(get_home))
        .route("/404", get(get_not_found))
        .route("/{resource}", get(get_resource))
        .route("/custom", get(get_custom))
        .route("/absent-document", get(get_absent_document))
        .route("/invalid-document", get(get_invalid_document))
        .route("/deleted", get(get_deleted_resource))
        .route("/deleted-custom", get(get_custom_deleted_resource))
        .with_kangaroo(
            KangarooConfig::new("tests/static")
                .with_document_for_status(StatusCode::GONE, "gone.html"),
        )
}

#[kangarooise]
async fn get_home() -> ExampleData {
    ExampleData {
        resource: "sample resource".to_string(),
    }
}

#[kangarooise]
async fn get_resource(Path(resource): Path<String>) -> Result<ExampleData, Error> {
    Ok(ExampleData { resource })
}

#[kangarooise(file = "custom.html")]
async fn get_custom() -> ExampleData {
    ExampleData {
        resource: "sample resource".to_string(),
    }
}

#[kangarooise]
async fn get_not_found() -> Result<ExampleData, Error> {
    Err(Error {
        code: StatusCode::NOT_FOUND,
    })
}

#[kangarooise(file = "absent.html")]
async fn get_absent_document() -> ExampleData {
    ExampleData {
        resource: "sample resource".to_string(),
    }
}

#[kangarooise(file = "invalid.html")]
async fn get_invalid_document() -> ExampleData {
    ExampleData {
        resource: "sample resource".to_string(),
    }
}

#[kangarooise]
async fn get_deleted_resource() -> Result<ExampleData, Error> {
    Err(Error {
        code: StatusCode::GONE,
    })
}

#[kangarooise(gone = "custom.html")]
async fn get_custom_deleted_resource() -> Result<ExampleData, Error> {
    Err(Error {
        code: StatusCode::GONE,
    })
}
