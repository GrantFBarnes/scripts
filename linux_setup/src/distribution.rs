use rust_cli::commands::Operation;
use rust_cli::prompts::confirm::Confirm;

use std::collections::HashSet;
use std::fs;
use std::io;
use std::process::{Command, Stdio};
use std::str::{Split, SplitWhitespace};

use crate::flatpak;
use crate::helper;
use crate::other;
use crate::package::Package;
use crate::snap;

#[derive(PartialEq, Eq, Hash)]
pub enum DesktopEnvironment {
    Gnome,
    KDE,
}

#[derive(Debug, PartialEq, Eq, Hash)]
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

pub struct Distribution {
    pub repository: Repository,
    pub package_manager: PackageManager,
    pub desktop_environments: HashSet<DesktopEnvironment>,
    pub packages: HashSet<String>,
    pub flatpaks: HashSet<String>,
    pub snaps: HashSet<String>,
    pub others: HashSet<String>,
}

impl Distribution {
    pub fn new() -> Result<Self, io::Error> {
        let mut distribution = Distribution {
            repository: Repository::Arch,
            package_manager: PackageManager::PACMAN,
            desktop_environments: HashSet::new(),
            packages: HashSet::new(),
            flatpaks: HashSet::new(),
            snaps: HashSet::new(),
            others: HashSet::new(),
        };

        match fs::read_to_string("/etc/os-release")? {
            x if x.contains("Arch") => {
                distribution.repository = Repository::Arch;
                distribution.package_manager = PackageManager::PACMAN;
            }
            x if x.contains("Alma") => {
                distribution.repository = Repository::RedHat;
                distribution.package_manager = PackageManager::DNF;
            }
            x if x.contains("CentOS") => {
                distribution.repository = Repository::RedHat;
                distribution.package_manager = PackageManager::DNF;
            }
            x if x.contains("Debian") => {
                distribution.repository = Repository::Debian;
                distribution.package_manager = PackageManager::APT;
            }
            x if x.contains("Silverblue") => {
                distribution.repository = Repository::Fedora;
                distribution.package_manager = PackageManager::RPMOSTree;
            }
            x if x.contains("Fedora") => {
                distribution.repository = Repository::Fedora;
                distribution.package_manager = PackageManager::DNF;
            }
            x if x.contains("Mint") => {
                distribution.repository = Repository::Ubuntu;
                distribution.package_manager = PackageManager::APT;
            }
            x if x.contains("Ubuntu") => {
                distribution.repository = Repository::Ubuntu;
                distribution.package_manager = PackageManager::APT;
            }
            _ => return Err(io::Error::other("distribution not found")),
        };

        if Operation::new("gnome-shell").exists()? {
            distribution
                .desktop_environments
                .insert(DesktopEnvironment::Gnome);
        }

        if Operation::new("plasmashell").exists()? {
            distribution
                .desktop_environments
                .insert(DesktopEnvironment::KDE);
        }

        distribution.packages = distribution.get_installed()?;

        if distribution.packages.contains("flatpak") {
            distribution.flatpaks = flatpak::get_installed()?;
        }

        if distribution.packages.contains("snapd") {
            distribution.snaps = snap::get_installed()?;
        }

        distribution.others = other::get_installed()?;

        Ok(distribution)
    }

