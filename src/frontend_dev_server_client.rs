use axum::{body::Body, response::Response};
use http_body_util::BodyExt;
use reqwest::{Client, StatusCode};

#[derive(Clone)]
pub struct FrontendDevelopmentServerClient {
    client: Client,
    frontend_development_server_url: String,
    static_files_folder_path: String,
}

impl FrontendDevelopmentServerClient {
    pub fn new(frontend_development_server_url: &str, static_files_folder_path: &str) -> Self {
        Self {
            client: Client::builder()
                .https_only(false)
                .build()
                .expect("unexpected reqwest client creation failure"),
            frontend_development_server_url: frontend_development_server_url.to_string(),
            static_files_folder_path: static_files_folder_path.to_string(),
        }
    }

    pub async fn get_response_with_not_found_fallback(
        &self,
        relative_path: &str,
        full_not_found_path: &str,
    ) -> Response<Body> {
        let response = self.get_response(relative_path).await;
        if response.status() == StatusCode::NOT_FOUND {
            let not_found_path = full_not_found_path
                .strip_prefix(&self.static_files_folder_path)
                .expect("unexpected missing prefix in not found path");
            let mut response = self.get_response(not_found_path).await;
            *response.status_mut() = StatusCode::NOT_FOUND;
            response
        } else {
            response
        }
    }

    pub async fn get_response_content(&self, full_path: &str) -> String {
        let path = full_path
            .strip_prefix(&self.static_files_folder_path)
            .expect("unexpected missing prefix in path");
        let response = self.get_response(path).await;

        String::from_utf8(
            response
                .into_body()
                .collect()
                .await
                .expect("unexpected development server response body collection failure")
                .to_bytes()
                .to_vec(),
        )
        .expect("unexpected development server response body parsing failure")
    }

    async fn get_response(&self, path: &str) -> Response<Body> {
        let url = if path.starts_with("/") {
            format!("{}{}", self.frontend_development_server_url, path)
        } else {
            format!("{}/{}", self.frontend_development_server_url, path)
        };
        let response = match self.client.get(url).send().await {
            Ok(response) => response,
            Err(error) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header("Content-Type", "text/plain")
                    .body(Body::from(error.to_string()))
                    .unwrap();
            }
        };

        let status = response.status();
        let headers = response.headers().clone();

        let body = Body::from(
            response
                .bytes()
                .await
                .expect("unexpected missing frontend development server body"),
        );

        let mut response = Response::new(body);
        *response.status_mut() = status;
        *response.headers_mut() = headers;

        response
    }
}
