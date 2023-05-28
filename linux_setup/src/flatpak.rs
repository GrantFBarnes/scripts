use std::process::{Command, Stdio};
use std::str::Split;

use crate::distribution::Distribution;
use crate::helper;
use crate::Info;

pub fn setup() {
    println!("Setup flatpak...");

    let _ = Command::new("flatpak")
        .arg("remote-add")
        .arg("--if-not-exists")
        .arg("fedora")
        .arg("oci+https://registry.fedoraproject.org")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("flatpak fedora setup failed")
        .wait();

    let _ = Command::new("flatpak")
        .arg("remote-add")
        .arg("--if-not-exists")
        .arg("flathub")
        .arg("https://flathub.org/repo/flathub.flatpakrepo")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("flatpak flathub setup failed")
        .wait();
}

fn get_package(package: &str) -> Option<&str> {
    match package {
        "0ad" => Option::from("com.play0ad.zeroad"),
        "aisleriot" => Option::from("org.gnome.Aisleriot"),
        "ark" => Option::from("org.kde.ark"),
        "baobab" => Option::from("org.gnome.baobab"),
        "blender" => Option::from("org.blender.Blender"),
        "cheese" => Option::from("org.gnome.Cheese"),
        "chromium" => Option::from("org.chromium.Chromium"),
        "code" => Option::from("com.visualstudio.code"),
        "dconf-editor" => Option::from("ca.desrt.dconf-editor"),
        "deja-dup" => Option::from("org.gnome.DejaDup"),
        "discord" => Option::from("com.discordapp.Discord"),
        "elisa" => Option::from("org.kde.elisa"),
        "eog" => Option::from("org.gnome.eog"),
        "epiphany" => Option::from("org.gnome.Epiphany"),
        "evince" => Option::from("org.gnome.Evince"),
        "firefox" => Option::from("org.mozilla.firefox"),
        "gedit" => Option::from("org.gnome.gedit"),
        "gimp" => Option::from("org.gimp.GIMP"),
        "gnome-2048" => Option::from("org.gnome.TwentyFortyEight"),
        "gnome-books" => Option::from("org.gnome.Books"),
        "gnome-boxes" => Option::from("org.gnome.Boxes"),
        "gnome-builder" => Option::from("org.gnome.Builder"),
        "gnome-calculator" => Option::from("org.gnome.Calculator"),
        "gnome-calendar" => Option::from("org.gnome.Calendar"),
        "gnome-chess" => Option::from("org.gnome.Chess"),
        "gnome-clocks" => Option::from("org.gnome.clocks"),
        "gnome-connections" => Option::from("org.gnome.Connections"),
        "gnome-contacts" => Option::from("org.gnome.Contacts"),
        "gnome-maps" => Option::from("org.gnome.Maps"),
        "gnome-mines" => Option::from("org.gnome.Mines"),
        "gnome-music" => Option::from("org.gnome.Music"),
        "gnome-passwordsafe" => Option::from("org.gnome.World.Secrets"),
        "gnome-photos" => Option::from("org.gnome.Photos"),
        "gnome-sound-recorder" => Option::from("org.gnome.SoundRecorder"),
        "gnome-sudoku" => Option::from("org.gnome.Sudoku"),
        "gnome-text-editor" => Option::from("org.gnome.TextEditor"),
        "gnome-weather" => Option::from("org.gnome.Weather"),
        "gnucash" => Option::from("org.gnucash.GnuCash"),
        "gwenview" => Option::from("org.kde.gwenview"),
        "intellij" => Option::from("com.jetbrains.IntelliJ-IDEA-Community"),
        "kcalc" => Option::from("org.kde.kcalc"),
        "kdenlive" => Option::from("org.kde.kdenlive"),
        "kdevelop" => Option::from("org.kde.kdevelop"),
        "ksudoku" => Option::from("org.kde.ksudoku"),
        "kwrite" => Option::from("org.kde.kwrite"),
        "libreoffice" => Option::from("org.libreoffice.LibreOffice"),
        "mediawriter" => Option::from("org.fedoraproject.MediaWriter"),
        "okular" => Option::from("org.kde.okular"),
        "pycharm" => Option::from("com.jetbrains.PyCharm-Community"),
        "quadrapassel" => Option::from("org.gnome.Quadrapassel"),
        "rhythmbox" => Option::from("org.gnome.Rhythmbox3"),
        "shotwell" => Option::from("org.gnome.Shotwell"),
        "steam" => Option::from("com.valvesoftware.Steam"),
        "supertuxkart" => Option::from("net.supertuxkart.SuperTuxKart"),
        "thunderbird" => Option::from("org.mozilla.Thunderbird"),
        "torbrowser-launcher" => Option::from("com.github.micahflee.torbrowser-launcher"),
        "totem" => Option::from("org.gnome.Totem"),
        "transmission-gtk" => Option::from("com.transmissionbt.Transmission"),
        "transmission-qt" => Option::from("com.transmissionbt.Transmission"),
        "vlc" => Option::from("org.videolan.VLC"),
        "xonotic" => Option::from("org.xonotic.Xonotic"),
        _ => None,
    }
}

