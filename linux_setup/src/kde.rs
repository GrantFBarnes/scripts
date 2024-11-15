use std::env;
use std::env::VarError;
use std::fs;
use std::io;
use std::process::Command;
use std::str::Split;

use rust_cli::commands::Operation;

pub fn setup() -> Result<(), io::Error> {
    let plasma_version: u8 = if Operation::new("plasmashell --version")
        .hide_output(true)
        .output()
        .unwrap_or("".to_string())
        .contains(" 6")
    {
        6
    } else {
        5
    };

    const APPLET_PATH: &str = "plasma-org.kde.plasma.desktop-appletsrc";

    // Configure Clock
    let group = find_group(APPLET_PATH, "org.kde.plasma.digitalclock");
    if group.is_ok() {
        let group = group.unwrap();
        let mut groups: Vec<&str> = convert_group_to_groups(&group);
        groups.push("Configuration");
        groups.push("Appearance");
        set_config(
            plasma_version,
            APPLET_PATH,
            &groups,
            "dateDisplayFormat",
            "BelowTime".to_string(),
        )?;
        set_config(
            plasma_version,
            APPLET_PATH,
            &groups,
            "dateFormat",
            "isoDate".to_string(),
        )?;
        set_config(
            plasma_version,
            APPLET_PATH,
            &groups,
            "showSeconds",
            "Always".to_string(),
        )?;
    }

    // Show Battery Percentage
    let group = find_group(APPLET_PATH, "org.kde.plasma.battery");
    if group.is_ok() {
        let group = group.unwrap();
        let mut groups: Vec<&str> = convert_group_to_groups(&group);
        groups.push("Configuration");
        groups.push("General");
        set_config(
            plasma_version,
            APPLET_PATH,
            &groups,
            "showPercentage",
            true.to_string(),
        )?;
    }

    // Set NumLock
    let groups: Vec<&str> = vec!["Keyboard"];
    set_config(
        plasma_version,
        "kcminputrc",
        &groups,
        "NumLock",
        "0".to_string(),
    )?;

    // Set Dolphin to always open home
    let groups: Vec<&str> = vec!["General"];
    set_config(
        plasma_version,
        "dolphinrc",
        &groups,
        "RememberOpenedTabs",
        false.to_string(),
    )?;

    // Set Screen Lock Timeout
    let groups: Vec<&str> = vec!["Daemon"];
    set_config(
        plasma_version,
        "kscreenlockerrc",
        &groups,
        "Timeout",
        "15".to_string(),
    )?;

    // Start Empty Session on Login
    let groups: Vec<&str> = vec!["General"];
    set_config(
        plasma_version,
        "ksmserverrc",
        &groups,
        "loginMode",
        "emptySession".to_string(),
    )?;

    // Set Kate
    let groups: Vec<&str> = vec!["General"];
    set_config(
        plasma_version,
        "katerc",
        &groups,
        "Show Full Path in Title",
        true.to_string(),
    )?;
    let groups: Vec<&str> = vec!["KTextEditor Renderer"];
    set_config(
        plasma_version,
        "katerc",
        &groups,
        "Show Indentation Lines",
        true.to_string(),
    )?;
    set_config(
        plasma_version,
        "katerc",
        &groups,
        "Animate Bracket Matching",
        true.to_string(),
    )?;
    set_config(
        plasma_version,
        "katerc",
        &groups,
        "Word Wrap Marker",
        true.to_string(),
    )?;
    let groups: Vec<&str> = vec!["KTextEditor Document"];
    set_config(
        plasma_version,
        "katerc",
        &groups,
        "Show Spaces",
        "2".to_string(),
    )?;
    let groups: Vec<&str> = vec!["KTextEditor View"];
    set_config(
        plasma_version,
        "katerc",
        &groups,
        "Scroll Past End",
        true.to_string(),
    )?;
    set_config(
        plasma_version,
        "katerc",
        &groups,
        "Input Mode",
        "1".to_string(),
    )?;
    set_config(
        plasma_version,
        "katerc",
        &groups,
        "Vi Relative Line Numbers",
        true.to_string(),
    )?;
    let groups: Vec<&str> = vec!["lspclient"];
    set_config(
        plasma_version,
        "katerc",
        &groups,
        "FormatOnSave",
        true.to_string(),
    )?;
    Ok(())
}

fn find_group(path: &str, plugin: &str) -> Result<String, io::Error> {
    let home_dir: Result<String, VarError> = env::var("HOME");
    if home_dir.is_err() {
        return Err(io::Error::other("HOME directory could not be determined"));
    }
    let home_dir: String = home_dir.unwrap();

    let file_path: String = format!("{}/.config/{}", &home_dir, path);
    let file_content = fs::read_to_string(&file_path)?;
    let mut group: String = String::new();
    for line in file_content.split("\n") {
        if line.starts_with("[") {
            group = line.to_string();
        } else if line.starts_with("plugin") {
            if line.contains(plugin) {
                if !group.is_empty() {
                    return Ok(group);
                }
            }
        }
    }
    return Err(io::Error::other("group not found"));
}

fn convert_group_to_groups(group: &String) -> Vec<&str> {
    let group: &str = &group[1..group.len() - 1];
    let groups: Split<&str> = group.split("][");
    groups.collect::<Vec<&str>>()
}

fn set_config(
    version: u8,
    file: &str,
    groups: &Vec<&str>,
    key: &str,
    value: String,
) -> Result<(), io::Error> {
    let mut cmd: Command = Command::new(format!("kwriteconfig{}", version));
    cmd.arg("--file");
    cmd.arg(file);
    for group in groups {
        cmd.arg("--group");
        cmd.arg(group);
    }
    cmd.arg("--key");
    cmd.arg(key);
    cmd.arg(value);
    cmd.status()?;
    Ok(())
}