    fn get_installed(&self) -> Result<HashSet<String>, io::Error> {
        let mut packages: HashSet<String> = HashSet::new();

        let output: String = match self.package_manager {
            PackageManager::APT => Operation::new("apt list --installed").output()?,
            PackageManager::DNF => Operation::new("dnf list installed").output()?,
            PackageManager::PACMAN => Operation::new("pacman -Q").output()?,
            PackageManager::RPMOSTree => Operation::new("rpm -qa").output()?,
        };

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
                    let full_package_split: Option<(&str, &str)> = full_package.rsplit_once(".");
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
                packages.insert(package);
            }
        }

        Ok(packages)
    }

    pub fn setup(&mut self) -> Result<(), io::Error> {
        println!("Setup repository...");

        if self.package_manager == PackageManager::DNF {
            helper::append_to_file_if_not_found(
                &"/etc/dnf/dnf.conf".to_owned(),
                "max_parallel_downloads",
                "max_parallel_downloads=10",
                true,
            )?;

            if Confirm::new("Do you want to enable EPEL/RPM Fusion Repositories?")
                .default_no(true)
                .run()?
            {
                match self.repository {
                    Repository::Fedora => {
                        self.install_package("https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-38.noarch.rpm")?;
                    }
                    Repository::RedHat => {
                        self.install_package("https://dl.fedoraproject.org/pub/epel/epel-release-latest-9.noarch.rpm")?;
                        self.install_package("https://download1.rpmfusion.org/free/el/rpmfusion-free-release-9.noarch.rpm")?;
                        Operation::new("sudo dnf config-manager --set-enabled crb")
                            .hide_output(true)
                            .run()?;
                    }
                    _ => (),
                }
                if Confirm::new("Do you want to enable Non-Free EPEL/RPM Fusion Repositories?")
                    .default_no(true)
                    .run()?
                {
                    match self.repository {
                        Repository::Fedora => {
                            self.install_package("https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-38.noarch.rpm")?;
                        }
                        Repository::RedHat => {
                            self.install_package("https://download1.rpmfusion.org/nonfree/el/rpmfusion-nonfree-release-9.noarch.rpm")?;
                        }
                        _ => (),
                    }
                }

                self.update()?;
            }
        }

        Ok(())
    }

    pub fn is_available(&self, package: &Package) -> bool {
        package.repository.contains_key(&self.repository)
    }

    pub fn is_installed(&self, package: &Package) -> bool {
        if let Some(packages) = package.repository.get(&self.repository) {
            for pkg in packages {
                if self.packages.contains(&pkg.to_string()) {
                    return true;
                }
            }
        }
        false
    }

    pub fn install(&mut self, package: &Package) -> Result<(), io::Error> {
        if let Some(packages) = package.repository.get(&self.repository) {
            for pkg in packages {
                self.install_package(pkg)?;
            }
        }
        Ok(())
    }

    pub fn install_package(&mut self, package: &str) -> Result<(), io::Error> {
        if !self.packages.contains(&package.to_string()) {
            self.packages.insert(package.to_string());

            println!("Installing repository {}...", package);

            match self.package_manager {
                PackageManager::APT => {
                    Operation::new(format!("sudo apt install {} -Vy", package)).run()?;
                }
                PackageManager::DNF => {
                    Operation::new(format!("sudo dnf install {} -y", package)).run()?;
                }
                PackageManager::PACMAN => {
                    Operation::new(format!("sudo pacman -S {} --noconfirm --needed", package))
                        .run()?;
                }
                PackageManager::RPMOSTree => {
                    Operation::new(format!("sudo rpm-ostree install {} -y", package)).run()?;
                }
            }
        }
        Ok(())
    }

    pub fn uninstall(&mut self, package: &Package) -> Result<(), io::Error> {
        if let Some(packages) = package.repository.get(&self.repository) {
            for pkg in packages {
                if !self.packages.contains(&pkg.to_string()) {
                    continue;
                }
                self.packages.remove(&pkg.to_string());

                println!("Uninstalling repository {}...", pkg);

                match self.package_manager {
                    PackageManager::APT => {
                        Operation::new(format!("sudo apt remove {} -Vy", pkg)).run()?;
                    }
                    PackageManager::DNF => {
                        Operation::new(format!("sudo dnf remove {} -y", pkg)).run()?;
                    }
                    PackageManager::PACMAN => {
                        Operation::new(format!("sudo pacman -Rsun {} --noconfirm", pkg)).run()?;
                    }
                    PackageManager::RPMOSTree => {
                        Operation::new(format!("sudo rpm-ostree uninstall {} -y", pkg)).run()?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn update(&self) -> Result<(), io::Error> {
        println!("Updating repository...");

        match self.package_manager {
            PackageManager::APT => {
                Operation::new("sudo apt update").run()?;
                Operation::new("sudo apt upgrade -Vy").run()?;
            }
            PackageManager::DNF => {
                Operation::new("sudo dnf upgrade --refresh -y").run()?;
            }
            PackageManager::PACMAN => {
                Operation::new("sudo pacman -Syu --noconfirm").run()?;
            }
            PackageManager::RPMOSTree => {
                Operation::new("rpm-ostree upgrade").run()?;
            }
        }
        Ok(())
    }

    pub fn auto_remove(&self) -> Result<(), io::Error> {
        println!("Auto removing repository...");

        match self.package_manager {
            PackageManager::APT => {
                Operation::new("sudo apt autoremove -Vy").run()?;
            }
            PackageManager::DNF => {
                Operation::new("sudo dnf autoremove -y").run()?;
            }
            PackageManager::PACMAN => {
                let orphans: String = Operation::new("pacman -Qdtq").output()?;
                let mut rm_cmd: Command = Command::new("sudo");
                rm_cmd.arg("pacman");
                rm_cmd.arg("-Rsun");
                for line in orphans.split("\n") {
                    if line.is_empty() {
                        continue;
                    }
                    rm_cmd.arg(line);
                }
                if rm_cmd.get_args().len() > 2 {
                    rm_cmd.arg("--noconfirm");
                    rm_cmd
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()?
                        .wait()?;
                }
            }
            _ => (),
        }
        Ok(())
    }

    pub fn setup_snap(&self) -> Result<(), io::Error> {
        if self.package_manager == PackageManager::DNF {
            Operation::new("sudo systemctl enable --now snapd.socket").run()?;
            Operation::new("sudo ln -s /var/lib/snapd/snap /snap").run()?;
        }
        Ok(())
    }
}
