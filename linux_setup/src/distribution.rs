use dialoguer::Confirm;
use std::fs;
use std::process::{Command, Stdio};
use std::str::Split;

use crate::flatpak;
use crate::helper;
use crate::Info;

pub struct Distribution {
    pub name: &'static str,
    pub repository: &'static str,
    pub package_manager: &'static str,
}

impl Distribution {
    pub fn setup(&self, info: &mut Info) {
        println!("Setup repository...");

        if self.package_manager == "dnf" {
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
                    "fedora" => {
                        self.install("https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-37.noarch.rpm",info);
                    }
                    "redhat" => {
                        self.install("https://dl.fedoraproject.org/pub/epel/epel-release-latest-9.noarch.rpm",info);
                        self.install("https://download1.rpmfusion.org/free/el/rpmfusion-free-release-9.noarch.rpm",info);
                        let _ = Command::new("sudo")
                            .arg("dnf")
                            .arg("config-manager")
                            .arg("--set-enabled")
                            .arg("crb")
                            .stdout(Stdio::inherit())
                            .stderr(Stdio::inherit())
                            .spawn()
                            .expect("enable crb failed")
                            .wait();
                    }
                    _ => (),
                }
                if Confirm::new()
                    .with_prompt("Do you want to enable Non-Free EPEL/RPM Fusion Repositories?")
                    .interact()
                    .expect("confirm failed")
                {
                    match self.repository {
                        "fedora" => {
                            self.install("https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-37.noarch.rpm",info);
                        }
                        "redhat" => {
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
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["0ad"])
            }
            "aisleriot" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["aisleriot"])
            }
            "ark" => Option::from(vec!["ark"]),
            "baobab" => Option::from(vec!["baobab"]),
            "blender" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["blender"])
            }
            "cheese" => Option::from(vec!["cheese"]),
            "chromium" => {
                if self.name == "ubuntu" || self.repository == "redhat" {
                    return None;
                }
                return Option::from(vec!["chromium"]);
            }
            "cockpit" => Option::from(vec!["cockpit"]),
            "codium" => {
                if self.package_manager == "pacman" {
                    return Option::from(vec!["code"]);
                }
                None
            }
            "cups" => Option::from(vec!["cups"]),
            "curl" => Option::from(vec!["curl"]),
            "dconf-editor" => Option::from(vec!["dconf-editor"]),
            "deja-dup" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["deja-dup"])
            }
            "discord" => {
                if self.package_manager == "pacman" {
                    return Option::from(vec!["discord"]);
                }
                None
            }
            "elisa" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["elisa"])
            }
            "eog" => Option::from(vec!["eog"]),
            "epiphany" => {
                if self.repository == "redhat" {
                    return None;
                }
                if self.package_manager == "apt" {
                    return Option::from(vec!["epiphany-browser"]);
                }
                return Option::from(vec!["epiphany"]);
            }
            "evince" => Option::from(vec!["evince"]),
            "ffmpeg" => Option::from(vec!["ffmpeg"]),
            "filelight" => Option::from(vec!["filelight"]),
            "firefox" => {
                if self.name == "ubuntu"
                    || self.repository == "debian"
                    || self.repository == "redhat"
                {
                    return None;
                }
                return Option::from(vec!["firefox"]);
            }
            "firefox-esr" => {
                if self.repository == "redhat" {
                    return Option::from(vec!["firefox"]);
                }
                if self.repository == "debian" {
                    return Option::from(vec!["firefox-esr"]);
                }
                None
            }
            "flatpak" => Option::from(vec!["flatpak"]),
            "gedit" => Option::from(vec!["gedit"]),
            "gimp" => Option::from(vec!["gimp"]),
            "git" => Option::from(vec!["git"]),
            "gnome-2048" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["gnome-2048"])
            }
            "gnome-books" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["gnome-books"])
            }
            "gnome-boxes" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["gnome-boxes"])
            }
            "gnome-builder" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["gnome-builder"])
            }
            "gnome-calculator" => Option::from(vec!["gnome-calculator"]),
            "gnome-calendar" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["gnome-calendar"])
            }
            "gnome-chess" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["gnome-chess"])
            }
            "gnome-clocks" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["gnome-clocks"])
            }
            "gnome-connections" => {
                if self.repository == "debian" {
                    return None;
                }
                Option::from(vec!["gnome-connections"])
            }
            "gnome-contacts" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["gnome-contacts"])
            }
            "gnome-disk-utility" => Option::from(vec!["gnome-disk-utility"]),
            "gnome-maps" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["gnome-maps"])
            }
            "gnome-mines" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["gnome-mines"])
            }
            "gnome-music" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["gnome-music"])
            }
            "gnome-passwordsafe" => {
                if self.repository == "redhat" {
                    return None;
                }
                if self.repository == "fedora" {
                    return Option::from(vec!["secrets"]);
                }
                Option::from(vec!["gnome-passwordsafe"])
            }
            "gnome-photos" => Option::from(vec!["gnome-photos"]),
            "gnome-shell-extension-manager" => {
                if self.name == "ubuntu" {
                    return Option::from(vec!["gnome-shell-extension-manager"]);
                }
                None
            }
            "gnome-shell-extensions" => {
                if self.package_manager == "dnf" {
                    return Option::from(vec!["gnome-extensions-app"]);
                }
                Option::from(vec!["gnome-shell-extensions"])
            }
            "gnome-software" => {
                if self.name == "pop" {
                    return None;
                }
                Option::from(vec!["gnome-software"])
            }
            "gnome-sound-recorder" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["gnome-sound-recorder"])
            }
            "gnome-sudoku" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["gnome-sudoku"])
            }
            "gnome-system-monitor" => Option::from(vec!["gnome-system-monitor"]),
            "gnome-text-editor" => {
                if self.repository == "redhat" || self.repository == "debian" {
                    return None;
                }
                Option::from(vec!["gnome-text-editor"])
            }
            "gnome-tweaks" => Option::from(vec!["gnome-tweaks"]),
            "gnome-weather" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["gnome-weather"])
            }
            "gnucash" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["gnucash"])
            }
            "gwenview" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["gwenview"])
            }
            "htop" => Option::from(vec!["htop"]),
            "ibus-unikey" => {
                if self.repository == "redhat" {
                    return Option::from(vec!["https://rpmfind.net/linux/fedora/linux/releases/34/Everything/x86_64/os/Packages/i/ibus-unikey-0.6.1-26.20190311git46b5b9e.fc34.x86_64.rpm"]);
                }
                Option::from(vec!["ibus-unikey"])
            }
            "icecat" => {
                if self.repository == "fedora" {
                    return Option::from(vec!["icecat"]);
                }
                None
            }
            "id3v2" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["id3v2"])
            }
            "imagemagick" => {
                if self.package_manager == "dnf" {
                    return Option::from(vec!["ImageMagick"]);
                }
                Option::from(vec!["imagemagick"])
            }
            "intellij" => {
                if self.package_manager == "pacman" {
                    return Option::from(vec!["intellij-idea-community-edition"]);
                }
                None
            }
            "kate" => Option::from(vec!["kate"]),
            "kcalc" => Option::from(vec!["kcalc"]),
            "kdenlive" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["kdenlive"])
            }
            "kdevelop" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["kdevelop"])
            }
            "kile" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["kile"])
            }
            "kmines" => Option::from(vec!["kmines"]),
            "knights" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["knights"])
            }
            "ksudoku" => Option::from(vec!["ksudoku"]),
            "ksysguard" => Option::from(vec!["ksysguard"]),
            "kwrite" => Option::from(vec!["kwrite"]),
            "latex" => {
                if self.package_manager == "apt" {
                    return Option::from(vec!["texlive-latex-base", "texlive-latex-extra"]);
                }
                if self.package_manager == "dnf" {
                    if self.repository == "fedora" {
                        return Option::from(vec![
                            "texlive-latex",
                            "texlive-collection-latexextra",
                        ]);
                    }
                    return Option::from(vec!["texlive-latex"]);
                }
                if self.package_manager == "pacman" {
                    return Option::from(vec!["texlive-core", "texlive-latexextra"]);
                }
                None
            }
            "libreoffice" => {
                if self.package_manager == "pacman" {
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
                if self.package_manager == "pacman" {
                    return Option::from(vec!["mariadb"]);
                }
                Option::from(vec!["mariadb-server"])
            }
            "mediawriter" => {
                if self.repository == "fedora" {
                    return Option::from(vec!["mediawriter"]);
                }
                None
            }
            "nano" => Option::from(vec!["nano"]),
            "ncdu" => Option::from(vec!["ncdu"]),
            "node" => Option::from(vec!["nodejs", "npm"]),
            "okular" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["okular"])
            }
            "plasma-discover" => {
                if self.repository == "redhat" {
                    return None;
                }
                if self.package_manager == "pacman" {
                    return Option::from(vec!["discover"]);
                }
                Option::from(vec!["plasma-discover"])
            }
            "plasma-systemmonitor" => Option::from(vec!["plasma-systemmonitor"]),
            "podman" => Option::from(vec!["podman"]),
            "pycharm" => {
                if self.package_manager == "pacman" {
                    return Option::from(vec!["pycharm-community-edition"]);
                }
                None
            }
            "qtile" => {
                if self.package_manager == "pacman" {
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
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["quadrapassel"])
            }
            "rhythmbox" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["rhythmbox"])
            }
            "rust" => {
                if self.package_manager == "apt" {
                    if self.repository == "ubuntu" {
                        return Option::from(vec!["rustc", "rustfmt", "cargo"]);
                    }
                    if self.repository == "debian" {
                        return Option::from(vec!["rustc", "cargo"]);
                    }
                }
                if self.package_manager == "dnf" {
                    return Option::from(vec!["rust", "rustfmt", "cargo"]);
                }
                if self.package_manager == "pacman" {
                    return Option::from(vec!["rustup"]);
                }
                None
            }
            "shotwell" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["shotwell"])
            }
            "simple-scan" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["simple-scan"])
            }
            "snapd" => Option::from(vec!["snapd"]),
            "spectacle" => Option::from(vec!["spectacle"]),
            "ssh" => {
                if self.package_manager == "apt" {
                    return Option::from(vec!["ssh"]);
                }
                Option::from(vec!["libssh", "openssh"])
            }
            "steam" => {
                if self.package_manager == "pacman" {
                    return Option::from(vec!["steam"]);
                }
                None
            }
            "supertuxkart" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["supertuxkart"])
            }
            "thunderbird" => Option::from(vec!["thunderbird"]),
            "torbrowser-launcher" => Option::from(vec!["torbrowser-launcher"]),
            "totem" => Option::from(vec!["totem"]),
            "transmission-gtk" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["transmission-gtk"])
            }
            "transmission-qt" => {
                if self.repository == "redhat" {
                    return None;
                }
                Option::from(vec!["transmission-qt"])
            }
            "vim" => {
                if self.package_manager == "dnf" {
                    return Option::from(vec!["vim-enhanced"]);
                }
                Option::from(vec!["vim"])
            }
            "virt-manager" => Option::from(vec!["virt-manager"]),
            "vlc" => Option::from(vec!["vlc"]),
            "xonotic" => {
                if self.package_manager == "pacman" || self.repository == "fedora" {
                    return Option::from(vec!["xonotic"]);
                }
                None
            }
            "yt-dlp" => {
                if self.repository == "debian" {
                    return None;
                }
                Option::from(vec!["yt-dlp"])
            }
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
                    "apt" => {
                        cmd.arg("apt");
                        cmd.arg("install");
                        cmd.arg(pkg);
                        cmd.arg("-Vy");
                    }
                    "dnf" => {
                        cmd.arg("dnf");
                        cmd.arg("install");
                        cmd.arg(pkg);
                        cmd.arg("-y");
                    }
                    "pacman" => {
                        cmd.arg("pacman");
                        cmd.arg("-S");
                        cmd.arg(pkg);
                        cmd.arg("--noconfirm");
                        cmd.arg("--needed");
                    }
                    _ => continue,
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
                    "apt" => {
                        cmd.arg("apt");
                        cmd.arg("remove");
                        cmd.arg(pkg);
                        cmd.arg("-Vy");
                    }
                    "dnf" => {
                        cmd.arg("dnf");
                        cmd.arg("remove");
                        cmd.arg(pkg);
                        cmd.arg("-y");
                    }
                    "pacman" => {
                        cmd.arg("pacman");
                        cmd.arg("-Rsun");
                        cmd.arg(pkg);
                        cmd.arg("--noconfirm");
                    }
                    _ => continue,
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
            "apt" => {
                let _ = Command::new("sudo")
                    .arg("apt")
                    .arg("update")
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("apt update failed")
                    .wait();

                let _ = Command::new("sudo")
                    .arg("apt")
                    .arg("upgrade")
                    .arg("-Vy")
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("apt upgrade failed")
                    .wait();
            }
            "dnf" => {
                let _ = Command::new("sudo")
                    .arg("dnf")
                    .arg("upgrade")
                    .arg("--refresh")
                    .arg("-y")
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("dnf upgrade failed")
                    .wait();
            }
            "pacman" => {
                let _ = Command::new("sudo")
                    .arg("pacman")
                    .arg("-Syu")
                    .arg("--noconfirm")
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("pacman update failed")
                    .wait();
            }
            _ => (),
        }
    }

    pub fn auto_remove(&self) {
        println!("Auto removing repository...");

        match self.package_manager {
            "apt" => {
                let _ = Command::new("sudo")
                    .arg("apt")
                    .arg("autoremove")
                    .arg("-Vy")
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("apt auto remove failed")
                    .wait();
            }
            "dnf" => {
                let _ = Command::new("sudo")
                    .arg("dnf")
                    .arg("autoremove")
                    .arg("-y")
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("dnf auto remove failed")
                    .wait();
            }
            "pacman" => {
                let mut list_cmd: Command = Command::new("pacman");
                list_cmd.arg("-Qdtq");
                let orphans: Option<String> = helper::get_command_output(list_cmd);
                if orphans.is_some() {
                    let mut rm_cmd: Command = Command::new("sudo");
                    rm_cmd.arg("pacman");
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

    pub fn install_flatpak(&self, info: &mut Info) {
        if !info.has_flatpak {
            self.install("flatpak", info);
            flatpak::setup();
        }
    }

    pub fn install_snap(&self, info: &mut Info) {
        if !info.has_snap {
            self.install("snapd", info);
            if self.package_manager == "dnf" {
                let _ = Command::new("sudo")
                    .arg("systemctl")
                    .arg("enable")
                    .arg("--now")
                    .arg("snapd.socket")
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("start snap failed")
                    .wait();

                let _ = Command::new("sudo")
                    .arg("ln")
                    .arg("-s")
                    .arg("/var/lib/snapd/snap")
                    .arg("/snap")
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("link snap failed")
                    .wait();
            }
        }
    }

    pub fn get_installed(&self) -> Vec<String> {
        let mut packages: Vec<String> = vec![];

        let mut cmd: Command = Command::new("");
        match self.package_manager {
            "apt" => {
                cmd = Command::new("apt");
                cmd.arg("list");
                cmd.arg("--installed");
            }
            "dnf" => {
                cmd = Command::new("dnf");
                cmd.arg("list");
                cmd.arg("installed");
            }
            "pacman" => {
                cmd = Command::new("pacman");
                cmd.arg("-Q");
            }
            _ => (),
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
                    "apt" => {
                        let columns: Split<&str> = line.split("/");
                        let columns: Vec<&str> = columns.collect::<Vec<&str>>();
                        package = columns[0].to_owned();
                    }
                    "dnf" => {
                        let columns: Split<&str> = line.split(".");
                        let columns: Vec<&str> = columns.collect::<Vec<&str>>();
                        package = columns[0].to_owned();
                    }
                    "pacman" => {
                        let columns: Split<&str> = line.split(" ");
                        let columns: Vec<&str> = columns.collect::<Vec<&str>>();
                        package = columns[0].to_owned();
                    }
                    _ => (),
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
            name: "arch",
            repository: "arch",
            package_manager: "pacman",
        }),
        x if x.contains("Alma") => Option::from(Distribution {
            name: "alma",
            repository: "redhat",
            package_manager: "dnf",
        }),
        x if x.contains("CentOS") => Option::from(Distribution {
            name: "centos",
            repository: "redhat",
            package_manager: "dnf",
        }),
        x if x.contains("Debian") => Option::from(Distribution {
            name: "debian",
            repository: "debian",
            package_manager: "apt",
        }),
        x if x.contains("Fedora") => Option::from(Distribution {
            name: "fedora",
            repository: "fedora",
            package_manager: "dnf",
        }),
        x if x.contains("Mint") => Option::from(Distribution {
            name: "mint",
            repository: "ubuntu",
            package_manager: "apt",
        }),
        x if x.contains("Pop!_OS") => Option::from(Distribution {
            name: "pop",
            repository: "ubuntu",
            package_manager: "apt",
        }),
        x if x.contains("Ubuntu") => Option::from(Distribution {
            name: "ubuntu",
            repository: "ubuntu",
            package_manager: "apt",
        }),
        _ => None,
    }
}