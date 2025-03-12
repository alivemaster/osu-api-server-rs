use super::*;
use utils::replace_assets_urls::user_extended_assets;

pub async fn handler(
    State(osu_client): State<Arc<Osu>>,
    Path(paths): Path<UserPaths>,
) -> Result<UserResponse, OsuErrorResponse> {
    let user = osu_client.user(paths.user_id);
    let user = if let Some(mode) = paths.mode {
        user.mode(GameMode::from(mode))
    } else {
        user
    };
    let mut user = user.await?;

    // replace assets urls
    if CONFIG.server.cache {
        user_extended_assets(&mut user).await
    }

    Ok(Json(user))
}

#[derive(Deserialize)]
pub struct UserPaths {
    pub user_id: u32,
    pub mode: Option<u8>,
}

impl Clone for UserPaths {
    fn clone(&self) -> Self {
        Self {
            user_id: self.user_id,
            mode: self.mode,
        }
    }
}

type UserResponse = Json<UserExtended>;
