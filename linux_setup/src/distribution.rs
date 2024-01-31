use rust_cli::commands::Operation;
use rust_cli::prompts::confirm::Confirm;

use std::fs;
use std::io;
use std::process::{Command, Stdio};
use std::str::{Split, SplitWhitespace};

use crate::helper;
use crate::package::Package;
use crate::Info;

#[derive(PartialEq)]
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
}

impl Distribution {
    pub fn new() -> Result<Self, io::Error> {
        match fs::read_to_string("/etc/os-release")? {
            x if x.contains("Arch") => Ok(Distribution {
                repository: Repository::Arch,
                package_manager: PackageManager::PACMAN,
            }),
            x if x.contains("Alma") => Ok(Distribution {
                repository: Repository::RedHat,
                package_manager: PackageManager::DNF,
            }),
            x if x.contains("CentOS") => Ok(Distribution {
                repository: Repository::RedHat,
                package_manager: PackageManager::DNF,
            }),
            x if x.contains("Debian") => Ok(Distribution {
                repository: Repository::Debian,
                package_manager: PackageManager::APT,
            }),
            x if x.contains("Silverblue") => Ok(Distribution {
                repository: Repository::Fedora,
                package_manager: PackageManager::RPMOSTree,
            }),
            x if x.contains("Fedora") => Ok(Distribution {
                repository: Repository::Fedora,
                package_manager: PackageManager::DNF,
            }),
            x if x.contains("Mint") => Ok(Distribution {
                repository: Repository::Ubuntu,
                package_manager: PackageManager::APT,
            }),
            x if x.contains("Ubuntu") => Ok(Distribution {
                repository: Repository::Ubuntu,
                package_manager: PackageManager::APT,
            }),
            _ => Err(io::Error::other("distribution not found")),
        }
    }

    pub fn setup(&self, info: &mut Info) -> Result<(), io::Error> {
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
                        self.install_package("https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-38.noarch.rpm",info)?;
                    }
                    Repository::RedHat => {
                        self.install_package("https://dl.fedoraproject.org/pub/epel/epel-release-latest-9.noarch.rpm",info)?;
                        self.install_package("https://download1.rpmfusion.org/free/el/rpmfusion-free-release-9.noarch.rpm",info)?;
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
                            self.install_package("https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-38.noarch.rpm",info)?;
                        }
                        Repository::RedHat => {
                            self.install_package("https://download1.rpmfusion.org/nonfree/el/rpmfusion-nonfree-release-9.noarch.rpm",info)?;
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

    pub fn is_installed(&self, package: &Package, info: &Info) -> bool {
        if let Some(packages) = package.repository.get(&self.repository) {
            for pkg in packages {
                if info.repository_installed.contains(&pkg.to_string()) {
                    return true;
                }
            }
        }
        false
    }

    pub fn install(&self, package: &Package, info: &mut Info) -> Result<(), io::Error> {
        if let Some(packages) = package.repository.get(&self.repository) {
            for pkg in packages {
                self.install_package(pkg, info)?;
            }
        }
        Ok(())
    }

    pub fn install_package(&self, package: &str, info: &mut Info) -> Result<(), io::Error> {
        if !info.repository_installed.contains(&package.to_string()) {
            info.repository_installed.push(package.to_string());

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

    pub fn uninstall(&self, package: &Package, info: &mut Info) -> Result<(), io::Error> {
        if let Some(packages) = package.repository.get(&self.repository) {
            for pkg in packages {
                if !info.repository_installed.contains(&pkg.to_string()) {
                    continue;
                }
                let index: Option<usize> = info
                    .repository_installed
                    .iter()
                    .position(|x| *x == pkg.to_string());
                if index.is_some() {
                    info.repository_installed.remove(index.unwrap());
                }

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

    pub fn get_installed(&self) -> Result<Vec<String>, io::Error> {
        let mut packages: Vec<String> = vec![];

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
                packages.push(package);
            }
        }

        Ok(packages)
    }
}
