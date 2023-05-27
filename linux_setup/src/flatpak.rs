use std::process::{Command, Stdio};
use std::str::Split;

use crate::distribution::Distribution;
use crate::helper;
use crate::Info;

pub struct Flatpak {
    pub name: &'static str,
    pub remotes: Vec<&'static str>,
}

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

fn get_package(package: &str) -> Option<Flatpak> {
    match package {
        "0ad" => Option::from(Flatpak {
            name: "com.play0ad.zeroad",
            remotes: vec!["flathub"],
        }),
        "aisleriot" => Option::from(Flatpak {
            name: "org.gnome.Aisleriot",
            remotes: vec!["flathub"],
        }),
        "ark" => Option::from(Flatpak {
            name: "org.kde.ark",
            remotes: vec!["flathub"],
        }),
        "baobab" => Option::from(Flatpak {
            name: "org.gnome.baobab",
            remotes: vec!["flathub"],
        }),
        "blender" => Option::from(Flatpak {
            name: "org.blender.Blender",
            remotes: vec!["flathub"],
        }),
        "cheese" => Option::from(Flatpak {
            name: "org.gnome.Cheese",
            remotes: vec!["flathub"],
        }),
        "chromium" => Option::from(Flatpak {
            name: "org.chromium.Chromium",
            remotes: vec!["flathub"],
        }),
        "deja-dup" => Option::from(Flatpak {
            name: "org.gnome.DejaDup",
            remotes: vec!["flathub"],
        }),
        "discord" => Option::from(Flatpak {
            name: "com.discordapp.Discord",
            remotes: vec!["flathub"],
        }),
        "elisa" => Option::from(Flatpak {
            name: "org.kde.elisa",
            remotes: vec!["flathub"],
        }),
        "eog" => Option::from(Flatpak {
            name: "org.gnome.eog",
            remotes: vec!["flathub"],
        }),
        "epiphany" => Option::from(Flatpak {
            name: "org.gnome.Epiphany",
            remotes: vec!["flathub"],
        }),
        "evince" => Option::from(Flatpak {
            name: "org.gnome.Evince",
            remotes: vec!["flathub"],
        }),
        "firefox" => Option::from(Flatpak {
            name: "org.mozilla.firefox",
            remotes: vec!["flathub"],
        }),
        "gedit" => Option::from(Flatpak {
            name: "org.gnome.gedit",
            remotes: vec!["flathub"],
        }),
        "gimp" => Option::from(Flatpak {
            name: "org.gimp.GIMP",
            remotes: vec!["flathub"],
        }),
        "gnome-2048" => Option::from(Flatpak {
            name: "org.gnome.TwentyFortyEight",
            remotes: vec!["flathub"],
        }),
        "gnome-books" => Option::from(Flatpak {
            name: "org.gnome.Books",
            remotes: vec!["flathub"],
        }),
        "gnome-boxes" => Option::from(Flatpak {
            name: "org.gnome.Boxes",
            remotes: vec!["flathub"],
        }),
        "gnome-builder" => Option::from(Flatpak {
            name: "org.gnome.Builder",
            remotes: vec!["flathub"],
        }),
        "gnome-calculator" => Option::from(Flatpak {
            name: "org.gnome.Calculator",
            remotes: vec!["flathub"],
        }),
        "gnome-calendar" => Option::from(Flatpak {
            name: "org.gnome.Calendar",
            remotes: vec!["flathub"],
        }),
        "gnome-chess" => Option::from(Flatpak {
            name: "org.gnome.Chess",
            remotes: vec!["flathub"],
        }),
        "gnome-clocks" => Option::from(Flatpak {
            name: "org.gnome.clocks",
            remotes: vec!["flathub"],
        }),
        "gnome-connections" => Option::from(Flatpak {
            name: "org.gnome.Connections",
            remotes: vec!["flathub"],
        }),
        "gnome-contacts" => Option::from(Flatpak {
            name: "org.gnome.Contacts",
            remotes: vec!["flathub"],
        }),
        "gnome-maps" => Option::from(Flatpak {
            name: "org.gnome.Maps",
            remotes: vec!["flathub"],
        }),
        "gnome-mines" => Option::from(Flatpak {
            name: "org.gnome.Mines",
            remotes: vec!["flathub"],
        }),
        "gnome-music" => Option::from(Flatpak {
            name: "org.gnome.Music",
            remotes: vec!["flathub"],
        }),
        "gnome-passwordsafe" => Option::from(Flatpak {
            name: "org.gnome.PasswordSafe",
            remotes: vec!["flathub"],
        }),
        "gnome-photos" => Option::from(Flatpak {
            name: "org.gnome.Photos",
            remotes: vec!["flathub"],
        }),
        "gnome-sound-recorder" => Option::from(Flatpak {
            name: "org.gnome.SoundRecorder",
            remotes: vec!["flathub"],
        }),
        "gnome-sudoku" => Option::from(Flatpak {
            name: "org.gnome.Sudoku",
            remotes: vec!["flathub"],
        }),
        "gnome-text-editor" => Option::from(Flatpak {
            name: "org.gnome.TextEditor",
            remotes: vec!["flathub"],
        }),
        "gnome-weather" => Option::from(Flatpak {
            name: "org.gnome.Weather",
            remotes: vec!["flathub"],
        }),
        "gnucash" => Option::from(Flatpak {
            name: "org.gnucash.GnuCash",
            remotes: vec!["flathub"],
        }),
        "gwenview" => Option::from(Flatpak {
            name: "org.kde.gwenview",
            remotes: vec!["flathub"],
        }),
        "intellij" => Option::from(Flatpak {
            name: "com.jetbrains.IntelliJ-IDEA-Community",
            remotes: vec!["flathub"],
        }),
        "kcalc" => Option::from(Flatpak {
            name: "org.kde.kcalc",
            remotes: vec!["flathub"],
        }),
        "kdenlive" => Option::from(Flatpak {
            name: "org.kde.kdenlive",
            remotes: vec!["flathub"],
        }),
        "kdevelop" => Option::from(Flatpak {
            name: "org.kde.kdevelop",
            remotes: vec!["flathub"],
        }),
        "ksudoku" => Option::from(Flatpak {
            name: "org.kde.ksudoku",
            remotes: vec!["flathub"],
        }),
        "kwrite" => Option::from(Flatpak {
            name: "org.kde.kwrite",
            remotes: vec!["flathub"],
        }),
        "libreoffice" => Option::from(Flatpak {
            name: "org.libreoffice.LibreOffice",
            remotes: vec!["flathub"],
        }),
        "mediawriter" => Option::from(Flatpak {
            name: "org.fedoraproject.MediaWriter",
            remotes: vec!["flathub"],
        }),
        "okular" => Option::from(Flatpak {
            name: "org.kde.okular",
            remotes: vec!["flathub"],
        }),
        "pycharm" => Option::from(Flatpak {
            name: "com.jetbrains.PyCharm-Community",
            remotes: vec!["flathub"],
        }),
        "quadrapassel" => Option::from(Flatpak {
            name: "org.gnome.Quadrapassel",
            remotes: vec!["flathub"],
        }),
        "rhythmbox" => Option::from(Flatpak {
            name: "org.gnome.Rhythmbox3",
            remotes: vec!["flathub"],
        }),
        "shotwell" => Option::from(Flatpak {
            name: "org.gnome.Shotwell",
            remotes: vec!["flathub"],
        }),
        "steam" => Option::from(Flatpak {
            name: "com.valvesoftware.Steam",
            remotes: vec!["flathub"],
        }),
        "supertuxkart" => Option::from(Flatpak {
            name: "net.supertuxkart.SuperTuxKart",
            remotes: vec!["flathub"],
        }),
        "thunderbird" => Option::from(Flatpak {
            name: "org.mozilla.Thunderbird",
            remotes: vec!["flathub"],
        }),
        "torbrowser-launcher" => Option::from(Flatpak {
            name: "com.github.micahflee.torbrowser-launcher",
            remotes: vec!["flathub"],
        }),
        "totem" => Option::from(Flatpak {
            name: "org.gnome.Totem",
            remotes: vec!["flathub"],
        }),
        "vlc" => Option::from(Flatpak {
            name: "org.videolan.VLC",
            remotes: vec!["flathub"],
        }),
        "xonotic" => Option::from(Flatpak {
            name: "org.xonotic.Xonotic",
            remotes: vec!["flathub"],
        }),
        _ => None,
    }
}

