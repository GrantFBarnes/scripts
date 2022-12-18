use std::env;
use std::env::VarError;
use std::fs;
use std::process::Command;
use std::str::Split;

fn find_group(path: &str, plugin: &str) -> Option<String> {
    let home_dir: Result<String, VarError> = env::var("HOME");
    if home_dir.is_ok() {
        let home_dir: String = home_dir.unwrap();
        let file_path: String = format!("{}/.config/{}", &home_dir, path);
        let file_content: std::io::Result<String> = fs::read_to_string(&file_path);
        if file_content.is_ok() {
            let file_content: String = file_content.unwrap();
            let file_content: Split<&str> = file_content.split("\n");
            let mut group: String = String::new();
            for line in file_content {
                if line.starts_with("[") {
                    group = line.to_string();
                } else if line.starts_with("plugin") {
                    if line.contains(plugin) {
                        if !group.is_empty() {
                            return Option::from(group);
                        }
                    }
                }
            }
        }
    }
    None
}

fn convert_group_to_groups(group: &String) -> Vec<&str> {
    let group: &str = &group[1..group.len() - 1];
    let groups: Split<&str> = group.split("][");
    groups.collect::<Vec<&str>>()
}

fn set_config(file: &str, groups: &Vec<&str>, key: &str, value: String) {
    let mut cmd: Command = Command::new("kwriteconfig5");
    cmd.arg("--file");
    cmd.arg(file);
    for group in groups {
        cmd.arg("--group");
        cmd.arg(group);
    }
    cmd.arg("--key");
    cmd.arg(key);
    cmd.arg(value);
    let _ = cmd.status();
}

pub fn setup() {
    const APPLET_PATH: &str = "plasma-org.kde.plasma.desktop-appletsrc";

    // Configure Clock
    let group: Option<String> = find_group(APPLET_PATH, "org.kde.plasma.digitalclock");
    if group.is_some() {
        let group = group.unwrap();
        let mut groups: Vec<&str> = convert_group_to_groups(&group);
        groups.push("Configuration");
        groups.push("Appearance");
        set_config(
            APPLET_PATH,
            &groups,
            "dateFormat",
            format!("\"{}\"", "isoDate"),
        );
        set_config(APPLET_PATH, &groups, "showSeconds", true.to_string());
    }

    // Show Battery Percentage
    let group: Option<String> = find_group(APPLET_PATH, "org.kde.plasma.battery");
    if group.is_some() {
        let group = group.unwrap();
        let mut groups: Vec<&str> = convert_group_to_groups(&group);
        groups.push("Configuration");
        groups.push("General");
        set_config(APPLET_PATH, &groups, "showPercentage", true.to_string());
    }

    // Set Night Color
    let groups: Vec<&str> = vec!["NightColor"];
    set_config("kwinrc", &groups, "Active", true.to_string());
    set_config("kwinrc", &groups, "Mode", format!("\"{}\"", "Times"));
    set_config("kwinrc", &groups, "MorningBeginFixed", "0700".to_string());
    set_config("kwinrc", &groups, "EveningBeginFixed", "1900".to_string());
    set_config("kwinrc", &groups, "NightTemperature", "2300".to_string());

    // Set NumLock
    let groups: Vec<&str> = vec!["Keyboard"];
    set_config("kcminputrc", &groups, "NumLock", "0".to_string());

    // Set File Click to Double
    let groups: Vec<&str> = vec!["KDE"];
    set_config("kdeglobals", &groups, "SingleClick", false.to_string());

    // Set Dolphin to always open home
    let groups: Vec<&str> = vec!["General"];
    set_config(
        "dolphinrc",
        &groups,
        "RememberOpenedTabs",
        false.to_string(),
    );

    // Set Screen Lock Timeout
    let groups: Vec<&str> = vec!["Daemon"];
    set_config("kscreenlockerrc", &groups, "Timeout", "15".to_string());

    // Start Empty Session on Login
    let groups: Vec<&str> = vec!["General"];
    set_config(
        "ksmserverrc",
        &groups,
        "loginMode",
        format!("\"{}\"", "emptySession"),
    );

    // Set Kate
    let groups: Vec<&str> = vec!["General"];
    set_config(
        "katerc",
        &groups,
        "Show Full Path in Title",
        true.to_string(),
    );
    set_config("katerc", &groups, "Show Menu Bar", true.to_string());
    let groups: Vec<&str> = vec!["KTextEditor Renderer"];
    set_config(
        "katerc",
        &groups,
        "Show Indentation Lines",
        true.to_string(),
    );
    set_config(
        "katerc",
        &groups,
        "Show Whole Bracket Expression",
        true.to_string(),
    );
    let groups: Vec<&str> = vec!["KTextEditor Document"];
    set_config("katerc", &groups, "Show Spaces", "1".to_string());
    let groups: Vec<&str> = vec!["KTextEditor View"];
    set_config("katerc", &groups, "Scroll Past End", true.to_string());
    set_config("katerc", &groups, "Show Line Count", true.to_string());
    set_config("katerc", &groups, "Show Word Count", true.to_string());
    set_config("katerc", &groups, "Line Numbers", true.to_string());
    set_config("katerc", &groups, "Smart Copy Cut", true.to_string());
    set_config("katerc", &groups, "Input Mode", "1".to_string());
    set_config(
        "katerc",
        &groups,
        "Vi Input Mode Steal Keys",
        false.to_string(),
    );
}
