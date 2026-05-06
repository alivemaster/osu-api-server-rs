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

#[derive(Deserialize, Clone)]
pub struct BeatmapsetPaths {
    pub mapset_id: u32,
}

type BeatmapsetResponse = Json<BeatmapsetExtended>;
