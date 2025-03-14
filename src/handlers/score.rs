use super::*;
use utils::{cal_beatmap_attrs, replace_assets_urls::score_assets};

pub async fn handler(
    State(osu_client): State<Arc<Osu>>,
    Path(paths): Path<ScorePaths>,
) -> Result<ScoreResponse, OsuErrorResponse> {
    let score = osu_client.score(paths.score_id);
    let score = if let Some(mode) = paths.mode {
        score.mode(mode.into())
    } else {
        score
    };
    let mut score = score.await?;

    // replace beatmap attrs
    if let Some(ref mut beatmap) = score.map {
        cal_beatmap_attrs(
            beatmap,
            Some(score.mode as u8),
            Some(score.mods.clone().into()),
        );

        // star, max combo
        let diff_attrs = osu_client
            .beatmap_difficulty_attributes(score.map_id)
            .mode(score.mode)
            .mods(score.mods.clone())
            .await?;
        beatmap.stars = diff_attrs.stars as f32;
        if let Some(ref mut max_combo) = beatmap.max_combo {
            *max_combo = diff_attrs.max_combo
        }
    }

    // replace assets urls
    if CONFIG.server.cache {
        score_assets(&mut score).await
    }

    Ok(Json(score))
}

#[derive(Deserialize)]
pub struct ScorePaths {
    pub mode: Option<u8>,
    pub score_id: u64,
}

impl Clone for ScorePaths {
    fn clone(&self) -> Self {
        Self {
            mode: self.mode,
            score_id: self.score_id,
        }
    }
}

type ScoreResponse = Json<Score>;
