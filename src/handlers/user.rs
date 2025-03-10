use super::*;

pub async fn handler(
    State(osu_client): State<Arc<Osu>>,
    Path(paths): Path<UserPaths>,
) -> Result<UserResponse, OsuErrorResponse> {
    let user = osu_client.user(paths.user_id);
    let user = if let Some(game_mode) = paths.game_mode {
        user.mode(GameMode::from(game_mode))
    } else {
        user
    };
    let mut user = user.await?;

    // replace assets urls
    // avatar
    if let Ok(path) = utils::cache_api_assets(&user.avatar_url).await {
        user.avatar_url = path
    }
    // custom cover (if have)
    if let Some(ref mut url) = user
        .cover
        .custom_url
    {
        if let Ok(path) = utils::cache_api_assets(&url).await {
            *url = path
        }
    }
    // cover
    if let Ok(path) = utils::cache_api_assets(&user.cover.url).await {
        user.cover.url = path
    }
    // team flag
    if let Some(ref mut team) = user.team {
        if let Some(ref mut url) = team.flag_url {
            if let Ok(path) = utils::cache_api_assets(&url).await {
                *url = path
            }
        }
    }
    // badges
    if let Some(ref mut items) = user.badges {
        for item in items {
            let url = &mut item.image_url;
            if let Ok(path) = utils::cache_api_assets(url).await {
                *url = path
            }
        }
    }

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

type UserResponse = Json<UserExtended>;
