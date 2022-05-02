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
    "Source" => 13,
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
    "Exp" => &[ "T1 Level1", "T1 Level2", "T2 Level1", "T2 Level2", "Current Level1", "CUR Level2",
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

pub static DESCRIPTIONS: phf::Map<&'static str, &'static str> = phf_map! {
    // TRACK
    "Rev" => "\
Specifies conventional playback (Off) or reverse playback (On). When REVERSE is set to 'On' you won't be able to switch to overdubbing after a recording has been completed.
",

    "PlyLvl" => "Adjusts the playback level of the tracks.",

    "Pan" => "Below 50 is left above 50 is right",

    "One" => "\
Specifies whether the track playback will be one-shot (On) or not one-shot (Off; conventional loop playback).

Track 1: Off   |---||---||---||---|
Track 2: On    |-----------|   <- 1SHOT is On, playback will stop when it reaches the end of the phrase.
",

    "LoopFx" => "Specifies whether to apply LOOP FX (On) or not (Off) for this particular track.",

    "StrtMod" => "Specifies whether playback starts with a fade-in or immediately when the track plays. You can use 'FADE TIME' (p. 5) to specify the length of the fade-in.",

    "StpMod" => "\
Specifies how the track will stop when you press the [STOP] switch.
- The PLAY indicator will blink until playback stops.
- If you press the [STOP] switch once again before playback stops, playback will stop immediately.
- You can't overdubbing during the time until playback stops.

Immediate: Playback will stop immediately.
Fade Out: Playback will fade out and then stop. You can use 'Fade Time' (p. 5) to specify the length of the fade-out.
Loop End: Playback will continue to the end of the loop, and then stop.
",

    "Measure" => "\
You can specify the number of measures for each track. When recording along with rhythm sounds or other tracks, it's convenient to specify the number of measures before you record, so that looping will occur at the specified measure length, even if you don't operate the switch when you've finished recording. 
This parameter is available only if LOOP.S (p. 3) is 'On'.

Track 1: One Measure   |---||---||---||---| |---||---||---||---|
Track 2: Four Measures |----|----|----|---| |----|----|----|---|

Auto: Tracks that are set to AUTO will have the same number of measures. The number of measures is determined by the first-recorded track of the tracks that are set to AUTO. For example, if all tracks are set to AUTO, the value set as the number of measures for the second and subsequent tracks will be identical to the number of measures in the first track that was recorded.

Free: The number of measures will be set automatically, corresponding to the length of the recording.

1 Measure - ... : The number of measures will be set manually. ",

    "LoopSync" => "\
Specifies whether the beginning of the track 1 and track 2 phrases are aligned for loop playback (On), or whether they loop-play at the length of their respective phrases (Off).

On:  Track 1   |------------||------------|
     Track 2   |----||----|  |----||----|
                             ^-> Retriggers at the beginning of the first recorded phrase.

Off: Track 1   |------------||------------|
     Track 2   |----||----||----||----|
                           ^-> Playback will repeate at the length of the phrase.


If you turn LOOP.S on, the beginning of the tracks will always be aligned. This means that if tracks are already playing back, the playback start location of the other tracks will become 'the current location of the phrase being played.' The following illustration is an example of how playback will occur with track 1 and 2 that contain an identical melody but have different loop sync settings

On:  Track 1   |-1-2-3-4-5-6-|
     Track 2         |-4-5-6-|    <- Playback starts midway through the phrase (from the current location).

Off:  Track 1   |-1-2-3-4-5-6-|
     Track 2         |-1-2-3-4-5-6-|    <- Playback always starts at the beginning of the phrase.

",

    "TempoSync" => "\
Off: The track will play at its own original tempo.
On: The track will play at the memory tempo.  Tracks 1 and 2 play at the same tempo.
",

    "Input" => "\
Specifies the input from which sound is recorded

ALL: Sound that is input from the MIC IN jack and the INST IN jacks (A/MONO, B) is mixed and
recorded.
MIC IN: Sound that is input from the MIC IN jack is recorded in stereo.
INST IN: Sound that is input to the INST IN jacks (A/MONO, B) is assigned and recorded to A and B (L, R).
INST IN-A: Sound that is input from the INST IN A/MONO jack is recorded in stereo.
INST IN-B: Sound that is input from the INST IN B jack is recorded in stereo.
MIC/INST: Sound that is input to the MIC IN jack and the INST IN jacks (A/MONO, B) is assigned and recorded to A and B (L, R).
",

    "Output" => "\
Specifies the output destination of the playback audio.

ALL: Audio is output from both OUTPUT A/MONO and B jacks.
OUT-A: Audio is output from the OUTPUT A/MONO jack.
OUT-B: Audio is output from the OUTPUT B jack.
",
    // "MeasMod" => "", // TODO
    // "MeasLen" => "", // TODO
    // "MeasBtLp" => "", // TODO
    // "RecTmp" => "", // TODO
    // "WavStat" => "", // TODO
    // "WavLen" => "", // TODO

    // MASTER
    // "Tempo" => "",
    "DubMode" => "\
Specifies the overdubbing method.

Overdub: The new performance is layered onto the prerecorded tracks.
If overdubbing is repeated, the next performance is layered on top of the previous material,
allowing you to create an ensemble in a single track.

Replace: Tracks with existing recordings are overwritten as new tracks are recorded over them.
Overwriting takes places while the previously recorded tracks are played back, allowing you
to achieve a kind of delay effect similar to that obtained from an effects processor.
",

    "RecAction" => "\
Specifies the order in which record/playback/overdubbing are switched when you press the [REC/PLAY] switch.

Record -> Dub: Operation will switch in the order of Recording -> Overdubbing -> Playback.
Record -> Play: Operation will switch in the order of Recording -> Playback -> Overdubbing.
",

    "RecQuantize" => "\
With tracks for which Loop Sync is 'On', under any of the conditions listed below, your timing will be corrected
(Loop Quantize) based on the tempo and time signature of the rhythm, even if the timing at which you press a button is
slightly inaccurate.
- If the rhythm is on
- If there is an already-recorded track whose Loop Sync is turned on
- If the MIDI Sync is on

The Record Quantize setting applies only during recording. It is ignored during overdubbing or playback.
When you stop recording, it is quantized to match the measure length of the previously-recorded track (Loop Sync: On) or
rhythm.

Off: Recording begins the instant you perform the operation. When you stop recording, it is quantized to match the measure.
Measure: Quantize to the measure start location for recording.
",

    "AutoRec" => "\
Starts recording when there is audio input from your guitar performance or the mic.
Off: Recording will begin the instant you press the [REC/PLAY] switch.
On: When you press the [REC/PLAY] switch, the REC/PLAY indicator will blink rapidly, and the RC-500 will enter recording-standby mode.
When you begin playing, the REC/PLAY indicator will light and recording will start.
",

    "AutoRecSens" => "Auto Record Sensitivity",

    "AutoRecSrc" => "Auto Record Source",

    "PlayMode" => "\
Specifies whether tracks 1 and 2 play back simultaneously.

Multi:   Track 1   |-------------|
         Track 2   |----------------------|

Play back all tracks. To ensure that playback always takes place from the beginning of the phrase, you need to
set Loop Sync to 'Off'.

Single:  Track 1   |-------------|   <- Stops when the other track starts playing
         Track 2                  |--------|

Play back only a single track. The currently-playing track stops when the other track starts playing. If Single Change is set to 'Loop End' the track will change at the end of the loop.
",

    "SinglPlayeChange" => "\
Specifies how the tracks will be switched when Play Mode is 'Single'.
Immediate: The change will occur immediately.
Loop End: The change will occur after playback has reached the end of the loop.
",

    "FadeTime" => "Fade Time",

    "AllStart" => "\
Normally, tracks 1 and 2 start playing simultaneously when a MIDI start message is received (the All setting). However, you can set this to make only the specified track (Track 1 or Track 2) start playing.
",

    "TrackChain" => "\
Specifies how tracks 1 and 2 are connected.
Parallel: The two loop tracks are connected in parallel.
Series: The output of track 1 is connected to track 2.
",

    "CurrentTrack" => "Current Track",

    "AllTrackSel" => "All Tracks Selected",

    // "Level" => "",
    // "LpMod" => "", // TODO

    "LpLen" => "\
Specifies the length to which Loop Sync aligns the beginnings of the material for loop sync.
Auto: The length of the first-recorded phrase will be the Loop Length.
1-25362: Manually specifies the number of measures that will be looped.    
",

    // "TrkMod" => "", // TODO
    // "Sync" => "", // TODO

    // LOOP FX
    "Sw" => "Turn the current system on or off",
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
};
