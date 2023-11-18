use std::env;
use std::env::VarError;
use std::fs;
use std::io;
use std::process::{Command, Stdio};

extern crate rust_cli;

mod distribution;
mod flatpak;
mod gnome;
mod helper;
mod kde;
mod other;
mod snap;

use crate::distribution::{Distribution, DistributionName, PackageManager, Repository};
use crate::snap::Snap;

pub struct Info {
    has_gnome: bool,
    has_kde: bool,
    has_flatpak: bool,
    has_snap: bool,
    repository_installed: Vec<String>,
    flatpak_installed: Vec<String>,
    snap_installed: Vec<String>,
    other_installed: Vec<String>,
}

struct Package {
    display: &'static str,
    key: &'static str,
    category: &'static str,
    desktop_environment: &'static str,
}

const CATEGORIES: [&str; 10] = [
    "Server",
    "Desktop",
    "Applications",
    "Browsers",
    "Communication",
    "Games",
    "Multi Media",
    "Editors",
    "Software",
    "Utilities",
];

const ALL_PACKAGES: [Package; 105] = [
    Package {
        display: "0 A.D.",
        key: "0ad",
        category: "Games",
        desktop_environment: "",
    },
    Package {
        display: "Ark Archiving",
        key: "ark",
        category: "Utilities",
        desktop_environment: "kde",
    },
    Package {
        display: "Blender",
        key: "blender",
        category: "Multi Media",
        desktop_environment: "",
    },
    Package {
        display: "Cheese - Webcam",
        key: "cheese",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Chromium",
        key: "chromium",
        category: "Browsers",
        desktop_environment: "",
    },
    Package {
        display: "Cockpit - Web Interface",
        key: "cockpit",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "cups - Printer Support",
        key: "cups",
        category: "Desktop",
        desktop_environment: "",
    },
    Package {
        display: "cURL - Client URL",
        key: "curl",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "dconf Editor",
        key: "dconf-editor",
        category: "Utilities",
        desktop_environment: "gnome",
    },
    Package {
        display: "Deja Dup - Backups",
        key: "deja-dup",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Discord",
        key: "discord",
        category: "Communication",
        desktop_environment: "",
    },
    Package {
        display: "dotnet - C# runtime 8.0 LTS",
        key: "dotnet-runtime-8",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "dotnet - C# SDK 8.0 LTS",
        key: "dotnet-sdk-8",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "Elisa Music Player",
        key: "elisa",
        category: "Multi Media",
        desktop_environment: "kde",
    },
    Package {
        display: "Epiphany - Gnome Web",
        key: "epiphany",
        category: "Browsers",
        desktop_environment: "gnome",
    },
    Package {
        display: "Evince - Document Viewer",
        key: "evince",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Eye of Gnome - Image Viewer",
        key: "eog",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Fedora Media Writer",
        key: "mediawriter",
        category: "Utilities",
        desktop_environment: "",
    },
    Package {
        display: "ffmpeg - Media Codecs",
        key: "ffmpeg",
        category: "Desktop",
        desktop_environment: "",
    },
    Package {
        display: "FileLight Disk Usage",
        key: "filelight",
        category: "Utilities",
        desktop_environment: "kde",
    },
    Package {
        display: "Firefox",
        key: "firefox",
        category: "Browsers",
        desktop_environment: "",
    },
    Package {
        display: "Firefox ESR",
        key: "firefox-esr",
        category: "Browsers",
        desktop_environment: "",
    },
    Package {
        display: "Flutter",
        key: "flutter",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "gedit",
        key: "gedit",
        category: "Editors",
        desktop_environment: "gnome",
    },
    Package {
        display: "GIMP",
        key: "gimp",
        category: "Multi Media",
        desktop_environment: "",
    },
    Package {
        display: "git - Version Control",
        key: "git",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "Go Language",
        key: "golang",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "GParted",
        key: "gparted",
        category: "Utilities",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome 2048",
        key: "gnome-2048",
        category: "Games",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Boxes - VM Manager",
        key: "gnome-boxes",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Builder",
        key: "gnome-builder",
        category: "Editors",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Calculator",
        key: "gnome-calculator",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Calendar",
        key: "gnome-calendar",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Chess",
        key: "gnome-chess",
        category: "Games",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Clocks",
        key: "gnome-clocks",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Connections",
        key: "gnome-connections",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Contacts",
        key: "gnome-contacts",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Disk Usage",
        key: "baobab",
        category: "Utilities",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Disk Utility",
        key: "gnome-disk-utility",
        category: "Utilities",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Maps",
        key: "gnome-maps",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Mines",
        key: "gnome-mines",
        category: "Games",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Music",
        key: "gnome-music",
        category: "Multi Media",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Password Safe",
        key: "gnome-passwordsafe",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Photos",
        key: "gnome-photos",
        category: "Multi Media",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Shell Extension",
        key: "gnome-shell-extensions",
        category: "Utilities",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Shell Extension Manager",
        key: "gnome-shell-extension-manager",
        category: "Utilities",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Software",
        key: "gnome-software",
        category: "Software",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Solitaire",
        key: "aisleriot",
        category: "Games",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Sound Recorder",
        key: "gnome-sound-recorder",
        category: "Multi Media",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Sudoku",
        key: "gnome-sudoku",
        category: "Games",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome System Monitor",
        key: "gnome-system-monitor",
        category: "Utilities",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Tetris",
        key: "quadrapassel",
        category: "Games",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Text Editor",
        key: "gnome-text-editor",
        category: "Editors",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Tweaks",
        key: "gnome-tweaks",
        category: "Utilities",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Weather",
        key: "gnome-weather",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "GNU Cash - Accounting",
        key: "gnucash",
        category: "Applications",
        desktop_environment: "",
    },
    Package {
        display: "Gwenview - Image Viewer",
        key: "gwenview",
        category: "Applications",
        desktop_environment: "kde",
    },
    Package {
        display: "htop - Process Reviewer",
        key: "htop",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "IceCat - GNU Browser",
        key: "icecat",
        category: "Browsers",
        desktop_environment: "",
    },
    Package {
        display: "imagemagick",
        key: "imagemagick",
        category: "Desktop",
        desktop_environment: "",
    },
    Package {
        display: "Intellij",
        key: "intellij",
        category: "Editors",
        desktop_environment: "",
    },
    Package {
        display: "Kate",
        key: "kate",
        category: "Editors",
        desktop_environment: "kde",
    },
    Package {
        display: "KCalc - Calculator",
        key: "kcalc",
        category: "Applications",
        desktop_environment: "kde",
    },
    Package {
        display: "KDE Chess",
        key: "knights",
        category: "Games",
        desktop_environment: "kde",
    },
    Package {
        display: "KDE Mines",
        key: "kmines",
        category: "Games",
        desktop_environment: "kde",
    },
    Package {
        display: "KDE Sudoku",
        key: "ksudoku",
        category: "Games",
        desktop_environment: "kde",
    },
    Package {
        display: "KdenLive Video Editor",
        key: "kdenlive",
        category: "Multi Media",
        desktop_environment: "kde",
    },
    Package {
        display: "KDevelop",
        key: "kdevelop",
        category: "Editors",
        desktop_environment: "kde",
    },
    Package {
        display: "Kile - LaTex Editor",
        key: "kile",
        category: "Editors",
        desktop_environment: "kde",
    },
    Package {
        display: "KSysGuard",
        key: "ksysguard",
        category: "Utilities",
        desktop_environment: "kde",
    },
    Package {
        display: "KWrite",
        key: "kwrite",
        category: "Editors",
        desktop_environment: "kde",
    },
    Package {
        display: "LaTex - Compiler",
        key: "latex",
        category: "Desktop",
        desktop_environment: "",
    },
    Package {
        display: "LibreOffice",
        key: "libreoffice",
        category: "Editors",
        desktop_environment: "",
    },
    Package {
        display: "MariaDB - Database",
        key: "mariadb",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "MP3 Metadata Editor",
        key: "id3v2",
        category: "Desktop",
        desktop_environment: "",
    },
    Package {
        display: "nano - Text Editor",
        key: "nano",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "Node.js - JavaScript RE",
        key: "node",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "Okular - Document Viewer",
        key: "okular",
        category: "Applications",
        desktop_environment: "kde",
    },
    Package {
        display: "Plasma Discover",
        key: "plasma-discover",
        category: "Software",
        desktop_environment: "kde",
    },
    Package {
        display: "Plasma System Monitor",
        key: "plasma-systemmonitor",
        category: "Utilities",
        desktop_environment: "kde",
    },
    Package {
        display: "Podman - Containers",
        key: "podman",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "Pycharm",
        key: "pycharm",
        category: "Editors",
        desktop_environment: "",
    },
    Package {
        display: "qtile - Window Manager",
        key: "qtile",
        category: "Desktop",
        desktop_environment: "",
    },
    Package {
        display: "RhythmBox",
        key: "rhythmbox",
        category: "Multi Media",
        desktop_environment: "gnome",
    },
    Package {
        display: "Rust Language",
        key: "rust",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "Shotwell",
        key: "shotwell",
        category: "Multi Media",
        desktop_environment: "gnome",
    },
    Package {
        display: "Simple Scan",
        key: "simple-scan",
        category: "Utilities",
        desktop_environment: "",
    },
    Package {
        display: "Snap",
        key: "snapd",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "Snap Store",
        key: "snap-store",
        category: "Software",
        desktop_environment: "",
    },
    Package {
        display: "Spectacle Screenshot",
        key: "spectacle",
        category: "Utilities",
        desktop_environment: "kde",
    },
    Package {
        display: "SSH - Secure Shell Protocol",
        key: "ssh",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "Steam",
        key: "steam",
        category: "Games",
        desktop_environment: "",
    },
    Package {
        display: "Super Tux Kart",
        key: "supertuxkart",
        category: "Games",
        desktop_environment: "",
    },
    Package {
        display: "Thunderbird",
        key: "thunderbird",
        category: "Communication",
        desktop_environment: "",
    },
    Package {
        display: "TOR - The Onion Router",
        key: "torbrowser-launcher",
        category: "Browsers",
        desktop_environment: "",
    },
    Package {
        display: "Totem Video Player",
        key: "totem",
        category: "Multi Media",
        desktop_environment: "gnome",
    },
    Package {
        display: "Transmission (GTK) - Torrent",
        key: "transmission-gtk",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Transmission (QT) - Torrent",
        key: "transmission-qt",
        category: "Applications",
        desktop_environment: "kde",
    },
    Package {
        display: "Vietnamese Keyboard",
        key: "ibus-unikey",
        category: "Desktop",
        desktop_environment: "",
    },
    Package {
        display: "VIM - Text Editor",
        key: "vim",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "Virt Manager",
        key: "virt-manager",
        category: "Applications",
        desktop_environment: "",
    },
    Package {
        display: "VLC",
        key: "vlc",
        category: "Multi Media",
        desktop_environment: "",
    },
    Package {
        display: "VS Code",
        key: "code",
        category: "Editors",
        desktop_environment: "",
    },
    Package {
        display: "Xonotic",
        key: "xonotic",
        category: "Games",
        desktop_environment: "",
    },
    Package {
        display: "yt-dlp - Download YouTube",
        key: "yt-dlp",
        category: "Desktop",
        desktop_environment: "",
    },
];

