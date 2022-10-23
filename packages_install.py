#!/usr/bin/env python3

from __future__ import annotations
from helpers.helper_functions import *
from simple_term_menu import TerminalMenu
import os


class SnapPackage:
    def __init__(self, name: str, is_official: bool, is_classic: bool, channel: str | None):
        self.name: str = name
        self.is_official: bool = is_official
        self.is_classic: bool = is_classic
        self.channel: str = channel


class Package:
    def __init__(self, repository: bool, flatpak: str | None, snap: SnapPackage | None):
        self.repository: bool = repository
        self.flatpak: str | None = flatpak
        self.snap: SnapPackage | None = snap


all_packages: dict[str, dict[str, Package]] = {
    "Server": {
        "bash-completion": Package(True, None, None),
        "cockpit": Package(True, None, None),
        "curl": Package(True, None, None),
        "htop": Package(True, None, None),
        "git": Package(True, None, None),
        "mariadb": Package(True, None, None),
        "nano": Package(True, None, None),
        "ncdu": Package(True, None, None),
        "node": Package(True, None, None),
        "podman": Package(True, None, None),
        "rust": Package(True, None, None),
        "ssh": Package(True, None, None),
        "vim": Package(True, None, None)
    },
    "Desktop": {
        "cups": Package(True, None, None),
        "ffmpeg": Package(True, None, None),
        "ibus-unikey": Package(True, None, None),
        "id3v2": Package(True, None, None),
        "imagemagick": Package(True, None, None),
        "latex": Package(True, None, None),
        "qtile": Package(True, None, None),
        "yt-dlp": Package(True, None, None)
    }
}


def setup_environment() -> None:
    username: str = os.environ.get("SUDO_USER")
    user_uid: int = int(os.environ.get("SUDO_UID"))
    user_gid: int = int(os.environ.get("SUDO_GID"))

    # bashrc
    bashrc: str = "/home/" + username + "/.bashrc"
    add_to_file_if_not_found(bashrc, "export EDITOR", 'export EDITOR="/usr/bin/vim"\n\n')
    add_to_file_if_not_found(bashrc, "export NODE_OPTIONS", "export NODE_OPTIONS=--max_old_space_size=8192\n\n")
    add_to_file_if_not_found(bashrc, "export GFB_HOSTING_ENV", 'export GFB_HOSTING_ENV="dev"\n')
    add_to_file_if_not_found(bashrc, "export GFB_EDIT_SECRET", 'export GFB_EDIT_SECRET=""\n')
    add_to_file_if_not_found(bashrc, "export JWT_SECRET", 'export JWT_SECRET=""\n')
    add_to_file_if_not_found(bashrc, "export SQL_TU_PASSWORD", 'export SQL_TU_PASSWORD=""\n\n')
    os.chown(bashrc, user_uid, user_gid)

    # vimrc
    vimrc: str = "/home/" + username + "/.vimrc"
    add_to_file_if_not_found(vimrc, "syntax on", "syntax on\n")
    add_to_file_if_not_found(vimrc, "filetype plugin indent on", "filetype plugin indent on\n")
    add_to_file_if_not_found(vimrc, "set scrolloff", "set scrolloff=10\n")
    add_to_file_if_not_found(vimrc, "set number", "set number\n")
    add_to_file_if_not_found(vimrc, "set ignorecase smartcase", "set ignorecase smartcase\n")
    add_to_file_if_not_found(vimrc, "set incsearch hlsearch", "set incsearch hlsearch\n")
    os.chown(vimrc, user_uid, user_gid)


