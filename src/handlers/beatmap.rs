use super::*;

pub async fn handler(
    State(osu_client): State<Arc<Osu>>,
    Path(paths): Path<BeatmapPaths>,
) -> Result<BeatmapResponse, OsuErrorResponse> {
    let mut beatmap = osu_client
        .beatmap()
        .map_id(paths.map_id)
        .await?;

    // replace assets urls
    if CONFIG.server.cache {
        // covers
        if let Some(ref mut mapset) = beatmap.mapset {
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

    Ok(Json(beatmap))
}

#[derive(Deserialize)]
pub struct BeatmapPaths {
    pub map_id: u32,
}

impl Clone for BeatmapPaths {
    fn clone(&self) -> Self {
        Self {
            map_id: self.map_id,
        }
    }
}

type BeatmapResponse = Json<BeatmapExtended>;
