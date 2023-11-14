use dialoguer::Confirm;
use std::fs;
use std::process::{Command, Stdio};
use std::str::{Split, SplitWhitespace};

extern crate rust_cli;

use crate::helper;
use crate::Info;

#[derive(Debug, PartialEq, Eq)]
pub enum DistributionName {
    Alma,
    Arch,
    CentOS,
    Debian,
    Fedora,
    Mint,
    PopOS,
    SilverBlue,
    Ubuntu,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Repository {
    Arch,
    Debian,
    Fedora,
    RedHat,
    Ubuntu,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PackageManager {
    APT,
    DNF,
    PACMAN,
    RPMOSTree,
}

fn get_package_manager_name(pm: PackageManager) -> &'static str {
    match pm {
        PackageManager::APT => "apt",
        PackageManager::DNF => "dnf",
        PackageManager::PACMAN => "pacman",
        PackageManager::RPMOSTree => "rpm-ostree",
    }
}

pub struct Distribution {
    pub name: DistributionName,
    pub repository: Repository,
    pub package_manager: PackageManager,
}

impl Distribution {
    pub fn setup(&self, info: &mut Info) {
        println!("Setup repository...");

        if self.package_manager == PackageManager::DNF {
            helper::append_to_file_if_not_found(
                &"/etc/dnf/dnf.conf".to_owned(),
                "max_parallel_downloads",
                "max_parallel_downloads=10",
                true,
            );

            if Confirm::new()
                .with_prompt("Do you want to enable EPEL/RPM Fusion Repositories?")
                .interact()
                .expect("confirm failed")
            {
                match self.repository {
                    Repository::Fedora => {
                        self.install("https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-38.noarch.rpm",info);
                    }
                    Repository::RedHat => {
                        self.install("https://dl.fedoraproject.org/pub/epel/epel-release-latest-9.noarch.rpm",info);
                        self.install("https://download1.rpmfusion.org/free/el/rpmfusion-free-release-9.noarch.rpm",info);
                        rust_cli::commands::run("sudo dnf config-manager --set-enabled crb")
                            .expect("enable crb failed");
                    }
                    _ => (),
                }
                if Confirm::new()
                    .with_prompt("Do you want to enable Non-Free EPEL/RPM Fusion Repositories?")
                    .interact()
                    .expect("confirm failed")
                {
                    match self.repository {
                        Repository::Fedora => {
                            self.install("https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-38.noarch.rpm",info);
                        }
                        Repository::RedHat => {
                            self.install("https://download1.rpmfusion.org/nonfree/el/rpmfusion-nonfree-release-9.noarch.rpm",info);
                        }
                        _ => (),
                    }
                }

                self.update();
            }
        }
    }

