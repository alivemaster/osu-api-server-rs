use super::*;
use utils::replace_assets_urls::beatmapset_extended_assets;

pub async fn handler(
    State(osu_client): State<Arc<Osu>>,
    Path(paths): Path<BeatmapsetPaths>,
) -> Result<BeatmapsetResponse, OsuErrorResponse> {
    let mut beatmapset = osu_client
        .beatmapset(paths.mapset_id)
        .await?;

    // replace assets urls
    if CONFIG.server.cache {
        beatmapset_extended_assets(&mut beatmapset).await
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
