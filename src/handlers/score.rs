use super::*;

pub async fn handler(
    State(osu_client): State<Arc<Osu>>,
    Path(paths): Path<ScorePaths>,
) -> Json<ScoreResponse> {
    let score = osu_client.score(paths.score_id);
    let score = if let Some(game_mode) = paths.game_mode {
        score.mode(GameMode::from(game_mode))
    } else {
        score
    };
    let score = score.await.unwrap();

    Json(score)
}

#[derive(Deserialize)]
pub struct ScorePaths {
    game_mode: Option<u8>,
    score_id: u64,
}

type ScoreResponse = Score;
