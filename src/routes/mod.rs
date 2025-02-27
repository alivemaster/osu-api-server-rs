use axum::{Router, extract::Path, routing::get};
use osu_api_server_rs::api;

pub async fn get_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "osu-api-server-rs is running!" }))
        .route(
            "/beatmap/{map_id}",
            get(|Path(map_id): Path<u32>| async move {
                let beatmap = api::v2::beatmap_test(map_id).await;
                let beatmapset = beatmap
                    .mapset
                    .unwrap();
                format!("Artist: {}, Title: {}", beatmapset.artist, beatmapset.title)
            }),
        )
        .route(
            "/beatmap/{game_mode}/{map_id}",
            get(|Path((game_mode, map_id)): Path<(u8, u32)>| async move {
                let beatmap_attrs = api::v2::beatmap_attrs_test(game_mode, map_id).await;
                format!("Star: {}", beatmap_attrs.stars)
            }),
        )
        .route(
            "/score/{score_id}",
            get(|Path(score_id): Path<u64>| async move {
                let score = api::v2::score_test(score_id).await;
                format!(
                    "Score: {}, Accuracy: {}, Max Combo: {}",
                    score.score, score.accuracy, score.max_combo
                )
            }),
        )
        .route(
            "/score/{game_mode}/{score_id}",
            get(|Path((game_mode, score_id)): Path<(u8, u64)>| async move {
                let score = api::v2::score_with_mode_test(game_mode, score_id).await;
                format!(
                    "Score: {}, Accuracy: {}, Max Combo: {}",
                    score.score, score.accuracy, score.max_combo
                )
            }),
        )
        .route(
            "/user/{user_id}",
            get(|Path(user_id): Path<u32>| async move {
                let user = api::v2::user_test(user_id).await;
                let stats = user
                    .statistics
                    .unwrap();
                format!("Username: {}, PP: {}", user.username, stats.pp)
            }),
        )
}
