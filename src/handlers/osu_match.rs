use super::*;

pub async fn handler(
    State(osu_client): State<Arc<Osu>>,
    Path(paths): Path<OsuMatchPaths>,
) -> Result<OsuMatchResponse, OsuErrorResponse> {
    let osu_match = osu_client
        .osu_match(paths.match_id)
        .await?;
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
