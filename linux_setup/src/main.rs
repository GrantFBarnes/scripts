use rust_cli::ansi::Color;
use rust_cli::commands::Operation;
use rust_cli::prompts::Select;

use std::env;
use std::env::VarError;
use std::fs;
use std::io;
use std::process::{Command, Stdio};

extern crate rust_cli;

mod distribution;
mod flatpak;
mod gnome;
mod helper;
mod kde;
mod other;
mod package;
mod snap;

use crate::distribution::{
    DesktopEnvironment, Distribution, DistributionName, PackageManager, Repository,
};
use crate::package::Package;
use crate::snap::Snap;

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
        .title(format!("Flatpak Remote: {}", package.key))
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

fn post_uninstall(
    package: &str,
    distribution: &Distribution,
    method: &InstallMethod,
) -> Result<(), io::Error> {
    let home_dir: Result<String, VarError> = env::var("HOME");
    if home_dir.is_err() {
        return Err(io::Error::other("HOME directory could not be determined"));
    }
    let home_dir: String = home_dir.unwrap();

    match package {
        "code" => {
            if method != &InstallMethod::Repository {
                if distribution.package_manager == PackageManager::APT {
                    Operation::new()
                        .command("sudo rm /etc/apt/sources.list.d/vscode.list")
                        .run()?;
                }
                if distribution.package_manager == PackageManager::DNF {
                    Operation::new()
                        .command("sudo dnf config-manager --set-disabled code")
                        .run()?;
                    Operation::new()
                        .command("sudo rm /etc/yum.repos.d/vscode.repo")
                        .run()?;
                }
            }
            if method == &InstallMethod::Uninstall {
                Operation::new()
                    .command(format!(
                        "sudo rm -r {}{} {}{}",
                        &home_dir, "/.vscode", &home_dir, "/.config/Code"
                    ))
                    .run()?;
            }
        }
        "golang" => {
            if method == &InstallMethod::Uninstall {
                Operation::new()
                    .command(format!("sudo rm -r {}{}", &home_dir, "/.go"))
                    .run()?;
            }
        }
        "neovim" => {
            if method == &InstallMethod::Uninstall {
                Operation::new()
                    .command(format!("sudo rm -r {}{}", &home_dir, "/.config/nvim"))
                    .run()?;
            }
        }
        "pycharm" => {
            if method != &InstallMethod::Repository {
                if distribution.name == DistributionName::Fedora {
                    Operation::new()
                        .command("sudo dnf config-manager --set-disabled phracek-PyCharm")
                        .run()?;
                }
            }
        }
        "rust" => {
            if method != &InstallMethod::Other {
                Operation::new()
                    .command(format!("sudo rm -r {}{}", &home_dir, "/.cargo/bin/rustup"))
                    .run()?;
            }
        }
        "vim" => {
            if method == &InstallMethod::Uninstall {
                Operation::new()
                    .command(format!(
                        "sudo rm -r {}{} {}{} {}{}",
                        &home_dir, "/.vim", &home_dir, "/.viminfo", &home_dir, "/.vimrc"
                    ))
                    .run()?;
            }
        }
        _ => (),
    }

    Ok(())
}

