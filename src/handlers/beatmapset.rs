use super::*;

pub async fn handler(
    State(osu_client): State<Arc<Osu>>,
    Path(paths): Path<BeatmapsetPaths>,
) -> Result<BeatmapsetResponse, OsuErrorResponse> {
    let mut beatmapset = osu_client
        .beatmapset(paths.mapset_id)
        .await?;

    // replace assets urls
    // covers
    let covers = &mut beatmapset.covers;
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
    // avatars
    if let Some(ref mut users) = beatmapset.recent_favourites {
        for user in users {
            let url = &mut user.avatar_url;
            if let Ok(path) = utils::cache_api_assets(url).await {
                *url = path
            }
        }
    }

    Ok(Json(beatmapset))
}

#[derive(Deserialize)]
pub struct BeatmapsetPaths {
    pub mapset_id: u32,
}

impl Clone for BeatmapsetPaths {
    fn clone(&self) -> Self {
        Self {
            mapset_id: self.mapset_id,
        }
    }
}

type BeatmapsetResponse = Json<BeatmapsetExtended>;
