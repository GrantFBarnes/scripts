use rust_cli::ansi::Color;
use rust_cli::commands::Operation;
use rust_cli::prompts::Select;

use std::io;

extern crate rust_cli;

mod distribution;
mod flatpak;
mod gnome;
mod helper;
mod kde;
mod other;
mod package;
mod snap;

use crate::distribution::{DesktopEnvironment, Distribution};
use crate::package::{Category, Package};

#[derive(PartialEq)]
enum InstallMethod {
    Repository,
    Flatpak,
    Snap,
    Other,
    Uninstall,
    Cancel,
}

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

fn repository_setup(distribution: &Distribution, info: &mut Info) -> Result<(), io::Error> {
    distribution.setup(info)?;
    if info.has_flatpak {
        flatpak::setup(distribution)?;
    }
    Ok(())
}

fn run_flatpak_remote_select(
    package: &Package,
    distribution: &Distribution,
    info: &mut Info,
) -> Result<(), io::Error> {
    let mut options: Vec<&str> = vec![];
    if let Some(fp) = &package.flatpak {
        for remote in &fp.remotes {
            options.push(remote);
        }
    }
    options.push("Cancel");

    let remote = Select::new()
        .title(format!("Flatpak Remote: {}", package.name))
        .options(&options)
        .erase_after(true)
        .run_select_value()?;
    if remote.is_none() {
        return Ok(());
    }
    let remote: String = remote.unwrap();
    if remote == "Cancel" {
        return Ok(());
    }

    flatpak::install(package, remote.as_str(), distribution, info)
}

fn get_install_method(package: &Package, distribution: &Distribution, info: &Info) -> String {
    if distribution.is_installed(package, info) {
        return helper::get_colored_string("Repository", Color::Green);
    }
    if flatpak::is_installed(package, info) {
        return helper::get_colored_string("Flatpak", Color::Blue);
    }
    if snap::is_installed(package, info) {
        return helper::get_colored_string("Snap", Color::Magenta);
    }
    if other::is_installed(package, info) {
        return helper::get_colored_string("Other", Color::Yellow);
    }
    return helper::get_colored_string("Uninstalled", Color::Red);
}

fn is_installed(package: &Package, distribution: &Distribution, info: &Info) -> bool {
    distribution.is_installed(package, info)
        || flatpak::is_installed(package, info)
        || snap::is_installed(package, info)
        || other::is_installed(package, info)
}

fn run_package_select(
    package: &Package,
    distribution: &Distribution,
    info: &mut Info,
) -> Result<(), io::Error> {
    let mut options_display: Vec<String> = vec![];
    let mut options_value: Vec<InstallMethod> = vec![];

    if distribution.is_available(package) {
        options_display.push(helper::get_colored_string(
            "Install Repository",
            Color::Green,
        ));
        options_value.push(InstallMethod::Repository);
    }

    if flatpak::is_available(package) {
        options_display.push(helper::get_colored_string("Install Flatpak", Color::Blue));
        options_value.push(InstallMethod::Flatpak);
    }

    if snap::is_available(package) {
        let mut display: String = String::from("Install Snap");
        if let Some(snp) = &package.snap {
            if snp.is_official {
                display.push_str(" (Official)");
            }
            if snp.is_classic {
                display.push_str(" (classic)");
            }
        }
        options_display.push(helper::get_colored_string(display, Color::Magenta));
        options_value.push(InstallMethod::Snap);
    }

    if other::is_available(package) {
        options_display.push(helper::get_colored_string("Install Other", Color::Yellow));
        options_value.push(InstallMethod::Other);
    }

    options_display.push(helper::get_colored_string("Uninstall", Color::Red));
    options_value.push(InstallMethod::Uninstall);

    options_display.push(String::from("Cancel"));
    options_value.push(InstallMethod::Cancel);

    let selection = Select::new()
        .title(format!(
            "Package: {} ({})",
            package.name,
            get_install_method(package, distribution, &info)
        ))
        .options(&options_display)
        .erase_after(true)
        .run_select_index()?;
    if selection.is_none() {
        return Ok(());
    }
    let selection: usize = selection.unwrap();
    let method: &InstallMethod = &options_value[selection];

    if method == &InstallMethod::Cancel {
        return Ok(());
    }

    if method != &InstallMethod::Repository {
        distribution.uninstall(package, info)?;
    }

    if method != &InstallMethod::Flatpak {
        if info.has_flatpak {
            flatpak::uninstall(package, info)?;
        }
    }

    if method != &InstallMethod::Snap {
        if info.has_snap {
            snap::uninstall(package, info)?;
        }
    }

    if method != &InstallMethod::Other {
        other::uninstall(package, info)?;
    }

    if let Some(pre_install) = &package.pre_install {
        (pre_install)(distribution, info, &method)?;
    }
    match method {
        InstallMethod::Repository => distribution.install(package, info)?,
        InstallMethod::Flatpak => run_flatpak_remote_select(package, distribution, info)?,
        InstallMethod::Snap => snap::install(package, distribution, info)?,
        InstallMethod::Other => other::install(package, info)?,
        _ => (),
    }
    if let Some(post_install) = &package.post_install {
        (post_install)(distribution, &method)?;
    }
    Ok(())
}

