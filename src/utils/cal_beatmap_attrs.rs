use rosu_pp::model::beatmap::BeatmapAttributesBuilder;
use rosu_v2::prelude::{BeatmapExtended, GameModIntermode, GameModsIntermode};

pub fn cal_beatmap_attrs(
    beatmap: &mut BeatmapExtended,
    mode: Option<u8>,
    mods: Option<GameModsIntermode>,
) {
    let mut attrs_builder = BeatmapAttributesBuilder::new()
        .ar(beatmap.ar, false)
        .cs(beatmap.cs, false)
        .od(beatmap.od, false)
        .hp(beatmap.hp, false);

    // is a converted map?
    if let Some(mode) = mode {
        if beatmap.mode as u8 != mode {
            if beatmap.mode as u8 == 0 {
                // can convert
                attrs_builder = attrs_builder.mode(mode.into(), true);

                // is a converted map
                beatmap.convert = true;
                beatmap.mode = mode.into();
            } else {
                // can't convert
                return;
            }
        }
    } else {
        attrs_builder = attrs_builder.mode((beatmap.mode as u8).into(), false)
    };

    // has mods?
    if let Some(mods) = &mods {
        attrs_builder = attrs_builder.mods(mods);

        // bpm and length
        if mods.contains(GameModIntermode::DoubleTime) || mods.contains(GameModIntermode::Nightcore)
        {
            beatmap.bpm = beatmap.bpm * 1.5;
            beatmap.seconds_drain = (beatmap.seconds_drain as f64 / 1.5) as u32;
            beatmap.seconds_total = (beatmap.seconds_total as f64 / 1.5) as u32;
        }
        if mods.contains(GameModIntermode::HalfTime) || mods.contains(GameModIntermode::Daycore) {
            beatmap.bpm = beatmap.bpm * 0.75;
            beatmap.seconds_drain = (beatmap.seconds_drain as f64 / 0.75) as u32;
            beatmap.seconds_total = (beatmap.seconds_total as f64 / 0.75) as u32;
        }
    }

    let attrs = attrs_builder.build();
    beatmap.ar = attrs.ar as f32;
    beatmap.cs = attrs.cs as f32;
    beatmap.od = attrs.od as f32;
    beatmap.hp = attrs.hp as f32;
}
