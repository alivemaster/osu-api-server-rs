use warp::{Filter, Rejection, Reply};

// i cant understand the return type here
pub fn get_routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let index = warp::path::end().map(|| "osu-api-server-rs is running!");
    let beatmap = warp::path!("beatmap").map(|| "Returns a beatmap data json");
    let score = warp::path!("score").map(|| "Returns a score data json");
    let user = warp::path!("user").map(|| "Returns an user data json");

    index.or(warp::get().and(
        beatmap
            .or(score)
            .or(user),
    ))
}
