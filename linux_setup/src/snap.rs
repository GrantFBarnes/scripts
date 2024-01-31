use rust_cli::commands::Operation;

use std::collections::HashSet;
use std::io;
use std::process::{Command, Stdio};
use std::str::SplitWhitespace;

use crate::distribution::Distribution;
use crate::package::Package;
use crate::Info;

pub struct Snap {
    pub name: &'static str,
    pub is_official: bool,
    pub is_classic: bool,
    pub channel: &'static str,
}

pub fn is_available(package: &Package) -> bool {
    package.snap.is_some()
}

pub fn is_installed(package: &Package, info: &Info) -> bool {
    if let Some(snp) = &package.snap {
        if info.snap_installed.contains(&snp.name.to_owned()) {
            return true;
        }
    }
    false
}

pub fn install(
    package: &Package,
    distribution: &mut Distribution,
    info: &mut Info,
) -> Result<(), io::Error> {
    distribution.install_package("snapd")?;

    if let Some(snp) = &package.snap {
        if !info.snap_installed.contains(&snp.name.to_owned()) {
            info.snap_installed.insert(snp.name.to_owned());

            println!("Installing snap {}...", snp.name);

            let mut cmd: Command = Command::new("sudo");
            cmd.arg("snap");
            cmd.arg("install");
            cmd.arg(snp.name);
            if snp.is_classic {
                cmd.arg("--classic");
            }
            if !snp.channel.is_empty() {
                cmd.arg("--channel");
                cmd.arg(snp.channel);
            }
            cmd.stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()?
                .wait()?;
        }
    }
    Ok(())
}

pub fn uninstall(package: &Package, info: &mut Info) -> Result<(), io::Error> {
    if let Some(snp) = &package.snap {
        if info.snap_installed.contains(&snp.name.to_owned()) {
            info.snap_installed.remove(&snp.name.to_owned());

            println!("Uninstalling snap {}...", snp.name);

            Operation::new(format!("sudo snap remove {}", snp.name)).run()?;
        }
    }
    Ok(())
}

pub fn update() -> Result<(), io::Error> {
    println!("Update snap...");
    Operation::new("sudo snap refresh").run()?;
    Ok(())
}

pub fn get_installed() -> Result<HashSet<String>, io::Error> {
    let mut packages: HashSet<String> = HashSet::new();

    let output = Operation::new("snap list").output()?;
    for line in output.split("\n") {
        if line.is_empty() {
            continue;
        }
        let columns: SplitWhitespace = line.split_whitespace();
        let columns: Vec<&str> = columns.collect::<Vec<&str>>();
        if !columns.is_empty() {
            packages.insert(columns[0].to_owned());
        }
    }

    Ok(packages)
}
