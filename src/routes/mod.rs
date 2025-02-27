use osu_api_server_rs::api;
use warp::{Filter, Rejection, Reply};

pub async fn get_routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let index = warp::path::end().map(|| "osu-api-server-rs is running!");
    let beatmap = warp::path!("beatmap" / u32).then(|map_id: u32| async move {
        let beatmap = api::v2::beatmap_test(map_id).await;
        let beatmapset = beatmap
            .mapset
            .unwrap();
        format!("Artist: {}, Title: {}", beatmapset.artist, beatmapset.title)
    });
    let beatmap_attrs =
        warp::path!("beatmap" / u8 / u32).then(|game_mode: u8, map_id: u32| async move {
            let beatmap_attrs = api::v2::beatmap_attrs_test(game_mode, map_id).await;
            format!("Star: {}", beatmap_attrs.stars)
        });
    let score = warp::path!("score" / u64).then(|score_id: u64| async move {
        let score = api::v2::score_test(score_id).await;
        format!(
            "Score: {}, Accuracy: {}, Max Combo: {}",
            score.score, score.accuracy, score.max_combo
        )
    });
    let score_with_mode =
        warp::path!("score" / u8 / u64).then(|game_mode: u8, score_id: u64| async move {
            let score = api::v2::score_with_mode_test(game_mode, score_id).await;
            format!(
                "Score: {}, Accuracy: {}, Max Combo: {}",
                score.score, score.accuracy, score.max_combo
            )
        });

    let user = warp::path!("user" / u32).then(|user_id: u32| async move {
        let user = api::v2::user_test(user_id).await;
        let stats = user
            .statistics
            .unwrap();
        format!("Username: {}, PP: {}", user.username, stats.pp)
    });

    index.or(warp::get().and(
        beatmap
            .or(beatmap_attrs)
            .or(score)
            .or(score_with_mode)
            .or(user),
    ))
}
