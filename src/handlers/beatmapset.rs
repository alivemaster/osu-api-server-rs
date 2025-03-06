use super::*;

pub async fn handler(
    State(osu_client): State<Arc<Osu>>,
    Path(paths): Path<BeatmapsetPaths>,
) -> Result<Json<BeatmapsetResponse>, OsuErrorResponse> {
    let beatmapset = osu_client
        .beatmapset(paths.mapset_id)
        .await?;
    Ok(Json(beatmapset))
}

#[derive(Deserialize)]
pub struct BeatmapsetPaths {
    mapset_id: u32,
}

type BeatmapsetResponse = BeatmapsetExtended;
