use super::*;

pub async fn handler(
    State(osu_client): State<Arc<Osu>>,
    Path(paths): Path<OsuMatchPaths>,
) -> Result<OsuMatchResponse, OsuErrorResponse> {
    let mut osu_match = osu_client
        .osu_match(paths.match_id)
        .await?;

    // replace assets urls
    if CONFIG.server.cache {
        // beatmap covers
        for event in &mut osu_match.events {
            if let MatchEvent::Game { game, .. } = event {
                if let Some(map) = &mut game.map {
                    if let Some(mapset) = &mut map.mapset {
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
                }
            }
        }
        for user in &mut osu_match.users {
            // avatar
            if let Ok(path) = utils::cache_api_assets(&user.1.avatar_url).await {
                user.1.avatar_url = path
            }
        }
    }

    Ok(Json(osu_match))
}

#[derive(Deserialize)]
pub struct OsuMatchPaths {
    pub match_id: u32,
}

impl Clone for OsuMatchPaths {
    fn clone(&self) -> Self {
        Self {
            match_id: self.match_id,
        }
    }
}

type OsuMatchResponse = Json<OsuMatch>;
