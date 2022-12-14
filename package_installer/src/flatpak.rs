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
        "codium" => Option::from("com.vscodium.codium"),
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
        "gnome-passwordsafe" => Option::from("org.gnome.PasswordSafe"),
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
        "vlc" => Option::from("org.videolan.VLC"),
        "xonotic" => Option::from("org.xonotic.Xonotic"),
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

pub fn install(package: &str, distribution: &Distribution, info: &mut Info) {
    distribution.install_flatpak(info);

    let pkg: Option<&str> = get_package(package);
    if pkg.is_some() {
        let pkg: &str = pkg.unwrap();
        if !info.flatpak_installed.contains(&pkg.to_owned()) {
            info.flatpak_installed.push(pkg.to_owned());

            println!("Installing flatpak {}...", pkg);

            let _ = Command::new("flatpak")
                .arg("install")
                .arg("flathub")
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