pub fn get_remotes(package: &str) -> Option<Vec<&str>> {
    match package {
        "0ad" => Option::from(vec!["fedora", "flathub"]),
        "aisleriot" => Option::from(vec!["fedora", "flathub"]),
        "ark" => Option::from(vec!["fedora", "flathub"]),
        "baobab" => Option::from(vec!["fedora", "flathub"]),
        "blender" => Option::from(vec!["flathub"]),
        "cheese" => Option::from(vec!["fedora", "flathub"]),
        "chromium" => Option::from(vec!["flathub"]),
        "code" => Option::from(vec!["flathub"]),
        "dconf-editor" => Option::from(vec!["fedora", "flathub"]),
        "deja-dup" => Option::from(vec!["flathub"]),
        "discord" => Option::from(vec!["flathub"]),
        "elisa" => Option::from(vec!["fedora", "flathub"]),
        "eog" => Option::from(vec!["fedora", "flathub"]),
        "epiphany" => Option::from(vec!["fedora", "flathub"]),
        "evince" => Option::from(vec!["fedora", "flathub"]),
        "firefox" => Option::from(vec!["flathub"]),
        "gedit" => Option::from(vec!["fedora", "flathub"]),
        "gimp" => Option::from(vec!["fedora", "flathub"]),
        "gnome-2048" => Option::from(vec!["fedora", "flathub"]),
        "gnome-books" => Option::from(vec!["fedora"]),
        "gnome-boxes" => Option::from(vec!["flathub"]),
        "gnome-builder" => Option::from(vec!["flathub"]),
        "gnome-calculator" => Option::from(vec!["fedora", "flathub"]),
        "gnome-calendar" => Option::from(vec!["fedora", "flathub"]),
        "gnome-chess" => Option::from(vec!["fedora", "flathub"]),
        "gnome-clocks" => Option::from(vec!["fedora", "flathub"]),
        "gnome-connections" => Option::from(vec!["fedora", "flathub"]),
        "gnome-contacts" => Option::from(vec!["fedora", "flathub"]),
        "gnome-maps" => Option::from(vec!["fedora", "flathub"]),
        "gnome-mines" => Option::from(vec!["fedora", "flathub"]),
        "gnome-music" => Option::from(vec!["fedora", "flathub"]),
        "gnome-passwordsafe" => Option::from(vec!["fedora", "flathub"]),
        "gnome-photos" => Option::from(vec!["fedora", "flathub"]),
        "gnome-sound-recorder" => Option::from(vec!["fedora", "flathub"]),
        "gnome-sudoku" => Option::from(vec!["fedora", "flathub"]),
        "gnome-text-editor" => Option::from(vec!["fedora", "flathub"]),
        "gnome-weather" => Option::from(vec!["fedora", "flathub"]),
        "gnucash" => Option::from(vec!["flathub"]),
        "gwenview" => Option::from(vec!["fedora", "flathub"]),
        "intellij" => Option::from(vec!["flathub"]),
        "kcalc" => Option::from(vec!["fedora", "flathub"]),
        "kdenlive" => Option::from(vec!["flathub"]),
        "kdevelop" => Option::from(vec!["flathub"]),
        "ksudoku" => Option::from(vec!["fedora", "flathub"]),
        "kwrite" => Option::from(vec!["fedora", "flathub"]),
        "libreoffice" => Option::from(vec!["fedora", "flathub"]),
        "mediawriter" => Option::from(vec!["fedora", "flathub"]),
        "okular" => Option::from(vec!["fedora", "flathub"]),
        "pycharm" => Option::from(vec!["flathub"]),
        "quadrapassel" => Option::from(vec!["fedora", "flathub"]),
        "rhythmbox" => Option::from(vec!["fedora", "flathub"]),
        "shotwell" => Option::from(vec!["fedora", "flathub"]),
        "steam" => Option::from(vec!["flathub"]),
        "supertuxkart" => Option::from(vec!["fedora", "flathub"]),
        "thunderbird" => Option::from(vec!["fedora", "flathub"]),
        "torbrowser-launcher" => Option::from(vec!["flathub"]),
        "totem" => Option::from(vec!["fedora", "flathub"]),
        "transmission-gtk" => Option::from(vec!["fedora", "flathub"]),
        "transmission-qt" => Option::from(vec!["fedora", "flathub"]),
        "vlc" => Option::from(vec!["flathub"]),
        "xonotic" => Option::from(vec!["flathub"]),
        _ => None,
    }
}