fn repository_setup(distribution: &Distribution, info: &mut Info) -> Result<(), io::Error> {
    distribution.setup(info)?;
    if info.has_flatpak {
        flatpak::setup(distribution)?;
    }
    Ok(())
}

fn run_flatpak_remote_select(
    package: &str,
    distribution: &Distribution,
    info: &mut Info,
) -> Result<(), io::Error> {
    let mut options: Vec<&str> = vec![];

    let remotes: Option<Vec<&str>> = flatpak::get_remotes(package);
    if remotes.is_none() {
        return Err(io::Error::other("failed to get flatpak remotes"));
    }
    let remotes: Vec<&str> = remotes.unwrap();
    for remote in remotes {
        options.push(remote);
    }
    options.push("Cancel");

    let remote = rust_cli::prompts::Select::new()
        .title(format!("Flatpak Remote: {}", package))
        .options(&options)
        .erase_after(true)
        .prompt_for_value()?;
    if remote.is_none() {
        return Ok(());
    }
    let remote: String = remote.unwrap();
    if remote == "Cancel" {
        return Ok(());
    }

    flatpak::install(package, remote.as_str(), distribution, info)
}

fn post_uninstall(
    package: &str,
    distribution: &Distribution,
    method: &str,
) -> Result<(), io::Error> {
    let home_dir: Result<String, VarError> = env::var("HOME");
    if home_dir.is_err() {
        return Err(io::Error::other("HOME directory could not be determined"));
    }
    let home_dir: String = home_dir.unwrap();

    match package {
        "code" => {
            if method != "repository" {
                if distribution.package_manager == PackageManager::APT {
                    rust_cli::commands::Operation::new()
                        .command("sudo rm /etc/apt/sources.list.d/vscode.list")
                        .run()?;
                }
                if distribution.package_manager == PackageManager::DNF {
                    rust_cli::commands::Operation::new()
                        .command("sudo dnf config-manager --set-disabled code")
                        .run()?;
                    rust_cli::commands::Operation::new()
                        .command("sudo rm /etc/yum.repos.d/vscode.repo")
                        .run()?;
                }
            }
            if method == "uninstall" {
                rust_cli::commands::Operation::new()
                    .command(format!(
                        "sudo rm -r {}{} {}{}",
                        &home_dir, "/.vscode", &home_dir, "/.config/Code"
                    ))
                    .run()?;
            }
        }
        "golang" => {
            if method == "uninstall" {
                rust_cli::commands::Operation::new()
                    .command(format!("sudo rm -r {}{}", &home_dir, "/.go"))
                    .run()?;
            }
        }
        "pycharm" => {
            if method != "repository" {
                if distribution.name == DistributionName::Fedora {
                    rust_cli::commands::Operation::new()
                        .command("sudo dnf config-manager --set-disabled phracek-PyCharm")
                        .run()?;
                }
            }
        }
        "rust" => {
            if method != "other" {
                rust_cli::commands::Operation::new()
                    .command(format!("sudo rm -r {}{}", &home_dir, "/.cargo/bin/rustup"))
                    .run()?;
            }
        }
        "vim" => {
            if method != "repository" {
                rust_cli::commands::Operation::new()
                    .command(format!(
                        "sudo rm -r {}{} {}{} {}{}",
                        &home_dir, "/.vim", &home_dir, "/.viminfo", &home_dir, "/.vimrc"
                    ))
                    .run()?;
            }
        }
        _ => (),
    }

    Ok(())
}

