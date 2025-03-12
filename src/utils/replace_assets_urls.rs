use super::cache_api_assets;
use rosu_v2::prelude::*;

pub async fn badge_assets(badge: &mut Badge) {
    let url = &mut badge.image_url;
    if let Ok(path) = cache_api_assets(url).await {
        *url = path
    }
}

pub async fn beatmapset_assets(mapset: &mut Beatmapset) {
    // covers
    beatmapset_covers(&mut mapset.covers).await;
}

pub async fn beatmapset_covers(covers: &mut BeatmapsetCovers) {
    if let Ok(path) = cache_api_assets(&covers.cover).await {
        covers.cover = path
    }
    if let Ok(path) = cache_api_assets(&covers.cover_2x).await {
        covers.cover_2x = path
    }
    if let Ok(path) = cache_api_assets(&covers.card).await {
        covers.card = path
    }
    if let Ok(path) = cache_api_assets(&covers.card_2x).await {
        covers.card_2x = path
    }
    if let Ok(path) = cache_api_assets(&covers.list).await {
        covers.list = path
    }
    if let Ok(path) = cache_api_assets(&covers.list_2x).await {
        covers.list_2x = path
    }
    if let Ok(path) = cache_api_assets(&covers.slim_cover).await {
        covers.slim_cover = path
    }
    if let Ok(path) = cache_api_assets(&covers.slim_cover_2x).await {
        covers.slim_cover_2x = path
    }
}

pub async fn beatmapset_extended_assets(mapset: &mut BeatmapsetExtended) {
    // covers
    beatmapset_covers(&mut mapset.covers).await;
    // creator user
    if let Some(ref mut user) = mapset.creator {
        user_assets(user).await
    }
    // favorate users
    if let Some(ref mut users) = mapset.recent_favourites {
        for user in users {
            user_assets(user).await
        }
    }
}

pub async fn osu_match_assets(osu_match: &mut OsuMatch) {
    // beatmapsets
    for event in &mut osu_match.events {
        if let MatchEvent::Game { game, .. } = event {
            if let Some(map) = &mut game.map {
                if let Some(mapset) = &mut map.mapset {
                    beatmapset_assets(mapset).await
                }
            }
        }
    }
    // users
    for user_hashmap in &mut osu_match.users {
        let user = user_hashmap.1;
        user_assets(user).await
    }
}

pub async fn score_assets(score: &mut Score) {
    if let Some(ref mut mapset) = score.mapset {
        beatmapset_assets(mapset).await
    }
    if let Some(ref mut user) = score.user {
        user_assets(user).await
    }
}

pub async fn team_assets(team: &mut Team) {
    if let Some(ref mut url) = team.flag_url {
        if let Ok(path) = cache_api_assets(&url).await {
            *url = path
        }
    }
}

pub async fn user_avatar(url: &mut String) {
    if let Ok(path) = cache_api_assets(url).await {
        *url = path
    }
}

pub async fn user_assets(user: &mut User) {
    // avatar
    user_avatar(&mut user.avatar_url).await;
    // cover
    if let Some(ref mut cover) = user.cover {
        user_cover(cover).await;
    }
    // team
    if let Some(ref mut team) = user.team {
        team_assets(team).await
    }
    // badges
    if let Some(ref mut badges) = user.badges {
        for badge in badges {
            badge_assets(badge).await
        }
    }
}

pub async fn user_cover(cover: &mut UserCover) {
    {
        let url = &mut cover.url;
        if let Ok(path) = cache_api_assets(&url).await {
            *url = path
        }
    }
    // custom cover (if have)
    if let Some(ref mut url) = cover.custom_url {
        if let Ok(path) = cache_api_assets(&url).await {
            *url = path
        }
    }
}

pub async fn user_extended_assets(user: &mut UserExtended) {
    // avatar
    user_avatar(&mut user.avatar_url).await;
    // cover
    user_cover(&mut user.cover).await;
    // team
    if let Some(ref mut team) = user.team {
        team_assets(team).await
    }
    // badges
    if let Some(ref mut badges) = user.badges {
        for badge in badges {
            badge_assets(badge).await
        }
    }
}
