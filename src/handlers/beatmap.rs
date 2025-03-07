use super::*;

pub async fn handler(
    State(osu_client): State<Arc<Osu>>,
    Path(paths): Path<BeatmapPaths>,
) -> Result<BeatmapResponse, OsuErrorResponse> {
    let beatmap = osu_client
        .beatmap()
        .map_id(paths.map_id)
        .await?;

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
