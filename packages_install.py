#!/usr/bin/env python3

from __future__ import annotations
from helpers.helper_functions import *
from simple_term_menu import TerminalMenu
import os


class Repo:
    def __init__(self, name: str):
        self.name: str = name

    def get_packages(self) -> list[str]:
        # Check distribution exceptions
        if distribution.name == "":
            return []

        # Check repository exceptions
        if distribution.repository == "redhat":
            if self.name == "gnome-clocks":
                return []
            elif self.name == "ibus-unikey":
                return ["https://rpmfind.net/linux/fedora/linux/releases/34/Everything/x86_64/os/Packages"
                        "/i/ibus-unikey-0.6.1-26.20190311git46b5b9e.fc34.x86_64.rpm"]
            elif self.name == "id3v2":
                return []
        elif distribution.repository == "debian":
            if self.name == "yt-dlp":
                return []

        # Check self.name manager exceptions
        if distribution.package_manager == "dnf":
            if self.name == "imagemagick":
                return ["ImageMagick"]
        elif distribution.package_manager == "zypper":
            if self.name == "ffmpeg":
                return ["ffmpeg-4"]

        # Check self.name exceptions
        if self.name == "latex":
            if distribution.repository == "fedora":
                return ["texlive-latex", "texlive-collection-latexextra"]

            if distribution.package_manager == "dnf":
                return ["texlive-latex"]
            elif distribution.package_manager == "apt":
                return ["texlive-latex-base", "texlive-latex-extra"]
            elif distribution.package_manager == "pacman":
                return ["texlive-core", "texlive-latexextra"]
        elif self.name == "mariadb":
            if distribution.package_manager == "pacman" or distribution.package_manager == "zypper":
                return ["mariadb"]
            else:
                return ["mariadb-server"]
        elif self.name == "node":
            if distribution.package_manager == "zypper":
                return ["nodejs16", "npm16"]
            else:
                return ["nodejs", "npm"]
        elif self.name == "rust":
            if distribution.package_manager == "pacman":
                return ["rustup"]
            else:
                return ["rust", "rustfmt", "cargo"]
        elif self.name == "ssh":
            if distribution.package_manager == "apt":
                return ["ssh"]
            elif distribution.package_manager == "zypper":
                return ["libssh4", "openssh"]
            else:
                return ["libssh", "openssh"]
        elif self.name == "qtile":
            if distribution.package_manager == "arch":
                return ["qtile", "alacritty", "rofi", "numlockx", "playerctl"]
            else:
                return []

        return [self.name]

    def get_modules(self) -> list[str]:
        if self.name == "node":
            if distribution.package_manager == "dnf":
                return ["nodejs:18"]
        return []

    def is_installed(self) -> bool:
        global repository_installed
        packages = self.get_packages()
        for pkg in packages:
            if pkg in repository_installed:
                return True
        return False

    def install(self):
        modules = self.get_modules()
        for module in modules:
            distribution.module_enable(module)

        global repository_installed
        packages = self.get_packages()
        for pkg in packages:
            if pkg not in repository_installed:
                distribution.install(pkg)
                repository_installed.add(pkg)

    def remove(self):
        global repository_installed
        packages = self.get_packages()
        for pkg in packages:
            if pkg in repository_installed:
                distribution.remove(pkg)
                repository_installed.remove(pkg)


class Flatpak:
    def __init__(self, name: str):
        self.name: str = name

    def is_installed(self) -> bool:
        global flatpak_installed
        if self.name in flatpak_installed:
            return True
        return False

    def install(self):
        global flatpak_installed
        if self.name not in flatpak_installed:
            run_command("flatpak install flathub " + self.name + " -y")
            flatpak_installed.add(self.name)

    def remove(self):
        global flatpak_installed
        if self.name in flatpak_installed:
            run_command("flatpak remove " + self.name + " -y")
            flatpak_installed.remove(self.name)


class Snap:
    def __init__(self, name: str, is_classic: bool = False, channel: str | None = None):
        self.name: str = name
        self.is_classic: bool = is_classic
        self.channel: str = channel

    def is_installed(self) -> bool:
        global snap_installed
        if self.name in snap_installed:
            return True
        return False

    def install(self):
        global snap_installed
        if self.name not in snap_installed:
            command = "sudo snap install " + self.name
            if self.is_classic:
                command += " --classic"
            if self.channel is not None:
                command += " --channel=" + self.channel
            run_command(command)
            snap_installed.add(self.name)

    def remove(self):
        global snap_installed
        if self.name in snap_installed:
            run_command("sudo snap remove " + self.name)
            snap_installed.remove(self.name)


class Package:
    def __init__(self, repository: Repo | None, flatpak: Flatpak | None, snap: Snap | None):
        self.repository: Repo | None = repository
        self.flatpak: Flatpak | None = flatpak
        self.snap: Snap | None = snap