def install_packages(category: str) -> None:
    menu_entries: list[str] = []
    for pkg in all_packages[category]:
        menu_entries.append(pkg)

    term_menu = TerminalMenu(
        title="\nInstall " + category + " Packages (Press Q or Esc to quit)\n",
        menu_entries=menu_entries,
        cycle_cursor=True,
        multi_select=True,
        show_multi_select_hint=True,
        clear_screen=False
    )

    term_menu.show()
    if term_menu.chosen_menu_entries is None:
        return

    modules_to_enable: list[str] = []
    packages_to_install: list[str] = []

    for pkg in term_menu.chosen_menu_entries:
        if pkg == "node":
            if distribution.package_manager == "dnf":
                modules_to_enable.append("nodejs:18")
        packages_to_install += distribution.repository_get_package_names(pkg)

    distribution.repository_module(modules_to_enable)
    distribution.repository_install(packages_to_install)


def remove_packages() -> None:
    packages_to_remove: list[str] = [
        "akregator",
        "evolution",
        "konqueror",
        "kmail",
        "mpv"
    ]

    if distribution == "mint":
        packages_to_remove.append("celluloid")
        packages_to_remove.append("drawing")
        packages_to_remove.append("hexchat*")
        packages_to_remove.append("mintbackup")
        packages_to_remove.append("pix*")
        packages_to_remove.append("xed")
    elif distribution == "ubuntu" or distribution == "debian":
        packages_to_remove.append("gnome-mahjongg")
        packages_to_remove.append("gnome-todo")
        packages_to_remove.append("remmina*")
        packages_to_remove.append("seahorse")

        if distribution == "debian":
            packages_to_remove.append("five-or-more")
            packages_to_remove.append("four-in-a-row")
            packages_to_remove.append("gnome-klotski")
            packages_to_remove.append("gnome-nibbles")
            packages_to_remove.append("gnome-robots")
            packages_to_remove.append("gnome-taquin")
            packages_to_remove.append("gnome-tetravex")
            packages_to_remove.append("iagno")
            packages_to_remove.append("lightsoff")
            packages_to_remove.append("anthy*")
            packages_to_remove.append("fcitx*")
            packages_to_remove.append("goldendict")
            packages_to_remove.append("hitori")
            packages_to_remove.append("hdate-applet")
            packages_to_remove.append("*mozc*")
            packages_to_remove.append("mlterm*")
            packages_to_remove.append("malcontent")
            packages_to_remove.append("swell-foop")
            packages_to_remove.append("tali")
            packages_to_remove.append("xiterm*")
            packages_to_remove.append("xterm")

            # Remove Languages
            packages_to_remove.append("firefox-esr-l10n-*")
            packages_to_remove.append("libreoffice-l10n-*")
            packages_to_remove.append("hunspell-*")
            packages_to_remove.append("aspell-*")
            packages_to_remove.append("task-*-desktop")

    distribution.repository_remove(packages_to_remove)
    distribution.repository_autoremove()


def run() -> None:
    menu_entries: list[str] = [
        "Update Packages",
        "Repository Setup",
        "Environment Setup"
    ]
    menu_entries += all_packages.keys()
    menu_entries += [
        "Remove Packages",
        "Exit"
    ]

    cursor_index = 0
    while True:
        menu_selection_idx: int = TerminalMenu(
            title="\n(Press Q or Esc to quit)\n",
            menu_entries=menu_entries,
            cycle_cursor=False,
            clear_screen=False,
            cursor_index=cursor_index
        ).show()

        if menu_selection_idx is None:
            break

        if menu_selection_idx == len(menu_entries) - 1:
            break

        cursor_index = menu_selection_idx + 1

        if menu_selection_idx == 0:
            distribution.repository_update()
        elif menu_selection_idx == 1:
            distribution.repository_setup()
        elif menu_selection_idx == 2:
            setup_environment()
        elif menu_selection_idx == len(menu_entries) - 2:
            remove_packages()
        else:
            install_packages(menu_entries[menu_selection_idx])


# Global Variables

distribution: Distribution | None = None


def main():
    if os.geteuid() != 0:
        print_error("Must be run as root", True)
        exit()

    global distribution
    distribution = get_distribution()
    if distribution is None:
        print_error("Distribution not recognized", True)
        exit()

    run()


if __name__ == "__main__":
    main()
