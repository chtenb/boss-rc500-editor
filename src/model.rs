use phf::phf_map;

#[derive(Clone, Debug)]
pub struct Config {
    pub memories: Vec<Memory>,
}

#[derive(Clone, Debug)]
pub struct Memory {
    pub id: usize, // This could be implicit
    pub menus: Vec<Menu>,
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

pub static DESCRIPTIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "Rev" => "T1 REVERSE",
    "PlyLvl" => "Loudness of the track",
    "Pan" => "Below 50 is left above 50 is right",
    "One" => "",
    "LoopFx" => "",
    "StrtMod" => "",
    "StpMod" => "",
    "Measure" => "",
    "LoopSync" => "",
    "TempoSync" => "",
};

pub static DISPLAY_KEYS: phf::Map<&'static str, &'static str> = phf_map! {
    "Rev" => "T1 Reverse",
    "PlyLvl" => "Playback Level",
    "Pan" => "Pan",
    "One" => "1Shot",
    "LoopFx" => "",
    "StrtMod" => "Start",
    "StpMod" => "Stop",
    "Measure" => "",
    "LoopSync" => "",
    "TempoSync" => "",
};

pub static BOUNDS: phf::Map<&'static str, i32> = phf_map! {
    "Rev" => 1,
    "PlyLvl" => 100,
    "Pan" => 100,
    "One" => 1,
    "LoopFx" => 1,
    "StrtMod" => 1,
    "StpMod" => 1,
    "Measure" => 1,
    "LoopSync" => 1,
    "TempoSync" => 1,
};

macro_rules! vec_of_strings {
    // match a list of expressions separated by comma:
    ($($str:expr),*) => ({
        // create a Vec with this list of expressions,
        // calling String::from on each:
        vec![$($str,)*] as Vec<&str>
    });
}

// pub static DISPLAY_VALUES: phf::Map<&'static str, Vec<&'static str>> = phf_map! {
//     "Rev" => vec_of_strings!["NO", "YES"],
// };
