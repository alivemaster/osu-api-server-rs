use super::*;

pub async fn handler(
    State(osu_client): State<Arc<Osu>>,
    Path(paths): Path<UserPaths>,
) -> Result<Json<UserResponse>, OsuErrorResponse> {
    let user = osu_client.user(paths.user_id);
    let user = if let Some(game_mode) = paths.game_mode {
        user.mode(GameMode::from(game_mode))
    } else {
        user
    };
    let user = user.await?;
    Ok(Json(user))
}

#[derive(Deserialize)]
pub struct UserPaths {
    user_id: u32,
    game_mode: Option<u8>,
}

type UserResponse = UserExtended;
