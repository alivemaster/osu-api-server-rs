# osu-api-server-rs

A simple API server based on [rosu-v2](https://github.com/MaxOhn/rosu-v2) with additional features. Developed for my personal usage.

## Currently available routes

`/beatmaps/{map_id}` gets a [BeatmapExtended](https://osu.ppy.sh/docs/index.html#beatmapextended)

`/beatmaps/{map_id}/attributes?[mode][mods]` gets a [BeatmapDifficultyAttributes](https://osu.ppy.sh/docs/index.html#beatmapdifficultyattributes)

`/beatmaps/{map_id}/with-attributes?[mode][mods]` gets a json including both [BeatmapExtended](https://osu.ppy.sh/docs/index.html#beatmapextended) and [BeatmapDifficultyAttributes](https://osu.ppy.sh/docs/index.html#beatmapdifficultyattributes)

`/beatmapsets/{mapset_id}` gets a [BeatmapSetExtended](https://osu.ppy.sh/docs/index.html#beatmapsetextended)

`/matches/{match_id}` gets an [OsuMatch](https://docs.rs/rosu-v2/latest/rosu_v2/model/matches/struct.OsuMatch.html)

`/scores/{score_id}` gets a [Score](https://osu.ppy.sh/docs/index.html#score)

`/scores/{mode}/{score_id}` gets a [Score](https://osu.ppy.sh/docs/index.html#score)

`/users/{user_id}` gets a [UserExtended](https://osu.ppy.sh/docs/index.html#userextended)

`/users/{user_id}/{mode}` gets a [UserExtended](https://osu.ppy.sh/docs/index.html#userextended)

## Additional features

- [x]  Cache image assets from osu! api
- [ ]  Extract original background images from beatmapset files(.osz)
- [ ]  parse replay file(.osr) from requests and reponse score json
