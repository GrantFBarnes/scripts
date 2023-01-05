use dialoguer::Select;
use std::env;
use std::env::VarError;
use std::fs;
use std::process::Command;

mod distribution;
mod flatpak;
mod gnome;
mod helper;
mod kde;
mod snap;

use crate::distribution::Distribution;
use crate::snap::Snap;

pub struct Info {
    has_gnome: bool,
    has_kde: bool,
    has_flatpak: bool,
    has_snap: bool,
    repository_installed: Vec<String>,
    flatpak_installed: Vec<String>,
    snap_installed: Vec<String>,
}

struct Package {
    display: &'static str,
    key: &'static str,
    category: &'static str,
    desktop_environment: &'static str,
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

const ALL_PACKAGES: [Package; 102] = [
    Package {
        display: "0 A.D.",
        key: "0ad",
        category: "Games",
        desktop_environment: "",
    },
    Package {
        display: "Ark Archiving",
        key: "ark",
        category: "Utilities",
        desktop_environment: "kde",
    },
    Package {
        display: "Blender",
        key: "blender",
        category: "Multi Media",
        desktop_environment: "",
    },
    Package {
        display: "Cheese - Webcam",
        key: "cheese",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Chromium",
        key: "chromium",
        category: "Browsers",
        desktop_environment: "",
    },
    Package {
        display: "Cockpit - Web Interface",
        key: "cockpit",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "cups - Printer Support",
        key: "cups",
        category: "Desktop",
        desktop_environment: "",
    },
    Package {
        display: "cURL - Client URL",
        key: "curl",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "dconf Editor",
        key: "dconf-editor",
        category: "Utilities",
        desktop_environment: "gnome",
    },
    Package {
        display: "Deja Dup - Backups",
        key: "deja-dup",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Discord",
        key: "discord",
        category: "Communication",
        desktop_environment: "",
    },
    Package {
        display: "Elisa Music Player",
        key: "elisa",
        category: "Multi Media",
        desktop_environment: "kde",
    },
    Package {
        display: "Epiphany - Gnome Web",
        key: "epiphany",
        category: "Browsers",
        desktop_environment: "gnome",
    },
    Package {
        display: "Evince - Document Viewer",
        key: "evince",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Eye of Gnome - Image Viewer",
        key: "eog",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Fedora Media Writer",
        key: "mediawriter",
        category: "Utilities",
        desktop_environment: "",
    },
    Package {
        display: "ffmpeg - Media Codecs",
        key: "ffmpeg",
        category: "Desktop",
        desktop_environment: "",
    },
    Package {
        display: "FileLight Disk Usage",
        key: "filelight",
        category: "Utilities",
        desktop_environment: "kde",
    },
    Package {
        display: "Firefox",
        key: "firefox",
        category: "Browsers",
        desktop_environment: "",
    },
    Package {
        display: "Firefox ESR",
        key: "firefox-esr",
        category: "Browsers",
        desktop_environment: "",
    },
    Package {
        display: "gedit",
        key: "gedit",
        category: "Editors",
        desktop_environment: "gnome",
    },
    Package {
        display: "GIMP",
        key: "gimp",
        category: "Multi Media",
        desktop_environment: "",
    },
    Package {
        display: "git - Version Control",
        key: "git",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "Gnome 2048",
        key: "gnome-2048",
        category: "Games",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Books",
        key: "gnome-books",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Boxes - VM Manager",
        key: "gnome-boxes",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Builder",
        key: "gnome-builder",
        category: "Editors",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Calculator",
        key: "gnome-calculator",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Calendar",
        key: "gnome-calendar",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Chess",
        key: "gnome-chess",
        category: "Games",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Clocks",
        key: "gnome-clocks",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Connections",
        key: "gnome-connections",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Contacts",
        key: "gnome-contacts",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Disk Usage",
        key: "baobab",
        category: "Utilities",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Disk Utility",
        key: "gnome-disk-utility",
        category: "Utilities",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Maps",
        key: "gnome-maps",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Mines",
        key: "gnome-mines",
        category: "Games",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Music",
        key: "gnome-music",
        category: "Multi Media",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Password Safe",
        key: "gnome-passwordsafe",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Photos",
        key: "gnome-photos",
        category: "Multi Media",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Shell Extension",
        key: "gnome-shell-extensions",
        category: "Utilities",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Shell Extension Manager",
        key: "gnome-shell-extension-manager",
        category: "Utilities",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Software",
        key: "gnome-software",
        category: "Software",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Solitaire",
        key: "aisleriot",
        category: "Games",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Sound Recorder",
        key: "gnome-sound-recorder",
        category: "Multi Media",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Sudoku",
        key: "gnome-sudoku",
        category: "Games",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome System Monitor",
        key: "gnome-system-monitor",
        category: "Utilities",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Tetris",
        key: "quadrapassel",
        category: "Games",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Text Editor",
        key: "gnome-text-editor",
        category: "Editors",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Tweaks",
        key: "gnome-tweaks",
        category: "Utilities",
        desktop_environment: "gnome",
    },
    Package {
        display: "Gnome Weather",
        key: "gnome-weather",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "GNU Cash - Accounting",
        key: "gnucash",
        category: "Applications",
        desktop_environment: "",
    },
    Package {
        display: "Gwenview - Image Viewer",
        key: "gwenview",
        category: "Applications",
        desktop_environment: "kde",
    },
    Package {
        display: "htop - Process Reviewer",
        key: "htop",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "IceCat - GNU Browser",
        key: "icecat",
        category: "Browsers",
        desktop_environment: "",
    },
    Package {
        display: "imagemagick",
        key: "imagemagick",
        category: "Desktop",
        desktop_environment: "",
    },
    Package {
        display: "Intellij",
        key: "intellij",
        category: "Editors",
        desktop_environment: "",
    },
    Package {
        display: "Kate",
        key: "kate",
        category: "Editors",
        desktop_environment: "kde",
    },
    Package {
        display: "KCalc - Calculator",
        key: "kcalc",
        category: "Applications",
        desktop_environment: "kde",
    },
    Package {
        display: "KDE Chess",
        key: "knights",
        category: "Games",
        desktop_environment: "kde",
    },
    Package {
        display: "KDE Mines",
        key: "kmines",
        category: "Games",
        desktop_environment: "kde",
    },
    Package {
        display: "KDE Sudoku",
        key: "ksudoku",
        category: "Games",
        desktop_environment: "kde",
    },
    Package {
        display: "KdenLive Video Editor",
        key: "kdenlive",
        category: "Multi Media",
        desktop_environment: "kde",
    },
    Package {
        display: "KDevelop",
        key: "kdevelop",
        category: "Editors",
        desktop_environment: "kde",
    },
    Package {
        display: "Kile - LaTex Editor",
        key: "kile",
        category: "Editors",
        desktop_environment: "kde",
    },
    Package {
        display: "KSysGuard",
        key: "ksysguard",
        category: "Utilities",
        desktop_environment: "kde",
    },
    Package {
        display: "KWrite",
        key: "kwrite",
        category: "Editors",
        desktop_environment: "kde",
    },
    Package {
        display: "LaTex - Compiler",
        key: "latex",
        category: "Desktop",
        desktop_environment: "",
    },
    Package {
        display: "LibreOffice",
        key: "libreoffice",
        category: "Editors",
        desktop_environment: "",
    },
    Package {
        display: "MariaDB - Database",
        key: "mariadb",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "MP3 Metadata Editor",
        key: "id3v2",
        category: "Desktop",
        desktop_environment: "",
    },
    Package {
        display: "nano - Text Editor",
        key: "nano",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "ncdu - Disk Usage",
        key: "ncdu",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "Node.js - JavaScript RE",
        key: "node",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "Okular - Document Viewer",
        key: "okular",
        category: "Applications",
        desktop_environment: "kde",
    },
    Package {
        display: "Plasma Discover",
        key: "plasma-discover",
        category: "Software",
        desktop_environment: "kde",
    },
    Package {
        display: "Plasma System Monitor",
        key: "plasma-systemmonitor",
        category: "Utilities",
        desktop_environment: "kde",
    },
    Package {
        display: "Podman - Containers",
        key: "podman",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "Pycharm",
        key: "pycharm",
        category: "Editors",
        desktop_environment: "",
    },
    Package {
        display: "qtile - Window Manager",
        key: "qtile",
        category: "Desktop",
        desktop_environment: "",
    },
    Package {
        display: "RhythmBox",
        key: "rhythmbox",
        category: "Multi Media",
        desktop_environment: "gnome",
    },
    Package {
        display: "Rust Language",
        key: "rust",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "Shotwell",
        key: "shotwell",
        category: "Multi Media",
        desktop_environment: "gnome",
    },
    Package {
        display: "Simple Scan",
        key: "simple-scan",
        category: "Utilities",
        desktop_environment: "",
    },
    Package {
        display: "Snap Store",
        key: "snap-store",
        category: "Software",
        desktop_environment: "",
    },
    Package {
        display: "Spectacle Screenshot",
        key: "spectacle",
        category: "Utilities",
        desktop_environment: "kde",
    },
    Package {
        display: "SSH - Secure Shell Protocol",
        key: "ssh",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "Steam",
        key: "steam",
        category: "Games",
        desktop_environment: "",
    },
    Package {
        display: "Super Tux Kart",
        key: "supertuxkart",
        category: "Games",
        desktop_environment: "",
    },
    Package {
        display: "Thunderbird",
        key: "thunderbird",
        category: "Communication",
        desktop_environment: "",
    },
    Package {
        display: "TOR - The Onion Router",
        key: "torbrowser-launcher",
        category: "Browsers",
        desktop_environment: "",
    },
    Package {
        display: "Totem Video Player",
        key: "totem",
        category: "Multi Media",
        desktop_environment: "gnome",
    },
    Package {
        display: "Transmission (GTK) - Torrent",
        key: "transmission-gtk",
        category: "Applications",
        desktop_environment: "gnome",
    },
    Package {
        display: "Transmission (QT) - Torrent",
        key: "transmission-qt",
        category: "Applications",
        desktop_environment: "kde",
    },
    Package {
        display: "Vietnamese Keyboard",
        key: "ibus-unikey",
        category: "Desktop",
        desktop_environment: "",
    },
    Package {
        display: "VIM - Text Editor",
        key: "vim",
        category: "Server",
        desktop_environment: "",
    },
    Package {
        display: "Virt Manager",
        key: "virt-manager",
        category: "Applications",
        desktop_environment: "",
    },
    Package {
        display: "VLC",
        key: "vlc",
        category: "Multi Media",
        desktop_environment: "",
    },
    Package {
        display: "VS Code",
        key: "code",
        category: "Editors",
        desktop_environment: "",
    },
    Package {
        display: "VS Codium",
        key: "codium",
        category: "Editors",
        desktop_environment: "",
    },
    Package {
        display: "Xonotic",
        key: "xonotic",
        category: "Games",
        desktop_environment: "",
    },
    Package {
        display: "yt-dlp - Download YouTube",
        key: "yt-dlp",
        category: "Desktop",
        desktop_environment: "",
    },
];

fn environment_setup() {
    let home_dir: Result<String, VarError> = env::var("HOME");
    if home_dir.is_ok() {
        let home_dir: String = home_dir.unwrap();

        let bashrc: String = format!("{}{}", &home_dir, "/.bashrc");
        helper::append_to_file_if_not_found(
            &bashrc,
            "export EDITOR",
            "export EDITOR=\"/usr/bin/vim\"\n",
            false,
        );
        helper::append_to_file_if_not_found(
            &bashrc,
            "export GFB_MANAGER_SECRET",
            "export GFB_MANAGER_SECRET=\"\"",
            false,
        );
        helper::append_to_file_if_not_found(
            &bashrc,
            "export GFB_JWT_SECRET",
            "export GFB_JWT_SECRET=\"\"",
            false,
        );
        helper::append_to_file_if_not_found(
            &bashrc,
            "export GFB_SQL_HOST",
            "export GFB_SQL_HOST=\"\"\n",
            false,
        );
        helper::append_to_file_if_not_found(
            &bashrc,
            "export GFB_SQL_USER",
            "export GFB_SQL_USER=\"\"\n",
            false,
        );
        helper::append_to_file_if_not_found(
            &bashrc,
            "export GFB_SQL_PASSWORD",
            "export GFB_SQL_PASSWORD=\"\"\n",
            false,
        );

        let _ = fs::write(
            format!("{}{}", &home_dir, "/.vimrc"),
            r#"
set nocompatible

set encoding=utf-8

set noswapfile
set nobackup
set nowritebackup

set mouse=a
set updatetime=300
set scrolloff=10
set number
set ignorecase smartcase
set incsearch hlsearch

syntax on
filetype plugin indent on

call plug#begin()
Plug 'neoclide/coc.nvim', {'branch': 'release'}
Plug 'rust-lang/rust.vim'
call plug#end()

let g:rustfmt_autosave = 1
let g:ale_linters = { "rust": ["analyzer"] }
let g:ale_fixers = { "rust": ["rustfmt"] }

" -----------------------------------------------------------------------------
"  NERDTree

nnoremap <C-n> :NERDTreeToggle<CR>

" -----------------------------------------------------------------------------
"  CoC

" Use tab for trigger completion with characters ahead and navigate
" NOTE: There's always complete item selected by default, you may want to enable
" no select by `"suggest.noselect": true` in your configuration file
" NOTE: Use command ':verbose imap <tab>' to make sure tab is not mapped by
" other plugin before putting this into your config
inoremap <silent><expr> <TAB>
      \ coc#pum#visible() ? coc#pum#next(1) :
      \ CheckBackspace() ? "\<Tab>" :
      \ coc#refresh()
inoremap <expr><S-TAB> coc#pum#visible() ? coc#pum#prev(1) : "\<C-h>"

" Make <CR> to accept selected completion item or notify coc.nvim to format
" <C-g>u breaks current undo, please make your own choice
inoremap <silent><expr> <CR> coc#pum#visible() ? coc#pum#confirm()
                              \: "\<C-g>u\<CR>\<c-r>=coc#on_enter()\<CR>"

function! CheckBackspace() abort
  let col = col('.') - 1
  return !col || getline('.')[col - 1]  =~# '\s'
endfunction
"#,
        );
    }
}

fn repository_setup(distribution: &Distribution, info: &mut Info) {
    distribution.setup(info);
    if info.has_flatpak {
        flatpak::setup();
    }
}

fn get_install_method(package: &str, distribution: &Distribution, info: &Info) -> String {
    if distribution.is_installed(package, info) {
        return helper::get_colored_string("Repository", "green");
    }
    if flatpak::is_installed(package, info) {
        return helper::get_colored_string("Flatpak", "blue");
    }
    if snap::is_installed(package, info) {
        return helper::get_colored_string("Snap", "magenta");
    }
    return helper::get_colored_string("Uninstalled", "red");
}

fn run_package_select(package: &str, distribution: &Distribution, info: &mut Info) {
    let mut options_display: Vec<String> = vec![];
    let mut options_value: Vec<&str> = vec![];

    if distribution.is_available(package) {
        options_display.push(helper::get_colored_string("Install Repository", "green"));
        options_value.push("repository");
    }

    if flatpak::is_available(package) {
        options_display.push(helper::get_colored_string("Install Flatpak", "blue"));
        options_value.push("flatpak");
    }

    if snap::is_available(package) {
        let mut display: String = String::from("Install Snap");
        let pkg: Option<Snap> = snap::get_package(package);
        if pkg.is_some() {
            let pkg: Snap = pkg.unwrap();
            if pkg.is_official {
                display.push_str(" (Official)");
            }
            if pkg.is_classic {
                display.push_str(" (classic)");
            }
        }
        options_display.push(helper::get_colored_string(display, "magenta"));
        options_value.push("snap");
    }

    options_display.push(helper::get_colored_string("Uninstall", "red"));
    options_value.push("uninstall");

    options_display.push(helper::get_colored_string("Cancel", ""));
    options_value.push("cancel");

    let selection: std::io::Result<Option<usize>> = Select::new()
        .with_prompt(format!(
            "Package: {} ({})",
            package,
            get_install_method(package, distribution, &info)
        ))
        .items(&options_display)
        .default(0)
        .interact_opt();
    if selection.is_err() {
        return;
    }
    let selection: Option<usize> = selection.unwrap();
    if selection.is_none() {
        return;
    }
    let selection: usize = selection.unwrap();

    if options_value[selection] == "cancel" {
        return;
    }

    if options_value[selection] != "repository" {
        distribution.uninstall(package, info);
    }

    if options_value[selection] != "flatpak" {
        if info.has_flatpak {
            flatpak::uninstall(package, info);
        }
    }

    if options_value[selection] != "snap" {
        if info.has_snap {
            snap::uninstall(package, info);
        }
    }

    match options_value[selection] {
        "repository" => distribution.install(package, info),
        "flatpak" => flatpak::install(package, distribution, info),
        "snap" => snap::install(package, distribution, info),
        _ => (),
    }
}

fn run_category_select(
    category: &str,
    start_idx: usize,
    show_all_desktop_environments: bool,
    distribution: &Distribution,
    info: &mut Info,
) {
    let mut options_display: Vec<String> = vec![];
    let mut options_value: Vec<&str> = vec![];

    let mut missing_desktop_environment: bool = false;

    for pkg in ALL_PACKAGES {
        if pkg.category != category {
            continue;
        }

        if !distribution.is_available(pkg.key)
            && !flatpak::is_available(pkg.key)
            && !snap::is_available(pkg.key)
        {
            continue;
        }

        let mut missing_pkg_desktop_environment: bool = false;

        if (pkg.desktop_environment == "gnome" && !info.has_gnome)
            || (pkg.desktop_environment == "kde" && !info.has_kde)
        {
            missing_desktop_environment = true;
            if !show_all_desktop_environments {
                continue;
            }
            missing_pkg_desktop_environment = true;
        }

        options_display.push(format!(
            "{} ({})",
            helper::get_colored_string(
                pkg.display,
                if missing_pkg_desktop_environment {
                    "yellow"
                } else {
                    ""
                }
            ),
            get_install_method(pkg.key, distribution, info)
        ));
        options_value.push(pkg.key);
    }

    if missing_desktop_environment {
        options_display.reverse();
        options_display.push(format!(
            "[{} Uninstalled Desktop Environments]",
            if show_all_desktop_environments {
                helper::get_colored_string("Hide", "yellow")
            } else {
                helper::get_colored_string("Show", "cyan")
            }
        ));
        options_display.reverse();

        options_value.reverse();
        options_value.push("toggle_show_all_desktop_environments");
        options_value.reverse();
    }

    options_display.push(helper::get_colored_string("Exit", ""));
    options_value.push("exit");

    let selection: std::io::Result<Option<usize>> = Select::new()
        .with_prompt(format!("Category: {}", category))
        .items(&options_display)
        .default(start_idx)
        .interact_opt();
    if selection.is_err() {
        return;
    }
    let selection: Option<usize> = selection.unwrap();
    if selection.is_none() {
        return;
    }
    let selection: usize = selection.unwrap();

    match options_value[selection] {
        "exit" => (),
        "toggle_show_all_desktop_environments" => {
            run_category_select(
                category,
                selection,
                !show_all_desktop_environments,
                distribution,
                info,
            );
        }
        _ => {
            run_package_select(options_value[selection], distribution, info);
            run_category_select(
                category,
                selection + 1,
                show_all_desktop_environments,
                distribution,
                info,
            );
        }
    }
}

fn run_install_packages(start_idx: usize, distribution: &Distribution, info: &mut Info) {
    let mut options: Vec<&str> = vec![];
    for category in CATEGORIES {
        options.push(category);
    }
    options.push("Exit");

    let selection: std::io::Result<Option<usize>> = Select::new()
        .with_prompt("Choose a Category")
        .items(&options)
        .default(start_idx)
        .interact_opt();
    if selection.is_err() {
        return;
    }
    let selection: Option<usize> = selection.unwrap();
    if selection.is_none() {
        return;
    }
    let selection: usize = selection.unwrap();

    match options[selection] {
        "Exit" => return,
        _ => run_category_select(options[selection], 0, false, distribution, info),
    }

    run_install_packages(selection + 1, distribution, info);
}

fn run_menu(start_idx: usize, distribution: &Distribution, info: &mut Info) {
    let mut options: Vec<&str> = vec!["Environment Setup", "Repository Setup"];
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

    let selection: std::io::Result<Option<usize>> = Select::new()
        .items(&options)
        .default(start_idx)
        .interact_opt();
    if selection.is_err() {
        return;
    }
    let selection: Option<usize> = selection.unwrap();
    if selection.is_none() {
        return;
    }
    let selection: usize = selection.unwrap();

    match options[selection] {
        "Environment Setup" => environment_setup(),
        "Repository Setup" => repository_setup(distribution, info),
        "GNOME Setup" => gnome::setup(distribution),
        "KDE Setup" => kde::setup(),
        "Update Packages" => {
            distribution.update();
            if info.has_flatpak {
                flatpak::update();
            }
            if info.has_snap {
                snap::update();
            }
        }
        "Auto Remove Packages" => {
            distribution.auto_remove();
            if info.has_flatpak {
                flatpak::auto_remove();
            }
        }
        "Install Packages" => run_install_packages(0, distribution, info),
        "Exit" => return,
        _ => (),
    }

    run_menu(selection + 1, distribution, info);
}

fn main() {
    let mut has_gnome: bool = false;
    match Command::new("gnome-shell").arg("--version").output() {
        Ok(_) => has_gnome = true,
        _ => (),
    }

    let mut has_kde: bool = false;
    match Command::new("plasmashell").arg("--version").output() {
        Ok(_) => has_kde = true,
        _ => (),
    }

    let distribution: Option<Distribution> = distribution::get_distribution();
    if distribution.is_none() {
        println!("Distribution is not recognized");
        return;
    }
    let distribution: Distribution = distribution.unwrap();
    let repository_installed: Vec<String> = distribution.get_installed();

    let mut has_flatpak: bool = false;
    let mut flatpak_installed: Vec<String> = vec![];
    match Command::new("flatpak").arg("--version").output() {
        Ok(_) => {
            has_flatpak = true;
            flatpak_installed = flatpak::get_installed();
        }
        _ => (),
    }

    let mut has_snap: bool = false;
    let mut snap_installed: Vec<String> = vec![];
    match Command::new("snap").arg("--version").output() {
        Ok(_) => {
            has_snap = true;
            snap_installed = snap::get_installed();
        }
        _ => (),
    }

    let mut info: Info = Info {
        has_gnome,
        has_kde,
        has_flatpak,
        has_snap,
        repository_installed,
        flatpak_installed,
        snap_installed,
    };
    run_menu(0, &distribution, &mut info);
}
