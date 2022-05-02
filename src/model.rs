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

pub fn is_memory_empty(memory: &Memory) -> bool {
    let track1 = memory.menus.iter().filter(|menu| menu.name == "TRACK1").next().unwrap();
    if let MenuContent::KeyValueMenu(menu) = &track1.content {
        let wavstat = menu
            .settings
            .iter()
            .filter(|setting| setting.key == "WavStat")
            .next()
            .unwrap()
            .value;
        if wavstat == 1 {
            return false;
        }
    }
    return true;
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
    "RecAction" => "Record Action",
    "RecQuantize" => "Record Quantize",
    "AutoRec" => "Auto Record",
    "AutoRecSens" => "Auto Record Sensitivity",
    "AutoRecSrc" => "Auto Record Source",
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
    "Sw" => "Enabled",
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
    "RecCount" => "Record Count",
    "PlayCount" => "Play Count",
    // "Start" => "",
    // "Stop" => "",
    "ToneLow" => "Tone Low",
    "ToneHigh" => "Tone High",
    "State" => "Current State",

    // CTL
    "Pedal1" => "Pedal 1",
    "Pedal2" => "Pedal 2",
    "Pedal3" => "Pedal 3",
    "Ctl1" => "Control 1",
    "Ctl2" => "Control 2",
    "Exp" => "Expression Pedal",

    // ASSIGN
    // "Sw" => "",
    // "Source" => "",
    "SourceMode" => "Source Mode",
    // "Target" => "",
    "TargetMin" => "Target Min",
    "TargetMax" => "Target Max",
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
    "LpLen" => 25362,
    // "TrkMod" => "", // TODO
    // "Sync" => "", // TODO

    // LOOP FX
    "Sw" => 1,
    "FxType" => 10,
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

    // CTL
    "Pedal1" => 58,
    "Pedal2" => 58,
    "Pedal3" => 58,
    "Ctl1" => 58,
    "Ctl2" => 58,
    "Exp" => 13,

    // ASSIGN
    // "Sw" => "",
    "Source" => 12 + 31 + (95-64),
    "SourceMode" => 1,
    "Target" => 77 + 31 + (95-64),
    // "TargetMin" => 0, // TODO
    // "TargetMax" => 0, // TODO
};

