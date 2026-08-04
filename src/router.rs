use axum::{Extension, Router, extract::Path, http::StatusCode, response::Response, routing::get};
use tower_http::services::{ServeDir, ServeFile};

use crate::{config::KangarooConfig, frontend_dev_server_client::FrontendDevelopmentServerClient};

/**
Extension for `axum::Router`. Import it to make the `.with_kangaroo(...)` method available.
Required to make Kangaroo endpoints work.
*/
pub trait KangarooRouterExtension {
    fn with_kangaroo(self, config: KangarooConfig) -> Self;
}

impl KangarooRouterExtension for Router {
    fn with_kangaroo(self, config: KangarooConfig) -> Self {
        let router = if let Some(frontend_development_server_url) =
            config.get_frontend_development_server_url()
        {
            let frontend_development_server_client = FrontendDevelopmentServerClient::new(
                &frontend_development_server_url,
                &config.get_folder_path(),
            );

            self.route("/{*path}", get(get_frontend_development_server_page))
                .layer(Extension(Some(frontend_development_server_client)))
        } else {
            self.fallback_service(
                ServeDir::new(config.get_folder_path()).not_found_service(ServeFile::new(
                    config
                        .get_full_path_from_status_code(StatusCode::NOT_FOUND)
                        .unwrap_or(config.get_default_path()),
                )),
            )
            .layer(Extension::<Option<FrontendDevelopmentServerClient>>(None))
        };

        router.layer(Extension(config))
    }
}

async fn get_frontend_development_server_page(
    Path(path): Path<String>,
    Extension(frontend_development_server_client): Extension<
        Option<FrontendDevelopmentServerClient>,
    >,
    Extension(config): Extension<KangarooConfig>,
) -> Response {
    let client = frontend_development_server_client
        .expect("unexpected missing frontend development server client");

    let not_found_path = config
        .get_full_path_from_status_code(StatusCode::NOT_FOUND)
        .unwrap_or(config.get_default_path());

    client
        .get_response_with_not_found_fallback(&path, &not_found_path)
        .await
}
