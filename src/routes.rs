use crate::handlers::*;
use axum::{Router, routing::get};

pub fn create() -> Router<std::sync::Arc<rosu_v2::Osu>> {
    let index = Router::new().route("/", get(|| async { "osu-api-server-rs is running!" }));
    let beatmaps = Router::new().route("/{map_id}", get(beatmap_handler));
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
        .nest("/beatmaps", beatmaps)
        .nest("/beatmapsets", beatmapsets)
        .nest("/matches", matches)
        .nest("/scores", scores)
        .nest("/users", users)
}
