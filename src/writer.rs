use crate::model;
use std::fs::File;
use std::io;
use std::io::Write;

pub fn write(filename: &str, config: model::Config) -> Result<(), String> {
    _write(filename, config).map_err(|e| format!("Error while writing: {}", e))
}

fn _write(filename: &str, config: model::Config) -> io::Result<()> {
    let mut file = File::create(filename)?;
    writeln!(&mut file, r#"<?xml version="1.0" encoding="utf-8"?>"#)?;
    writeln!(&mut file, r#"<database name="RC-500" revision="0">"#)?;

    for mem in config.memories {
        writeln!(&mut file, "<mem id=\"{}\">", mem.id)?;
        for menu in mem.menus {
            writeln!(&mut file, "<{}>", menu.name)?;
            match menu.content {
                model::MenuContent::StringValueMenu(menu) => write_string_menu(&mut file, menu)?,
                model::MenuContent::KeyValueMenu(menu) => write_key_value_menu(&mut file, menu)?,
            };
            writeln!(&mut file, "</{}>", menu.name);
        }
        writeln!(&mut file, "</mem>")?;
    }

    writeln!(&mut file, r#"</database>"#)?;
    writeln!(&mut file, r#"6!  "#)?;
    Ok(())
}

fn write_string_menu(file: &mut File, menu: model::StringValueMenu) -> io::Result<()> {
    let i: i32 = 1;
    for c in menu.value.chars() {
        writeln!(file, "\t<C{:02}>{}</C{:02}>", i, c as u8, i)?;
    }
    Ok(())
}

fn write_key_value_menu(file: &mut File, menu: model::UntypedMenu) -> io::Result<()> {
    for setting in menu.settings {
        writeln!(file, "\t<{}>{}</{}>", setting.key, setting.value, setting.key)?;
    }
    Ok(())
}