pub static DISPLAY_VALUES: phf::Map<&'static str, &'static [&'static str]> = phf_map! {
    // TRACK
    "Rev" => &["Off", "On"],
    // "PlyLvl" => 200,
    // "Pan" => 100,
    "One" => &["Off", "On"],
    "LoopFx" => &["Off", "On"],
    "StrtMod" => &["Immediate", "Fade In"],
    "StpMod" => &["Immediate", "Fade Out", "Loop End"],
    // "Measure" => , // TODO
    "LoopSync" => &["Off", "On"],
    "TempoSync" => &["Off", "On"],
    "Input" => &["All", "MIC IN", "INST IN", "INST IN A", "INST IN B", "MIC/INST"],
    "Output" => &["All", "OUT A", "OUT B"],
    // "MeasMod" => "", // TODO
    // "MeasLen" => "", // TODO
    // "MeasBtLp" => "", // TODO
    // "RecTmp" => "", // TODO
    // "WavStat" => "", // TODO
    // "WavLen" => "", // TODO

    // MASTER
    // "Tempo" => "", // TODO
    "DubMode" => &["Overdub", "Replace"],
    "RecAction" => &["Record -> Dub", "Record -> Play"],
    "RecQuantize" => &["Off", "Measure"],
    "AutoRec" => &["Off", "On"],
    // "AutoRecSens" => 100,
    "AutoRecSrc" => &["All", "MIC IN", "INST", "INST A", "INST B"],
    "PlayMode" => &["Multi", "Single"],
    "SinglPlayeChange" => &["Immediate", "Loop End"],
    // "FadeTime" => &["16th", "8th", "4th", "2nd", "1 Measure", ...], // TODO
    "AllStart" => &["All", "Track 1", "Track 2"],
    "TrackChain" => &["Parallel", "Series"],
    "CurrentTrack" => &["Track 1", "Track 2"],
    "AllTrackSel" => &["Yes", "No"],
    // "Level" => 200,
    // "LpMod" => "", // TODO
    // "LpLen" => "Loop Length", // TODO
    // "TrkMod" => "", // TODO
    // "Sync" => "", // TODO

    // LOOP FX
    "Sw" => &["Off", "On"],
    // "FxType" => 10, // TODO
    // "RepeatLength" => 6, // TODO
    // "ShiftShift" => 6, // TODO
    // "ScatterLength" => 6, // TODO
    // "VinylFlickFlick" => 100,

    // RHYTHM
    // "Level" => 200,
    // "Reverb" => 100,
    // "Pattern" => "", // TODO
    "Variation" => &["A", "B"],
    "VariationChange" => &["Measure", "Loop End"],
    "Kit" => &["Studio", "Live", "Light", "Heavy", "Rock", "Metal", "Jazz", "Brush", "Cajon", "Drum&Bass", "R&B", "Dance", "Techno", "Dance Beats", "HipHop",
    "808+909"],
    // "Beat" => "", // TODO
    "Fill" => &["Off", "On"],
    "Part1" => &["Off", "On"],
    "Part2" => &["Off", "On"],
    "Part3" => &["Off", "On"],
    "Part4" => &["Off", "On"],
    "RecCount" => &["Off", "1 Measure"],
    "PlayCount" => &["Off", "1 Measure"],
    "Start" => &["Loop Start", "Record End", "Before Loop"],
    "Stop" => &["Never", "Loop Stop", "Record End"],
    // "ToneLow" => 20, // TODO
    // "ToneHigh" => 20, // TODO
    // "State" => 2,// TODO

    // CTL
    "Pedal1" => PEDAL_CTL_VALUES,
    "Pedal2" => PEDAL_CTL_VALUES,
    "Pedal3" => PEDAL_CTL_VALUES,
    "Ctl1" => PEDAL_CTL_VALUES,
    "Ctl2" => PEDAL_CTL_VALUES,
    "Exp" => &[ "Off", "T1 Level1", "T1 Level2", "T2 Level1", "T2 Level2", "Current Level1", "Current Level2",
    "Tempo Up", "Tempo Down", "FX Control", "Rhythm Level1", "Rhythm Level2", "Memory Level1", "Memory Level2",
    ],

    // ASSIGN
    // "Sw" => "",
    "Source" => &[
    "Pedal 1", "Pedal 2", "Pedal 3", "Expression Pedal", "CTL1 Pedal", "CTL2 Pedal", "TR1 Knob", "TR2 Knob",
    "TR1 Play/Stop", "TR2 Play/Stop", "Current Track Change", "Sync Start",
    "CC 1", "CC 2", "CC 3", "CC 4", "CC 5", "CC 6", "CC 7", "CC 8", "CC 9", "CC 10", "CC 11", "CC 12", "CC 13", "CC 14", "CC 15", "CC 16", "CC 17", "CC 18", "CC 19", "CC 20", "CC 21", "CC 22", "CC 23", "CC 24", "CC 25", "CC 26", "CC 27", "CC 28", "CC 29", "CC 30", "CC 31", "CC 64", "CC 65", "CC 66", "CC 67", "CC 68", "CC 69", "CC 70", "CC 71", "CC 72", "CC 73", "CC 74", "CC 75", "CC 76", "CC 77", "CC 78", "CC 79", "CC 80", "CC 81", "CC 82", "CC 83", "CC 84", "CC 85", "CC 86", "CC 87", "CC 88", "CC 89", "CC 90", "CC 91", "CC 92", "CC 93", "CC 94", "CC 95",
    ],
    "SourceMode" => &["Moment", "Toggle"],
    "Target" =>
    &[
        "T1 Record/Play", "T1 Play/Stop", "T1 Clear", "T1 Undo/Redo", "T1 Reverse", "T1 1Shot", "T1 Level1", "T1 Level2",
        "T1 Pan", "T1 Start", "T1 Stop", "T1 Loop Sync", "T1 Tempo Sync", "T1 Input", "T1 Output",

        "T2 Record/Play", "T2 Play/Stop", "T2 Clear", "T2 Undo/Redo", "T2 Reverse", "T2 1Shot", "T2 Level1", "T2 Level2",
        "T2 Pan", "T2 Start", "T2 Stop", "T2 Loop Sync", "T2 Tempo Sync", "T2 Input", "T2 Output",

        "TRK SELECT",
        "Current Record/Play", "Current Play/Stop", "Current Clear", "Current Undo/Redo", "Current Reverse", "Current 1Shot", "Current Level1", "Current Level2",
        "Current Pan", "Current Start", "Current Stop", "Current Loop Sync", "Current Tempo Sync", "Current Input", "Current Output",

        "Undo/Redo", "All Start", "Tap Tempo", "Tempo Up", "Tempo Down", "Tempo", "Dub Mode", "Record Action",
        "Auto Record", "Auto Record Sensitivity", "Auto Rec Source", "Loop Length", "Play Mode", "Single Change", "Fade Time",
        "All Start Track", "Track Chain", "Loop FX", "TR1 FX", "TR2 FX", "Current Track FX", "FX Type", "FX Inc", "FX Dec",
        "FX Control",
        "Rhythm Play/Stop", "Rhythm Play", "Rhythm Stop", "Rhythm Level1", "Rhythm Level2", "Rhythm Reverb",
        "Rhythm Pattern", "Variation", "Variation Change", "Kit", "Rhythm Start", "Rhythm Stop", "Record Count",
        "Play Count", "Rhythm Fill",
        "Rhythm Part1", "Rhythm Part2", "Rhythm Part3", "Rhythm Part4",
        "Tone Low", "Tone High",
        "Memory Inc", "Memory Dec", "Memory Level1", "Memory Level2",
        "CC 1", "CC 2", "CC 3", "CC 4", "CC 5", "CC 6", "CC 7", "CC 8", "CC 9",
        "CC 10", "CC 11", "CC 12", "CC 13", "CC 14", "CC 15", "CC 16", "CC 17", "CC 18", "CC 19",
        "CC 20", "CC 21", "CC 22", "CC 23", "CC 24", "CC 25", "CC 26", "CC 27", "CC 28", "CC 29",
        "CC 30", "CC 31",

        "CC 64", "CC 65", "CC 66", "CC 67", "CC 68", "CC 69",
        "CC 70", "CC 71", "CC 72", "CC 73", "CC 74", "CC 75", "CC 76", "CC 77", "CC 78", "CC 79",
        "CC 80", "CC 81", "CC 82", "CC 83", "CC 84", "CC 85", "CC 86", "CC 87", "CC 88", "CC 89",
        "CC 90", "CC 91", "CC 92", "CC 93", "CC 94", "CC 95"
    ],
    // "TargetMin" => 0, // TODO
    // "TargetMax" => 0, // TODO
};

