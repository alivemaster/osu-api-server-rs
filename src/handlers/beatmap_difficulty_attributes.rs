use super::*;

pub async fn handler(
    State(osu_client): State<Arc<Osu>>,
    Path(paths): Path<beatmap::BeatmapPaths>,
    Query(params): Query<beatmap::BeatmapParams>,
) -> Result<BeatmapDifficultyAttributesResponse, OsuErrorResponse> {
    let attributes = osu_client.beatmap_difficulty_attributes(paths.map_id);
    let attributes = if let Some(mode) = params.mode {
        attributes.mode(mode.into())
    } else {
        attributes
    };
    let attributes = if let Some(mods) = &params.mods {
        attributes.mods(GameModsIntermode::from_acronyms(mods))
    } else {
        attributes
    };
    let attributes = attributes.await?;

    Ok(Json(attributes))
}

type BeatmapDifficultyAttributesResponse = Json<BeatmapDifficultyAttributes>;