fn pre_install(
    package: &str,
    distribution: &Distribution,
    info: &mut Info,
    method: &InstallMethod,
) -> Result<(), io::Error> {
    match package {
        "code" => {
            if method == &InstallMethod::Repository {
                if distribution.package_manager == PackageManager::APT {
                    distribution.install("wget", info)?;
                    distribution.install("gpg", info)?;

                    let key: String = Operation::new()
                        .command("wget -qO- https://packages.microsoft.com/keys/microsoft.asc")
                        .run_output()?;
                    fs::write("packages.microsoft", key)?;

                    Operation::new()
                        .command("gpg --dearmor packages.microsoft")
                        .run()?;

                    Operation::new()
                        .command("sudo install -D -o root -g root -m 644 packages.microsoft.gpg /etc/apt/keyrings/packages.microsoft.gpg")
                        .run()?;

                    fs::remove_file("packages.microsoft")?;
                    fs::remove_file("packages.microsoft.gpg")?;

                    let echo_cmd = Command::new("echo")
                        .arg("deb [arch=amd64,arm64,armhf signed-by=/etc/apt/keyrings/packages.microsoft.gpg] https://packages.microsoft.com/repos/code stable main")
                        .stdout(Stdio::piped())
                        .spawn()?;
                    Command::new("sudo")
                        .arg("tee")
                        .arg("/etc/apt/sources.list.d/vscode.list")
                        .stdin(Stdio::from(echo_cmd.stdout.unwrap()))
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()?
                        .wait()?;

                    Operation::new().command("sudo apt update").run()?;
                }
                if distribution.package_manager == PackageManager::DNF {
                    Operation::new()
                        .command(
                            "sudo rpm --import https://packages.microsoft.com/keys/microsoft.asc",
                        )
                        .run()?;

                    let echo_cmd = Command::new("echo")
                        .arg("-e")
                        .arg("[code]\nname=Visual Studio Code\nbaseurl=https://packages.microsoft.com/yumrepos/vscode\nenabled=1\ngpgcheck=1\ngpgkey=https://packages.microsoft.com/keys/microsoft.asc")
                        .stdout(Stdio::piped())
                        .spawn()?;
                    Command::new("sudo")
                        .arg("tee")
                        .arg("/etc/yum.repos.d/vscode.repo")
                        .stdin(Stdio::from(echo_cmd.stdout.unwrap()))
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()?
                        .wait()?;
                }
            }
        }
        p if p.contains("dotnet") => {
            if method == &InstallMethod::Repository {
                if distribution.repository == Repository::Debian {
                    distribution.install("wget", info)?;

                    Operation::new()
                        .command("wget https://packages.microsoft.com/config/debian/12/packages-microsoft-prod.deb -O packages-microsoft-prod.deb")
                        .show_output(true)
                        .run()?;
                    Operation::new()
                        .command("sudo dpkg -i packages-microsoft-prod.deb")
                        .show_output(true)
                        .run()?;
                    Operation::new()
                        .command("rm packages-microsoft-prod.deb")
                        .run()?;
                    Operation::new()
                        .command("sudo apt update")
                        .show_output(true)
                        .run()?;
                }
            }
        }
        "nodejs" => {
            if method == &InstallMethod::Repository {
                if distribution.repository == Repository::RedHat {
                    Operation::new()
                        .command("sudo dnf module enable nodejs:20 -y")
                        .run()?;
                }
            }
        }
        "pycharm" => {
            if method == &InstallMethod::Repository {
                if distribution.repository == Repository::Fedora {
                    Operation::new()
                        .command("sudo dnf config-manager --set-enabled phracek-PyCharm")
                        .run()?;
                }
            }
        }
        "rust" => {
            if method == &InstallMethod::Other {
                distribution.install("curl", info)?;
            }
        }
        _ => (),
    }

    Ok(())
}