pub static PEDAL_CTL_VALUES: &'static [&'static str] = &[
    "Off",
    "T1 Rec/Play",
    "T1 Record/Play/Stop",
    "T1 Record/Play/Stop(Clear)",
    "T1 Moment Record/Play",
    "T1 Play/Stop",
    "T1 Play/Stop(Clear)",
    "T1 Stop",
    "T1 Stop(Tempo Tap)",
    "T1 Stop(Clear)",
    "T1 Stop(Tempo Tab/Clear)",
    "T1 Clear",
    "T1 Undo/Redo",
    "T1 Reverse",
    "T2 Rec/Play",
    "T2 Record/Play/Stop",
    "T2 Record/Play/Stop(Clear)",
    "T2 Moment Record/Play",
    "T2 Play/Stop",
    "T2 Play/Stop(Clear)",
    "T2 Stop",
    "T2 Stop(Tempo Tap)",
    "T2 Stop(Clear)",
    "T2 Stop(Tempo Tab/Clear)",
    "T2 Clear",
    "T2 Undo/Redo",
    "T2 Reverse",
    "Track Select",
    "Current Rec/Play",
    "Current Record/Play/Stop",
    "Current Record/Play/Stop(Clear)",
    "Current Moment Record/Play",
    "Current Play/Stop",
    "Current Play/Stop(Clear)",
    "Current Stop",
    "Current Stop(Tempo Tap)",
    "Current Stop(Clear)",
    "Current Stop(Tempo Tab/Clear)",
    "Current Clear",
    "Current Undo/Redo",
    "Current Reverse",
    "Undo/Redo",
    "All Start",
    "Tap Tempo",
    "Loop FX",
    "Track1 FX",
    "Track2 FX",
    "Current Track FX",
    "FX Inc",
    "FX Dec",
    "Rhythm Play/Stop",
    "Rhythm Play",
    "Rhythm Stop",
    "Memory Inc",
    "Memory Dec",
    "Mic Mute",
    "Extent Inc",
    "Extent Dec",
];
