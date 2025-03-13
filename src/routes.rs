use crate::handlers::*;
use axum::{Router, routing::get};

pub fn create() -> Router<std::sync::Arc<rosu_v2::Osu>> {
    let index = Router::new().route("/", get(|| async { "osu-api-server-rs is running!" }));
    let assets = Router::new().route("/{*path}", get(assets_handler));
    let beatmaps = Router::new()
        .route("/{map_id}", get(beatmap_handler))
        .route("/{map_id}/attributes", get(beatmap_attributes_handler));
    let beatmapsets = Router::new().route("/{mapset_id}", get(beatmapset_handler));
    let matches = Router::new().route("/{match_id}", get(osu_match_handler));
    let scores = Router::new()
        .route("/{score_id}", get(score_handler))
        .route("/{game_mode}/{score_id}", get(score_handler));
    let users = Router::new()
        .route("/{user_id}", get(user_handler))
        .route("/{user_id}/{game_mode}", get(user_handler));

    Router::new()
        .merge(index)
        .nest("/assets", assets)
        .nest("/beatmaps", beatmaps)
        .nest("/beatmapsets", beatmapsets)
        .nest("/matches", matches)
        .nest("/scores", scores)
        .nest("/users", users)
}
