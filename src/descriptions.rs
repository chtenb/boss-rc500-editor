use phf::phf_map;

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
    // "Tempo" => "", //TODO
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
    //"Level" => "", //TODO
    // "Reverb" => "", //TODO
    // "Pattern" => "", //TODO
    // "Variation" => "", //TODO
    // "VariationChange" => "", //TODO
    // "Kit" => "", //TODO
    // "Beat" => "", //TODO
    // "Fill" => "", //TODO
    // "Part1" => "", //TODO
    // "Part2" => "", //TODO
    // "Part3" => "", //TODO
    // "Part4" => "", //TODO
    "RecCount" => "Record Count",
    "PlayCount" => "Play Count",
    // "Start" => "", //TODO
    // "Stop" => "", //TODO
    "ToneLow" => "Tone Low",
    "ToneHigh" => "Tone High",
    "State" => "Current State",

    // CTL
    "Pedal1" => "Specify the functions of the [REC/PLAY] switch (PDL1)",
    "Pedal2" => "Specify the functions of the [STOP] switch (PDL2)",
    "Pedal3" => "Specify the functions of the [TRACK SELECT] switch (PDL3)",
    "Ctl1" => "Specify the functions of the footswitch connected to the CTL 1, 2/EXP jack",
    "Ctl2" => "Specify the functions of the footswitch connected to the CTL 1, 2/EXP jack",
    "Exp" => "Specifies the function of a expression pedal connected to the CTL 1, 2/EXP jack.",

    // ASSIGN
    // "Sw" => "",
    "Source" => "Specify the controller (source) that will control the target.",
    "SourceMode" => "\
If a momentary-type footswitch (such as the separately sold FS-5U) is connected as the source, you can specify how footswitch operations will affect the value.",
    "Target" => "\
Specifies the function that is controlled. Depending on the specified function, control might not be possible while playing a track or the rhythm.",
    "TargetMin" => "\
Specifies the variable range of the function (parameter) that is specified as the target.
The value (MIN: minimum value, MAX: maximum value) depends on the parameter that is specified as the target. 
",
    "TargetMax" => "\
Specifies the variable range of the function (parameter) that is specified as the target.
The value (MIN: minimum value, MAX: maximum value) depends on the parameter that is specified as the target. 
",
};

pub static DESCRIPTIONS_BY_VALUE: phf::Map<&'static str, &'static [&'static str]> = phf_map! {
    // CTL
    "Pedal1" => PEDAL_VALUE_DESCRIPTIONS,
    "Pedal2" => PEDAL_VALUE_DESCRIPTIONS,
    "Pedal3" => PEDAL_VALUE_DESCRIPTIONS,
    "Ctl1" => PEDAL_VALUE_DESCRIPTIONS,
    "Ctl2" => PEDAL_VALUE_DESCRIPTIONS,
    "Exp" => &[
        "No function is assigned.",
        "Control the 'LEVEL' of Track 1 in the range of 0-200.",
        "Control the level in the range of 0-'maximum value', with the 'LEVEL' setting of Track 1 as the maximum value.",
        "Control the 'LEVEL' of Track 2 in the range of 0-200.",
        "Control the level in the range of 0-'maximum value', with the 'LEVEL' setting of Track 2 as the maximum value.",
        "Control the 'LEVEL' of the currently selected Track in the range of 0-200.",
        "Control the level of the currently selected Track in the range of 0-'maximum value', with the 'LEVEL' setting of the currently selected Track as the maximum value.",
        "Press the pedal to make the tempo faster.",
        "Press the pedal to make the tempo slower.",
        "Control a parameter according to the loop FX type.",
        "Control the 'LEVEL' of rhythm in the range of 0-200.",
        "Control the level in the range of 0-'maximum value', with the 'LEVEL' setting of rhythm as the maximum value.",
        "Control the 'LEVEL' of memory in the range of 0-200.",
        "Control the level in the range of 0-'maximum value', with the 'LEVEL' setting of memory as the maximum value.",
    ],

    // ASSIGN
    // "Sw" => "",
    "Source" => &[
        "[REC/PLAY] switch",
        "[STOP] switch",
        "[TRACK SELECT] switch",
        "An Expression pedal connected to the CTL 1, 2/EXP jack",
        "A footswitch (CTL1, CTL2) connected to the CTL 1, 2/EXP jack",
        "A footswitch (CTL1, CTL2) connected to the CTL 1, 2/EXP jack",
        "[TRACK 1] slider",
        "[TRACK 2] slider",
        "When track 1 playback starts or stops",
        "When track 2 playback starts or stops",
        "When the track for record/playback is switched",
        "All Start message from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",

        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
        "Control Change message (1-31, 64-95) from an external MIDI device",
     ],
    "SourceMode" => &[
        "The setting will normally be off (minimum value); it will be on (maximum value) only while you continue holding down the footswitch.",
        "The setting will alternate between off (minimum value) and on (maximum value) each time you press the footswitch."
    ],
    "Target" => ASSIGN_TARGET_VALUE_DESCRIPTIONS
};

