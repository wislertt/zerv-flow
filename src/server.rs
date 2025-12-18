//! Server utilities module
//!
//! Contains server setup, logging, and initialization utilities.

use crate::routes::create_router;
use std::net::IpAddr;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn create_app() -> axum::Router {
    create_router()
}

pub fn log_server_urls(ip: IpAddr, port: u16) {
    if ip.is_unspecified() {
        // 0.0.0.0 or :: (unspecified address)
        info!("Server listening on http://localhost:{}", port);
        info!("Server also accessible on http://127.0.0.1:{}", port);
    } else {
        // Specific IP address
        info!("Server listening on http://{}:{}", ip, port);
    }
}

pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "zerv_flow_demo=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use std::net::{IpAddr, Ipv4Addr};
    use tower_http::trace::TraceLayer;

    #[tokio::test]
    async fn test_create_app() {
        let app = create_app();

        let app_with_trace = app.layer(TraceLayer::new_for_http());

        let server = TestServer::new(app_with_trace).unwrap();

        let response = server.get("/").await;
        response.assert_status(StatusCode::OK);

        let response = server.get("/hello").await;
        response.assert_status(StatusCode::OK);

        let response = server.get("/nonexistent").await;
        response.assert_status(StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_log_server_urls_unspecified_address() {
        let ip: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        log_server_urls(ip, 3000);
        // Function doesn't panic - if we reach here, test passes
    }

    #[test]
    fn test_log_server_urls_specific_address() {
        let ip: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        log_server_urls(ip, 8080);
        // Function doesn't panic - if we reach here, test passes
    }

    #[test]
    fn test_log_server_urls_ipv6() {
        let ip: IpAddr = IpAddr::V6(std::net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
        log_server_urls(ip, 3000);
        // Function doesn't panic - if we reach here, test passes
    }

    #[test]
    fn test_init_tracing() {
        init_tracing();
        // If we reach here, tracing initialized successfully
    }
}
