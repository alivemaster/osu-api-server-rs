use rosu_v2::prelude::*;

async fn client() -> Osu {
    let client_id: u64 = 123456;
    let client_secret = String::from("asdfghjkl");
    Osu::new(client_id, client_secret)
        .await
        .unwrap()
}

pub async fn beatmap_test(map_id: u32) -> BeatmapExtended {
    let osu = client().await;
    osu.beatmap()
        .map_id(map_id)
        .await
        .unwrap()
}

pub async fn beatmap_attrs_test(game_mode: u8, map_id: u32) -> BeatmapDifficultyAttributes {
    let osu = client().await;
    osu.beatmap_difficulty_attributes(map_id)
        .mode(GameMode::from(game_mode))
        .await
        .unwrap()
}

pub async fn score_test(score_id: u64) -> Score {
    let osu = client().await;
    osu.score(score_id)
        .await
        .unwrap()
}

pub async fn score_with_mode_test(game_mode: u8, score_id: u64) -> Score {
    let osu = client().await;
    osu.score(score_id)
        .mode(GameMode::from(game_mode))
        .await
        .unwrap()
}

pub async fn user_test(user_id: u32) -> UserExtended {
    let osu = client().await;
    osu.user(user_id)
        .await
        .unwrap()
}