static PEDAL_VALUE_DESCRIPTIONS : &'static [&'static str] = &[
"No function is assigned.",
"Switch between record/play/overdubbing for track 1.  Long press (two seconds or longer) the switch during playback or overdubbing to Undo, long press the switch once again to Redo.",
"Switch between record/play/stop (press the switch twice) for track 1.  Long press (two seconds or longer) the switch during recording or playback to Undo, long press the switch once again to Redo.",
"Switch between record/play/stop (press the switch twice) for track 1.  Long press (two seconds or longer) the switch during recording or playback to Undo, long press the switch once again to Redo.  Long press (two seconds or longer) the switch during stopped, the track is cleared.",
"Put track 1 in record/play only while you hold down the switch.",
"Switch between play/stop for track 1.",
"Switch between play/stop for track 1.  Long press (two seconds or longer) the switch during recording or playback to Undo, long press the switch once again to Redo.  Long press (two seconds or longer) the switch during stopped, the track is cleared.",
"Stop record/play for track 1.",
"Stop record/play for track 1.  Specify the tempo (tap tempo) by pressing the switch several times at the desired interval while stopped.",
"Stop record/play for track 1.  Long press (two seconds or longer) the switch during stopped, the track is cleared.",
"Stop record/play for track 1.  Specify the tempo (tap tempo) by pressing the switch several times at the desired interval while stopped.  Long press (two seconds or longer) the switch during stopped, the track is cleared.",
"Clear track 1.",
"Undo/redo recording or the most recent overdubbing for track 1.",
"Turn reverse play on/off for track 1.",
"Switch between record/play/overdubbing for track 2.  Long press (two seconds or longer) the switch during playback or overdubbing to Undo, long press the switch once again to Redo.",
"Switch between record/play/stop (press the switch twice) for track 2.  Long press (two seconds or longer) the switch during recording or playback to Undo, long press the switch once again to Redo.",
"Switch between record/play/stop (press the switch twice) for track 2.  Long press (two seconds or longer) the switch during recording or playback to Undo, long press the switch once again to Redo.  Long press (two seconds or longer) the switch during stopped, the track is cleared.",
"Put track 2 in record/play only while you hold down the switch.",
"Switch between play/stop for track 2.",
"Switch between play/stop for track 2.  Long press (two seconds or longer) the switch during recording or playback to Undo, long press the switch once again to Redo.  Long press (two seconds or longer) the switch during stopped, the track is cleared.",
"Stop record/play for track 2.",
"Stop record/play for track 2.  Specify the tempo (tap tempo) by pressing the switch several times at the desired interval while stopped.",
"Stop record/play for track 2.  Long press (two seconds or longer) the switch during stopped, the track is cleared.",
"Stop record/play for track 2.  Specify the tempo (tap tempo) by pressing the switch several times at the desired interval while stopped.  Long press (two seconds or longer) the switch during stopped, the track is cleared.",
"Clear track 2.",
"Undo/redo recording or the most recent overdubbing for track 2.",
"Turn reverse play on/off for track 2.",
"Switch between tracks 1 and 2. Long-press the switch to select both tracks.  If both tracks are selected, operating the switch assigned to CUR REC/PLY lets you control play/stop for both tracks.",
"Switch between record/play/overdubbing for current track.  Long press (two seconds or longer) the switch during playback or overdubbing to Undo, long press the switch once again to Redo.",
"Switch between record/play/stop (press the switch twice) for current track.  Long press (two seconds or longer) the switch during recording or playback to Undo, long press the switch once again to Redo.",
"Switch between record/play/stop (press the switch twice) for current track.  Long press (two seconds or longer) the switch during recording or playback to Undo, long press the switch once again to Redo.  Long press (two seconds or longer) the switch during stopped, the track is cleared.",
"Put current track in record/play only while you hold down the switch.",
"Switch between play/stop for current track.",
"Switch between play/stop for current track.  Long press (two seconds or longer) the switch during recording or playback to Undo, long press the switch once again to Redo.  Long press (two seconds or longer) the switch during stopped, the track is cleared.",
"Stop record/play for current track.",
"Stop record/play for current track.  Specify the tempo (tap tempo) by pressing the switch several times at the desired interval while stopped.",
"Stop record/play for current track.  Long press (two seconds or longer) the switch during stopped, the track is cleared.",
"Stop record/play for current track.  Specify the tempo (tap tempo) by pressing the switch several times at the desired interval while stopped.  Long press (two seconds or longer) the switch during stopped, the track is cleared.",
"Clear current track.",
"Undo/redo recording or the most recent overdubbing for current track.",
"Turn reverse play on/off for current track.",
"Simultaneously start play for tracks 1 and 2.",
"Press the switch several times at the desired interval to specify the tempo.",
"Long-press the switch (two seconds or longer) to return to the previous tempo.",
"Turn the loop FX on/off.",
"Turn the loop FX on/off for track 1.",
"Turn the loop FX on/off for track 2.",
"Turn the loop FX on/off for the currently selected track.",
"Switch the loop FX in the order of SCATTER 1-4 -> REPEAT 1-3 -> SHIFT 1-2 -> VINYL FLICK -> SCATTER1...",
"Switch the loop FX in the order of VINYL FLICK -> SHIFT 2-1 -> REPEAT 3-1 -> SCATTER 4-1 -> VINYL FLICK...",
"Switch the rhythm between play/stop.",
"Play the rhythm.",
"Stop playing the rhythm.",
"Switch to the next memory.",
"Switch to the previous memory.",
"Mute the audio from the mic.",
"Switch the 'EXTENT' (p. 13) setting in the order of EXT10EXT2...EXT50...",
"Switch the 'EXTENT' (p. 13) setting in the order of EXT50EXT4...EXT10...",
];

