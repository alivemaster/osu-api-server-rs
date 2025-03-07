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
    pub user_id: u32,
    pub game_mode: Option<u8>,
}

impl Clone for UserPaths {
    fn clone(&self) -> Self {
        Self {
            user_id: self.user_id,
            game_mode: self.game_mode,
        }
    }
}

type UserResponse = UserExtended;