    fn get_packages<'a>(&self, package: &'a str) -> Option<Vec<&'a str>> {
        match package {
            "0ad" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["0ad"])
            }
            "aisleriot" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["aisleriot"])
            }
            "ark" => Option::from(vec!["ark"]),
            "baobab" => Option::from(vec!["baobab"]),
            "blender" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["blender"])
            }
            "cheese" => Option::from(vec!["cheese"]),
            "chromium" => {
                if self.name == DistributionName::Ubuntu || self.repository == Repository::RedHat {
                    return None;
                }
                return Option::from(vec!["chromium"]);
            }
            "cockpit" => Option::from(vec!["cockpit"]),
            "code" => {
                if self.package_manager == PackageManager::DNF {
                    return Option::from(vec!["code"]);
                }
                if self.package_manager == PackageManager::PACMAN {
                    return Option::from(vec!["code"]);
                }
                if self.repository == Repository::Debian {
                    return Option::from(vec!["code"]);
                }
                None
            }
            "cups" => Option::from(vec!["cups"]),
            "curl" => Option::from(vec!["curl"]),
            "dconf-editor" => Option::from(vec!["dconf-editor"]),
            "deja-dup" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["deja-dup"])
            }
            "discord" => {
                if self.package_manager == PackageManager::PACMAN {
                    return Option::from(vec!["discord"]);
                }
                None
            }
            "dotnet-runtime-8" => Option::from(vec!["dotnet-runtime-8.0"]),
            "dotnet-sdk-8" => Option::from(vec!["dotnet-sdk-8.0"]),
            "elisa" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["elisa"])
            }
            "eog" => Option::from(vec!["eog"]),
            "epiphany" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                if self.package_manager == PackageManager::APT {
                    return Option::from(vec!["epiphany-browser"]);
                }
                return Option::from(vec!["epiphany"]);
            }
            "evince" => Option::from(vec!["evince"]),
            "ffmpeg" => Option::from(vec!["ffmpeg"]),
            "filelight" => Option::from(vec!["filelight"]),
            "firefox" => {
                if self.name == DistributionName::Ubuntu
                    || self.repository == Repository::Debian
                    || self.repository == Repository::RedHat
                {
                    return None;
                }
                return Option::from(vec!["firefox"]);
            }
            "firefox-esr" => {
                if self.repository == Repository::RedHat {
                    return Option::from(vec!["firefox"]);
                }
                if self.repository == Repository::Debian {
                    return Option::from(vec!["firefox-esr"]);
                }
                None
            }
            "flatpak" => Option::from(vec!["flatpak"]),
            "gedit" => Option::from(vec!["gedit"]),
            "gimp" => Option::from(vec!["gimp"]),
            "git" => Option::from(vec!["git"]),
            "golang" => {
                if self.package_manager == PackageManager::APT {
                    return Option::from(vec!["golang"]);
                }
                if self.package_manager == PackageManager::DNF {
                    return Option::from(vec!["golang"]);
                }
                if self.package_manager == PackageManager::PACMAN {
                    return Option::from(vec!["go"]);
                }
                None
            }
            "gparted" => Option::from(vec!["gparted"]),
            "gnome-2048" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["gnome-2048"])
            }
            "gnome-boxes" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["gnome-boxes"])
            }
            "gnome-builder" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["gnome-builder"])
            }
            "gnome-calculator" => Option::from(vec!["gnome-calculator"]),
            "gnome-calendar" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["gnome-calendar"])
            }
            "gnome-chess" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["gnome-chess"])
            }
            "gnome-clocks" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["gnome-clocks"])
            }
            "gnome-connections" => Option::from(vec!["gnome-connections"]),
            "gnome-contacts" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["gnome-contacts"])
            }
            "gnome-disk-utility" => Option::from(vec!["gnome-disk-utility"]),
            "gnome-maps" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["gnome-maps"])
            }
            "gnome-mines" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["gnome-mines"])
            }
            "gnome-music" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["gnome-music"])
            }
            "gnome-passwordsafe" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                if self.repository == Repository::Fedora {
                    return Option::from(vec!["secrets"]);
                }
                Option::from(vec!["gnome-passwordsafe"])
            }
            "gnome-photos" => Option::from(vec!["gnome-photos"]),
            "gnome-shell-extension-manager" => {
                if self.package_manager == PackageManager::APT {
                    return Option::from(vec!["gnome-shell-extension-manager"]);
                }
                None
            }
            "gnome-shell-extensions" => {
                if self.package_manager == PackageManager::DNF {
                    return Option::from(vec!["gnome-extensions-app"]);
                }
                Option::from(vec!["gnome-shell-extensions"])
            }
            "gnome-software" => {
                if self.name == DistributionName::PopOS {
                    return None;
                }
                Option::from(vec!["gnome-software"])
            }
            "gnome-sound-recorder" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["gnome-sound-recorder"])
            }
            "gnome-sudoku" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["gnome-sudoku"])
            }
            "gnome-system-monitor" => Option::from(vec!["gnome-system-monitor"]),
            "gnome-text-editor" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["gnome-text-editor"])
            }
            "gnome-tweaks" => Option::from(vec!["gnome-tweaks"]),
            "gnome-weather" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["gnome-weather"])
            }
            "gnucash" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["gnucash"])
            }
            "gwenview" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["gwenview"])
            }
            "htop" => Option::from(vec!["htop"]),
            "ibus-unikey" => {
                if self.repository == Repository::RedHat {
                    return Option::from(vec!["https://rpmfind.net/linux/fedora/linux/releases/34/Everything/x86_64/os/Packages/i/ibus-unikey-0.6.1-26.20190311git46b5b9e.fc34.x86_64.rpm"]);
                }
                Option::from(vec!["ibus-unikey"])
            }
            "icecat" => {
                if self.repository == Repository::Fedora {
                    return Option::from(vec!["icecat"]);
                }
                None
            }
            "id3v2" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["id3v2"])
            }
            "imagemagick" => {
                if self.package_manager == PackageManager::DNF {
                    return Option::from(vec!["ImageMagick"]);
                }
                Option::from(vec!["imagemagick"])
            }
            "intellij" => {
                if self.package_manager == PackageManager::PACMAN {
                    return Option::from(vec!["intellij-idea-community-edition"]);
                }
                None
            }
            "kate" => Option::from(vec!["kate"]),
            "kcalc" => Option::from(vec!["kcalc"]),
            "kdenlive" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["kdenlive"])
            }
            "kdevelop" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["kdevelop"])
            }
            "kile" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["kile"])
            }
            "kmines" => Option::from(vec!["kmines"]),
            "knights" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["knights"])
            }
            "ksudoku" => Option::from(vec!["ksudoku"]),
            "ksysguard" => Option::from(vec!["ksysguard"]),
            "kwrite" => Option::from(vec!["kwrite"]),
            "latex" => {
                if self.package_manager == PackageManager::APT {
                    return Option::from(vec!["texlive-latex-base", "texlive-latex-extra"]);
                }
                if self.package_manager == PackageManager::DNF {
                    if self.repository == Repository::Fedora {
                        return Option::from(vec![
                            "texlive-latex",
                            "texlive-collection-latexextra",
                        ]);
                    }
                    return Option::from(vec!["texlive-latex"]);
                }
                if self.package_manager == PackageManager::PACMAN {
                    return Option::from(vec!["texlive-core", "texlive-latexextra"]);
                }
                None
            }
            "libreoffice" => {
                if self.package_manager == PackageManager::PACMAN {
                    return Option::from(vec!["libreoffice-fresh"]);
                }
                Option::from(vec![
                    "libreoffice-writer",
                    "libreoffice-calc",
                    "libreoffice-impress",
                    "libreoffice-draw",
                    "libreoffice-base",
                ])
            }
            "mariadb" => {
                if self.package_manager == PackageManager::PACMAN {
                    return Option::from(vec!["mariadb"]);
                }
                Option::from(vec!["mariadb-server"])
            }
            "mediawriter" => {
                if self.repository == Repository::Fedora {
                    return Option::from(vec!["mediawriter"]);
                }
                None
            }
            "nano" => Option::from(vec!["nano"]),
            "node" => Option::from(vec!["nodejs", "npm"]),
            "okular" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["okular"])
            }
            "plasma-discover" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                if self.package_manager == PackageManager::PACMAN {
                    return Option::from(vec!["discover"]);
                }
                Option::from(vec!["plasma-discover"])
            }
            "plasma-systemmonitor" => Option::from(vec!["plasma-systemmonitor"]),
            "podman" => Option::from(vec!["podman"]),
            "pycharm" => {
                if self.package_manager == PackageManager::PACMAN {
                    return Option::from(vec!["pycharm-community-edition"]);
                }
                if self.repository == Repository::Fedora {
                    return Option::from(vec!["pycharm-community"]);
                }
                None
            }
            "qtile" => {
                if self.package_manager == PackageManager::PACMAN {
                    return Option::from(vec![
                        "qtile",
                        "alacritty",
                        "rofi",
                        "numlockx",
                        "playerctl",
                    ]);
                }
                None
            }
            "quadrapassel" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["quadrapassel"])
            }
            "rhythmbox" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["rhythmbox"])
            }
            "rust" => {
                if self.package_manager == PackageManager::APT {
                    return Option::from(vec!["rustc", "rustfmt", "cargo"]);
                }
                if self.package_manager == PackageManager::DNF {
                    if self.repository == Repository::Fedora {
                        return Option::from(vec!["rust", "rustfmt", "cargo", "rust-analyzer"]);
                    }
                    return Option::from(vec!["rust", "rustfmt", "cargo"]);
                }
                if self.package_manager == PackageManager::PACMAN {
                    return Option::from(vec!["rustup"]);
                }
                None
            }
            "shotwell" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["shotwell"])
            }
            "simple-scan" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["simple-scan"])
            }
            "snapd" => {
                if self.package_manager == PackageManager::PACMAN {
                    return None;
                }
                Option::from(vec!["snapd"])
            }
            "spectacle" => Option::from(vec!["spectacle"]),
            "ssh" => {
                if self.package_manager == PackageManager::APT {
                    return Option::from(vec!["ssh"]);
                }
                Option::from(vec!["libssh", "openssh"])
            }
            "steam" => {
                if self.package_manager == PackageManager::PACMAN {
                    return Option::from(vec!["steam"]);
                }
                None
            }
            "supertuxkart" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["supertuxkart"])
            }
            "thunderbird" => Option::from(vec!["thunderbird"]),
            "torbrowser-launcher" => Option::from(vec!["torbrowser-launcher"]),
            "totem" => Option::from(vec!["totem"]),
            "transmission-gtk" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["transmission-gtk"])
            }
            "transmission-qt" => {
                if self.repository == Repository::RedHat {
                    return None;
                }
                Option::from(vec!["transmission-qt"])
            }
            "vim" => {
                if self.package_manager == PackageManager::DNF {
                    return Option::from(vec!["vim-enhanced"]);
                }
                Option::from(vec!["vim"])
            }
            "virt-manager" => Option::from(vec!["virt-manager"]),
            "vlc" => Option::from(vec!["vlc"]),
            "xonotic" => {
                if self.package_manager == PackageManager::PACMAN
                    || self.repository == Repository::Fedora
                {
                    return Option::from(vec!["xonotic"]);
                }
                None
            }
            "yt-dlp" => Option::from(vec!["yt-dlp"]),
            x if x.contains("http") => Option::from(vec![package]),
            _ => None,
        }
    }

    pub fn is_available(&self, package: &str) -> bool {
        self.get_packages(package).is_some()
    }

    pub fn is_installed(&self, package: &str, info: &Info) -> bool {
        let packages: Option<Vec<&str>> = self.get_packages(package);
        if packages.is_some() {
            for pkg in packages.unwrap() {
                if info.repository_installed.contains(&pkg.to_owned()) {
                    return true;
                }
            }
        }
        false
    }

    pub fn install(&self, package: &str, info: &mut Info) {
        let packages: Option<Vec<&str>> = self.get_packages(package);
        if packages.is_some() {
            for pkg in packages.unwrap() {
                if info.repository_installed.contains(&pkg.to_owned()) {
                    continue;
                }
                info.repository_installed.push(pkg.to_owned());

                println!("Installing repository {}...", pkg);

                let mut cmd: Command = Command::new("sudo");
                match self.package_manager {
                    PackageManager::APT => {
                        cmd.arg(get_package_manager_name(PackageManager::APT));
                        cmd.arg("install");
                        cmd.arg(pkg);
                        cmd.arg("-Vy");
                    }
                    PackageManager::DNF => {
                        cmd.arg(get_package_manager_name(PackageManager::DNF));
                        cmd.arg("install");
                        cmd.arg(pkg);
                        cmd.arg("-y");
                    }
                    PackageManager::PACMAN => {
                        cmd.arg(get_package_manager_name(PackageManager::PACMAN));
                        cmd.arg("-S");
                        cmd.arg(pkg);
                        cmd.arg("--noconfirm");
                        cmd.arg("--needed");
                    }
                    PackageManager::RPMOSTree => {
                        cmd.arg(get_package_manager_name(PackageManager::RPMOSTree));
                        cmd.arg("install");
                        cmd.arg(pkg);
                        cmd.arg("-y");
                    }
                }
                let _ = cmd
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("install repository failed")
                    .wait();
            }
        }
    }

    pub fn uninstall(&self, package: &str, info: &mut Info) {
        let packages: Option<Vec<&str>> = self.get_packages(package);
        if packages.is_some() {
            for pkg in packages.unwrap() {
                if !info.repository_installed.contains(&pkg.to_owned()) {
                    continue;
                }
                let index: Option<usize> = info.repository_installed.iter().position(|x| *x == pkg);
                if index.is_some() {
                    info.repository_installed.remove(index.unwrap());
                }

                println!("Uninstalling repository {}...", pkg);

                let mut cmd: Command = Command::new("sudo");
                match self.package_manager {
                    PackageManager::APT => {
                        cmd.arg(get_package_manager_name(PackageManager::APT));
                        cmd.arg("remove");
                        cmd.arg(pkg);
                        cmd.arg("-Vy");
                    }
                    PackageManager::DNF => {
                        cmd.arg(get_package_manager_name(PackageManager::DNF));
                        cmd.arg("remove");
                        cmd.arg(pkg);
                        cmd.arg("-y");
                    }
                    PackageManager::PACMAN => {
                        cmd.arg(get_package_manager_name(PackageManager::PACMAN));
                        cmd.arg("-Rsun");
                        cmd.arg(pkg);
                        cmd.arg("--noconfirm");
                    }
                    PackageManager::RPMOSTree => {
                        cmd.arg(get_package_manager_name(PackageManager::RPMOSTree));
                        cmd.arg("uninstall");
                        cmd.arg(pkg);
                        cmd.arg("-y");
                    }
                }
                let _ = cmd
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("uninstall repository failed")
                    .wait();
            }
        }
    }

    pub fn update(&self) {
        println!("Updating repository...");

        match self.package_manager {
            PackageManager::APT => {
                rust_cli::commands::run("sudo apt update").expect("apt update failed");
                rust_cli::commands::run("sudo apt upgrade -Vy").expect("apt upgrade failed");
            }
            PackageManager::DNF => {
                rust_cli::commands::run("sudo dnf upgrade --refresh -y")
                    .expect("dnf upgrade failed");
            }
            PackageManager::PACMAN => {
                rust_cli::commands::run("sudo pacman -Syu --noconfirm")
                    .expect("pacman update failed");
            }
            PackageManager::RPMOSTree => {
                rust_cli::commands::run("rpm-ostree upgrade").expect("rpm-ostree upgrade failed");
            }
        }
    }

    pub fn auto_remove(&self) {
        println!("Auto removing repository...");

        match self.package_manager {
            PackageManager::APT => {
                rust_cli::commands::run("sudo apt autoremove -Vy").expect("apt auto remove failed");
            }
            PackageManager::DNF => {
                rust_cli::commands::run("sudo dnf autoremove -y").expect("dnf auto remove failed");
            }
            PackageManager::PACMAN => {
                let mut list_cmd: Command =
                    Command::new(get_package_manager_name(PackageManager::PACMAN));
                list_cmd.arg("-Qdtq");
                let orphans: Option<String> = helper::get_command_output(list_cmd);
                if orphans.is_some() {
                    let mut rm_cmd: Command = Command::new("sudo");
                    rm_cmd.arg(get_package_manager_name(PackageManager::PACMAN));
                    rm_cmd.arg("-Rsun");
                    let orphans: String = orphans.unwrap();
                    for line in orphans.split("\n") {
                        if line.is_empty() {
                            continue;
                        }
                        rm_cmd.arg(line);
                    }
                    if rm_cmd.get_args().len() > 2 {
                        rm_cmd.arg("--noconfirm");
                        let _ = rm_cmd
                            .stdout(Stdio::inherit())
                            .stderr(Stdio::inherit())
                            .spawn()
                            .expect("pacman auto remove failed")
                            .wait();
                    }
                }
            }
            _ => (),
        }
    }

    pub fn setup_snap(&self) {
        if self.package_manager == PackageManager::DNF {
            rust_cli::commands::run("sudo systemctl enable --now snapd.socket")
                .expect("start snap failed");
            rust_cli::commands::run("sudo ln -s /var/lib/snapd/snap /snap")
                .expect("link snap failed");
        }
    }

    pub fn get_installed(&self) -> Vec<String> {
        let mut packages: Vec<String> = vec![];

        let mut cmd: Command = match self.package_manager {
            PackageManager::APT => Command::new(get_package_manager_name(PackageManager::APT)),
            PackageManager::DNF => Command::new(get_package_manager_name(PackageManager::DNF)),
            PackageManager::PACMAN => {
                Command::new(get_package_manager_name(PackageManager::PACMAN))
            }
            PackageManager::RPMOSTree => Command::new("rpm"),
        };

        match self.package_manager {
            PackageManager::APT => {
                cmd.arg("list");
                cmd.arg("--installed");
            }
            PackageManager::DNF => {
                cmd.arg("list");
                cmd.arg("installed");
            }
            PackageManager::PACMAN => {
                cmd.arg("-Q");
            }
            PackageManager::RPMOSTree => {
                cmd.arg("-qa");
            }
        }

        let output: Option<String> = helper::get_command_output(cmd);
        if output.is_some() {
            let output: String = output.unwrap();
            for line in output.split("\n") {
                if line.is_empty() {
                    continue;
                }
                let mut package: String = String::new();
                match self.package_manager {
                    PackageManager::APT => {
                        let columns: Split<&str> = line.split("/");
                        let columns: Vec<&str> = columns.collect::<Vec<&str>>();
                        package = columns[0].to_owned();
                    }
                    PackageManager::DNF => {
                        let columns: SplitWhitespace = line.split_whitespace();
                        let columns: Vec<&str> = columns.collect::<Vec<&str>>();
                        let full_package: String = columns[0].to_owned();
                        let full_package_split: Option<(&str, &str)> =
                            full_package.rsplit_once(".");
                        if full_package_split.is_some() {
                            package = full_package_split.unwrap().0.to_owned();
                        }
                    }
                    PackageManager::PACMAN => {
                        let columns: Split<&str> = line.split(" ");
                        let columns: Vec<&str> = columns.collect::<Vec<&str>>();
                        package = columns[0].to_owned();
                    }
                    PackageManager::RPMOSTree => {
                        let first_numeric: Option<usize> = line.find(|c: char| c.is_numeric());
                        if first_numeric.is_some() {
                            let first_numeric: usize = first_numeric.unwrap();
                            if first_numeric > 0 {
                                let prev_char: Option<char> = line.chars().nth(first_numeric - 1);
                                if prev_char.is_some() {
                                    if prev_char.unwrap() == '-' {
                                        package = line.chars().take(first_numeric - 1).collect();
                                    }
                                }
                            }
                        }
                    }
                }
                if !package.is_empty() {
                    packages.push(package);
                }
            }
        }

        return packages;
    }
}

