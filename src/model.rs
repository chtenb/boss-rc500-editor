use phf::phf_map;

#[derive(Clone, Debug)]
pub struct Config {
    pub filename: String,
    pub suffix: Vec<u8>,
    pub memories: Vec<Memory>,
}

#[derive(Clone, Debug)]
pub struct Memory {
    pub id: usize,
    pub menus: Vec<Menu>,
}

pub fn get_memory_name(memory: &Memory) -> &str {
    match &memory.menus[0].content {
        MenuContent::StringValueMenu(menu) => menu.value.as_ref(),
        _ => "",
    }
}

#[derive(Clone, Debug)]
pub struct UntypedMenu {
    pub settings: Vec<UntypedKeyValue>,
}

#[derive(Clone, Debug)]
pub struct StringValueMenu {
    pub value: String,
}

#[derive(Clone, Debug)]
pub struct Menu {
    pub name: String,
    pub content: MenuContent,
}

#[derive(Clone, Debug)]
pub enum MenuContent {
    StringValueMenu(StringValueMenu),
    KeyValueMenu(UntypedMenu),
}

#[derive(Clone, Debug)]
pub struct UntypedKeyValue {
    pub key: String,
    pub value: usize,
}

pub static STRING_MAX_WIDTH: usize = 12;

pub static DISPLAY_KEYS: phf::Map<&'static str, &'static str> = phf_map! {
    // TRACK
    "Rev" => "Reverse",
    "PlyLvl" => "Playback Level",
    // "Pan" => "",
    "One" => "1Shot",
    "LoopFx" => "Loop FX",
    "StrtMod" => "Start",
    "StpMod" => "Stop",
    "Measure" => "Measure",
    "LoopSync" => "Loop Sync",
    "TempoSync" => "Tempo Sync",
    // "Input" => "",
    // "Output" => "",
    // "MeasMod" => "", // TODO
    // "MeasLen" => "", // TODO
    // "MeasBtLp" => "", // TODO
    // "RecTmp" => "", // TODO
    // "WavStat" => "", // TODO
    // "WavLen" => "", // TODO

    // MASTER
    // "Tempo" => "",
    "DubMode" => "Dub Mode",
    "RecAction" => "Rec Action",
    "RecQuantize" => "Rec Quantize",
    "AutoRec" => "Auto Rec",
    "AutoRecSens" => "Auto Rec Sensitivity",
    "AutoRecSrc" => "Auto Rec Source",
    "PlayMode" => "Play Mode",
    "SinglPlayeChange" => "Single Change",
    "FadeTime" => "Fade Time",
    "AllStart" => "All Start",
    "TrackChain" => "Track Chain",
    "CurrentTrack" => "Current Track",
    "AllTrackSel" => "All Tracks Selected",
    // "Level" => "",
    // "LpMod" => "", // TODO
    "LpLen" => "Loop Length",
    // "TrkMod" => "", // TODO
    // "Sync" => "", // TODO

    // LOOP FX
    "Sw" => "Loop FX",
    "FxType" => "FX Type",
    "RepeatLength" => "Repeat Length",
    "ShiftShift" => "Shift",
    "ScatterLength" => "Scatter Length",
    "VinylFlickFlick" => "Vinyl Flick",

    // RHYTHM
    //"Level" => "",
    // "Reverb" => "",
    // "Pattern" => "",
    // "Variation" => "",
    // "VariationChange" => "",
    // "Kit" => "",
    // "Beat" => "",
    // "Fill" => "",
    // "Part1" => "",
    // "Part2" => "",
    // "Part3" => "",
    // "Part4" => "",
    "RecCount" => "Rec Count",
    "PlayCount" => "Play Count",
    // "Start" => "",
    // "Stop" => "",
    "ToneLow" => "Tone Low",
    "ToneHigh" => "Tone High",
    "State" => "Current State",
};

