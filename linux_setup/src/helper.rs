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

pub fn append_to_file_if_not_found<S: Into<String>>(
    path: S,
    find_value: S,
    add_value: S,
    sudo: bool,
) -> Result<(), io::Error> {
    let path = path.into();
    let find_value = find_value.into();
    let add_value = add_value.into();

    match fs::read_to_string(&path) {
        Ok(content) => {
            if !content.contains(&find_value) {
                append_to_file(path, add_value, sudo)?;
            }
        }
        Err(_) => append_to_file(path, add_value, sudo)?,
    }
    Ok(())
}

fn append_to_file<S: Into<String> + std::fmt::Display>(
    path: S,
    value: S,
    sudo: bool,
) -> Result<(), io::Error> {
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

pub fn replace_in_file<S: Into<String>>(
    path: S,
    find_value: S,
    replace_value: S,
    sudo: bool,
) -> Result<(), io::Error> {
    let path = path.into();
    let find_value = find_value.into();
    let replace_value = replace_value.into();

    let content = fs::read_to_string(&path)?;
    if content.contains(&find_value) {
        let content = content.replace(&find_value, &replace_value);
        write_file(path, content, sudo)?;
    }
    Ok(())
}

pub fn write_file<S: Into<String> + std::fmt::Display>(
    path: S,
    content: S,
    sudo: bool,
) -> Result<(), io::Error> {
    if sudo {
        Command::new("sudo")
            .arg("sh")
            .arg("-c")
            .arg(format!("echo \'{}\' > {}", content, path))
            .spawn()?;
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(format!("echo \'{}\' > {}", content, path))
            .spawn()?;
    }
    Ok(())
}