pub fn get_distribution() -> Option<Distribution> {
    let os_release: String =
        fs::read_to_string("/etc/os-release").expect("failed to read os release");
    match &os_release {
        x if x.contains("Arch") => Option::from(Distribution {
            name: DistributionName::Arch,
            repository: Repository::Arch,
            package_manager: PackageManager::PACMAN,
        }),
        x if x.contains("Alma") => Option::from(Distribution {
            name: DistributionName::Alma,
            repository: Repository::RedHat,
            package_manager: PackageManager::DNF,
        }),
        x if x.contains("CentOS") => Option::from(Distribution {
            name: DistributionName::CentOS,
            repository: Repository::RedHat,
            package_manager: PackageManager::DNF,
        }),
        x if x.contains("Debian") => Option::from(Distribution {
            name: DistributionName::Debian,
            repository: Repository::Debian,
            package_manager: PackageManager::APT,
        }),
        x if x.contains("Silverblue") => Option::from(Distribution {
            name: DistributionName::SilverBlue,
            repository: Repository::Fedora,
            package_manager: PackageManager::RPMOSTree,
        }),
        x if x.contains("Fedora") => Option::from(Distribution {
            name: DistributionName::Fedora,
            repository: Repository::Fedora,
            package_manager: PackageManager::DNF,
        }),
        x if x.contains("Mint") => Option::from(Distribution {
            name: DistributionName::Mint,
            repository: Repository::Ubuntu,
            package_manager: PackageManager::APT,
        }),
        x if x.contains("Pop!_OS") => Option::from(Distribution {
            name: DistributionName::PopOS,
            repository: Repository::Ubuntu,
            package_manager: PackageManager::APT,
        }),
        x if x.contains("Ubuntu") => Option::from(Distribution {
            name: DistributionName::Ubuntu,
            repository: Repository::Ubuntu,
            package_manager: PackageManager::APT,
        }),
        _ => None,
    }
}
