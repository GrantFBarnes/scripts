use rust_cli::commands::Operation;

use std::io;
use std::str::Split;

use crate::distribution::{Distribution, PackageManager};
use crate::package::Package;
use crate::Info;

pub struct Flatpak {
    pub name: &'static str,
    pub remotes: Vec<&'static str>,
}

pub fn setup(distribution: &Distribution) -> Result<(), io::Error> {
    println!("Setup flatpak...");

    Operation::new().command(
        "flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo",
    ).run()?;

    if distribution.package_manager == PackageManager::DNF {
        Operation::new()
            .command(
                "flatpak remote-add --if-not-exists fedora oci+https://registry.fedoraproject.org",
            )
            .run()?;
    }
    Ok(())
}

pub fn is_available(package: &Package) -> bool {
    package.flatpak.is_some()
}

pub fn is_installed(package: &Package, info: &Info) -> bool {
    if let Some(fp) = &package.flatpak {
        let package: &str = fp.name;
        if info.flatpak_installed.contains(&package.to_owned()) {
            return true;
        }
    }
    false
}

pub fn install(
    package: &Package,
    remote: &str,
    distribution: &Distribution,
    info: &mut Info,
) -> Result<(), io::Error> {
    distribution.install_package("flatpak", info)?;
    setup(distribution)?;

    if let Some(fp) = &package.flatpak {
        let package = fp.name;
        if !info.flatpak_installed.contains(&package.to_owned()) {
            info.flatpak_installed.push(package.to_owned());

            println!("Installing flatpak {} from {}...", package, remote);

            Operation::new()
                .command(format!("flatpak install {} {} -y", remote, package))
                .show_output(true)
                .run()?;
        }
    }
    Ok(())
}

pub fn uninstall(package: &Package, info: &mut Info) -> Result<(), io::Error> {
    if let Some(fp) = &package.flatpak {
        let package = fp.name;
        if info.flatpak_installed.contains(&package.to_owned()) {
            let index: Option<usize> = info.flatpak_installed.iter().position(|x| *x == package);
            if index.is_some() {
                info.flatpak_installed.remove(index.unwrap());
            }

            println!("Uninstalling flatpak {}...", package);

            Operation::new()
                .command(format!("flatpak remove {} -y", package))
                .show_output(true)
                .run()?;
        }
    }
    Ok(())
}

pub fn update() -> Result<(), io::Error> {
    println!("Update flatpak...");
    Operation::new()
        .command("flatpak update -y")
        .show_output(true)
        .run()?;
    Ok(())
}

pub fn auto_remove() -> Result<(), io::Error> {
    println!("Auto removing flatpak...");
    Operation::new()
        .command("flatpak remove --unused -y")
        .show_output(true)
        .run()?;
    Ok(())
}

pub fn get_installed() -> Result<Vec<String>, io::Error> {
    let mut packages: Vec<String> = vec![];

    let output = Operation::new()
        .command("flatpak list --app")
        .run_output()?;
    for line in output.split("\n") {
        if line.is_empty() {
            continue;
        }
        let columns: Split<&str> = line.split("\t");
        let columns: Vec<&str> = columns.collect::<Vec<&str>>();
        if columns.len() > 1 {
            packages.push(columns[1].to_owned());
        }
    }

    return Ok(packages);
}
