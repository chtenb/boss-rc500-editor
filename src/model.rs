#[derive(Clone, Debug)]
pub struct Config {
    pub memories: Vec<Memory>,
}

#[derive(Clone, Debug)]
pub struct Memory {
    pub id: i32,
    pub menus: Vec<UntypedMenu>,
}

#[derive(Clone, Debug)]
pub struct UntypedMenu {
    pub name: String,
    pub settings: Vec<UntypedKeyValue>,
}

#[derive(Clone, Debug)]
pub struct UntypedKeyValue {
    pub key: String,
    pub value: i32,
}
