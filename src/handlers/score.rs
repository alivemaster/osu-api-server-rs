use super::*;
use utils::replace_assets_urls::score_assets;

pub async fn handler(
    State(osu_client): State<Arc<Osu>>,
    Path(paths): Path<ScorePaths>,
) -> Result<ScoreResponse, OsuErrorResponse> {
    let score = osu_client.score(paths.score_id);
    let score = if let Some(mode) = paths.mode {
        score.mode(GameMode::from(mode))
    } else {
        score
    };
    let mut score = score.await?;

    // replace assets urls
    if CONFIG.server.cache {
        score_assets(&mut score).await
    }

    Ok(Json(score))
}

#[derive(Deserialize)]
pub struct ScorePaths {
    pub mode: Option<u8>,
    pub score_id: u64,
}

impl Clone for ScorePaths {
    fn clone(&self) -> Self {
        Self {
            mode: self.mode,
            score_id: self.score_id,
        }
    }
}

type ScoreResponse = Json<Score>;
