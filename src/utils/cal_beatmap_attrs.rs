use rosu_pp::model::{
    beatmap::BeatmapAttributesBuilder,
    mods::rosu_mods::{GameMode, GameMods, GameModsIntermode},
};
use rosu_v2::prelude::BeatmapExtended;

pub fn cal_beatmap_attrs(
    beatmap: &mut BeatmapExtended,
    mode: Option<u8>,
    mods: Option<GameModsIntermode>,
) {
    let mut attrs_builder = BeatmapAttributesBuilder::new();
    attrs_builder
        .ar(beatmap.ar, false)
        .cs(beatmap.cs, false)
        .od(beatmap.od, false)
        .hp(beatmap.hp, false);

    // game mode specified?
    if let Some(mode) = mode {
        let game_mode = GameMode::from(mode);
        if beatmap.mode != game_mode {
            // can't convert
            if beatmap.mode != GameMode::Osu {
                return;
            }

            // is a converted map
            beatmap.mode = game_mode;
            beatmap.convert = true;
        }
    }

    attrs_builder.mode((beatmap.mode as u8).into(), beatmap.convert);

    // has mods?
    if let Some(mods) = &mods {
        attrs_builder.mods(mods);

        // DT/NC or HT/DC
        if let Some(clock_rate) = GameMods::from_intermode(mods, beatmap.mode).clock_rate()
            && clock_rate != 1.0
        {
            beatmap.bpm = (beatmap.bpm as f64 * clock_rate) as f32;
            beatmap.seconds_drain = (beatmap.seconds_drain as f64 / clock_rate) as u32;
            beatmap.seconds_total = (beatmap.seconds_total as f64 / clock_rate) as u32;
        }
    }

    let attrs = attrs_builder.build();
    beatmap.ar = attrs.ar();
    beatmap.cs = attrs.cs();
    beatmap.od = attrs.od();
    beatmap.hp = attrs.hp();
}