fn post_install(
    package: &str,
    distribution: &Distribution,
    method: &InstallMethod,
) -> Result<(), io::Error> {
    let home_dir: Result<String, VarError> = env::var("HOME");
    if home_dir.is_err() {
        return Err(io::Error::other("HOME directory could not be determined"));
    }
    let home_dir: String = home_dir.unwrap();
    let bashrc: String = format!("{}{}", &home_dir, "/.bashrc");

    match package {
        "code" => {
            if method != &InstallMethod::Uninstall {
                let extensions: Vec<&str> = Vec::from(["esbenp.prettier-vscode", "vscodevim.vim"]);
                for ext in extensions {
                    Operation::new()
                        .command(format!("code --install-extension {}", ext))
                        .run()?;
                }

                fs::write(
                    format!("{}{}", &home_dir, "/.config/Code/User/settings.json"),
                    r#"{
  "[css]": { "editor.defaultFormatter": "esbenp.prettier-vscode" },
  "[html]": { "editor.defaultFormatter": "esbenp.prettier-vscode" },
  "[javascript]": { "editor.defaultFormatter": "esbenp.prettier-vscode" },
  "[json]": { "editor.defaultFormatter": "esbenp.prettier-vscode" },
  "[jsonc]": { "editor.defaultFormatter": "esbenp.prettier-vscode" },
  "[scss]": { "editor.defaultFormatter": "esbenp.prettier-vscode" },
  "[typescript]": { "editor.defaultFormatter": "esbenp.prettier-vscode" },
  "editor.formatOnSave": true,
  "editor.rulers": [80, 160],
  "extensions.ignoreRecommendations": true,
  "git.openRepositoryInParentFolders": "always",
  "telemetry.telemetryLevel": "off",
  "vim.smartRelativeLine": true,
  "vim.useCtrlKeys": false,
  "workbench.startupEditor": "none"
}
"#,
                )?;
            }
        }
        "golang" => {
            if method != &InstallMethod::Uninstall {
                Operation::new()
                    .command(format!("go env -w GOPATH={}/.go", &home_dir))
                    .run()?;
                if method == &InstallMethod::Snap || distribution.repository == Repository::RedHat {
                    helper::append_to_file_if_not_found(
                        &bashrc,
                        "export GOPATH",
                        r#"
export GOPATH=$HOME/.go
export PATH=$PATH:$GOPATH/bin
                        "#,
                        false,
                    )?;

                    Operation::new()
                        .command("go install golang.org/x/tools/gopls@latest")
                        .show_output(true)
                        .run()?;
                }
            }
        }
        "intellij" | "pycharm" => {
            if method != &InstallMethod::Uninstall {
                fs::write(
                    format!("{}{}", &home_dir, "/.ideavimrc"),
                    "sethandler a:ide",
                )?;
            }
        }
        "rust" => {
            if method == &InstallMethod::Other {
                Operation::new()
                    .command(format!(
                        "{}{} component add rust-analyzer",
                        home_dir, "/.cargo/bin/rustup"
                    ))
                    .run()?;
            }
        }
        "snapd" => {
            if method != &InstallMethod::Uninstall {
                distribution.setup_snap()?;
            }
        }
        "vim" | "neovim" => {
            if method != &InstallMethod::Uninstall {
                helper::append_to_file_if_not_found(
                    &bashrc,
                    "export EDITOR",
                    "export EDITOR=\"/usr/bin/vim\"\n",
                    false,
                )?;

                let config_file: String = if package == "vim" {
                    format!("{}{}", &home_dir, "/.vimrc")
                } else {
                    format!("{}{}", &home_dir, "/.config/nvim/init.vim")
                };

                if package == "neovim" {
                    Operation::new()
                        .command(format!("mkdir -p {}/.config/nvim", &home_dir))
                        .run()?;
                }

                fs::write(
                    &config_file,
                    r#"""""""""""""""""""""""""""""""""""""""""
" vim settings

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
set foldmethod=indent
set foldlevel=99

syntax on
colorscheme desert
filetype plugin indent on

""""""""""""""""""""""""""""""""""""""""
" normal mode remaps

let mapleader = " "

" window split
nnoremap <Leader>vs <C-w>v
nnoremap <Leader>hs <C-w>s

" window navigation
nnoremap <C-h> <C-w>h
nnoremap <C-j> <C-w>j
nnoremap <C-k> <C-w>k
nnoremap <C-l> <C-w>l

" text insert
nnoremap <Leader>go iif err != nil {}<ESC>

" file explore
nnoremap <Leader>ex :Explore<CR>
"#,
                )?;

                if distribution.repository != Repository::RedHat {
                    if distribution.repository == Repository::Arch
                        || distribution.repository == Repository::Fedora
                    {
                        helper::append_to_file_if_not_found(
                            &config_file,
                            "NERDTree",
                            "nnoremap <C-n> :NERDTreeToggle<CR>",
                            false,
                        )?;
                    }

                    helper::append_to_file_if_not_found(
                        &config_file,
                        "ale settings",
                        r#"
""""""""""""""""""""""""""""""""""""""""
" ale settings

let g:ale_fix_on_save = 1
let g:ale_completion_enabled = 1
let g:ale_linters = { "go": ["gopls"], "rust": ["analyzer"] }
let g:ale_fixers = { "*": ["remove_trailing_lines", "trim_whitespace"], "go": ["gofmt"], "rust": ["rustfmt"] }

nnoremap K :ALEHover<CR>
nnoremap gd :ALEGoToDefinition<CR>
nnoremap gn :ALERename<CR>
nnoremap gr :ALEFindReferences<CR>

""""""""""""""""""""""""""""""""""""""""
" insert mode remaps

inoremap <silent><expr> <Tab> pumvisible() ? "\<C-n>" : "\<TAB>"
inoremap <silent><expr> <S-Tab> pumvisible() ? "\<C-n>" : "\<S-TAB>"
"#,
                        false,
                    )?;
                }
            }
        }
        _ => (),
    }

    Ok(())
}

fn get_install_method(package: &Package, distribution: &Distribution, info: &Info) -> String {
    if distribution.is_installed(package.key, info) {
        return helper::get_colored_string("Repository", Color::Green);
    }
    if flatpak::is_installed(package, info) {
        return helper::get_colored_string("Flatpak", Color::Blue);
    }
    if snap::is_installed(package.key, info) {
        return helper::get_colored_string("Snap", Color::Magenta);
    }
    if other::is_installed(package.key, info) {
        return helper::get_colored_string("Other", Color::Yellow);
    }
    return helper::get_colored_string("Uninstalled", Color::Red);
}

fn is_installed(package: &Package, distribution: &Distribution, info: &Info) -> bool {
    distribution.is_installed(package.key, info)
        || flatpak::is_installed(package, info)
        || snap::is_installed(package.key, info)
        || other::is_installed(package.key, info)
}

fn run_package_select(
    package: &Package,
    distribution: &Distribution,
    info: &mut Info,
) -> Result<(), io::Error> {
    let mut options_display: Vec<String> = vec![];
    let mut options_value: Vec<InstallMethod> = vec![];

    if distribution.is_available(package.key) {
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

    if snap::is_available(package.key) {
        let mut display: String = String::from("Install Snap");
        let pkg: Option<Snap> = snap::get_package(package.key);
        if pkg.is_some() {
            let pkg: Snap = pkg.unwrap();
            if pkg.is_official {
                display.push_str(" (Official)");
            }
            if pkg.is_classic {
                display.push_str(" (classic)");
            }
        }
        options_display.push(helper::get_colored_string(display, Color::Magenta));
        options_value.push(InstallMethod::Snap);
    }

    if other::is_available(package.key) {
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
            package.key,
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
        distribution.uninstall(package.key, info)?;
    }

    if method != &InstallMethod::Flatpak {
        if info.has_flatpak {
            flatpak::uninstall(package, info)?;
        }
    }

    if method != &InstallMethod::Snap {
        if info.has_snap {
            snap::uninstall(package.key, info)?;
        }
    }

    if method != &InstallMethod::Other {
        other::uninstall(package.key, info)?;
    }
    post_uninstall(package.key, distribution, &method)?;

    pre_install(package.key, distribution, info, &method)?;
    match method {
        InstallMethod::Repository => distribution.install(package.key, info)?,
        InstallMethod::Flatpak => run_flatpak_remote_select(package, distribution, info)?,
        InstallMethod::Snap => snap::install(package.key, distribution, info)?,
        InstallMethod::Other => other::install(package.key, info)?,
        _ => (),
    }
    post_install(package.key, distribution, &method)
}

fn run_category_select(
    category: &str,
    start_idx: usize,
    show_all_desktop_environments: bool,
    distribution: &Distribution,
    info: &mut Info,
) -> Result<(), io::Error> {
    let mut options_display: Vec<String> = vec![];
    let mut options_value: Vec<Package> = vec![];

    let mut missing_desktop_environment: bool = false;

    for package in package::get_all_packages() {
        if &package.category != &category {
            continue;
        }

        if !distribution.is_available(&package.key)
            && !flatpak::is_available(&package)
            && !snap::is_available(&package.key)
            && !other::is_available(&package.key)
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
                package.display,
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
        .title(format!("Category: {}", category))
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
    let mut options: Vec<&str> = vec![];
    for category in CATEGORIES {
        options.push(category);
    }
    options.push("Exit");

    let selection = Select::new()
        .title("Choose a Category")
        .options(&options)
        .default_index(start_idx)
        .erase_after(true)
        .run_select_index()?;
    if selection.is_none() {
        return Ok(());
    }
    let selection = selection.unwrap();
    match options[selection] {
        "Exit" => return Ok(()),
        _ => run_category_select(options[selection], 0, false, distribution, info)?,
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

    let distribution: Distribution = distribution::get_distribution()?;
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
