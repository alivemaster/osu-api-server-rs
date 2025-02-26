mod routes;

#[tokio::main]
async fn main() {
    warp::serve(routes::get_routes())
        .run(([127, 0, 0, 1], 7270))
        .await
}
