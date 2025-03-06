use super::*;

pub async fn handler(
    State(osu_client): State<Arc<Osu>>,
    Path(paths): Path<OsuMatchPaths>,
) -> Result<Json<OsuMatchResponse>, OsuErrorResponse> {
    let osu_match = osu_client
        .osu_match(paths.match_id)
        .await?;
    Ok(Json(osu_match))
}

#[derive(Deserialize)]
pub struct OsuMatchPaths {
    match_id: u32,
}

type OsuMatchResponse = OsuMatch;
