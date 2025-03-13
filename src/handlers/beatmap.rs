use super::*;
use utils::{cal_beatmap_attrs, replace_assets_urls::beatmapset_extended_assets};

pub async fn handler(
    State(osu_client): State<Arc<Osu>>,
    Path(paths): Path<BeatmapPaths>,
    Query(params): Query<BeatmapParams>,
) -> Result<BeatmapResponse, OsuErrorResponse> {
    let mut beatmap = osu_client
        .beatmap()
        .map_id(paths.map_id)
        .await?;

    // replace attrs
    if params
        .mode
        .is_some()
        || params
            .mods
            .is_some()
    {
        // mode(if it's a converted map), ar, cs, od, hp, bpm, length
        let mods = match &params.mods {
            Some(mods) => Some(GameModsIntermode::from_acronyms(mods)),
            None => None,
        };
        cal_beatmap_attrs(&mut beatmap, params.mode, mods);

        // star, max combo
        let diff_attrs = beatmap_difficulty_attributes_handler(
            State(osu_client),
            Path(paths),
            Query(params.clone()),
        )
        .await?;
        beatmap.stars = diff_attrs.stars as f32;
        if let Some(ref mut max_combo) = beatmap.max_combo {
            *max_combo = diff_attrs.max_combo
        }
    }

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

#[derive(Deserialize)]
pub struct BeatmapParams {
    pub mode: Option<u8>,
    pub mods: Option<String>,
}

impl Clone for BeatmapParams {
    fn clone(&self) -> Self {
        Self {
            mode: self.mode,
            mods: self.mods.to_owned(),
        }
    }
}

type BeatmapResponse = Json<BeatmapExtended>;