fn run_category_select(
    category: &Category,
    start_idx: usize,
    show_all_desktop_environments: bool,
    distribution: &Distribution,
    info: &mut Info,
) -> Result<(), io::Error> {
    let mut options_display: Vec<String> = vec![];
    let mut options_value: Vec<Package> = vec![];

    let mut missing_desktop_environment: bool = false;

    for package in package::get_category_packages(category) {
        if !distribution.is_available(&package)
            && !flatpak::is_available(&package)
            && !snap::is_available(&package)
            && !other::is_available(&package)
        {
            continue;
        }

        let mut missing_pkg_desktop_environment: bool = false;

        if let Some(de) = &package.desktop_environment {
            if (de == &DesktopEnvironment::Gnome && !info.has_gnome)
                || (de == &DesktopEnvironment::KDE && !info.has_kde)
            {
                missing_desktop_environment = true;
                if !show_all_desktop_environments && !is_installed(&package, distribution, info) {
                    continue;
                }
                missing_pkg_desktop_environment = true;
            }
        }

        options_display.push(format!(
            "{} ({})",
            helper::get_colored_string(
                package.name,
                if missing_pkg_desktop_environment {
                    Color::Yellow
                } else {
                    Color::White
                }
            ),
            get_install_method(&package, distribution, info)
        ));
        options_value.push(package);
    }

    if missing_desktop_environment {
        options_display.reverse();
        options_display.push(format!(
            "[{} Uninstalled Desktop Environments]",
            if show_all_desktop_environments {
                helper::get_colored_string("Hide", Color::Yellow)
            } else {
                helper::get_colored_string("Show", Color::Cyan)
            }
        ));
        options_display.reverse();

        options_value.reverse();
        options_value.push(Package::new());
        options_value.reverse();
    }

    options_display.push(String::from("Exit"));
    options_value.push(Package::new());

    let selection = Select::new()
        .title(format!("Category: {}", category.as_str()))
        .options(&options_display)
        .default_index(start_idx)
        .erase_after(true)
        .run_select_index()?;
    if selection.is_none() {
        return Ok(());
    }
    let selection: usize = selection.unwrap();

    if missing_desktop_environment && selection == 0 {
        // toggle show all desktop environments
        run_category_select(
            category,
            selection,
            !show_all_desktop_environments,
            distribution,
            info,
        )?;
    } else if selection < options_value.len() - 1 {
        // not exit
        run_package_select(&options_value[selection], distribution, info)?;
        run_category_select(
            category,
            selection + 1,
            show_all_desktop_environments,
            distribution,
            info,
        )?;
    }

    Ok(())
}

fn run_install_packages(
    start_idx: usize,
    distribution: &Distribution,
    info: &mut Info,
) -> Result<(), io::Error> {
    let mut options_display: Vec<&str> = vec![];
    let mut options_value: Vec<&Category> = vec![];
    for category in Category::iterator() {
        options_display.push(category.as_str());
        options_value.push(category);
    }
    options_display.push("Exit");

    let selection = Select::new()
        .title("Choose a Category")
        .options(&options_display)
        .default_index(start_idx)
        .erase_after(true)
        .run_select_index()?;
    if selection.is_none() {
        return Ok(());
    }
    let selection = selection.unwrap();
    match options_display[selection] {
        "Exit" => return Ok(()),
        _ => run_category_select(options_value[selection], 0, false, distribution, info)?,
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

    let selection = Select::new()
        .title("Linux Setup")
        .options(&options)
        .default_index(start_idx)
        .erase_after(true)
        .run_select_index()?;
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
    let has_gnome: bool = Operation::new()
        .command("gnome-shell --version")
        .run()
        .is_ok();

    let has_kde: bool = Operation::new()
        .command("plasmashell --version")
        .run()
        .is_ok();

    let distribution: Distribution = distribution::Distribution::new()?;
    let repository_installed: Vec<String> = distribution.get_installed()?;

    let has_flatpak: bool = Operation::new().command("flatpak --version").run().is_ok();
    let flatpak_installed: Vec<String> = match has_flatpak {
        true => flatpak::get_installed()?,
        false => vec![],
    };

    let has_snap: bool = Operation::new().command("snap --version").run().is_ok();
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
