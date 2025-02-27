mod routes;

#[tokio::main]
async fn main() {
    let app = routes::get_routes().await;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7270")
        .await
        .unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}
