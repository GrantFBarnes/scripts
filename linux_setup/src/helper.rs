use rust_cli::ansi::Color;
use rust_cli::ansi::Font;

use std::fs;
use std::io;
use std::process::Command;

pub fn get_colored_string<S: Into<String>>(string: S, color: Color) -> String {
    format!(
        "{}{}{}",
        color.as_str(),
        string.into(),
        Font::Reset.as_str()
    )
}

fn append_to_file(path: &String, value: &str, sudo: bool) -> Result<(), io::Error> {
    if sudo {
        Command::new("sudo")
            .arg("sh")
            .arg("-c")
            .arg(format!("echo \'{}\' >> {}", value, path))
            .spawn()?;
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(format!("echo \'{}\' >> {}", value, path))
            .spawn()?;
    }
    Ok(())
}

pub fn append_to_file_if_not_found(
    path: &String,
    find_value: &str,
    add_value: &str,
    sudo: bool,
) -> Result<(), io::Error> {
    match fs::read_to_string(path) {
        Ok(content) => {
            if !content.contains(find_value) {
                append_to_file(path, add_value, sudo)?;
            }
        }
        Err(_) => append_to_file(path, add_value, sudo)?,
    }
    Ok(())
}
