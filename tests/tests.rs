use crate::mocks::{create_mock_app, create_mock_frontend_dev_server};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

mod mocks;

struct TestResponse {
    status_code: StatusCode,
    body: String,
}

async fn send_test_request(uri: &str, frontend_dev_server_url: Option<&str>) -> TestResponse {
    let app = create_mock_app(frontend_dev_server_url);

    let response = app
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();

    let status_code = response.status();

    let body = String::from_utf8(
        response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes()
            .to_vec(),
    )
    .unwrap();

    TestResponse { status_code, body }
}

#[tokio::test]
async fn home_succeeds() {
    let response = send_test_request("/", None).await;

    assert_eq!(response.status_code, StatusCode::OK);
    assert_eq!(&response.body, "<html>
    <head><script type=\"application/json\" id=\"kangaroo-data\">{\"resource\":\"sample resource\"}</script>
        <title>Example</title>
    </head>
    <body>
        <h1>Home Page</h1>
    </body>
</html>
");
}

#[tokio::test]
async fn not_found_error_gives_404() {
    let response = send_test_request("/404", None).await;

    assert_eq!(response.status_code, StatusCode::NOT_FOUND);
    assert_eq!(
        &response.body,
        "<html>
    <head><script type=\"application/json\" id=\"kangaroo-data\">\"example error\"</script>
        <title>Example</title>
    </head>
    <body>
        <h1>Home Page</h1>
    </body>
</html>
"
    );
}

#[tokio::test]
async fn parametrised_path_succeeds() {
    for parameter in &["example-1", "example-2", "example-3"] {
        let uri = format!("/resources/{}", parameter);
        let response = send_test_request(&uri, None).await;

        assert_eq!(response.status_code, StatusCode::OK);
        assert_eq!(
            response.body,
            format!(
                "<html>
    <head><script type=\"application/json\" id=\"kangaroo-data\">{{\"resource\":\"{}\"}}</script>
        <title>Example</title>
    </head>
    <body>
        <h1>Home Page</h1>
    </body>
</html>
",
                parameter
            )
        );
    }
}

#[tokio::test]
async fn custom_succeeds() {
    let response = send_test_request("/custom", None).await;

    assert_eq!(response.status_code, StatusCode::OK);
    assert_eq!(&response.body, "<html>
    <head><script type=\"application/json\" id=\"kangaroo-data\">{\"resource\":\"sample resource\"}</script>
        <title>Example</title>
    </head>
    <body>
        <h1>Custom</h1>
    </body>
</html>
");
}

#[tokio::test]
async fn absent_document_gives_internal_error() {
    let response = send_test_request("/absent-document", None).await;

    assert_eq!(response.status_code, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(&response.body, "error reading document");
}

#[tokio::test]
async fn invalid_document_gives_internal_error() {
    let response = send_test_request("/invalid-document", None).await;

    assert_eq!(response.status_code, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(&response.body, "invalid html document");
}

#[tokio::test]
async fn deleted_resources_gives_status_code_specific_page() {
    let response = send_test_request("/deleted", None).await;

    assert_eq!(response.status_code, StatusCode::GONE);
    assert_eq!(
        &response.body,
        "<html>
    <head><script type=\"application/json\" id=\"kangaroo-data\">\"example error\"</script>
        <title>Example</title>
    </head>
    <body>
        <h1>Gone</h1>
    </body>
</html>
"
    );
}

#[tokio::test]
async fn custom_deleted_resources_gives_custom_status_code_specific_page() {
    let response = send_test_request("/deleted-custom", None).await;

    assert_eq!(response.status_code, StatusCode::GONE);
    assert_eq!(
        &response.body,
        "<html>
    <head><script type=\"application/json\" id=\"kangaroo-data\">\"example error\"</script>
        <title>Example</title>
    </head>
    <body>
        <h1>Custom</h1>
    </body>
</html>
"
    );
}

#[tokio::test]
async fn directly_accessing_static_resource_succeeds() {
    let response = send_test_request("/index.css", None).await;

    assert_eq!(response.status_code, StatusCode::OK);
    assert_eq!(
        &response.body,
        "body {
    width: 100%;
}
"
    );
}

#[tokio::test]
async fn directly_accessing_missing_static_resource_gives_not_found() {
    let response = send_test_request("/missing.css", None).await;

    assert_eq!(response.status_code, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn frontend_dev_server_home_succeeds() {
    let frontend_dev_server = create_mock_frontend_dev_server().await;
    let response = send_test_request("/", Some(&frontend_dev_server.address)).await;

    assert_eq!(response.status_code, StatusCode::OK);
    assert_eq!(&response.body, "<html>
    <head><script type=\"application/json\" id=\"kangaroo-data\">{\"resource\":\"sample resource\"}</script>
        <title>Dev Server Example</title>
    </head>
    <body>
        <h1>Home Page</h1>
    </body>
</html>
");
}

#[tokio::test]
async fn directly_accessing_frontend_dev_server_static_resource_succeeds() {
    let frontend_dev_server = create_mock_frontend_dev_server().await;
    let response = send_test_request("/index.css", Some(&frontend_dev_server.address)).await;

    assert_eq!(response.status_code, StatusCode::OK);
    assert_eq!(
        &response.body,
        ".body-from-dev-server {
    width: 100%;
}
"
    );
}

#[tokio::test]
async fn directly_accessing_frontend_dev_server_missing_static_resource_gives_not_found() {
    let frontend_dev_server = create_mock_frontend_dev_server().await;
    let response = send_test_request("/missing.css", Some(&frontend_dev_server.address)).await;

    assert_eq!(response.status_code, StatusCode::NOT_FOUND);
}
