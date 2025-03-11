mod config;
mod handlers;
mod routes;
mod utils;

use config::Config;
use rosu_v2::prelude::*;
use routes::create as create_routes;
use std::sync::{Arc, LazyLock};

static SELF_DIR: LazyLock<std::path::PathBuf> = LazyLock::new(|| {
    let exe_path = std::env::current_exe().unwrap();
    let self_path = exe_path
        .parent()
        .unwrap();
    self_path.to_path_buf()
});

#[tokio::main]
async fn main() {
    let config = Config::parse();
    let osu_client = Arc::new(
        Osu::new(
            config.osu.client_id,
            config
                .osu
                .client_secret,
        )
        .await
        .unwrap(),
    );

    let app = create_routes().with_state(osu_client);
    let listener = tokio::net::TcpListener::bind((
        config
            .listener
            .address,
        config.listener.port,
    ))
    .await
    .unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}
