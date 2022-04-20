use crate::model;
use std::fs::File;
use std::io;
use std::io::Write;

pub fn write(filename: &str, config: model::Config) -> io::Result<()> {
    let mut file = File::create(filename)?;
    writeln!(&mut file, r#"<?xml version="1.0" encoding="utf-8"?>"#)?;
    writeln!(&mut file, r#"<database name="RC-500" revision="0">"#)?;

    for mem in config.memories {
        writeln!(&mut file, "<mem id=\"{}\">", mem.id)?;
        for menu in mem.menus {
            writeln!(&mut file, "<{}>", menu.name)?;
            for setting in menu.settings {
                writeln!(
                    &mut file,
                    "  <{}>{}</{}>",
                    setting.key, setting.value, setting.key
                )?;
            }
            writeln!(&mut file, "</{}>", menu.name)?;
        }
        writeln!(&mut file, "</mem>")?;
    }

    writeln!(&mut file, r#"</database>"#)?;
    writeln!(&mut file, r#"6!  "#)?;
    Ok(())
}
