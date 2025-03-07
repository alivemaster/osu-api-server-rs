use super::*;

pub async fn handler(
    State(osu_client): State<Arc<Osu>>,
    Path(paths): Path<ScorePaths>,
) -> Result<ScoreResponse, OsuErrorResponse> {
    let score = osu_client.score(paths.score_id);
    let score = if let Some(game_mode) = paths.game_mode {
        score.mode(GameMode::from(game_mode))
    } else {
        score
    };
    let score = score.await?;

    Ok(Json(score))
}

#[derive(Deserialize)]
pub struct ScorePaths {
    pub game_mode: Option<u8>,
    pub score_id: u64,
}

impl Clone for ScorePaths {
    fn clone(&self) -> Self {
        Self {
            game_mode: self.game_mode,
            score_id: self.score_id,
        }
    }
}

type ScoreResponse = Json<Score>;
