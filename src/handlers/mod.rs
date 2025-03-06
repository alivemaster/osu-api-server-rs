use axum::{
    Json,
    extract::{Path, Query, State},
};
use rosu_v2::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

mod beatmap;
pub use beatmap::handler as beatmap_handler;

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
