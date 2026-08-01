use axum::{Extension, Router, http::StatusCode};
use tower_http::services::{ServeDir, ServeFile};

use crate::config::KangarooConfig;

/**
Extension for `axum::Router`. Import it to make the `.with_kangaroo(...)` method available.
Required to make Kangaroo endpoints work.
*/
pub trait KangarooRouterExtension {
    fn with_kangaroo(self, config: KangarooConfig) -> Self;
}

impl KangarooRouterExtension for Router {
    fn with_kangaroo(self, config: KangarooConfig) -> Self {
        self.fallback_service(
            ServeDir::new(config.get_folder_path()).not_found_service(ServeFile::new(
                config
                    .get_full_path_from_status_code(StatusCode::NOT_FOUND)
                    .unwrap_or(config.get_default_path()),
            )),
        )
        .layer(Extension(config.clone()))
    }
}
