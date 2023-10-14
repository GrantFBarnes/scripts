use dialoguer::Select;
use std::env;
use std::env::VarError;
use std::fs;
use std::process::{Command, Stdio};

extern crate rust_cli;

mod distribution;
mod flatpak;
mod gnome;
mod helper;
mod kde;
mod other;
mod snap;

use crate::distribution::Distribution;
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

const ALL_PACKAGES: [Package; 107] = [
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
        display: "dotnet - C# runtime 6.0 LTS",
        key: "dotnet-runtime-6",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "dotnet - C# SDK 6.0 LTS",
        key: "dotnet-sdk-6",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "dotnet - C# runtime 7.0",
        key: "dotnet-runtime-7",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "dotnet - C# SDK 7.0",
        key: "dotnet-sdk-7",
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

fn repository_setup(distribution: &Distribution, info: &mut Info) {
    distribution.setup(info);
    if info.has_flatpak {
        flatpak::setup(distribution);
    }
}

fn run_flatpak_remote_select(package: &str, distribution: &Distribution, info: &mut Info) {
    let mut options: Vec<&str> = vec![];

    let remotes: Option<Vec<&str>> = flatpak::get_remotes(package);
    if remotes.is_none() {
        return;
    }
    let remotes: Vec<&str> = remotes.unwrap();
    for remote in remotes {
        options.push(remote);
    }
    options.push("Cancel");

    let selection: std::io::Result<Option<usize>> = Select::new()
        .with_prompt(format!("Flatpak Remote: {}", package))
        .items(&options)
        .default(0)
        .interact_opt();
    if selection.is_err() {
        return;
    }
    let selection: Option<usize> = selection.unwrap();
    if selection.is_none() {
        return;
    }
    let selection: usize = selection.unwrap();
    let remote: &str = options[selection];

    if remote == "Cancel" {
        return;
    }

    flatpak::install(package, remote, distribution, info);
}

fn post_uninstall(package: &str, distribution: &Distribution, method: &str) {
    let home_dir: Result<String, VarError> = env::var("HOME");
    if home_dir.is_err() {
        return;
    }
    let home_dir: String = home_dir.unwrap();

    match package {
        "code" => {
            if method != "repository" {
                if distribution.repository == "debian" {
                    rust_cli::commands::run("sudo rm /etc/apt/sources.list.d/vscode.list")
                        .expect("remove vscode repo failed");
                }
                if distribution.package_manager == "dnf" {
                    rust_cli::commands::run("sudo dnf config-manager --set-disabled code")
                        .expect("disable code repo failed");
                    rust_cli::commands::run("sudo rm /etc/yum.repos.d/vscode.repo")
                        .expect("remove code repo failed");
                }
            }
            if method == "uninstall" {
                rust_cli::commands::run(
                    format!(
                        "sudo rm -r {}{} {}{}",
                        &home_dir, "/.vscode", &home_dir, "/.config/Code"
                    )
                    .as_str(),
                )
                .expect("remove code files failed");
            }
        }
        "golang" => {
            if method == "uninstall" {
                rust_cli::commands::run(format!("sudo rm -r {}{}", &home_dir, "/.go").as_str())
                    .expect("remove go failed");
            }
        }
        "dotnet-runtime-6" | "dotnet-sdk-6" | "dotnet-runtime-7" | "dotnet-sdk-7" => {
            if method != "repository" {
                if distribution.repository == "debian" {
                    rust_cli::commands::run("sudo rm /etc/apt/sources.list.d/microsoft-prod.list")
                        .expect("remove microsoft repo failed");
                }
            }
            if method == "uninstall" {
                rust_cli::commands::run(format!("sudo rm -r {}{}", &home_dir, "/.dotnet").as_str())
                    .expect("remove dotnet files failed");
            }
        }
        "pycharm" => {
            if method != "repository" {
                if distribution.name == "fedora" {
                    rust_cli::commands::run(
                        "sudo dnf config-manager --set-disabled phracek-PyCharm",
                    )
                    .expect("disable pycharm repo failed");
                }
            }
        }
        "rust" => {
            if method != "other" {
                rust_cli::commands::run(
                    format!("sudo rm -r {}{}", &home_dir, "/.cargo/bin/rustup").as_str(),
                )
                .expect("remove rust failed");
            }
        }
        "vim" => {
            if method != "repository" {
                rust_cli::commands::run(
                    format!(
                        "sudo rm -r {}{} {}{} {}{}",
                        &home_dir, "/.vim", &home_dir, "/.viminfo", &home_dir, "/.vimrc"
                    )
                    .as_str(),
                )
                .expect("remove vim files failed");
            }
        }
        _ => (),
    }
}

fn pre_install(package: &str, distribution: &Distribution, info: &mut Info, method: &str) {
    match package {
        "code" => {
            if method == "repository" {
                if distribution.repository == "debian" {
                    distribution.install("wget", info);
                    distribution.install("gpg", info);
                    rust_cli::commands::run("wget -qO- https://packages.microsoft.com/keys/microsoft.asc | gpg --dearmor > packages.microsoft.gpg").expect("get microsoft key failed");
                    rust_cli::commands::run("sudo install -D -o root -g root -m 644 packages.microsoft.gpg /etc/apt/keyrings/packages.microsoft.gpg").expect("install microsoft key failed");
                    rust_cli::commands::run("sudo sh -c 'echo \"deb [arch=amd64,arm64,armhf signed-by=/etc/apt/keyrings/packages.microsoft.gpg] https://packages.microsoft.com/repos/code stable main\" > /etc/apt/sources.list.d/vscode.list'").expect("adding microsoft repo failed");
                    rust_cli::commands::run("rm -f packages.microsoft.gpg")
                        .expect("remove microsoft key failed");
                }
                if distribution.package_manager == "dnf" {
                    let _ = Command::new("sudo")
                        .arg("rpm")
                        .arg("--import")
                        .arg("https://packages.microsoft.com/keys/microsoft.asc")
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()
                        .expect("import microsoft package keys failed")
                        .wait();

                    let echo_cmd = Command::new("echo")
                        .arg("-e")
                        .arg("[code]\nname=Visual Studio Code\nbaseurl=https://packages.microsoft.com/yumrepos/vscode\nenabled=1\ngpgcheck=1\ngpgkey=https://packages.microsoft.com/keys/microsoft.asc")
                        .stdout(Stdio::piped())
                        .spawn()
                        .unwrap();
                    let _ = Command::new("sudo")
                        .arg("tee")
                        .arg("/etc/yum.repos.d/vscode.repo")
                        .stdin(Stdio::from(echo_cmd.stdout.unwrap()))
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()
                        .expect("add code repo failed")
                        .wait();
                }
            }
        }
        "dotnet-runtime-6" | "dotnet-sdk-6" | "dotnet-runtime-7" | "dotnet-sdk-7" => {
            if method == "repository" {
                if distribution.repository == "debian" {
                    distribution.install("wget", info);
                    rust_cli::commands::run("wget https://packages.microsoft.com/config/debian/12/packages-microsoft-prod.deb -O packages-microsoft-prod.deb").expect("get microsoft deb failed");
                    rust_cli::commands::run("sudo dpkg -i packages-microsoft-prod.deb")
                        .expect("install microsoft deb failed");
                    rust_cli::commands::run("rm packages-microsoft-prod.deb")
                        .expect("remove microsoft deb failed");
                }
            }
        }
        "nodejs" => {
            if method == "repository" {
                if distribution.package_manager == "dnf" {
                    rust_cli::commands::run("sudo dnf module enable nodejs:18 -y")
                        .expect("enable nodejs module failed");
                }
            }
        }
        "pycharm" => {
            if method == "repository" {
                if distribution.repository == "fedora" {
                    rust_cli::commands::run(
                        "sudo dnf config-manager --set-enabled phracek-PyCharm",
                    )
                    .expect("enable pycharm repo failed");
                }
            }
        }
        "rust" => {
            if method == "other" {
                distribution.install("curl", info);
            }
        }
        _ => (),
    }
}

fn post_install(package: &str, distribution: &Distribution, method: &str) {
    let home_dir: Result<String, VarError> = env::var("HOME");
    if home_dir.is_err() {
        return;
    }
    let home_dir: String = home_dir.unwrap();

    match package {
        "code" => {
            if method != "uninstall" {
                let extensions: Vec<&str> = Vec::from(["esbenp.prettier-vscode", "vscodevim.vim"]);
                for ext in extensions {
                    rust_cli::commands::run(format!("code --install-extension {}", ext).as_str())
                        .expect("install code extension failed");
                }

                let _ = fs::write(
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
                );
            }
        }
        "go" => {
            if method != "uninstall" {
                rust_cli::commands::run("go env -w GOPATH=$HOME/.go").expect("set go path failed");
            }
        }
        "intellij" | "pycharm" => {
            let _ = fs::write(
                format!("{}{}", &home_dir, "/.ideavimrc"),
                "sethandler a:ide",
            );
        }
        "rust" => {
            if method == "other" {
                rust_cli::commands::run(
                    format!(
                        "{}{} component add rust-analyzer",
                        home_dir, "/.cargo/bin/rustup"
                    )
                    .as_str(),
                )
                .expect("install rust analyzer failed");
            }
        }
        "snapd" => {
            if method == "repository" {
                distribution.setup_snap();
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
                );

                let _ = fs::write(
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
                );
            }
        }
        _ => (),
    }
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

fn run_package_select(package: &str, distribution: &Distribution, info: &mut Info) {
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

    let selection: std::io::Result<Option<usize>> = Select::new()
        .with_prompt(format!(
            "Package: {} ({})",
            package,
            get_install_method(package, distribution, &info)
        ))
        .items(&options_display)
        .default(0)
        .interact_opt();
    if selection.is_err() {
        return;
    }
    let selection: Option<usize> = selection.unwrap();
    if selection.is_none() {
        return;
    }
    let selection: usize = selection.unwrap();
    let method: &str = options_value[selection];

    if method == "cancel" {
        return;
    }

    if method != "repository" {
        distribution.uninstall(package, info);
    }

    if method != "flatpak" {
        if info.has_flatpak {
            flatpak::uninstall(package, info);
        }
    }

    if method != "snap" {
        if info.has_snap {
            snap::uninstall(package, info);
        }
    }

    if method != "other" {
        other::uninstall(package, info);
    }
    post_uninstall(package, distribution, method);

    pre_install(package, distribution, info, method);
    match method {
        "repository" => distribution.install(package, info),
        "flatpak" => run_flatpak_remote_select(package, distribution, info),
        "snap" => snap::install(package, distribution, info),
        "other" => other::install(package, info),
        _ => (),
    }
    post_install(package, distribution, method);
}

fn run_category_select(
    category: &str,
    start_idx: usize,
    show_all_desktop_environments: bool,
    distribution: &Distribution,
    info: &mut Info,
) {
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
            if !show_all_desktop_environments {
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

    let selection: std::io::Result<Option<usize>> = Select::new()
        .with_prompt(format!("Category: {}", category))
        .items(&options_display)
        .default(start_idx)
        .interact_opt();
    if selection.is_err() {
        return;
    }
    let selection: Option<usize> = selection.unwrap();
    if selection.is_none() {
        return;
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
            );
        }
        _ => {
            run_package_select(options_value[selection], distribution, info);
            run_category_select(
                category,
                selection + 1,
                show_all_desktop_environments,
                distribution,
                info,
            );
        }
    }
}

