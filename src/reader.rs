use crate::model;
use roxmltree;

pub fn read(filename: &str) -> Result<model::Config, String> {
    let text = std::fs::read_to_string(filename).map_err(|e| format!("Reading error: {}.", e))?;
    // TODO: preprocess text (drop last line)
    roxmltree::Document::parse(&text)
        .map_err(|e| format!("Parsing error: {}.", e))
        .and_then(|v| doc_to_config(v))
}

fn validate_mem_node(node: roxmltree::Node) -> Result<(), String> {
    let tag = node.tag_name().name();
    if tag != "mem" {
        Err(format!("Expected tag 'mem' but found {}", tag))
    } else {
        Ok(())
    }
}

fn doc_to_config(doc: roxmltree::Document) -> Result<model::Config, String> {
    let mut memories: Vec<model::Memory> = Vec::new();
    for mem_node in doc.root().children() {
        validate_mem_node(mem_node)?;
        let id = mem_node
            .attribute("id")
            .ok_or("Memory tag has no id attribute".to_string())
            .and_then(|text| {
                text.parse::<i32>()
                    .map_err(|_e| format!("Id attribute is not an int, but: {}.", text))
            })?;
        let mut menus: Vec<model::UntypedMenu> = Vec::new();
        for menu_node in mem_node.children() {
            let mut settings: Vec<model::UntypedKeyValue> = Vec::new();
            for setting_node in menu_node.children() {
                let key = setting_node.tag_name().name().to_string();
                let value = setting_node
                    .text()
                    .ok_or(format!("Setting {} has no value", key))
                    .and_then(|text| {
                        text.parse::<i32>()
                            .map_err(|_e| format!("Setting value is not an int, but: {}.", text))
                    })?;
                let setting = model::UntypedKeyValue {
                    key: key,
                    value: value,
                };
                settings.push(setting);
            }
            let menu = model::UntypedMenu {
                name: menu_node.tag_name().name().to_string(),
                settings: settings,
            };
            menus.push(menu);
        }
        let memory = model::Memory {
            id: id,
            menus: menus,
        };
        memories.push(memory);
    }
    Ok(model::Config { memories: memories })
}
