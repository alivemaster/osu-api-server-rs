use super::*;

pub async fn handler(
    State(osu_client): State<Arc<Osu>>,
    Path(paths): Path<beatmap::BeatmapPaths>,
    Query(params): Query<BeatmapAttributesParams>,
) -> Result<BeatmapAttributesResponse, OsuErrorResponse> {
    let attributes = osu_client.beatmap_difficulty_attributes(paths.map_id);
    let attributes = if let Some(game_mode) = params.game_mode {
        attributes.mode(GameMode::from(game_mode))
    } else {
        attributes
    };
    let attributes = if let Some(mods) = params.mods {
        attributes.mods(GameModsIntermode::from_acronyms(&mods))
    } else {
        attributes
    };
    let attributes = attributes.await?;

    Ok(Json(attributes))
}

#[derive(Deserialize)]
pub struct BeatmapAttributesParams {
    pub game_mode: Option<u8>,
    pub mods: Option<String>,
}

impl Clone for BeatmapAttributesParams {
    fn clone(&self) -> Self {
        Self {
            game_mode: self.game_mode,
            mods: self.mods.to_owned(),
        }
    }
}

type BeatmapAttributesResponse = Json<BeatmapDifficultyAttributes>;