pub fn is_available(package: &str) -> bool {
    get_package(package).is_some()
}

pub fn is_installed(package: &str, info: &Info) -> bool {
    let pkg: Option<&str> = get_package(package);
    if pkg.is_some() {
        if info.flatpak_installed.contains(&pkg.unwrap().to_owned()) {
            return true;
        }
    }
    false
}

pub fn install(package: &str, remote: &str, distribution: &Distribution, info: &mut Info) {
    distribution.install("flatpak", info);

    let pkg: Option<&str> = get_package(package);
    if pkg.is_some() {
        let pkg: &str = pkg.unwrap();
        if !info.flatpak_installed.contains(&pkg.to_owned()) {
            info.flatpak_installed.push(pkg.to_owned());

            println!("Installing flatpak {} from {}...", pkg, remote);

            let _ = Command::new("flatpak")
                .arg("install")
                .arg(remote)
                .arg(pkg)
                .arg("-y")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("install flatpak failed")
                .wait();
        }
    }
}

pub fn uninstall(package: &str, info: &mut Info) {
    let pkg: Option<&str> = get_package(package);
    if pkg.is_some() {
        let pkg: &str = pkg.unwrap();
        if info.flatpak_installed.contains(&pkg.to_owned()) {
            let index: Option<usize> = info.flatpak_installed.iter().position(|x| *x == pkg);
            if index.is_some() {
                info.flatpak_installed.remove(index.unwrap());
            }

            println!("Uninstalling flatpak {}...", pkg);

            let _ = Command::new("flatpak")
                .arg("remove")
                .arg(pkg)
                .arg("-y")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("uninstall flatpak failed")
                .wait();
        }
    }
}

pub fn update() {
    println!("Update flatpak...");

    let _ = Command::new("flatpak")
        .arg("update")
        .arg("-y")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("update flatpak failed")
        .wait();
}

pub fn auto_remove() {
    println!("Auto removing flatpak...");

    let _ = Command::new("flatpak")
        .arg("remove")
        .arg("--unused")
        .arg("-y")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("auto remove flatpak failed")
        .wait();
}

pub fn get_installed() -> Vec<String> {
    let mut packages: Vec<String> = vec![];

    let mut cmd: Command = Command::new("flatpak");
    cmd.arg("list");
    cmd.arg("--app");

    let output: Option<String> = helper::get_command_output(cmd);
    if output.is_some() {
        let output: String = output.unwrap();
        for line in output.split("\n") {
            if line.is_empty() {
                continue;
            }
            let columns: Split<&str> = line.split("\t");
            let columns: Vec<&str> = columns.collect::<Vec<&str>>();
            if columns.len() > 1 {
                packages.push(columns[1].to_owned());
            }
        }
    }

    return packages;
}
