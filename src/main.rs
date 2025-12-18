use zerv_flow_demo::{create_app, get_server_address, init_tracing, log_server_urls};

#[tokio::main]
async fn main() {
    init_tracing();

    let app = create_app();

    let addr = get_server_address();

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    let local_addr = listener.local_addr().unwrap();

    log_server_urls(local_addr.ip(), local_addr.port());

    axum::serve(listener, app).await.unwrap();
}
