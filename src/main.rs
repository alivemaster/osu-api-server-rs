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

static CONFIG: LazyLock<Config> = LazyLock::new(|| Config::parse());

#[tokio::main]
async fn main() {
    let osu_client = Arc::new(
        Osu::new(
            CONFIG.osu.client_id,
            CONFIG
                .osu
                .client_secret
                .to_owned(),
        )
        .await
        .unwrap(),
    );

    let app = create_routes().with_state(osu_client);
    let listener = tokio::net::TcpListener::bind((
        CONFIG
            .listener
            .address
            .to_owned(),
        CONFIG.listener.port,
    ))
    .await
    .unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}
