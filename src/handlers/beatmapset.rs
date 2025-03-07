use super::*;

pub async fn handler(
    State(osu_client): State<Arc<Osu>>,
    Path(paths): Path<BeatmapsetPaths>,
) -> Result<BeatmapsetResponse, OsuErrorResponse> {
    let beatmapset = osu_client
        .beatmapset(paths.mapset_id)
        .await?;

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
