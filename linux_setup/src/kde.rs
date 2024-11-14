use std::env;
use std::env::VarError;
use std::fs;
use std::io;
use std::process::Command;
use std::str::Split;

use crate::distribution::Distribution;

pub fn setup(distribution: &Distribution) -> Result<(), io::Error> {
    const APPLET_PATH: &str = "plasma-org.kde.plasma.desktop-appletsrc";

    // Configure Clock
    let group = find_group(APPLET_PATH, "org.kde.plasma.digitalclock");
    if group.is_ok() {
        let group = group.unwrap();
        let mut groups: Vec<&str> = convert_group_to_groups(&group);
        groups.push("Configuration");
        groups.push("Appearance");
        set_config(
            distribution,
            APPLET_PATH,
            &groups,
            "dateDisplayFormat",
            "BesideTime".to_string(),
        )?;
        set_config(
            distribution,
            APPLET_PATH,
            &groups,
            "dateFormat",
            "isoDate".to_string(),
        )?;
        set_config(
            distribution,
            APPLET_PATH,
            &groups,
            "showSeconds",
            true.to_string(),
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
            distribution,
            APPLET_PATH,
            &groups,
            "showPercentage",
            true.to_string(),
        )?;
    }

    // Set NumLock
    let groups: Vec<&str> = vec!["Keyboard"];
    set_config(
        distribution,
        "kcminputrc",
        &groups,
        "NumLock",
        "0".to_string(),
    )?;

    // Set File Click to Double
    let groups: Vec<&str> = vec!["KDE"];
    set_config(
        distribution,
        "kdeglobals",
        &groups,
        "SingleClick",
        false.to_string(),
    )?;

    // Set Dolphin to always open home
    let groups: Vec<&str> = vec!["General"];
    set_config(
        distribution,
        "dolphinrc",
        &groups,
        "RememberOpenedTabs",
        false.to_string(),
    )?;

    // Set Screen Lock Timeout
    let groups: Vec<&str> = vec!["Daemon"];
    set_config(
        distribution,
        "kscreenlockerrc",
        &groups,
        "Timeout",
        "15".to_string(),
    )?;

    // Start Empty Session on Login
    let groups: Vec<&str> = vec!["General"];
    set_config(
        distribution,
        "ksmserverrc",
        &groups,
        "loginMode",
        "emptySession".to_string(),
    )?;

    // Set Kate
    let groups: Vec<&str> = vec!["General"];
    set_config(
        distribution,
        "katerc",
        &groups,
        "Show Full Path in Title",
        true.to_string(),
    )?;
    set_config(
        distribution,
        "katerc",
        &groups,
        "Show Menu Bar",
        true.to_string(),
    )?;
    let groups: Vec<&str> = vec!["KTextEditor Renderer"];
    set_config(
        distribution,
        "katerc",
        &groups,
        "Show Indentation Lines",
        true.to_string(),
    )?;
    set_config(
        distribution,
        "katerc",
        &groups,
        "Show Whole Bracket Expression",
        true.to_string(),
    )?;
    let groups: Vec<&str> = vec!["KTextEditor Document"];
    set_config(
        distribution,
        "katerc",
        &groups,
        "Show Spaces",
        "1".to_string(),
    )?;
    let groups: Vec<&str> = vec!["KTextEditor View"];
    set_config(
        distribution,
        "katerc",
        &groups,
        "Scroll Past End",
        true.to_string(),
    )?;
    set_config(
        distribution,
        "katerc",
        &groups,
        "Show Line Count",
        true.to_string(),
    )?;
    set_config(
        distribution,
        "katerc",
        &groups,
        "Show Word Count",
        true.to_string(),
    )?;
    set_config(
        distribution,
        "katerc",
        &groups,
        "Line Numbers",
        true.to_string(),
    )?;
    set_config(
        distribution,
        "katerc",
        &groups,
        "Smart Copy Cut",
        true.to_string(),
    )?;
    set_config(
        distribution,
        "katerc",
        &groups,
        "Input Mode",
        "1".to_string(),
    )?;
    set_config(
        distribution,
        "katerc",
        &groups,
        "Vi Input Mode Steal Keys",
        false.to_string(),
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
    distribution: &Distribution,
    file: &str,
    groups: &Vec<&str>,
    key: &str,
    value: String,
) -> Result<(), io::Error> {
    let cmd: Option<Command> = if distribution.packages.contains("kwriteconfig5") {
        Some(Command::new("kwriteconfig5"))
    } else if distribution.packages.contains("kwriteconfig6") {
        Some(Command::new("kwriteconfig6"))
    } else {
        None
    };
    if cmd.is_none() {
        return Err(io::Error::other("kwriteconfig not found"));
    }
    let mut cmd: Command = cmd.unwrap();
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
