use super::*;
use utils::replace_assets_urls::osu_match_assets;

pub async fn handler(
    State(osu_client): State<Arc<Osu>>,
    Path(paths): Path<OsuMatchPaths>,
) -> Result<OsuMatchResponse, OsuErrorResponse> {
    let mut osu_match = osu_client
        .osu_match(paths.match_id)
        .await?;

    // replace assets urls
    if CONFIG.server.cache {
        osu_match_assets(&mut osu_match).await
    }

    Ok(Json(osu_match))
}

#[derive(Deserialize)]
pub struct OsuMatchPaths {
    pub match_id: u32,
}

impl Clone for OsuMatchPaths {
    fn clone(&self) -> Self {
        Self {
            match_id: self.match_id,
        }
    }
}

type OsuMatchResponse = Json<OsuMatch>;