pub fn is_available(package: &str) -> bool {
    get_package(package).is_some()
}

pub fn is_installed(package: &str, info: &Info) -> bool {
    let pkg: Option<Flatpak> = get_package(package);
    if pkg.is_some() {
        let pkg: Flatpak = pkg.unwrap();
        if info.flatpak_installed.contains(&pkg.name.to_owned()) {
            return true;
        }
    }
    false
}

pub fn install(package: &str, distribution: &Distribution, info: &mut Info) {
    distribution.install("flatpak", info);

    let pkg: Option<Flatpak> = get_package(package);
    if pkg.is_some() {
        let pkg: Flatpak = pkg.unwrap();
        if !info.flatpak_installed.contains(&pkg.name.to_owned()) {
            info.flatpak_installed.push(pkg.name.to_owned());

            println!("Installing flatpak {}...", pkg.name);

            let remote: Option<&&str> = pkg.remotes.first();
            // TODO: choose remote
            if remote.is_some() {
                let remote: &str = remote.unwrap();
                let _ = Command::new("flatpak")
                    .arg("install")
                    .arg(remote)
                    .arg(pkg.name)
                    .arg("-y")
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("install flatpak failed")
                    .wait();
            }
        }
    }
}

pub fn uninstall(package: &str, info: &mut Info) {
    let pkg: Option<Flatpak> = get_package(package);
    if pkg.is_some() {
        let pkg: Flatpak = pkg.unwrap();
        if info.flatpak_installed.contains(&pkg.name.to_owned()) {
            let index: Option<usize> = info.flatpak_installed.iter().position(|x| *x == pkg.name);
            if index.is_some() {
                info.flatpak_installed.remove(index.unwrap());
            }

            println!("Uninstalling flatpak {}...", pkg.name);

            let _ = Command::new("flatpak")
                .arg("remove")
                .arg(pkg.name)
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