fn pre_install(
    package: &str,
    distribution: &Distribution,
    info: &mut Info,
    method: &str,
) -> Result<(), io::Error> {
    match package {
        "code" => {
            if method == "repository" {
                if distribution.package_manager == PackageManager::APT {
                    distribution.install("wget", info)?;
                    distribution.install("gpg", info)?;

                    let key: String = rust_cli::commands::Operation::new()
                        .command("wget -qO- https://packages.microsoft.com/keys/microsoft.asc")
                        .run()?;
                    fs::write("packages.microsoft", key)?;

                    rust_cli::commands::Operation::new()
                        .command("gpg --dearmor packages.microsoft")
                        .run()?;

                    rust_cli::commands::Operation::new()
                        .command("sudo install -D -o root -g root -m 644 packages.microsoft.gpg /etc/apt/keyrings/packages.microsoft.gpg")
                        .run()?;

                    fs::remove_file("packages.microsoft")?;
                    fs::remove_file("packages.microsoft.gpg")?;

                    let echo_cmd = Command::new("echo")
                        .arg("deb [arch=amd64,arm64,armhf signed-by=/etc/apt/keyrings/packages.microsoft.gpg] https://packages.microsoft.com/repos/code stable main")
                        .stdout(Stdio::piped())
                        .spawn()?;
                    Command::new("sudo")
                        .arg("tee")
                        .arg("/etc/apt/sources.list.d/vscode.list")
                        .stdin(Stdio::from(echo_cmd.stdout.unwrap()))
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()?
                        .wait()?;

                    rust_cli::commands::Operation::new()
                        .command("sudo apt update")
                        .run()?;
                }
                if distribution.package_manager == PackageManager::DNF {
                    rust_cli::commands::Operation::new()
                        .command(
                            "sudo rpm --import https://packages.microsoft.com/keys/microsoft.asc",
                        )
                        .run()?;

                    let echo_cmd = Command::new("echo")
                        .arg("-e")
                        .arg("[code]\nname=Visual Studio Code\nbaseurl=https://packages.microsoft.com/yumrepos/vscode\nenabled=1\ngpgcheck=1\ngpgkey=https://packages.microsoft.com/keys/microsoft.asc")
                        .stdout(Stdio::piped())
                        .spawn()?;
                    Command::new("sudo")
                        .arg("tee")
                        .arg("/etc/yum.repos.d/vscode.repo")
                        .stdin(Stdio::from(echo_cmd.stdout.unwrap()))
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()?
                        .wait()?;
                }
            }
        }
        p if p.contains("dotnet") => {
            if method == "repository" {
                if distribution.repository == Repository::Debian {
                    distribution.install("wget", info)?;

                    rust_cli::commands::Operation::new()
                        .command("wget https://packages.microsoft.com/config/debian/12/packages-microsoft-prod.deb -O packages-microsoft-prod.deb")
                        .show_output(true)
                        .run()?;
                    rust_cli::commands::Operation::new()
                        .command("sudo dpkg -i packages-microsoft-prod.deb")
                        .show_output(true)
                        .run()?;
                    rust_cli::commands::Operation::new()
                        .command("rm packages-microsoft-prod.deb")
                        .run()?;
                    rust_cli::commands::Operation::new()
                        .command("sudo apt update")
                        .show_output(true)
                        .run()?;
                }
            }
        }
        "nodejs" => {
            if method == "repository" {
                if distribution.package_manager == PackageManager::DNF {
                    rust_cli::commands::Operation::new()
                        .command("sudo dnf module enable nodejs:18 -y")
                        .run()?;
                }
            }
        }
        "pycharm" => {
            if method == "repository" {
                if distribution.repository == Repository::Fedora {
                    rust_cli::commands::Operation::new()
                        .command("sudo dnf config-manager --set-enabled phracek-PyCharm")
                        .run()?;
                }
            }
        }
        "rust" => {
            if method == "other" {
                distribution.install("curl", info)?;
            }
        }
        _ => (),
    }

    Ok(())
}

