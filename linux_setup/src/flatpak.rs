use rust_cli::commands::Operation;

use std::collections::HashSet;
use std::io;
use std::str::Split;

use crate::distribution::{Distribution, PackageManager};
use crate::package::Package;

pub struct Flatpak {
    pub name: &'static str,
    pub is_verified: bool,
    pub remotes: Vec<&'static str>,
}

pub fn setup(distribution: &Distribution) -> Result<(), io::Error> {
    println!("Setup flatpak...");

    Operation::new(
        "flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo",
    )
    .hide_output(true)
    .run()?;

    if distribution.package_manager == PackageManager::DNF {
        Operation::new(
            "flatpak remote-add --if-not-exists fedora oci+https://registry.fedoraproject.org",
        )
        .hide_output(true)
        .run()?;
    }
    Ok(())
}

pub fn is_available(package: &Package) -> bool {
    package.flatpak.is_some()
}

pub fn is_installed(package: &Package, distribution: &Distribution) -> bool {
    if let Some(fp) = &package.flatpak {
        let package: &str = fp.name;
        if distribution.flatpaks.contains(&package.to_owned()) {
            return true;
        }
    }
    false
}

pub fn install<S: Into<String>>(
    package: &Package,
    remote: S,
    distribution: &mut Distribution,
) -> Result<(), io::Error> {
    let remote = remote.into();
    distribution.install_package("flatpak")?;
    setup(distribution)?;

    if let Some(fp) = &package.flatpak {
        let package = fp.name;
        if !distribution.flatpaks.contains(&package.to_owned()) {
            distribution.flatpaks.insert(package.to_owned());

            println!("Installing flatpak {} from {}...", package, remote);

            Operation::new(format!("flatpak install {} {} -y", remote, package)).run()?;
        }
    }
    Ok(())
}

pub fn uninstall(package: &Package, distribution: &mut Distribution) -> Result<(), io::Error> {
    if let Some(fp) = &package.flatpak {
        let package = fp.name;
        if distribution.flatpaks.contains(&package.to_owned()) {
            distribution.flatpaks.remove(&package.to_owned());

            println!("Uninstalling flatpak {}...", package);

            Operation::new(format!("flatpak remove {} -y", package)).run()?;
        }
    }
    Ok(())
}

pub fn update() -> Result<(), io::Error> {
    println!("Update flatpak...");
    Operation::new("flatpak update -y").run()?;
    Ok(())
}

pub fn auto_remove() -> Result<(), io::Error> {
    println!("Auto removing flatpak...");
    Operation::new("flatpak remove --unused -y").run()?;
    Ok(())
}

pub fn get_installed() -> Result<HashSet<String>, io::Error> {
    let mut packages: HashSet<String> = HashSet::new();

    let output = Operation::new("flatpak list --app").output()?;
    for line in output.split("\n") {
        if line.is_empty() {
            continue;
        }
        let columns: Split<&str> = line.split("\t");
        let columns: Vec<&str> = columns.collect::<Vec<&str>>();
        if columns.len() > 1 {
            packages.insert(columns[1].to_owned());
        }
    }

    return Ok(packages);
}
