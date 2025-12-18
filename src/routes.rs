use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

pub async fn handler() -> Json<Value> {
    Json(json!({
        "message": "Hello, Axum World!",
        "description": "Welcome to your Rust web server"
    }))
}

pub async fn hello_handler() -> Json<Value> {
    Json(json!({
        "message": "Hello from Axum!"
    }))
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/hello", get(hello_handler))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use serde_json::Value;
    use tower_http::trace::TraceLayer;

    fn create_test_app() -> Router {
        create_router().layer(TraceLayer::new_for_http())
    }

    #[tokio::test]
    async fn test_root_endpoint() {
        let app = create_test_app();
        let server = TestServer::new(app).unwrap();

        let response = server.get("/").await;
        response.assert_status(StatusCode::OK);

        let json_body: Value = response.json();
        assert_eq!(json_body["message"], "Hello, Axum World!");
        assert_eq!(json_body["description"], "Welcome to your Rust web server");
    }

    #[tokio::test]
    async fn test_hello_endpoint() {
        let app = create_test_app();
        let server = TestServer::new(app).unwrap();

        let response = server.get("/hello").await;
        response.assert_status(StatusCode::OK);

        let json_body: Value = response.json();
        assert_eq!(json_body["message"], "Hello from Axum!");
    }

    // Test that JSON responses are properly formatted
    #[test]
    fn test_json_response_structure() {
        // Use tokio_test to block on async functions for unit tests
        let runtime = tokio::runtime::Runtime::new().unwrap();

        // Test root endpoint JSON structure
        let json_result = runtime.block_on(async {
            let response = handler().await;
            response.0
        });

        assert_eq!(json_result["message"], "Hello, Axum World!");
        assert_eq!(
            json_result["description"],
            "Welcome to your Rust web server"
        );

        // Test hello endpoint JSON structure
        let json_result = runtime.block_on(async {
            let response = hello_handler().await;
            response.0
        });

        assert_eq!(json_result["message"], "Hello from Axum!");
    }
}
