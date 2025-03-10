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
    let mut score = score.await?;

    // replace assets urls
    // covers
    if let Some(ref mut mapset) = score.mapset {
        let covers = &mut mapset.covers;
        if let Ok(path) = utils::cache_api_assets(&covers.cover).await {
            covers.cover = path
        }
        if let Ok(path) = utils::cache_api_assets(&covers.cover_2x).await {
            covers.cover_2x = path
        }
        if let Ok(path) = utils::cache_api_assets(&covers.card).await {
            covers.card = path
        }
        if let Ok(path) = utils::cache_api_assets(&covers.card_2x).await {
            covers.card_2x = path
        }
        if let Ok(path) = utils::cache_api_assets(&covers.list).await {
            covers.list = path
        }
        if let Ok(path) = utils::cache_api_assets(&covers.list_2x).await {
            covers.list_2x = path
        }
        if let Ok(path) = utils::cache_api_assets(&covers.slim_cover).await {
            covers.slim_cover = path
        }
        if let Ok(path) = utils::cache_api_assets(&covers.slim_cover_2x).await {
            covers.slim_cover_2x = path
        }
    }
    if let Some(ref mut user) = score.user {
        // avatar
        if let Ok(path) = utils::cache_api_assets(&user.avatar_url).await {
            user.avatar_url = path
        }
        if let Some(ref mut cover) = user.cover {
            // custom cover (if have)
            if let Some(ref mut url) = cover.custom_url {
                if let Ok(path) = utils::cache_api_assets(&url).await {
                    *url = path
                }
            }
            // cover
            if let Ok(path) = utils::cache_api_assets(&cover.url).await {
                cover.url = path
            }
        }
        // team flag
        if let Some(ref mut team) = user.team {
            if let Some(ref mut url) = team.flag_url {
                if let Ok(path) = utils::cache_api_assets(&url).await {
                    *url = path
                }
            }
        }
    }

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
