use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::process::Command;
use std::process::Stdio;
use std::slice::Iter;

use rust_cli::commands::Operation;

use crate::distribution::{DesktopEnvironment, Distribution, PackageManager, Repository};
use crate::flatpak::Flatpak;
use crate::helper;
use crate::other::OtherPackage;
use crate::snap::Snap;

use crate::InstallMethod;

#[derive(PartialEq)]
pub enum Category {
    Server,
    Desktop,
    Applications,
    Browsers,
    Communication,
    Games,
    MultiMedia,
    Editors,
    Software,
    Utilities,
}

impl Category {
    pub fn as_str(&self) -> &'static str {
        match self {
            Category::Server => "Server",
            Category::Desktop => "Desktop",
            Category::Applications => "Applications",
            Category::Browsers => "Browsers",
            Category::Communication => "Communication",
            Category::Games => "Games",
            Category::MultiMedia => "Multi Media",
            Category::Editors => "Editors",
            Category::Software => "Software",
            Category::Utilities => "Utilities",
        }
    }

    pub fn iterator() -> Iter<'static, Category> {
        static CATEGORIES: [Category; 10] = [
            Category::Server,
            Category::Desktop,
            Category::Applications,
            Category::Browsers,
            Category::Communication,
            Category::Games,
            Category::MultiMedia,
            Category::Editors,
            Category::Software,
            Category::Utilities,
        ];
        CATEGORIES.iter()
    }
}

pub struct Package {
    pub name: &'static str,
    pub desktop_environment: Option<DesktopEnvironment>,
    pub repository: HashMap<Repository, Vec<&'static str>>,
    pub flatpak: Option<Flatpak>,
    pub snap: Option<Snap>,
    pub other: Option<OtherPackage>,
    pub pre_install:
        Option<Box<dyn Fn(&mut Distribution, &InstallMethod) -> Result<(), io::Error>>>,
    pub post_install:
        Option<Box<dyn Fn(&mut Distribution, &InstallMethod) -> Result<(), io::Error>>>,
}

impl Package {
    pub fn new() -> Self {
        Self {
            name: "",
            desktop_environment: None,
            repository: HashMap::new(),
            flatpak: None,
            snap: None,
            other: None,
            pre_install: None,
            post_install: None,
        }
    }
}

