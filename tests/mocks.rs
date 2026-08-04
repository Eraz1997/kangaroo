use axum::{Router, extract::Path, http::StatusCode, routing::get, serve};
use kangaroo_axum::{IntoKangarooError, KangarooConfig, KangarooRouterExtension, kangarooise};
use serde::Serialize;
use tokio::{net::TcpListener, spawn, task::JoinHandle};
use tower_http::services::ServeDir;

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

pub struct MockFrontendDevelopmentServer {
    pub address: String,
    _task_handle: JoinHandle<()>,
}

pub fn create_mock_app(frontend_dev_server_uri: Option<&str>) -> Router {
    Router::new()
        .route("/", get(get_home))
        .route("/404", get(get_not_found))
        .route("/resources/{resource}", get(get_resource))
        .route("/custom", get(get_custom))
        .route("/absent-document", get(get_absent_document))
        .route("/invalid-document", get(get_invalid_document))
        .route("/deleted", get(get_deleted_resource))
        .route("/deleted-custom", get(get_custom_deleted_resource))
        .with_kangaroo(
            KangarooConfig::new("tests/static")
                .with_document_for_status(StatusCode::GONE, "gone.html")
                .with_frontend_development_server(frontend_dev_server_uri),
        )
}

pub async fn create_mock_frontend_dev_server() -> MockFrontendDevelopmentServer {
    let app = Router::new().fallback_service(ServeDir::new("tests/static_frontend_dev_server"));

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("failed to start mock frontend dev server");
    let address = listener
        .local_addr()
        .expect("failed to extract mock frontend dev server address")
        .to_string();
    let task_handle = spawn(async move {
        serve(listener, app.clone().into_make_service())
            .await
            .expect("failed to serve mock frontend dev server");
    });

    MockFrontendDevelopmentServer {
        address: format!("http://{}", address),
        _task_handle: task_handle,
    }
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