fn post_install(package: &str, distribution: &Distribution, method: &str) -> Result<(), io::Error> {
    let home_dir: Result<String, VarError> = env::var("HOME");
    if home_dir.is_err() {
        return Err(io::Error::other("HOME directory could not be determined"));
    }
    let home_dir: String = home_dir.unwrap();

    match package {
        "code" => {
            if method != "uninstall" {
                let extensions: Vec<&str> = Vec::from(["esbenp.prettier-vscode", "vscodevim.vim"]);
                for ext in extensions {
                    rust_cli::commands::Operation::new()
                        .command(format!("code --install-extension {}", ext))
                        .run()?;
                }

                fs::write(
                    format!("{}{}", &home_dir, "/.config/Code/User/settings.json"),
                    r#"
{
  "telemetry.telemetryLevel": "off",
  "editor.formatOnSave": true,
  "editor.rulers": [80, 160],
  "extensions.ignoreRecommendations": true,
  "git.openRepositoryInParentFolders": "always",
  "workbench.startupEditor": "none",
  "vim.useCtrlKeys": false,
  "[css]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  },
  "[scss]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  },
  "[html]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  },
  "[javascript]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  },
  "[json]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  },
  "[jsonc]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  },
  "[typescript]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  }
}
"#,
                )?;
            }
        }
        "go" => {
            if method != "uninstall" {
                rust_cli::commands::Operation::new()
                    .command("go env -w GOPATH=$HOME/.go")
                    .run()?;
            }
        }
        "intellij" | "pycharm" => {
            fs::write(
                format!("{}{}", &home_dir, "/.ideavimrc"),
                "sethandler a:ide",
            )?;
        }
        "rust" => {
            if method == "other" {
                rust_cli::commands::Operation::new()
                    .command(format!(
                        "{}{} component add rust-analyzer",
                        home_dir, "/.cargo/bin/rustup"
                    ))
                    .run()?;
            }
        }
        "snapd" => {
            if method == "repository" {
                distribution.setup_snap()?;
            }
        }
        "vim" => {
            if method == "repository" {
                let bashrc: String = format!("{}{}", &home_dir, "/.bashrc");
                helper::append_to_file_if_not_found(
                    &bashrc,
                    "export EDITOR",
                    "export EDITOR=\"/usr/bin/vim\"\n",
                    false,
                )?;

                fs::write(
                    format!("{}{}", &home_dir, "/.vimrc"),
                    r#"
set nocompatible

set encoding=utf-8

set noswapfile
set nobackup
set nowritebackup

set mouse=a
set updatetime=300
set scrolloff=10
set number
set relativenumber
set ignorecase smartcase
set incsearch hlsearch

syntax on
filetype plugin indent on

""""""""""""""""""""""""""""""""""""""""
" Install VIM Plug

let data_dir = has('nvim') ? stdpath('data') . '/site' : '~/.vim'
if empty(glob(data_dir . '/autoload/plug.vim'))
  silent execute '!curl -fLo '.data_dir.'/autoload/plug.vim --create-dirs  https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim'
  autocmd VimEnter * PlugInstall --sync | source $MYVIMRC
endif

" Install plugins

call plug#begin()

Plug 'airblade/vim-gitgutter'
Plug 'fatih/vim-go', { 'do': ':GoUpdateBinaries' }
Plug 'kien/ctrlp.vim'
Plug 'scrooloose/nerdtree'
Plug 'scrooloose/syntastic'
Plug 'rust-lang/rust.vim'
Plug 'vim-airline/vim-airline'
Plug 'w0rp/ale'

call plug#end()

let mapleader = " "

let g:ale_completion_enabled = 1
let g:ale_linters = { "rust": ["analyzer"] }
let g:ale_fixers = { "rust": ["rustfmt"] }
let g:rustfmt_autosave = 1

""""""""""""""""""""""""""""""""""""""""
" normal mode remaps

nnoremap <Leader>ex :Explore<CR>
nnoremap <C-n> :NERDTreeToggle<CR>
" window split
nnoremap <Leader>vs <C-w>v
nnoremap <Leader>hs <C-w>s
" window navigation
nnoremap <C-h> <C-w>h
nnoremap <C-j> <C-w>j
nnoremap <C-k> <C-w>k
nnoremap <C-l> <C-w>l

""""""""""""""""""""""""""""""""""""""""
" insert mode remaps

inoremap <silent><expr> <Tab> pumvisible() ? "\<C-n>" : "\<TAB>"
inoremap <silent><expr> <S-Tab> pumvisible() ? "\<C-n>" : "\<S-TAB>"
"#,
                )?;
            }
        }
        _ => (),
    }

    Ok(())
}

fn get_install_method(package: &str, distribution: &Distribution, info: &Info) -> String {
    if distribution.is_installed(package, info) {
        return helper::get_colored_string("Repository", "green");
    }
    if flatpak::is_installed(package, info) {
        return helper::get_colored_string("Flatpak", "blue");
    }
    if snap::is_installed(package, info) {
        return helper::get_colored_string("Snap", "magenta");
    }
    if other::is_installed(package, info) {
        return helper::get_colored_string("Other", "yellow");
    }
    return helper::get_colored_string("Uninstalled", "red");
}

fn is_installed(package: &str, distribution: &Distribution, info: &Info) -> bool {
    distribution.is_installed(package, info)
        || flatpak::is_installed(package, info)
        || snap::is_installed(package, info)
        || other::is_installed(package, info)
}

fn run_package_select(
    package: &str,
    distribution: &Distribution,
    info: &mut Info,
) -> Result<(), io::Error> {
    let mut options_display: Vec<String> = vec![];
    let mut options_value: Vec<&str> = vec![];

    if distribution.is_available(package) {
        options_display.push(helper::get_colored_string("Install Repository", "green"));
        options_value.push("repository");
    }

    if flatpak::is_available(package) {
        options_display.push(helper::get_colored_string("Install Flatpak", "blue"));
        options_value.push("flatpak");
    }

    if snap::is_available(package) {
        let mut display: String = String::from("Install Snap");
        let pkg: Option<Snap> = snap::get_package(package);
        if pkg.is_some() {
            let pkg: Snap = pkg.unwrap();
            if pkg.is_official {
                display.push_str(" (Official)");
            }
            if pkg.is_classic {
                display.push_str(" (classic)");
            }
        }
        options_display.push(helper::get_colored_string(display, "magenta"));
        options_value.push("snap");
    }

    if other::is_available(package) {
        options_display.push(helper::get_colored_string("Install Other", "yellow"));
        options_value.push("other");
    }

    options_display.push(helper::get_colored_string("Uninstall", "red"));
    options_value.push("uninstall");

    options_display.push(helper::get_colored_string("Cancel", ""));
    options_value.push("cancel");

    let selection = rust_cli::prompts::Select::new()
        .title(format!(
            "Package: {} ({})",
            package,
            get_install_method(package, distribution, &info)
        ))
        .options(&options_display)
        .erase_after(true)
        .prompt_for_index()?;
    if selection.is_none() {
        return Ok(());
    }
    let selection: usize = selection.unwrap();
    let method: &str = options_value[selection];

    if method == "cancel" {
        return Ok(());
    }

    if method != "repository" {
        distribution.uninstall(package, info)?;
    }

    if method != "flatpak" {
        if info.has_flatpak {
            flatpak::uninstall(package, info)?;
        }
    }

    if method != "snap" {
        if info.has_snap {
            snap::uninstall(package, info)?;
        }
    }

    if method != "other" {
        other::uninstall(package, info)?;
    }
    post_uninstall(package, distribution, method)?;

    pre_install(package, distribution, info, method)?;
    match method {
        "repository" => distribution.install(package, info)?,
        "flatpak" => run_flatpak_remote_select(package, distribution, info)?,
        "snap" => snap::install(package, distribution, info)?,
        "other" => other::install(package, info)?,
        _ => (),
    }
    post_install(package, distribution, method)
}

fn run_category_select(
    category: &str,
    start_idx: usize,
    show_all_desktop_environments: bool,
    distribution: &Distribution,
    info: &mut Info,
) -> Result<(), io::Error> {
    let mut options_display: Vec<String> = vec![];
    let mut options_value: Vec<&str> = vec![];

    let mut missing_desktop_environment: bool = false;

    for pkg in ALL_PACKAGES {
        if pkg.category != category {
            continue;
        }

        if !distribution.is_available(pkg.key)
            && !flatpak::is_available(pkg.key)
            && !snap::is_available(pkg.key)
            && !other::is_available(pkg.key)
        {
            continue;
        }

        let mut missing_pkg_desktop_environment: bool = false;

        if (pkg.desktop_environment == "gnome" && !info.has_gnome)
            || (pkg.desktop_environment == "kde" && !info.has_kde)
        {
            missing_desktop_environment = true;
            if !show_all_desktop_environments && !is_installed(pkg.key, distribution, info) {
                continue;
            }
            missing_pkg_desktop_environment = true;
        }

        options_display.push(format!(
            "{} ({})",
            helper::get_colored_string(
                pkg.display,
                if missing_pkg_desktop_environment {
                    "yellow"
                } else {
                    ""
                }
            ),
            get_install_method(pkg.key, distribution, info)
        ));
        options_value.push(pkg.key);
    }

    if missing_desktop_environment {
        options_display.reverse();
        options_display.push(format!(
            "[{} Uninstalled Desktop Environments]",
            if show_all_desktop_environments {
                helper::get_colored_string("Hide", "yellow")
            } else {
                helper::get_colored_string("Show", "cyan")
            }
        ));
        options_display.reverse();

        options_value.reverse();
        options_value.push("toggle_show_all_desktop_environments");
        options_value.reverse();
    }

    options_display.push(helper::get_colored_string("Exit", ""));
    options_value.push("exit");

    let selection = rust_cli::prompts::Select::new()
        .title(format!("Category: {}", category))
        .options(&options_display)
        .default_index(start_idx)
        .erase_after(true)
        .prompt_for_index()?;
    if selection.is_none() {
        return Ok(());
    }
    let selection: usize = selection.unwrap();

    match options_value[selection] {
        "exit" => (),
        "toggle_show_all_desktop_environments" => {
            run_category_select(
                category,
                selection,
                !show_all_desktop_environments,
                distribution,
                info,
            )?;
        }
        _ => {
            run_package_select(options_value[selection], distribution, info)?;
            run_category_select(
                category,
                selection + 1,
                show_all_desktop_environments,
                distribution,
                info,
            )?;
        }
    }

    Ok(())
}

fn run_install_packages(
    start_idx: usize,
    distribution: &Distribution,
    info: &mut Info,
) -> Result<(), io::Error> {
    let mut options: Vec<&str> = vec![];
    for category in CATEGORIES {
        options.push(category);
    }
    options.push("Exit");

    let selection = rust_cli::prompts::Select::new()
        .title("Choose a Category")
        .options(&options)
        .default_index(start_idx)
        .erase_after(true)
        .prompt_for_index()?;
    if selection.is_none() {
        return Ok(());
    }
    let selection = selection.unwrap();
    match options[selection] {
        "Exit" => return Ok(()),
        _ => run_category_select(options[selection], 0, false, distribution, info)?,
    }

    run_install_packages(selection + 1, distribution, info)
}

fn run_menu(
    start_idx: usize,
    distribution: &Distribution,
    info: &mut Info,
) -> Result<(), io::Error> {
    let mut options: Vec<&str> = vec!["Repository Setup"];
    if info.has_gnome {
        options.push("GNOME Setup");
    }
    if info.has_kde {
        options.push("KDE Setup");
    }
    options.push("Update Packages");
    options.push("Auto Remove Packages");
    options.push("Install Packages");
    options.push("Exit");

    let selection = rust_cli::prompts::Select::new()
        .title("Linux Setup")
        .options(&options)
        .default_index(start_idx)
        .erase_after(true)
        .prompt_for_index()?;
    if selection.is_none() {
        return Ok(());
    }
    let selection: usize = selection.unwrap();

    match options[selection] {
        "Repository Setup" => repository_setup(distribution, info)?,
        "GNOME Setup" => gnome::setup(distribution)?,
        "KDE Setup" => kde::setup()?,
        "Update Packages" => {
            distribution.update()?;
            if info.has_flatpak {
                flatpak::update()?;
            }
            if info.has_snap {
                snap::update()?;
            }
            other::update(info)?;
        }
        "Auto Remove Packages" => {
            distribution.auto_remove()?;
            if info.has_flatpak {
                flatpak::auto_remove()?;
            }
        }
        "Install Packages" => run_install_packages(0, distribution, info)?,
        "Exit" => return Ok(()),
        _ => (),
    }

    run_menu(selection + 1, distribution, info)
}

fn main() -> Result<(), io::Error> {
    let has_gnome: bool = rust_cli::commands::Operation::new()
        .command("gnome-shell --version")
        .run()
        .is_ok();

    let has_kde: bool = rust_cli::commands::Operation::new()
        .command("plasmashell --version")
        .run()
        .is_ok();

    let distribution: Distribution = distribution::get_distribution()?;
    let repository_installed: Vec<String> = distribution.get_installed()?;

    let has_flatpak: bool = rust_cli::commands::Operation::new()
        .command("flatpak --version")
        .run()
        .is_ok();
    let flatpak_installed: Vec<String> = match has_flatpak {
        true => flatpak::get_installed()?,
        false => vec![],
    };

    let has_snap: bool = rust_cli::commands::Operation::new()
        .command("snap --version")
        .run()
        .is_ok();
    let snap_installed: Vec<String> = match has_snap {
        true => snap::get_installed()?,
        false => vec![],
    };

    let other_installed: Vec<String> = other::get_installed()?;

    let mut info: Info = Info {
        has_gnome,
        has_kde,
        has_flatpak,
        has_snap,
        repository_installed,
        flatpak_installed,
        snap_installed,
        other_installed,
    };
    run_menu(0, &distribution, &mut info)
}
