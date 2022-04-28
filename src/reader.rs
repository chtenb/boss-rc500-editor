use crate::model;
use roxmltree;
use std::path::Path;
use std::str;

fn find_subsequence(haystack: &Vec<u8>, needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

pub fn read(filename: &Path) -> Result<model::Config, String> {
    // NOTE: read binary because the suffix can contain null bytes
    let text = &std::fs::read(filename).map_err(|e| format!("Reading error: {}.", e))?;
    // The suffix characters are not valid xml
    let n = find_subsequence(text, b"</database>\n").ok_or("Could not find datebase end tag")?;
    let pivot = n + "</database>\n".len();
    let xml_bytes = &text[0..pivot].to_vec();
    let xml = str::from_utf8(xml_bytes).map_err(|_| "XML was not valid utf8")?;
    let suffix = text[pivot..].to_vec();

    let v = roxmltree::Document::parse(&xml).map_err(|e| format!("Parsing error: {}.", e))?;
    doc_to_config(v, &filename.to_string_lossy(), suffix)
}

fn validate_mem_node(node: roxmltree::Node) -> Result<(), String> {
    let tag = node.tag_name().name();
    if tag != "mem" {
        println!("{:?}", node);
        Err(format!("Expected tag 'mem' but found '{}'", tag))
    } else {
        Ok(())
    }
}

fn doc_to_config(doc: roxmltree::Document, filename: &str, suffix: Vec<u8>) -> Result<model::Config, String> {
    let mut memories: Vec<model::Memory> = Vec::new();
    let database = doc
        .root()
        .first_child()
        .ok_or("Could not find toplevel element".to_string())?;
    for mem_node in database.children().filter(|c| c.is_element()) {
        validate_mem_node(mem_node)?;
        let id = mem_node
            .attribute("id")
            .ok_or("Memory tag has no id attribute".to_string())
            .and_then(|text| {
                text.parse::<usize>()
                    .map_err(|_e| format!("Id attribute is not an int, but: {}.", text))
            })?;
        let mut menus: Vec<model::Menu> = Vec::new();
        for menu_node in mem_node.children().filter(|c| c.is_element()) {
            let mut settings: Vec<model::UntypedKeyValue> = Vec::new();
            for setting_node in menu_node.children().filter(|c| c.is_element()) {
                let key = setting_node.tag_name().name().to_string();
                let value = setting_node
                    .text()
                    .ok_or(format!("Setting {} has no value", key))
                    .and_then(|text| {
                        text.parse::<usize>()
                            .map_err(|_e| format!("Setting value is not an int, but: {}.", text))
                    })?;
                let setting = model::UntypedKeyValue { key: key, value: value };
                settings.push(setting);
            }
            let name = menu_node.tag_name().name().to_string();
            let menu = if name == "NAME" {
                read_string_menu(name, settings)
            } else {
                model::Menu {
                    name: name,
                    content: model::MenuContent::KeyValueMenu(model::UntypedMenu { settings: settings }),
                }
            };
            menus.push(menu);
        }
        let memory = model::Memory { id: id, menus: menus };
        memories.push(memory);
    }
    Ok(model::Config {
        filename: filename.to_string(),
        suffix: suffix,
        memories: memories,
    })
}

fn read_string_menu(name: String, settings: Vec<model::UntypedKeyValue>) -> model::Menu {
    let value = String::from_iter(settings.into_iter().map(|kv| (kv.value as u8) as char));
    let trimmed = value.trim_end().to_string();
    model::Menu {
        name: name,
        content: model::MenuContent::StringValueMenu(model::StringValueMenu { value: trimmed }),
    }
}
