use std::process::{Command, Stdio};
use std::str::SplitWhitespace;

use crate::distribution::Distribution;
use crate::helper;
use crate::Info;

pub struct Snap {
    pub name: &'static str,
    pub is_official: bool,
    pub is_classic: bool,
    pub channel: &'static str,
}

pub fn get_package(package: &str) -> Option<Snap> {
    match package {
        "0ad" => Option::from(Snap {
            name: "0ad",
            is_official: true,
            is_classic: false,
            channel: "",
        }),
        "ark" => Option::from(Snap {
            name: "ark",
            is_official: true,
            is_classic: false,
            channel: "",
        }),
        "blender" => Option::from(Snap {
            name: "blender",
            is_official: true,
            is_classic: true,
            channel: "",
        }),
        "chromium" => Option::from(Snap {
            name: "chromium",
            is_official: true,
            is_classic: false,
            channel: "",
        }),
        "code" => Option::from(Snap {
            name: "code",
            is_official: true,
            is_classic: true,
            channel: "",
        }),
        "codium" => Option::from(Snap {
            name: "codium",
            is_official: false,
            is_classic: true,
            channel: "",
        }),
        "discord" => Option::from(Snap {
            name: "discord",
            is_official: false,
            is_classic: false,
            channel: "",
        }),
        "eog" => Option::from(Snap {
            name: "eog",
            is_official: true,
            is_classic: false,
            channel: "",
        }),
        "firefox" => Option::from(Snap {
            name: "firefox",
            is_official: true,
            is_classic: false,
            channel: "",
        }),
        "firefox-esr" => Option::from(Snap {
            name: "firefox",
            is_official: true,
            is_classic: false,
            channel: "esr-stable",
        }),
        "gedit" => Option::from(Snap {
            name: "gedit",
            is_official: true,
            is_classic: false,
            channel: "",
        }),
        "gimp" => Option::from(Snap {
            name: "gimp",
            is_official: false,
            is_classic: false,
            channel: "",
        }),
        "gnome-calculator" => Option::from(Snap {
            name: "gnome-calculator",
            is_official: true,
            is_classic: false,
            channel: "",
        }),
        "gnome-clocks" => Option::from(Snap {
            name: "gnome-clocks",
            is_official: true,
            is_classic: false,
            channel: "",
        }),
        "gnome-sudoku" => Option::from(Snap {
            name: "gnome-sudoku",
            is_official: true,
            is_classic: false,
            channel: "",
        }),
        "gwenview" => Option::from(Snap {
            name: "gwenview",
            is_official: true,
            is_classic: false,
            channel: "",
        }),
        "intellij" => Option::from(Snap {
            name: "intellij-idea-community",
            is_official: true,
            is_classic: true,
            channel: "",
        }),
        "kate" => Option::from(Snap {
            name: "kate",
            is_official: true,
            is_classic: true,
            channel: "",
        }),
        "kcalc" => Option::from(Snap {
            name: "kcalc",
            is_official: true,
            is_classic: false,
            channel: "",
        }),
        "kdenlive" => Option::from(Snap {
            name: "kdenlive",
            is_official: true,
            is_classic: false,
            channel: "",
        }),
        "kdevelop" => Option::from(Snap {
            name: "kdevelop",
            is_official: true,
            is_classic: true,
            channel: "",
        }),
        "kmines" => Option::from(Snap {
            name: "kmines",
            is_official: true,
            is_classic: false,
            channel: "",
        }),
        "knights" => Option::from(Snap {
            name: "knights",
            is_official: true,
            is_classic: false,
            channel: "",
        }),
        "ksudoku" => Option::from(Snap {
            name: "ksudoku",
            is_official: true,
            is_classic: false,
            channel: "",
        }),
        "libreoffice" => Option::from(Snap {
            name: "libreoffice",
            is_official: true,
            is_classic: false,
            channel: "",
        }),
        "node" => Option::from(Snap {
            name: "node",
            is_official: true,
            is_classic: true,
            channel: "18/stable",
        }),
        "okular" => Option::from(Snap {
            name: "okular",
            is_official: true,
            is_classic: false,
            channel: "",
        }),
        "pycharm" => Option::from(Snap {
            name: "pycharm-community",
            is_official: true,
            is_classic: true,
            channel: "",
        }),
        "quadrapassel" => Option::from(Snap {
            name: "quadrapassel",
            is_official: true,
            is_classic: false,
            channel: "",
        }),
        "snap-store" => Option::from(Snap {
            name: "snap-store",
            is_official: true,
            is_classic: false,
            channel: "",
        }),
        "steam" => Option::from(Snap {
            name: "steam",
            is_official: true,
            is_classic: false,
            channel: "",
        }),
        "supertuxkart" => Option::from(Snap {
            name: "supertuxkart",
            is_official: false,
            is_classic: false,
            channel: "",
        }),
        "thunderbird" => Option::from(Snap {
            name: "thunderbird",
            is_official: true,
            is_classic: false,
            channel: "",
        }),
        "vlc" => Option::from(Snap {
            name: "vlc",
            is_official: true,
            is_classic: false,
            channel: "",
        }),
        "xonotic" => Option::from(Snap {
            name: "xonotic",
            is_official: false,
            is_classic: false,
            channel: "",
        }),
        _ => None,
    }
}

