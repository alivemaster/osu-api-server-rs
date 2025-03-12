use super::*;
use utils::replace_assets_urls::beatmapset_extended_assets;

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
        if let Some(ref mut mapset) = beatmap.mapset {
            beatmapset_extended_assets(mapset).await
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
