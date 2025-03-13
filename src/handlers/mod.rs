use super::*;
use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::Deserialize;

mod beatmap;
pub use beatmap::handler as beatmap_handler;

mod beatmap_difficulty_attributes;
pub use beatmap_difficulty_attributes::handler as beatmap_difficulty_attributes_handler;

mod beatmapset;
pub use beatmapset::handler as beatmapset_handler;

mod osu_match;
pub use osu_match::handler as osu_match_handler;

mod score;
pub use score::handler as score_handler;

mod user;
pub use user::handler as user_handler;

mod osu_error;
pub use osu_error::OsuErrorResponse;

mod assets;
pub use assets::handler as assets_handler;