fn run_install_packages(start_idx: usize, distribution: &Distribution, info: &mut Info) {
    let mut options: Vec<&str> = vec![];
    for category in CATEGORIES {
        options.push(category);
    }
    options.push("Exit");

    let selection: std::io::Result<Option<usize>> = Select::new()
        .with_prompt("Choose a Category")
        .items(&options)
        .default(start_idx)
        .interact_opt();
    if selection.is_err() {
        return;
    }
    let selection: Option<usize> = selection.unwrap();
    if selection.is_none() {
        return;
    }
    let selection: usize = selection.unwrap();

    match options[selection] {
        "Exit" => return,
        _ => run_category_select(options[selection], 0, false, distribution, info),
    }

    run_install_packages(selection + 1, distribution, info);
}

fn run_menu(start_idx: usize, distribution: &Distribution, info: &mut Info) {
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

    let selection: std::io::Result<Option<usize>> = Select::new()
        .items(&options)
        .default(start_idx)
        .interact_opt();
    if selection.is_err() {
        return;
    }
    let selection: Option<usize> = selection.unwrap();
    if selection.is_none() {
        return;
    }
    let selection: usize = selection.unwrap();

    match options[selection] {
        "Repository Setup" => repository_setup(distribution, info),
        "GNOME Setup" => gnome::setup(distribution),
        "KDE Setup" => kde::setup(),
        "Update Packages" => {
            distribution.update();
            if info.has_flatpak {
                flatpak::update();
            }
            if info.has_snap {
                snap::update();
            }
            other::update(info);
        }
        "Auto Remove Packages" => {
            distribution.auto_remove();
            if info.has_flatpak {
                flatpak::auto_remove();
            }
        }
        "Install Packages" => run_install_packages(0, distribution, info),
        "Exit" => return,
        _ => (),
    }

    run_menu(selection + 1, distribution, info);
}

fn main() {
    let mut has_gnome: bool = false;
    match Command::new("gnome-shell").arg("--version").output() {
        Ok(_) => has_gnome = true,
        _ => (),
    }

    let mut has_kde: bool = false;
    match Command::new("plasmashell").arg("--version").output() {
        Ok(_) => has_kde = true,
        _ => (),
    }

    let distribution: Option<Distribution> = distribution::get_distribution();
    if distribution.is_none() {
        println!("Distribution is not recognized");
        return;
    }
    let distribution: Distribution = distribution.unwrap();
    let repository_installed: Vec<String> = distribution.get_installed();

    let mut has_flatpak: bool = false;
    let mut flatpak_installed: Vec<String> = vec![];
    match Command::new("flatpak").arg("--version").output() {
        Ok(_) => {
            has_flatpak = true;
            flatpak_installed = flatpak::get_installed();
        }
        _ => (),
    }

    let mut has_snap: bool = false;
    let mut snap_installed: Vec<String> = vec![];
    match Command::new("snap").arg("--version").output() {
        Ok(_) => {
            has_snap = true;
            snap_installed = snap::get_installed();
        }
        _ => (),
    }

    let other_installed: Vec<String> = other::get_installed();

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
    run_menu(0, &distribution, &mut info);
}