static ASSIGN_TARGET_VALUE_DESCRIPTIONS : &'static [&'static str] = &[
"Switch between record/play/overdubbing for Track 1.",
"Switch between play/stop for Track 1.",
"Clear Track 1.",
"Undo/redo recording or the most recent overdubbing for Track 1.",
"Control 'REVERSE' for Track 1.",
"Control '1SHOT' for Track 1.",
"Control the 'LEVEL' (p. 2) of Track 1 in the range of 0-200.",
"Control the level in the range of 0-'maximum value,' with the 'LEVEL' setting of Track 1 as the maximum value.",
"Control 'PAN' for Track 1.",
"Control 'START' for Track 1.",
"Control 'STOP' for Track 1.",
"Control 'LOOP.S' for Track 1.",
"Control 'TEMPO.S' for Track 1.",
"Control 'INPUT' for Track 1.",
"Control 'OUTPUT' for Track 1.",
"Switch between record/play/overdubbing for Track 2.",
"Switch between play/stop for Track 2.",
"Clear Track 2.",
"Undo/redo recording or the most recent overdubbing for Track 2.",
"Control 'REVERSE' for Track 2.",
"Control '1SHOT' for Track 2.",
"Control the 'LEVEL' (p. 2) of Track 2 in the range of 0-200.",
"Control the level in the range of 0-'maximum value,' with the 'LEVEL' setting of Track 2 as the maximum value.",
"Control 'PAN' for Track 2.",
"Control 'START' for Track 2.",
"Control 'STOP' for Track 2.",
"Control 'LOOP.S' for Track 2.",
"Control 'TEMPO.S' for Track 2.",
"Control 'INPUT' for Track 2.",
"Control 'OUTPUT' for Track 2.",
"Switch between tracks 1 and 2.",
"Switch between record/play/overdubbing for the currently selected track.",
"Clear the currently selected track.",
"Undo/redo recording or the most recent overdubbing for the currently selected track.",
"Control 'REVERSE' for the currently selected track.",
"Control '1SHOT' for the currently selected track.",
"Control the 'LEVEL' of the currently selected track in the range of 0-200.",
"Control the level in the range of 0-'maximum value,' with the 'LEVEL' setting of track 1 or 2 as the maximum value.",
"Control 'PAN' for the currently selected track.",
"Control 'START' for the currently selected track.",
"Control 'STOP' for the currently selected track.",
"Control 'LOOP.S' for the currently selected track.",
"Control 'TEMPO.S' for the currently selected track.",
"Control 'INPUT' for the currently selected track.",
"Control 'OUTPUT' for the currently selected track.",

"Allows you to undo/redo the last recording or overdubbing of a track.",
"Simultaneously start play for tracks 1 and 2.",
"Press the switch several times at the desired interval to specify the tempo.",
"Operate the pedal to make the tempo faster.",
"Operate the pedal to make the tempo slower.",
"Control the tempo.",
"Control 'DUB MODE' of memory/REC.",
"Control 'REC ACTION' of memory/REC.",
"Control 'QUANTIZE' of memory/REC.",
"Control 'AUTO REC' of memory/REC.",
"Control 'A.REC SENS' of memory/REC.",
"Control 'A.REC SRC' of memory/REC.",
"Control 'LOOP LENGTH' of memory/REC.",
"Control 'PLAY MODE' of memory/REC.",
"Control 'SINGL CHNGE' of memory/REC.",
"Control 'FADE TIME' of memory/REC.",
"Control 'ALL START' of memory/REC.",
"Control 'TRK CHAIN' of memory/REC.",
"Turn the loop FX on/off.",
"Turn the loop FX on/off for track 1.",
"Turn the loop FX on/off for track 2.",
"Turn the loop FX on/off for the currently selected track.",
"Control 'TYPE' of memory/LOOP FX.",
"Switch the loop FX in the order of SCATTER1-40REPEAT1-30SHIFT1-20VINYL FLICK0 SCATTER1...",
"Switch the loop FX in the order of VINYL FLICK0SHIFT2-10REPEAT3-10SCATTER4-10 VINYL FLICK...",
"Control a parameter according to the loop FX type.",
"Switch the rhythm between play/stop.",
"Play the rhythm.",
"Stop playing the rhythm.",
"Control the 'LEVEL' (p. 6) of rhythm in the range of 0-200.",
"Control the level in the range of 0-'maximum value,' with the 'LEVEL' setting of rhythm as the maximum value.",
"Control 'REVERB' of memory/RHYTHM.",
"Control 'PATTERN' of memory/RHYTHM.",
"Control 'VARIATION' of memory/RHYTHM.",
"Control 'VAR.CHANGE' of memory/RHYTHM.",
"Control 'KIT' of memory/RHYTHM.",
"Control 'START' of memory/RHYTHM.",
"Control 'STOP' of memory/RHYTHM.",
"Control 'REC COUNT' of memory/RHYTHM.",
"Control 'PLAY COUNT' of memory/RHYTHM.",
"Control 'FILL' of memory/RHYTHM.",
"Control 'PART1'-'PART4' of memory/RHYTHM.",
"Control 'TONE LOW' of memory/RHYTHM.",
"Control 'TONE HIGH' of memory/RHYTHM.",
"Switch to the next memory.",
"Switch to the previous memory.",
"Control the 'LEVEL' (p. 5) of memory in the range of 0-200.",
"Control the level in the range of 0-'maximum value,' with the 'LEVEL' setting of memory as the maximum value.",

"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",

"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
"Transmit a control change message of the specified controller number from the MIDI OUT connector.",
];