all_packages: dict[str, dict[str, Package]] = {
    "Server": {
        "Cockpit - Web Interface": Package(Repo("cockpit"), None, None),
        "cURL - Client URL": Package(Repo("curl"), None, None),
        "htop - Process Reviewer": Package(Repo("htop"), None, None),
        "git - Version Control": Package(Repo("git"), None, None),
        "MariaDB - Database": Package(Repo("mariadb"), None, None),
        "nano - Text Editor": Package(Repo("nano"), None, None),
        "ncdu - Disk Usage": Package(Repo("ncdu"), None, None),
        "Node.js - JavaScript RE": Package(Repo("node"), None, Snap("node", True, "18/stable")),
        "Podman - Containers": Package(Repo("podman"), None, None),
        "Rust Language": Package(Repo("rust"), None, None),
        "SSH - Secure Shell Protocol": Package(Repo("ssh"), None, None),
        "VIM - Text Editor": Package(Repo("vim"), None, None)
    },
    "Desktop": {
        "cups - Printer Support": Package(Repo("cups"), None, None),
        "ffmpeg - Media Codecs": Package(Repo("ffmpeg"), None, None),
        "Vietnamese Keyboard": Package(Repo("ibus-unikey"), None, None),
        "MP3 Metadata Editor": Package(Repo("id3v2"), None, None),
        "imagemagick": Package(Repo("imagemagick"), None, None),
        "LaTex - Compiler": Package(Repo("latex"), None, None),
        "qtile - Window Manager": Package(Repo("qtile"), None, None),
        "yt-dlp - Download YouTube": Package(Repo("yt-dlp"), None, None)
    },
    "Applications": {
        "gnome-clocks": Package(Repo("gnome-clocks"), Flatpak("org.gnome.clocks"), Snap("gnome-clocks", True))
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


def handle_package(pkg: str, package: Package) -> None:
    current_installed = ""
    menu_entries: list[str] = []
    if package.repository is not None:
        menu_entries.append("[r] repository install")
        if package.repository.is_installed():
            current_installed = "Repository"
    if package.flatpak is not None:
        menu_entries.append("[f] flatpak install")
        if package.flatpak.is_installed():
            current_installed = "Flatpak"
    if package.snap is not None:
        menu_entries.append("[s] snap install")
        if package.snap.is_installed():
            current_installed = "Snap"
    menu_entries.append("[u] uninstall")
    menu_entries.append("[c] cancel")

    title = f"\n(Press Q or Esc to go back)\nPackage: {pkg}\n"
    if current_installed != "":
        title += f"({current_installed} Currently Installed)\n"
    term_menu = TerminalMenu(
        title=title,
        menu_entries=menu_entries,
        cycle_cursor=True,
        clear_screen=False
    )

    menu_selection_idx: int = term_menu.show()

    if menu_selection_idx is None:
        return

    if menu_selection_idx == len(menu_entries) - 1:
        return

    action: str = menu_entries[menu_selection_idx][1]

    # Uninstall non-selected options
    if action != "r":
        if package.repository is not None:
            package.repository.remove()
    if action != "f":
        if package.flatpak is not None:
            package.flatpak.remove()
    if action != "s":
        if package.snap is not None:
            package.snap.remove()

    # Install selected option
    if action == "r":
        package.repository.install()
    elif action == "f":
        distribution.install_flatpak()
        package.flatpak.install()
    elif action == "s":
        distribution.install_snap()
        package.snap.install()


def select_package(category: str) -> None:
    menu_entries: list[str] = []
    for pkg in all_packages[category]:
        menu_entries.append(pkg)
    menu_entries.append("Exit")

    cursor_index = 0
    while True:
        menu_selection_idx: int = TerminalMenu(
            title=f"\n(Press Q or Esc to go back)\nSelect {category} Package\n",
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

        pkg = menu_entries[menu_selection_idx]
        handle_package(pkg, all_packages[category][pkg])


def run() -> None:
    menu_entries: list[str] = [
        "Environment Setup",
        "Repository Setup",
        "Update Packages",
        "Autoremove Packages"
    ]
    menu_entries += all_packages.keys()
    menu_entries.append("Exit")

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
            setup_environment()
        elif menu_selection_idx == 1:
            distribution.setup()
            if has_command("flatpak"):
                run_command("flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo")
        elif menu_selection_idx == 2:
            distribution.update()
            if has_command("flatpak"):
                run_command("flatpak update -y")
            if has_command("snap"):
                run_command("sudo snap refresh")
        elif menu_selection_idx == 3:
            distribution.autoremove()
            if has_command("flatpak"):
                run_command("flatpak remove --unused -y")
        else:
            select_package(menu_entries[menu_selection_idx])


def flatpak_get_installed() -> set[str]:
    if has_command("flatpak"):
        return set(get_command("flatpak list --app | awk -F '\t' '{print $2}'").split("\n"))
    return set()


def snap_get_installed() -> set[str]:
    if has_command("snap"):
        return set(get_command("snap list | awk '{print $1}'").split("\n"))
    return set()


# Global Variables

distribution: Distribution = Distribution("", "", "")
repository_installed: set[str] = set()
flatpak_installed: set[str] = set()
snap_installed: set[str] = set()


def main():
    if os.geteuid() != 0:
        print_error("Must be run as root", True)
        exit()

    global distribution
    distribution = get_distribution()
    if distribution is None:
        print_error("Distribution not recognized", True)
        exit()

    global repository_installed
    repository_installed = distribution.get_installed()

    global flatpak_installed
    flatpak_installed = flatpak_get_installed()

    global snap_installed
    snap_installed = snap_get_installed()

    run()


if __name__ == "__main__":
    main()