pub fn get_category_packages(category: &Category) -> Vec<Package> {
    return match category {
        Category::Server => Vec::from([
            Package {
                name: "Cockpit - Web Interface",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["cockpit"]),
                    (Repository::Debian, vec!["cockpit"]),
                    (Repository::Fedora, vec!["cockpit"]),
                    (Repository::RedHat, vec!["cockpit"]),
                    (Repository::Ubuntu, vec!["cockpit"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "cURL - Client URL",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["curl"]),
                    (Repository::Debian, vec!["curl"]),
                    (Repository::Fedora, vec!["curl"]),
                    (Repository::RedHat, vec!["curl"]),
                    (Repository::Ubuntu, vec!["curl"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "dotnet - C# runtime 8.0 LTS",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["dotnet-runtime-8.0"]),
                    (Repository::Debian, vec!["dotnet-runtime-8.0"]),
                    (Repository::Fedora, vec!["dotnet-runtime-8.0"]),
                    (Repository::RedHat, vec!["dotnet-runtime-8.0"]),
                    (Repository::Ubuntu, vec!["dotnet-runtime-8.0"]),
                ]),
                flatpak: None,
                snap: Some(Snap {
                    name: "dotnet-runtime-80",
                    is_official: true,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: Some(Box::new(|distribution: &mut Distribution, method: &InstallMethod| {
                    if method == &InstallMethod::Repository {
                        if distribution.repository == Repository::Debian {
                            distribution.install_package("wget")?;

                            Operation::new("wget https://packages.microsoft.com/config/debian/12/packages-microsoft-prod.deb -O packages-microsoft-prod.deb").run()?;
                            Operation::new("sudo dpkg -i packages-microsoft-prod.deb").run()?;
                            Operation::new("rm packages-microsoft-prod.deb").run()?;
                            Operation::new("sudo apt update").run()?;
                        }
                    }
                    Ok(())
                })),
                post_install: None,
            },
            Package {
                name: "dotnet - C# SDK 8.0 LTS",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["dotnet-sdk-8.0"]),
                    (Repository::Debian, vec!["dotnet-sdk-8.0"]),
                    (Repository::Fedora, vec!["dotnet-sdk-8.0"]),
                    (Repository::RedHat, vec!["dotnet-sdk-8.0"]),
                    (Repository::Ubuntu, vec!["dotnet-sdk-8.0"]),
                ]),
                flatpak: None,
                snap: Some(Snap {
                    name: "dotnet-sdk",
                    is_official: true,
                    is_classic: true,
                    channel: "8.0/stable",
                }),
                other: None,
                pre_install: Some(Box::new(|distribution: &mut Distribution, method: &InstallMethod| {
                    if method == &InstallMethod::Repository {
                        if distribution.repository == Repository::Debian {
                            distribution.install_package("wget")?;

                            Operation::new("wget https://packages.microsoft.com/config/debian/12/packages-microsoft-prod.deb -O packages-microsoft-prod.deb").run()?;
                            Operation::new("sudo dpkg -i packages-microsoft-prod.deb").run()?;
                            Operation::new("rm packages-microsoft-prod.deb").run()?;
                            Operation::new("sudo apt update").run()?;
                        }
                    }
                    let home_dir: String = env::var("HOME").expect("HOME directory could not be determined");
                    if method == &InstallMethod::Uninstall {
                        Operation::new(format!("rm -r {}{} {}{}", &home_dir, "/.dotnet", &home_dir, "/.nuget"))
                            .hide_output(true)
                            .run()?;
                    }
                    let bashrc: String = format!("{}{}", &home_dir, "/.bashrc");
                    if method != &InstallMethod::Uninstall {
                        helper::append_to_file_if_not_found(
                            bashrc.as_str(),
                            "export DOTNET_CLI_TELEMETRY_OPTOUT",
                            r#"
export DOTNET_CLI_TELEMETRY_OPTOUT=true
"#,
                            false,
                        )?;
                    }
                    Ok(())
                })),
                post_install: None,
            },
            Package {
                name: "Flatpak",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["flatpak"]),
                    (Repository::Debian, vec!["flatpak"]),
                    (Repository::Fedora, vec!["flatpak"]),
                    (Repository::RedHat, vec!["flatpak"]),
                    (Repository::Ubuntu, vec!["flatpak"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Flutter",
                desktop_environment: None,
                repository: HashMap::new(),
                flatpak: None,
                snap: Some(Snap {
                    name: "flutter",
                    is_official: true,
                    is_classic: true,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "git - Version Control",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["git"]),
                    (Repository::Debian, vec!["git"]),
                    (Repository::Fedora, vec!["git"]),
                    (Repository::RedHat, vec!["git"]),
                    (Repository::Ubuntu, vec!["git"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Go Language",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["go", "gopls"]),
                    (Repository::Debian, vec!["golang", "gopls"]),
                    (Repository::Fedora, vec!["golang", "golang-x-tools-gopls"]),
                    (Repository::RedHat, vec!["golang"]),
                    (Repository::Ubuntu, vec!["golang", "gopls"]),
                ]),
                flatpak: None,
                snap: Some(Snap {
                    name: "go",
                    is_official: true,
                    is_classic: true,
                    channel: "",
                }),
                other: None,
                pre_install: Some(Box::new(|_: &mut Distribution, method: &InstallMethod| {
                    let home_dir: String = env::var("HOME").expect("HOME directory could not be determined");
                    if method == &InstallMethod::Uninstall {
                        Operation::new(format!("sudo rm -r {}{}", &home_dir, "/.go")).hide_output(true).run()?;
                    }
                    Ok(())
                })),
                post_install: Some(Box::new(|distribution: &mut Distribution, method: &InstallMethod| {
                    let home_dir: String = env::var("HOME").expect("HOME directory could not be determined");
                    let bashrc: String = format!("{}{}", &home_dir, "/.bashrc");
                    if method != &InstallMethod::Uninstall {
                        Operation::new(format!("go env -w GOPATH={}/.go", &home_dir)).hide_output(true).run()?;
                        if method == &InstallMethod::Snap || distribution.repository == Repository::RedHat {
                            helper::append_to_file_if_not_found(
                                bashrc.as_str(),
                                "export GOPATH",
                                r#"
export GOPATH=$HOME/.go
export PATH=$PATH:$GOPATH/bin
"#,
                                false,
                            )?;

                            Operation::new("go install golang.org/x/tools/gopls@latest").run()?;
                        }
                    }
                    Ok(())
                })),
            },
            Package {
                name: "htop - Process Reviewer",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["htop"]),
                    (Repository::Debian, vec!["htop"]),
                    (Repository::Fedora, vec!["htop"]),
                    (Repository::RedHat, vec!["htop"]),
                    (Repository::Ubuntu, vec!["htop"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "MariaDB - Database",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["mariadb"]),
                    (Repository::Debian, vec!["mariadb-server"]),
                    (Repository::Fedora, vec!["mariadb-server"]),
                    (Repository::RedHat, vec!["mariadb-server"]),
                    (Repository::Ubuntu, vec!["mariadb-server"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "neovim - Text Editor",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["neovim", "vim", "vim-airline", "vim-ale", "vim-ctrlp", "vim-gitgutter", "vim-nerdtree"]),
                    (Repository::Debian, vec!["neovim", "vim", "vim-airline", "vim-ale", "vim-ctrlp", "vim-gitgutter"]),
                    (Repository::Fedora, vec!["neovim", "vim-enhanced", "vim-airline", "vim-ale", "vim-ctrlp", "vim-gitgutter", "vim-nerdtree"]),
                    (Repository::Ubuntu, vec!["neovim", "vim", "vim-airline", "vim-ale", "vim-ctrlp", "vim-gitgutter"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: Some(Box::new(|_: &mut Distribution, method: &InstallMethod| {
                    if method == &InstallMethod::Uninstall {
                        let home_dir: String = env::var("HOME").expect("HOME directory could not be determined");
                        Operation::new(format!("sudo rm -r {}{}", &home_dir, "/.config/nvim")).hide_output(true).run()?;
                    }
                    Ok(())
                })),
                post_install: Some(Box::new(|distribution: &mut Distribution, method: &InstallMethod| {
                    let home_dir: String = env::var("HOME").expect("HOME directory could not be determined");
                    if method != &InstallMethod::Uninstall {
                        let config_file: String = format!("{}{}", &home_dir, "/.config/nvim/init.vim");

                        Operation::new(format!("mkdir -p {}/.config/nvim", &home_dir)).hide_output(true).run()?;

                        fs::write(
                            &config_file,
                            r#"""""""""""""""""""""""""""""""""""""""""
" neovim settings

set noswapfile
set nobackup
set nowritebackup

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
                                    config_file.as_str(),
                                    "NERDTree",
                                    "nnoremap <C-n> :NERDTreeToggle<CR>",
                                    false,
                                )?;
                            }

                            helper::append_to_file_if_not_found(
                                config_file.as_str(),
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
                    Ok(())
                })),
            },
            Package {
                name: "nano - Text Editor",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["nano"]),
                    (Repository::Debian, vec!["nano"]),
                    (Repository::Fedora, vec!["nano"]),
                    (Repository::RedHat, vec!["nano"]),
                    (Repository::Ubuntu, vec!["nano"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Node.js - JavaScript RE",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["nodejs", "npm"]),
                    (Repository::Debian, vec!["nodejs", "npm"]),
                    (Repository::Fedora, vec!["nodejs", "npm"]),
                    (Repository::RedHat, vec!["nodejs", "npm"]),
                    (Repository::Ubuntu, vec!["nodejs", "npm"]),
                ]),
                flatpak: None,
                snap: Some(Snap {
                    name: "node",
                    is_official: true,
                    is_classic: true,
                    channel: "18/stable",
                }),
                other: None,
                pre_install: Some(Box::new(|distribution: &mut Distribution, method: &InstallMethod| {
                    if method == &InstallMethod::Repository {
                        if distribution.repository == Repository::RedHat {
                            Operation::new("sudo dnf module enable nodejs:20 -y").hide_output(true).run()?;
                        }
                    }
                    Ok(())
                })),
                post_install: None,
            },
            Package {
                name: "Podman - Containers",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["podman"]),
                    (Repository::Debian, vec!["podman"]),
                    (Repository::Fedora, vec!["podman"]),
                    (Repository::RedHat, vec!["podman"]),
                    (Repository::Ubuntu, vec!["podman"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Rust Language",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["rust"]),
                    (Repository::Debian, vec!["rustc", "rustfmt", "cargo"]),
                    (Repository::Fedora, vec!["rust", "rustfmt", "cargo"]),
                    (Repository::RedHat, vec!["rust", "rustfmt", "cargo"]),
                    (Repository::Ubuntu, vec!["rustc", "rustfmt", "cargo"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: Some(Box::new(|_: &mut Distribution, method: &InstallMethod| {
                    let home_dir: String = env::var("HOME").expect("HOME directory could not be determined");
                    if method == &InstallMethod::Uninstall {
                        Operation::new(format!("rm -r {}{}", &home_dir, "/.cargo"))
                            .hide_output(true)
                            .run()?;
                    }
                    Ok(())
                })),
                post_install: None,
            },
            Package {
                name: "rustup",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["rustup"]),
                    (Repository::Fedora, vec!["rustup"]),
                    (Repository::Ubuntu, vec!["rustup"]),
                ]),
                flatpak: None,
                snap: Some(Snap {
                    name: "rustup",
                    is_official: true,
                    is_classic: true,
                    channel: "",
                }),
                other: Some(OtherPackage { name: "rustup" }),
                pre_install: Some(Box::new(|distribution: &mut Distribution, method: &InstallMethod| {
                    let home_dir: String = env::var("HOME").expect("HOME directory could not be determined");
                    if method == &InstallMethod::Uninstall {
                        Operation::new(format!("rm -r {}{}", &home_dir, "/.cargo"))
                            .hide_output(true)
                            .run()?;
                    }
                    if method == &InstallMethod::Other {
                        distribution.install_package("curl")?;
                    }
                    Ok(())
                })),
                post_install: Some(Box::new(|_: &mut Distribution, method: &InstallMethod| {
                    let home_dir: String = env::var("HOME").expect("HOME directory could not be determined");
                    if method == &InstallMethod::Other {
                        Operation::new(format!("{}{} component add rust-analyzer", &home_dir, "/.cargo/bin/rustup"))
                            .hide_output(true)
                            .run()?;
                    } else {
                        Operation::new("rustup component add rust-analyzer")
                            .hide_output(true)
                            .run()?;
                    }
                    Ok(())
                })),
            },
            Package {
                name: "Snap",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Debian, vec!["snapd"]),
                    (Repository::Fedora, vec!["snapd"]),
                    (Repository::RedHat, vec!["snapd"]),
                    (Repository::Ubuntu, vec!["snapd"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: Some(Box::new(|distribution: &mut Distribution, method: &InstallMethod| {
                    if method != &InstallMethod::Uninstall {
                        distribution.setup_snap()?;
                    }
                    Ok(())
                })),
            },
            Package {
                name: "SSH - Secure Shell Protocol",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["libssh", "openssh"]),
                    (Repository::Debian, vec!["ssh"]),
                    (Repository::Fedora, vec!["libssh", "openssh"]),
                    (Repository::RedHat, vec!["libssh", "openssh"]),
                    (Repository::Ubuntu, vec!["ssh"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "vim - Text Editor",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["vim", "vim-airline", "vim-ale", "vim-ctrlp", "vim-gitgutter", "vim-nerdtree"]),
                    (Repository::Debian, vec!["vim", "vim-airline", "vim-ale", "vim-ctrlp", "vim-gitgutter"]),
                    (Repository::Fedora, vec!["vim-enhanced", "vim-airline", "vim-ale", "vim-ctrlp", "vim-gitgutter", "vim-nerdtree"]),
                    (Repository::RedHat, vec!["vim-enhanced"]),
                    (Repository::Ubuntu, vec!["vim", "vim-airline", "vim-ale", "vim-ctrlp", "vim-gitgutter"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: Some(Box::new(|_: &mut Distribution, method: &InstallMethod| {
                    let home_dir: String = env::var("HOME").expect("HOME directory could not be determined");
                    if method == &InstallMethod::Uninstall {
                        Operation::new(format!("sudo rm -r {}{} {}{} {}{}", &home_dir, "/.vim", &home_dir, "/.viminfo", &home_dir, "/.vimrc"))
                            .hide_output(true)
                            .run()?;
                    }
                    Ok(())
                })),
                post_install: Some(Box::new(|distribution: &mut Distribution, method: &InstallMethod| {
                    let home_dir: String = env::var("HOME").expect("HOME directory could not be determined");
                    let bashrc: String = format!("{}{}", &home_dir, "/.bashrc");
                    if method != &InstallMethod::Uninstall {
                        helper::append_to_file_if_not_found(
                            bashrc.as_str(),
                            "export EDITOR",
                            "export EDITOR=\"/usr/bin/vim\"\n",
                            false,
                        )?;

                        let config_file: String = format!("{}{}", &home_dir, "/.vimrc");

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
                                    config_file.as_str(),
                                    "NERDTree",
                                    "nnoremap <C-n> :NERDTreeToggle<CR>",
                                    false,
                                )?;
                            }

                            helper::append_to_file_if_not_found(
                                config_file.as_str(),
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
                    Ok(())
                })),
            },
        ]),
        Category::Desktop => Vec::from([
            Package {
                name: "cups - Printer Support",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["cups"]),
                    (Repository::Debian, vec!["cups"]),
                    (Repository::Fedora, vec!["cups"]),
                    (Repository::RedHat, vec!["cups"]),
                    (Repository::Ubuntu, vec!["cups"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "ffmpeg - Media Codecs",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["ffmpeg"]),
                    (Repository::Debian, vec!["ffmpeg"]),
                    (Repository::Fedora, vec!["ffmpeg"]),
                    (Repository::RedHat, vec!["ffmpeg"]),
                    (Repository::Ubuntu, vec!["ffmpeg"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "imagemagick",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["imagemagick"]),
                    (Repository::Debian, vec!["imagemagick"]),
                    (Repository::Fedora, vec!["ImageMagick"]),
                    (Repository::RedHat, vec!["ImageMagick"]),
                    (Repository::Ubuntu, vec!["imagemagick"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "LaTex - Compiler",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["texlive-core", "texlive-latexextra"]),
                    (Repository::Debian, vec!["texlive-latex-base", "texlive-latex-extra"]),
                    (Repository::Fedora, vec!["texlive-latex", "texlive-collection-latexextra"]),
                    (Repository::RedHat, vec!["texlive-latex"]),
                    (Repository::Ubuntu, vec!["texlive-latex-base", "texlive-latex-extra"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "MP3 Metadata Editor",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["id3v2"]),
                    (Repository::Debian, vec!["id3v2"]),
                    (Repository::Fedora, vec!["id3v2"]),
                    (Repository::Ubuntu, vec!["id3v2"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "qtile - Window Manager",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["qtile", "alacritty", "rofi", "numlockx", "playerctl"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Vietnamese Keyboard",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["ibus-unikey"]),
                    (Repository::Debian, vec!["ibus-unikey"]),
                    (Repository::Fedora, vec!["ibus-unikey"]),
                    (Repository::RedHat, vec!["https://rpmfind.net/linux/fedora/linux/releases/34/Everything/x86_64/os/Packages/i/ibus-unikey-0.6.1-26.20190311git46b5b9e.fc34.x86_64.rpm"]),
                    (Repository::Ubuntu, vec!["ibus-unikey"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "yt-dlp - Download YouTube",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["yt-dlp"]),
                    (Repository::Debian, vec!["yt-dlp"]),
                    (Repository::Fedora, vec!["yt-dlp"]),
                    (Repository::RedHat, vec!["yt-dlp"]),
                    (Repository::Ubuntu, vec!["yt-dlp"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
        ]),
        Category::Applications => Vec::from([
            Package {
                name: "Cheese - Webcam",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["cheese"]),
                    (Repository::Debian, vec!["cheese"]),
                    (Repository::Fedora, vec!["cheese"]),
                    (Repository::RedHat, vec!["cheese"]),
                    (Repository::Ubuntu, vec!["cheese"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.Cheese",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Deja Dup - Backups",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["deja-dup"]),
                    (Repository::Debian, vec!["deja-dup"]),
                    (Repository::Fedora, vec!["deja-dup"]),
                    (Repository::Ubuntu, vec!["deja-dup"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.DejaDup",
                    is_verified: true,
                    remotes: vec!["flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Evince - Document Viewer",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["evince"]),
                    (Repository::Debian, vec!["evince"]),
                    (Repository::Fedora, vec!["evince"]),
                    (Repository::RedHat, vec!["evince"]),
                    (Repository::Ubuntu, vec!["evince"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.Evince",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Eye of Gnome - Image Viewer",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["eog"]),
                    (Repository::Debian, vec!["eog"]),
                    (Repository::Fedora, vec!["eog"]),
                    (Repository::RedHat, vec!["eog"]),
                    (Repository::Ubuntu, vec!["eog"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.eog",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: Some(Snap {
                    name: "eog",
                    is_official: true,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Boxes - VM Manager",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gnome-boxes"]),
                    (Repository::Debian, vec!["gnome-boxes"]),
                    (Repository::Fedora, vec!["gnome-boxes"]),
                    (Repository::Ubuntu, vec!["gnome-boxes"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.Boxes",
                    is_verified: true,
                    remotes: vec!["flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Calculator",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gnome-calculator"]),
                    (Repository::Debian, vec!["gnome-calculator"]),
                    (Repository::Fedora, vec!["gnome-calculator"]),
                    (Repository::RedHat, vec!["gnome-calculator"]),
                    (Repository::Ubuntu, vec!["gnome-calculator"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.Calculator",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: Some(Snap {
                    name: "gnome-calculator",
                    is_official: true,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Calendar",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gnome-calendar"]),
                    (Repository::Debian, vec!["gnome-calendar"]),
                    (Repository::Fedora, vec!["gnome-calendar"]),
                    (Repository::Ubuntu, vec!["gnome-calendar"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.Calendar",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Clocks",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gnome-clocks"]),
                    (Repository::Debian, vec!["gnome-clocks"]),
                    (Repository::Fedora, vec!["gnome-clocks"]),
                    (Repository::Ubuntu, vec!["gnome-clocks"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.clocks",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: Some(Snap {
                    name: "gnome-clocks",
                    is_official: true,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Camera",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["snapshot"]),
                    (Repository::Fedora, vec!["snapshot"]),
                    (Repository::Ubuntu, vec!["gnome-snapshot"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.Snapshot",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Connections",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gnome-connections"]),
                    (Repository::Debian, vec!["gnome-connections"]),
                    (Repository::Fedora, vec!["gnome-connections"]),
                    (Repository::RedHat, vec!["gnome-connections"]),
                    (Repository::Ubuntu, vec!["gnome-connections"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.Connections",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Contacts",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gnome-contacts"]),
                    (Repository::Debian, vec!["gnome-contacts"]),
                    (Repository::Fedora, vec!["gnome-contacts"]),
                    (Repository::Ubuntu, vec!["gnome-contacts"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.Contacts",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Image Viewer",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Fedora, vec!["loupe"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.Loupe",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: Some(Snap {
                    name: "loupe",
                    is_official: true,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Maps",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gnome-maps"]),
                    (Repository::Debian, vec!["gnome-maps"]),
                    (Repository::Fedora, vec!["gnome-maps"]),
                    (Repository::Ubuntu, vec!["gnome-maps"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.Maps",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Password Safe",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gnome-passwordsafe"]),
                    (Repository::Debian, vec!["gnome-passwordsafe"]),
                    (Repository::Fedora, vec!["secrets"]),
                    (Repository::Ubuntu, vec!["gnome-passwordsafe"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.World.Secrets",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Weather",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gnome-weather"]),
                    (Repository::Debian, vec!["gnome-weather"]),
                    (Repository::Fedora, vec!["gnome-weather"]),
                    (Repository::Ubuntu, vec!["gnome-weather"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.Weather",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "GNU Cash - Accounting",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["gnucash"]),
                    (Repository::Debian, vec!["gnucash"]),
                    (Repository::Fedora, vec!["gnucash"]),
                    (Repository::Ubuntu, vec!["gnucash"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnucash.GnuCash",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gwenview - Image Viewer",
                desktop_environment: Some(DesktopEnvironment::KDE),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gwenview"]),
                    (Repository::Debian, vec!["gwenview"]),
                    (Repository::Fedora, vec!["gwenview"]),
                    (Repository::Ubuntu, vec!["gwenview"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.kde.gwenview",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: Some(Snap {
                    name: "gwenview",
                    is_official: true,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "KCalc - Calculator",
                desktop_environment: Some(DesktopEnvironment::KDE),
                repository: HashMap::from([
                    (Repository::Arch, vec!["kcalc"]),
                    (Repository::Debian, vec!["kcalc"]),
                    (Repository::Fedora, vec!["kcalc"]),
                    (Repository::RedHat, vec!["kcalc"]),
                    (Repository::Ubuntu, vec!["kcalc"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.kde.kcalc",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: Some(Snap {
                    name: "kcalc",
                    is_official: true,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Okular - Document Viewer",
                desktop_environment: Some(DesktopEnvironment::KDE),
                repository: HashMap::from([
                    (Repository::Arch, vec!["okular"]),
                    (Repository::Debian, vec!["okular"]),
                    (Repository::Fedora, vec!["okular"]),
                    (Repository::Ubuntu, vec!["okular"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.kde.okular",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: Some(Snap {
                    name: "okular",
                    is_official: true,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Transmission (GTK) - Torrent",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["transmission-gtk"]),
                    (Repository::Debian, vec!["transmission-gtk"]),
                    (Repository::Fedora, vec!["transmission-gtk"]),
                    (Repository::Ubuntu, vec!["transmission-gtk"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "com.transmissionbt.Transmission",
                    is_verified: false,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Transmission (QT) - Torrent",
                desktop_environment: Some(DesktopEnvironment::KDE),
                repository: HashMap::from([
                    (Repository::Arch, vec!["transmission-qt"]),
                    (Repository::Debian, vec!["transmission-qt"]),
                    (Repository::Fedora, vec!["transmission-qt"]),
                    (Repository::Ubuntu, vec!["transmission-qt"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "com.transmissionbt.Transmission",
                    is_verified: false,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Virt Manager",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["virt-manager"]),
                    (Repository::Debian, vec!["virt-manager"]),
                    (Repository::Fedora, vec!["virt-manager"]),
                    (Repository::RedHat, vec!["virt-manager"]),
                    (Repository::Ubuntu, vec!["virt-manager"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
        ]),
        Category::Browsers => Vec::from([
            Package {
                name: "Chromium",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["chromium"]),
                    (Repository::Debian, vec!["chromium"]),
                    (Repository::Fedora, vec!["chromium"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.chromium.Chromium",
                    is_verified: false,
                    remotes: vec!["flathub"],
                }),
                snap: Some(Snap {
                    name: "chromium",
                    is_official: true,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Epiphany - Gnome Web",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["epiphany"]),
                    (Repository::Debian, vec!["epiphany-browser"]),
                    (Repository::Fedora, vec!["epiphany"]),
                    (Repository::Ubuntu, vec!["epiphany-browser"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.Epiphany",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Firefox",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["firefox"]),
                    (Repository::Fedora, vec!["firefox"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.mozilla.firefox",
                    is_verified: true,
                    remotes: vec!["flathub"],
                }),
                snap: Some(Snap {
                    name: "firefox",
                    is_official: true,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Firefox ESR",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Debian, vec!["firefox-esr"]),
                    (Repository::RedHat, vec!["firefox"]),
                ]),
                flatpak: None,
                snap: Some(Snap {
                    name: "firefox",
                    is_official: true,
                    is_classic: false,
                    channel: "esr-stable",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "IceCat - GNU Browser",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Fedora, vec!["icecat"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "TOR - The Onion Router",
                desktop_environment: None,
                repository: HashMap::new(),
                flatpak: Some(Flatpak {
                    name: "com.github.micahflee.torbrowser-launcher",
                    is_verified: false,
                    remotes: vec!["flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
        ]),
        Category::Communication => Vec::from([
            Package {
                name: "Discord",
                desktop_environment: None,
                repository: HashMap::from([(Repository::Arch, vec!["discord"])]),
                flatpak: Some(Flatpak {
                    name: "com.discordapp.Discord",
                    is_verified: true,
                    remotes: vec!["flathub"],
                }),
                snap: Some(Snap {
                    name: "discord",
                    is_official: false,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Thunderbird",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["thunderbird"]),
                    (Repository::Debian, vec!["thunderbird"]),
                    (Repository::Fedora, vec!["thunderbird"]),
                    (Repository::RedHat, vec!["thunderbird"]),
                    (Repository::Ubuntu, vec!["thunderbird"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.mozilla.Thunderbird",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: Some(Snap {
                    name: "thunderbird",
                    is_official: true,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
        ]),
        Category::Games => Vec::from([
            Package {
                name: "0 A.D.",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["0ad"]),
                    (Repository::Debian, vec!["0ad"]),
                    (Repository::Fedora, vec!["0ad"]),
                    (Repository::Ubuntu, vec!["0ad"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "com.play0ad.zeroad",
                    is_verified: false,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: Some(Snap {
                    name: "0ad",
                    is_official: true,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome 2048",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gnome-2048"]),
                    (Repository::Debian, vec!["gnome-2048"]),
                    (Repository::Fedora, vec!["gnome-2048"]),
                    (Repository::Ubuntu, vec!["gnome-2048"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.TwentyFortyEight",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Chess",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gnome-chess"]),
                    (Repository::Debian, vec!["gnome-chess"]),
                    (Repository::Fedora, vec!["gnome-chess"]),
                    (Repository::Ubuntu, vec!["gnome-chess"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.Chess",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Mines",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gnome-mines"]),
                    (Repository::Debian, vec!["gnome-mines"]),
                    (Repository::Fedora, vec!["gnome-mines"]),
                    (Repository::Ubuntu, vec!["gnome-mines"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.Mines",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Solitaire",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["aisleriot"]),
                    (Repository::Debian, vec!["aisleriot"]),
                    (Repository::Fedora, vec!["aisleriot"]),
                    (Repository::Ubuntu, vec!["aisleriot"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.Aisleriot",
                    is_verified: false,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Sudoku",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gnome-sudoku"]),
                    (Repository::Debian, vec!["gnome-sudoku"]),
                    (Repository::Fedora, vec!["gnome-sudoku"]),
                    (Repository::Ubuntu, vec!["gnome-sudoku"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.Sudoku",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: Some(Snap {
                    name: "gnome-sudoku",
                    is_official: true,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Tetris",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["quadrapassel"]),
                    (Repository::Debian, vec!["quadrapassel"]),
                    (Repository::Fedora, vec!["quadrapassel"]),
                    (Repository::Ubuntu, vec!["quadrapassel"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.Quadrapassel",
                    is_verified: false,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: Some(Snap {
                    name: "quadrapassel",
                    is_official: true,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "KDE Chess",
                desktop_environment: Some(DesktopEnvironment::KDE),
                repository: HashMap::from([
                    (Repository::Arch, vec!["knights"]),
                    (Repository::Debian, vec!["knights"]),
                    (Repository::Fedora, vec!["knights"]),
                    (Repository::Ubuntu, vec!["knights"]),
                ]),
                flatpak: None,
                snap: Some(Snap {
                    name: "knights",
                    is_official: true,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "KDE Mines",
                desktop_environment: Some(DesktopEnvironment::KDE),
                repository: HashMap::from([
                    (Repository::Arch, vec!["kmines"]),
                    (Repository::Debian, vec!["kmines"]),
                    (Repository::Fedora, vec!["kmines"]),
                    (Repository::RedHat, vec!["kmines"]),
                    (Repository::Ubuntu, vec!["kmines"]),
                ]),
                flatpak: None,
                snap: Some(Snap {
                    name: "kmines",
                    is_official: true,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "KDE Sudoku",
                desktop_environment: Some(DesktopEnvironment::KDE),
                repository: HashMap::from([
                    (Repository::Arch, vec!["ksudoku"]),
                    (Repository::Debian, vec!["ksudoku"]),
                    (Repository::Fedora, vec!["ksudoku"]),
                    (Repository::RedHat, vec!["ksudoku"]),
                    (Repository::Ubuntu, vec!["ksudoku"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.kde.ksudoku",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: Some(Snap {
                    name: "ksudoku",
                    is_official: true,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Steam",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["steam"]),
                    (Repository::Fedora, vec!["steam"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "com.valvesoftware.Steam",
                    is_verified: false,
                    remotes: vec!["flathub"],
                }),
                snap: Some(Snap {
                    name: "steam",
                    is_official: true,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: Some(Box::new(|distribution: &mut Distribution, method: &InstallMethod| {
                    if method != &InstallMethod::Repository {
                        if distribution.repository == Repository::Fedora {
                            Operation::new("sudo dnf config-manager --set-disabled rpmfusion-nonfree-steam")
                                .hide_output(true)
                                .run()?;
                        }
                    }
                    if method == &InstallMethod::Repository {
                        if distribution.repository == Repository::Fedora {
                            Operation::new("sudo dnf config-manager --set-enabled rpmfusion-nonfree-steam")
                                .hide_output(true)
                                .run()?;
                        }
                    }
                    Ok(())
                })),
                post_install: None,
            },
            Package {
                name: "Super Tux Kart",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["supertuxkart"]),
                    (Repository::Debian, vec!["supertuxkart"]),
                    (Repository::Fedora, vec!["supertuxkart"]),
                    (Repository::Ubuntu, vec!["supertuxkart"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "net.supertuxkart.SuperTuxKart",
                    is_verified: false,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: Some(Snap {
                    name: "supertuxkart",
                    is_official: false,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Xonotic",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["xonotic"]),
                    (Repository::Fedora, vec!["xonotic"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.xonotic.Xonotic",
                    is_verified: false,
                    remotes: vec!["flathub"],
                }),
                snap: Some(Snap {
                    name: "xonotic",
                    is_official: false,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
        ]),
        Category::MultiMedia => Vec::from([
            Package {
                name: "Blender",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["blender"]),
                    (Repository::Debian, vec!["blender"]),
                    (Repository::Fedora, vec!["blender"]),
                    (Repository::Ubuntu, vec!["blender"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.blender.Blender",
                    is_verified: false,
                    remotes: vec!["flathub"],
                }),
                snap: Some(Snap {
                    name: "blender",
                    is_official: true,
                    is_classic: true,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Elisa Music Player",
                desktop_environment: Some(DesktopEnvironment::KDE),
                repository: HashMap::from([
                    (Repository::Arch, vec!["elisa"]),
                    (Repository::Debian, vec!["elisa"]),
                    (Repository::Fedora, vec!["elisa"]),
                    (Repository::Ubuntu, vec!["elisa"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.kde.elisa",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "GIMP",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["gimp"]),
                    (Repository::Debian, vec!["gimp"]),
                    (Repository::Fedora, vec!["gimp"]),
                    (Repository::RedHat, vec!["gimp"]),
                    (Repository::Ubuntu, vec!["gimp"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gimp.GIMP",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: Some(Snap {
                    name: "gimp",
                    is_official: false,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Music",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gnome-music"]),
                    (Repository::Debian, vec!["gnome-music"]),
                    (Repository::Fedora, vec!["gnome-music"]),
                    (Repository::Ubuntu, vec!["gnome-music"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.Music",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Photos",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gnome-photos"]),
                    (Repository::Debian, vec!["gnome-photos"]),
                    (Repository::Fedora, vec!["gnome-photos"]),
                    (Repository::RedHat, vec!["gnome-photos"]),
                    (Repository::Ubuntu, vec!["gnome-photos"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.Photos",
                    is_verified: false,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Sound Recorder",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gnome-sound-recorder"]),
                    (Repository::Debian, vec!["gnome-sound-recorder"]),
                    (Repository::Fedora, vec!["gnome-sound-recorder"]),
                    (Repository::Ubuntu, vec!["gnome-sound-recorder"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.SoundRecorder",
                    is_verified: false,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "KdenLive Video Editor",
                desktop_environment: Some(DesktopEnvironment::KDE),
                repository: HashMap::from([
                    (Repository::Arch, vec!["kdenlive"]),
                    (Repository::Debian, vec!["kdenlive"]),
                    (Repository::Fedora, vec!["kdenlive"]),
                    (Repository::Ubuntu, vec!["kdenlive"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.kde.kdenlive",
                    is_verified: true,
                    remotes: vec!["flathub"],
                }),
                snap: Some(Snap {
                    name: "kdenlive",
                    is_official: true,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "RhythmBox",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["rhythmbox"]),
                    (Repository::Debian, vec!["rhythmbox"]),
                    (Repository::Fedora, vec!["rhythmbox"]),
                    (Repository::Ubuntu, vec!["rhythmbox"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.Rhythmbox3",
                    is_verified: false,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Shotwell",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["shotwell"]),
                    (Repository::Debian, vec!["shotwell"]),
                    (Repository::Fedora, vec!["shotwell"]),
                    (Repository::Ubuntu, vec!["shotwell"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.Shotwell",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Totem Video Player",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["totem"]),
                    (Repository::Debian, vec!["totem"]),
                    (Repository::Fedora, vec!["totem"]),
                    (Repository::RedHat, vec!["totem"]),
                    (Repository::Ubuntu, vec!["totem"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.Totem",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "VLC",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["vlc"]),
                    (Repository::Debian, vec!["vlc"]),
                    (Repository::Fedora, vec!["vlc"]),
                    (Repository::RedHat, vec!["vlc"]),
                    (Repository::Ubuntu, vec!["vlc"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.videolan.VLC",
                    is_verified: false,
                    remotes: vec!["flathub"],
                }),
                snap: Some(Snap {
                    name: "vlc",
                    is_official: true,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
        ]),
        Category::Editors => Vec::from([
            Package {
                name: "gedit",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gedit"]),
                    (Repository::Debian, vec!["gedit"]),
                    (Repository::Fedora, vec!["gedit"]),
                    (Repository::RedHat, vec!["gedit"]),
                    (Repository::Ubuntu, vec!["gedit"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.gedit",
                    is_verified: false,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: Some(Snap {
                    name: "gedit",
                    is_official: true,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Builder",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gnome-builder"]),
                    (Repository::Debian, vec!["gnome-builder"]),
                    (Repository::Fedora, vec!["gnome-builder"]),
                    (Repository::Ubuntu, vec!["gnome-builder"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.Builder",
                    is_verified: true,
                    remotes: vec!["flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Text Editor",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gnome-text-editor"]),
                    (Repository::Debian, vec!["gnome-text-editor"]),
                    (Repository::Fedora, vec!["gnome-text-editor"]),
                    (Repository::Ubuntu, vec!["gnome-text-editor"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.TextEditor",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Intellij",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["intellij-idea-community-edition"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "com.jetbrains.IntelliJ-IDEA-Community",
                    is_verified: false,
                    remotes: vec!["flathub"],
                }),
                snap: Some(Snap {
                    name: "intellij-idea-community",
                    is_official: true,
                    is_classic: true,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: Some(Box::new(|_: &mut Distribution, method: &InstallMethod| {
                    let home_dir: String = env::var("HOME").expect("HOME directory could not be determined");
                    if method != &InstallMethod::Uninstall {
                        fs::write(
                            format!("{}{}", &home_dir, "/.ideavimrc"),
                            "sethandler a:ide",
                        )?;
                    }
                    Ok(())
                })),
            },
            Package {
                name: "Kate",
                desktop_environment: Some(DesktopEnvironment::KDE),
                repository: HashMap::from([
                    (Repository::Arch, vec!["kate"]),
                    (Repository::Debian, vec!["kate"]),
                    (Repository::Fedora, vec!["kate"]),
                    (Repository::RedHat, vec!["kate"]),
                    (Repository::Ubuntu, vec!["kate"]),
                ]),
                flatpak: None,
                snap: Some(Snap {
                    name: "kate",
                    is_official: true,
                    is_classic: true,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "KDevelop",
                desktop_environment: Some(DesktopEnvironment::KDE),
                repository: HashMap::from([
                    (Repository::Arch, vec!["kdevelop"]),
                    (Repository::Debian, vec!["kdevelop"]),
                    (Repository::Fedora, vec!["kdevelop"]),
                    (Repository::Ubuntu, vec!["kdevelop"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.kde.kdevelop",
                    is_verified: true,
                    remotes: vec!["flathub"],
                }),
                snap: Some(Snap {
                    name: "kdevelop",
                    is_official: true,
                    is_classic: true,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Kile - LaTex Editor",
                desktop_environment: Some(DesktopEnvironment::KDE),
                repository: HashMap::from([
                    (Repository::Arch, vec!["kile"]),
                    (Repository::Debian, vec!["kile"]),
                    (Repository::Fedora, vec!["kile"]),
                    (Repository::Ubuntu, vec!["kile"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "KWrite",
                desktop_environment: Some(DesktopEnvironment::KDE),
                repository: HashMap::from([
                    (Repository::Arch, vec!["kwrite"]),
                    (Repository::Debian, vec!["kwrite"]),
                    (Repository::Fedora, vec!["kwrite"]),
                    (Repository::RedHat, vec!["kwrite"]),
                    (Repository::Ubuntu, vec!["kwrite"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.kde.kwrite",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "LibreOffice",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["libreoffice-fresh"]),
                    (Repository::Debian, vec!["libreoffice-writer", "libreoffice-calc", "libreoffice-impress", "libreoffice-draw", "libreoffice-base"]),
                    (Repository::Fedora, vec!["libreoffice-writer", "libreoffice-calc", "libreoffice-impress", "libreoffice-draw", "libreoffice-base"]),
                    (Repository::RedHat, vec!["libreoffice-writer", "libreoffice-calc", "libreoffice-impress", "libreoffice-draw", "libreoffice-base"]),
                    (Repository::Ubuntu, vec!["libreoffice-writer", "libreoffice-calc", "libreoffice-impress", "libreoffice-draw", "libreoffice-base"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.libreoffice.LibreOffice",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: Some(Snap {
                    name: "libreoffice",
                    is_official: true,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Pycharm",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["pycharm-community-edition"]),
                    (Repository::Fedora, vec!["pycharm-community"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "com.jetbrains.PyCharm-Community",
                    is_verified: false,
                    remotes: vec!["flathub"],
                }),
                snap: Some(Snap {
                    name: "pycharm-community",
                    is_official: true,
                    is_classic: true,
                    channel: "",
                }),
                other: None,
                pre_install: Some(Box::new(|distribution: &mut Distribution, method: &InstallMethod| {
                    if method != &InstallMethod::Repository {
                        if distribution.repository == Repository::Fedora {
                            Operation::new("sudo dnf config-manager --set-disabled phracek-PyCharm")
                                .hide_output(true)
                                .run()?;
                        }
                    }
                    if method == &InstallMethod::Repository {
                        if distribution.repository == Repository::Fedora {
                            Operation::new("sudo dnf config-manager --set-enabled phracek-PyCharm")
                                .hide_output(true)
                                .run()?;
                        }
                    }
                    Ok(())
                })),
                post_install: Some(Box::new(|_: &mut Distribution, method: &InstallMethod| {
                    let home_dir: String = env::var("HOME").expect("HOME directory could not be determined");
                    if method != &InstallMethod::Uninstall {
                        fs::write(
                            format!("{}{}", &home_dir, "/.ideavimrc"),
                            "sethandler a:ide",
                        )?;
                    }
                    Ok(())
                })),
            },
            Package {
                name: "VS Code",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["code"]),
                    (Repository::Debian, vec!["code"]),
                    (Repository::Fedora, vec!["code"]),
                    (Repository::RedHat, vec!["code"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "com.visualstudio.code",
                    is_verified: false,
                    remotes: vec!["flathub"],
                }),
                snap: Some(Snap {
                    name: "code",
                    is_official: true,
                    is_classic: true,
                    channel: "",
                }),
                other: None,
                pre_install: Some(Box::new(|distribution: &mut Distribution, method: &InstallMethod| {
                    let home_dir: String = env::var("HOME").expect("HOME directory could not be determined");
                    if method != &InstallMethod::Repository {
                        if distribution.package_manager == PackageManager::APT {
                            Operation::new("sudo rm /etc/apt/sources.list.d/vscode.list").hide_output(true).run()?;
                        }
                        if distribution.package_manager == PackageManager::DNF {
                            Operation::new("sudo dnf config-manager --set-disabled code").hide_output(true).run()?;
                            Operation::new("sudo rm /etc/yum.repos.d/vscode.repo").hide_output(true).run()?;
                        }
                    }
                    if method == &InstallMethod::Uninstall {
                        Operation::new(format!("sudo rm -r {}{} {}{}", &home_dir, "/.vscode", &home_dir, "/.config/Code"))
                            .hide_output(true)
                            .run()?;
                    }
                    if method == &InstallMethod::Repository {
                        if distribution.package_manager == PackageManager::APT {
                            distribution.install_package("wget")?;
                            distribution.install_package("gpg")?;

                            let key: String = Operation::new("wget -qO- https://packages.microsoft.com/keys/microsoft.asc").output()?;
                            fs::write("packages.microsoft", key)?;

                            Operation::new("gpg --dearmor packages.microsoft").hide_output(true).run()?;

                            Operation::new("sudo install -D -o root -g root -m 644 packages.microsoft.gpg /etc/apt/keyrings/packages.microsoft.gpg")
                                .hide_output(true)
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

                            Operation::new("sudo apt update").run()?;
                        }
                        if distribution.package_manager == PackageManager::DNF {
                            Operation::new("sudo rpm --import https://packages.microsoft.com/keys/microsoft.asc")
                                .hide_output(true)
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
                    Ok(())
                })),
                post_install: Some(Box::new(|_: &mut Distribution, method: &InstallMethod| {
                    let home_dir: String = env::var("HOME").expect("HOME directory could not be determined");
                    if method != &InstallMethod::Uninstall {
                        let extensions: Vec<&str> = Vec::from(["esbenp.prettier-vscode", "vscodevim.vim"]);
                        for ext in extensions {
                            Operation::new(format!("code --install-extension {}", ext)).run()?;
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
  "workbench.startupEditor": "none",
}
"#,
                        )?;
                    }
                    Ok(())
                })),
            },
        ]),
        Category::Software => Vec::from([
            Package {
                name: "Gnome Software",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gnome-software"]),
                    (Repository::Debian, vec!["gnome-software"]),
                    (Repository::Fedora, vec!["gnome-software"]),
                    (Repository::RedHat, vec!["gnome-software"]),
                    (Repository::Ubuntu, vec!["gnome-software"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Plasma Discover",
                desktop_environment: Some(DesktopEnvironment::KDE),
                repository: HashMap::from([
                    (Repository::Arch, vec!["discover"]),
                    (Repository::Debian, vec!["plasma-discover"]),
                    (Repository::Fedora, vec!["plasma-discover"]),
                    (Repository::Ubuntu, vec!["plasma-discover"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Snap Store",
                desktop_environment: None,
                repository: HashMap::new(),
                flatpak: None,
                snap: Some(Snap {
                    name: "snap-store",
                    is_official: true,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
        ]),
        Category::Utilities => Vec::from([
            Package {
                name: "Ark Archiving",
                desktop_environment: Some(DesktopEnvironment::KDE),
                repository: HashMap::from([
                    (Repository::Arch, vec!["ark"]),
                    (Repository::Debian, vec!["ark"]),
                    (Repository::Fedora, vec!["ark"]),
                    (Repository::RedHat, vec!["ark"]),
                    (Repository::Ubuntu, vec!["ark"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.kde.ark",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: Some(Snap {
                    name: "ark",
                    is_official: true,
                    is_classic: false,
                    channel: "",
                }),
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "dconf Editor",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["dconf-editor"]),
                    (Repository::Debian, vec!["dconf-editor"]),
                    (Repository::Fedora, vec!["dconf-editor"]),
                    (Repository::RedHat, vec!["dconf-editor"]),
                    (Repository::Ubuntu, vec!["dconf-editor"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "ca.desrt.dconf-editor",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Fedora Media Writer",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Fedora, vec!["mediawriter"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.fedoraproject.MediaWriter",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "FileLight Disk Usage",
                desktop_environment: Some(DesktopEnvironment::KDE),
                repository: HashMap::from([
                    (Repository::Arch, vec!["filelight"]),
                    (Repository::Debian, vec!["filelight"]),
                    (Repository::Fedora, vec!["filelight"]),
                    (Repository::RedHat, vec!["filelight"]),
                    (Repository::Ubuntu, vec!["filelight"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "GParted",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gparted"]),
                    (Repository::Debian, vec!["gparted"]),
                    (Repository::Fedora, vec!["gparted"]),
                    (Repository::RedHat, vec!["gparted"]),
                    (Repository::Ubuntu, vec!["gparted"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Disk Usage",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["baobab"]),
                    (Repository::Debian, vec!["baobab"]),
                    (Repository::Fedora, vec!["baobab"]),
                    (Repository::RedHat, vec!["baobab"]),
                    (Repository::Ubuntu, vec!["baobab"]),
                ]),
                flatpak: Some(Flatpak {
                    name: "org.gnome.baobab",
                    is_verified: true,
                    remotes: vec!["fedora", "flathub"],
                }),
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Disk Utility",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gnome-disk-utility"]),
                    (Repository::Debian, vec!["gnome-disk-utility"]),
                    (Repository::Fedora, vec!["gnome-disk-utility"]),
                    (Repository::RedHat, vec!["gnome-disk-utility"]),
                    (Repository::Ubuntu, vec!["gnome-disk-utility"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Shell Extension",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gnome-shell-extensions"]),
                    (Repository::Debian, vec!["gnome-shell-extensions"]),
                    (Repository::Fedora, vec!["gnome-extensions-app"]),
                    (Repository::RedHat, vec!["gnome-extensions-app"]),
                    (Repository::Ubuntu, vec!["gnome-shell-extensions"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Shell Extension Manager",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Debian, vec!["gnome-shell-extension-manager"]),
                    (Repository::Ubuntu, vec!["gnome-shell-extension-manager"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome System Monitor",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gnome-system-monitor"]),
                    (Repository::Debian, vec!["gnome-system-monitor"]),
                    (Repository::Fedora, vec!["gnome-system-monitor"]),
                    (Repository::RedHat, vec!["gnome-system-monitor"]),
                    (Repository::Ubuntu, vec!["gnome-system-monitor"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Gnome Tweaks",
                desktop_environment: Some(DesktopEnvironment::Gnome),
                repository: HashMap::from([
                    (Repository::Arch, vec!["gnome-tweaks"]),
                    (Repository::Debian, vec!["gnome-tweaks"]),
                    (Repository::Fedora, vec!["gnome-tweaks"]),
                    (Repository::RedHat, vec!["gnome-tweaks"]),
                    (Repository::Ubuntu, vec!["gnome-tweaks"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "KSysGuard",
                desktop_environment: Some(DesktopEnvironment::KDE),
                repository: HashMap::from([
                    (Repository::Arch, vec!["ksysguard"]),
                    (Repository::Debian, vec!["ksysguard"]),
                    (Repository::Fedora, vec!["ksysguard"]),
                    (Repository::RedHat, vec!["ksysguard"]),
                    (Repository::Ubuntu, vec!["ksysguard"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Plasma System Monitor",
                desktop_environment: Some(DesktopEnvironment::KDE),
                repository: HashMap::from([
                    (Repository::Arch, vec!["plasma-systemmonitor"]),
                    (Repository::Debian, vec!["plasma-systemmonitor"]),
                    (Repository::Fedora, vec!["plasma-systemmonitor"]),
                    (Repository::RedHat, vec!["plasma-systemmonitor"]),
                    (Repository::Ubuntu, vec!["plasma-systemmonitor"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Simple Scan",
                desktop_environment: None,
                repository: HashMap::from([
                    (Repository::Arch, vec!["simple-scan"]),
                    (Repository::Debian, vec!["simple-scan"]),
                    (Repository::Fedora, vec!["simple-scan"]),
                    (Repository::Ubuntu, vec!["simple-scan"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
            Package {
                name: "Spectacle Screenshot",
                desktop_environment: Some(DesktopEnvironment::KDE),
                repository: HashMap::from([
                    (Repository::Arch, vec!["spectacle"]),
                    (Repository::Debian, vec!["spectacle"]),
                    (Repository::Fedora, vec!["spectacle"]),
                    (Repository::RedHat, vec!["spectacle"]),
                    (Repository::Ubuntu, vec!["spectacle"]),
                ]),
                flatpak: None,
                snap: None,
                other: None,
                pre_install: None,
                post_install: None,
            },
        ]),
    };
}