pub static BOUNDS: phf::Map<&'static str, usize> = phf_map! {
    // TRACK
    "Rev" => 1,
    "PlyLvl" => 200,
    "Pan" => 100,
    "One" => 1,
    "LoopFx" => 1,
    "StrtMod" => 1,
    "StpMod" => 2,
    // "Measure" => , // TODO
    "LoopSync" => 1,
    "TempoSync" => 1,
    "Input" => 5,
    "Output" => 2,
    // "MeasMod" => "", // TODO
    // "MeasLen" => "", // TODO
    // "MeasBtLp" => "", // TODO
    // "RecTmp" => "", // TODO
    // "WavStat" => "", // TODO
    // "WavLen" => "", // TODO

    // MASTER
    // "Tempo" => "", // TODO
    "DubMode" => 1,
    "RecAction" => 1,
    "RecQuantize" => 1,
    "AutoRec" => 1,
    "AutoRecSens" => 100,
    "AutoRecSrc" => 4,
    "PlayMode" => 1,
    "SinglPlayeChange" => 1,
    "FadeTime" => 6,
    "AllStart" => 2,
    "TrackChain" => 1,
    "CurrentTrack" => 1,
    "AllTrackSel" => 1,
    "Level" => 200,
    // "LpMod" => "", // TODO
    // "LpLen" => "Loop Length", // TODO
    // "TrkMod" => "", // TODO
    // "Sync" => "", // TODO

    // LOOP FX
    "Sw" => 1,
    "FxType" => 3,
    "RepeatLength" => 6,
    "ShiftShift" => 6,
    "ScatterLength" => 6,
    "VinylFlickFlick" => 100,

    // RHYTHM
    // "Level" => 200,
    "Reverb" => 100,
    // "Pattern" => "", // TODO
    "Variation" => 1,
    "VariationChange" => 1,
    "Kit" => 16,
    // "Beat" => "", // TODO
    "Fill" => 1,
    "Part1" => 1,
    "Part2" => 1,
    "Part3" => 1,
    "Part4" => 1,
    "RecCount" => 1,
    "PlayCount" => 1,
    "Start" => 2,
    "Stop" => 2,
    "ToneLow" => 20, // TODO
    "ToneHigh" => 20, // TODO
    "State" => 2,
};

pub static DISPLAY_VALUES: phf::Map<&'static str, &'static [&'static str]> = phf_map! {
    "Rev" => &["OFF", "ON"],
    "LoopSync" => &["OFF", "ON"],
    "TempoSync" => &["OFF", "ON"],
    "One" => &["OFF", "ON"],
    "StrtMod" => &["IMMEDIATE", "FADE IN"],
};

pub static DESCRIPTIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "Rev" => "Specifies conventional playback (OFF) or reverse playback (ON). When REVERSE is set to 'ON' you won't be able to switch to overdubbing after a recording has been completed.",
    "PlyLvl" => "Adjusts the playback level of the tracks.",
    "Pan" => "Below 50 is left above 50 is right",
    "One" => "If 1SHOT is ON, playback will stop
    when it reaches the end of the
    phrase.",
    "LoopFx" => "",
    "StrtMod" => "Specifies whether playback starts with a fade-in or immediately when the track plays. You can use 'FADE TIME' (p. 5) to specify the length of the fade-in.",
    "StpMod" => "",
    "Measure" => "You can specify the number of measures for each track. When recording along with rhythm sounds or other tracks, it's convenient to specify the number of measures before you record, so that looping will occur at the specified measure length, even if you don't operate the switch when you've finished recording.
This parameter is available only if LOOP.S (p. 3) is 'ON.'

Tracks that are set to AUTO will have the same number of measures. The number of measures is determined by the first-recorded track of the tracks that are set to AUTO. For example, if all tracks are set to AUTO, the value set as the number of measures for the second and subsequent tracks will be identical to the number of measures in the first track that was recorded.

FREE: The number of measures will be set automatically, corresponding to the length of the recording.

1MEAS- The number of measures will be set manually. ",
    "LoopSync" => "",
    "TempoSync" => "",
};
