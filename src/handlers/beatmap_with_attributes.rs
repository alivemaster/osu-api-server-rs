use super::*;

pub async fn handler(
    State(osu_client): State<Arc<Osu>>,
    Path(paths): Path<beatmap::BeatmapPaths>,
    Query(params): Query<beatmap_attributes::BeatmapAttributesParams>,
) -> Result<BeatmapWithAttributesResponse, OsuErrorResponse> {
    let beatmap = beatmap_handler(State(osu_client.clone()), Path(paths.clone())).await?;
    let mut beatmap = beatmap.0;

    let attributes =
        beatmap_attributes_handler(State(osu_client), Path(paths), Query(params.clone())).await?;
    let attributes = attributes.0;

    if let Some(mode) = params.mode {
        let mode = GameMode::from(mode);
        if mode != beatmap.mode && beatmap.mode == GameMode::Osu {
            beatmap.mode = mode;
            beatmap.convert = true;
        }
    }

    Ok(Json(BeatmapWithAttributes {
        beatmap,
        attributes,
    }))
}

#[derive(Serialize)]
pub struct BeatmapWithAttributes {
    beatmap: BeatmapExtended,
    attributes: BeatmapDifficultyAttributes,
}

type BeatmapWithAttributesResponse = Json<BeatmapWithAttributes>;
