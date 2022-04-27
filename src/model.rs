use phf::phf_map;

#[derive(Clone, Debug)]
pub struct Config {
    pub filename: String,
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
    "LoopFx" => "Loop FX",
    "StrtMod" => "Start",
    "StpMod" => "Stop",
};

pub static BOUNDS: phf::Map<&'static str, usize> = phf_map! {
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

pub static DISPLAY_VALUES: phf::Map<&'static str, &'static [&'static str]> = phf_map! {
    "Rev" => &["NO", "YES"],
    "LoopSync" => &["NO", "YES"],
    "TempoSync" => &["NO", "YES"],
};
