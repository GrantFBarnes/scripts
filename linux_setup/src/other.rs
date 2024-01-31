use rust_cli::commands::Operation;

use std::collections::HashSet;
use std::io;
use std::process::{Command, Stdio};

use crate::package::Package;
use crate::Info;

const PACKAGES: [&str; 1] = ["rust"];
pub struct OtherPackage {
    pub name: &'static str,
}

pub fn is_available(package: &Package) -> bool {
    package.other.is_some()
}

pub fn is_installed(package: &Package, info: &Info) -> bool {
    if let Some(otr) = &package.other {
        if info.other_installed.contains(&otr.name.to_string()) {
            return true;
        }
    }
    false
}

pub fn install(package: &Package, info: &mut Info) -> Result<(), io::Error> {
    if let Some(otr) = &package.other {
        if !info.other_installed.contains(&otr.name.to_string()) {
            info.other_installed.insert(otr.name.to_owned());

            println!("Installing other {}...", otr.name);

            match otr.name {
                "rust" => {
                    let curl_cmd = Command::new("curl")
                        .arg("'=https'")
                        .arg("--tlsv1.2")
                        .arg("-sSf")
                        .arg("https://sh.rustup.rs")
                        .stdout(Stdio::piped())
                        .spawn()?;
                    Command::new("sh")
                        .stdin(Stdio::from(curl_cmd.stdout.unwrap()))
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()?
                        .wait()?;
                }
                _ => (),
            }
        }
    }
    Ok(())
}

pub fn uninstall(package: &Package, info: &mut Info) -> Result<(), io::Error> {
    if let Some(otr) = &package.other {
        if info.other_installed.contains(&otr.name.to_string()) {
            info.other_installed.remove(&otr.name.to_string());

            println!("Uninstalling other {}...", otr.name);

            match otr.name {
                "rust" => {
                    Operation::new("rustup self uninstall").run()?;
                }
                _ => (),
            }
        }
    }
    Ok(())
}

pub fn update(info: &Info) -> Result<(), io::Error> {
    println!("Update other...");

    for pkg in PACKAGES {
        if info.other_installed.contains(&pkg.to_string()) {
            match pkg {
                "rust" => {
                    Operation::new("rustup self update").run()?;
                    Operation::new("rustup update stable").run()?;
                }
                _ => (),
            }
        }
    }
    Ok(())
}

pub fn get_installed() -> Result<HashSet<String>, io::Error> {
    let mut packages: HashSet<String> = HashSet::new();

    for pkg in PACKAGES {
        match pkg {
            "rust" => match Operation::new("rustup --version").hide_output(true).run() {
                Ok(_) => {
                    packages.insert(pkg.to_string());
                }
                _ => (),
            },
            _ => (),
        }
    }

    Ok(packages)
}