pub fn is_available(package: &str) -> bool {
    get_package(package).is_some()
}

pub fn is_installed(package: &str, info: &Info) -> bool {
    let pkg: Option<Snap> = get_package(package);
    if pkg.is_some() {
        if info.snap_installed.contains(&pkg.unwrap().name.to_owned()) {
            return true;
        }
    }
    false
}

pub fn install(package: &str, distribution: &Distribution, info: &mut Info) {
    distribution.install_snap(info);

    let pkg: Option<Snap> = get_package(package);
    if pkg.is_some() {
        let pkg: Snap = pkg.unwrap();
        if !info.snap_installed.contains(&pkg.name.to_owned()) {
            info.snap_installed.push(pkg.name.to_owned());

            println!("Installing snap {}...", pkg.name);

            let mut cmd: Command = Command::new("sudo");
            cmd.arg("snap");
            cmd.arg("install");
            cmd.arg(pkg.name);
            if pkg.is_classic {
                cmd.arg("--classic");
            }
            if !pkg.channel.is_empty() {
                cmd.arg("--channel");
                cmd.arg(pkg.channel);
            }
            let _ = cmd
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("install snap failed")
                .wait();
        }
    }
}

pub fn uninstall(package: &str, info: &mut Info) {
    let pkg: Option<Snap> = get_package(package);
    if pkg.is_some() {
        let pkg: Snap = pkg.unwrap();
        if info.snap_installed.contains(&pkg.name.to_owned()) {
            let index: Option<usize> = info.snap_installed.iter().position(|x| *x == pkg.name);
            if index.is_some() {
                info.snap_installed.remove(index.unwrap());
            }

            println!("Uninstalling snap {}...", pkg.name);

            let _ = Command::new("sudo")
                .arg("snap")
                .arg("remove")
                .arg(pkg.name)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("uninstall snap failed")
                .wait();
        }
    }
}

pub fn update() {
    println!("Update snap...");

    let _ = Command::new("sudo")
        .arg("snap")
        .arg("refresh")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("update snap failed")
        .wait();
}

pub fn get_installed() -> Vec<String> {
    let mut packages: Vec<String> = vec![];

    let mut cmd: Command = Command::new("snap");
    cmd.arg("list");

    let output: Option<String> = helper::get_command_output(cmd);
    if output.is_some() {
        let output: String = output.unwrap();
        for line in output.split("\n") {
            if line.is_empty() {
                continue;
            }
            let columns: SplitWhitespace = line.split_whitespace();
            let columns: Vec<&str> = columns.collect::<Vec<&str>>();
            if !columns.is_empty() {
                packages.push(columns[0].to_owned());
            }
        }
    }

    return packages;
}
