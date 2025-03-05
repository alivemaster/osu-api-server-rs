mod handlers;
mod routes;
use routes::create as create_routes;

#[tokio::main]
async fn main() {
    let osu_client = std::sync::Arc::new(
        rosu_v2::Osu::new(123456, "asdfghjkl")
            .await
            .unwrap(),
    );
    let app = create_routes().with_state(osu_client);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7270")
        .await
        .unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}
