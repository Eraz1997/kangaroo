use axum::{Router, extract::Path, http::StatusCode, routing::get, serve};
use kangaroo_axum::{IntoKangarooError, KangarooConfig, KangarooRouterExtension, kangarooise};
use serde::Serialize;
use tokio::net::TcpListener;

#[derive(Serialize, Clone)]
struct User {
    pub username: &'static str,
    pub age: u16,
    pub active: bool,
}

static USERS: &[User] = &[
    User {
        username: "harry",
        age: 15,
        active: true,
    },
    User {
        username: "paul",
        age: 28,
        active: true,
    },
    User {
        username: "jack",
        age: 50,
        active: false,
    },
];

struct Error {
    pub code: StatusCode,
    pub payload: ErrorPayload,
}

#[derive(Serialize)]
struct ErrorPayload {
    pub message: String,
}

impl IntoKangarooError for Error {
    fn into_kangaroo_error(self) -> (StatusCode, impl Serialize) {
        (self.code, self.payload)
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_home))
        .route("/users/{username}", get(get_user))
        .with_kangaroo(
            KangarooConfig::new("examples/static")
                .with_document_for_status(StatusCode::NOT_FOUND, "404.html"),
        );

    let listener = TcpListener::bind("127.0.0.1:5000").await.unwrap();
    serve(listener, app.clone().into_make_service())
        .await
        .unwrap();
}

#[kangarooise]
async fn get_home() {}

#[kangarooise(file = "user.html", not_found = "user.html")]
async fn get_user(Path(username): Path<String>) -> Result<User, Error> {
    let user = USERS.iter().find(|user| user.username == username).cloned();

    match user {
        Some(user) => Ok(user),
        None => Err(Error {
            code: StatusCode::NOT_FOUND,
            payload: ErrorPayload {
                message: "user not found".to_string(),
            },
        }),
    }
}
