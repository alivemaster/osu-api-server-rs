use super::*;

pub async fn handler(
    State(osu_client): State<Arc<Osu>>,
    Path(paths): Path<BeatmapPaths>,
    Query(params): Query<BeatmapParams>,
) -> Result<BeatmapResponse, OsuErrorResponse> {
    let mut beatmap = osu_client
        .beatmap()
        .map_id(paths.map_id)
        .await?;

    let attributes = osu_client.beatmap_difficulty_attributes(paths.map_id);
    let attributes = if let Some(game_mode) = params.game_mode {
        let mode = GameMode::from(game_mode);
        if mode != beatmap.mode && beatmap.mode == GameMode::Osu {
            beatmap.mode = mode;
            beatmap.convert = true;
            attributes.mode(mode)
        } else {
            attributes
        }
    } else {
        attributes
    };
    let attributes = if let Some(mods) = params.mods {
        attributes.mods(GameModsIntermode::from_acronyms(&mods))
    } else {
        attributes
    };
    let attributes = attributes.await?;

    if let Some(ref mut max_combo) = beatmap.max_combo {
        *max_combo = attributes.max_combo;
    }
    beatmap.stars = attributes.stars as f32;

    Ok(Json(BeatmapResponse {
        beatmap,
        attributes: attributes.attrs,
    }))
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
