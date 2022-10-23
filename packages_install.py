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
            if self.name == "deja-dup":
                return []
            elif self.name == "gnome-books":
                return []
            elif self.name == "gnome-boxes":
                return []
            elif self.name == "gnome-calendar":
                return []
            elif self.name == "gnome-clocks":
                return []
            elif self.name == "gnome-contacts":
                return []
            elif self.name == "gnome-maps":
                return []
            elif self.name == "gnome-passwordsafe":
                return []
            elif self.name == "gnome-weather":
                return []
            elif self.name == "gnucash":
                return []
            elif self.name == "gwenview":
                return []
            elif self.name == "ibus-unikey":
                return ["https://rpmfind.net/linux/fedora/linux/releases/34/Everything/x86_64/os/Packages"
                        "/i/ibus-unikey-0.6.1-26.20190311git46b5b9e.fc34.x86_64.rpm"]
            elif self.name == "id3v2":
                return []
            elif self.name == "okular":
                return []
            elif self.name == "transmission-gtk":
                return []
            elif self.name == "transmission-qt":
                return []
        elif distribution.repository == "fedora":
            if self.name == "gnome-passwordsafe":
                return ["secrets"]
        elif distribution.repository == "debian":
            if self.name == "gnome-connections":
                return []
            elif self.name == "yt-dlp":
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

    def is_available(self) -> bool:
        packages = self.get_packages()
        if len(packages) > 0:
            return True
        return False

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
    def __init__(self, name: str, is_official: bool = False, is_classic: bool = False, channel: str | None = None):
        self.name: str = name
        self.is_official: bool = is_official
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
    def __init__(self, repository: Repo | None, flatpak: Flatpak | None = None, snap: Snap | None = None,
                 desktop_environment: str | None = None):
        self.repository: Repo | None = repository
        self.flatpak: Flatpak | None = flatpak
        self.snap: Snap | None = snap
        self.desktop_environment: str | None = desktop_environment

    def is_available(self) -> bool:
        if self.repository is not None:
            if self.repository.is_available():
                return True
        if self.flatpak is not None:
            return True
        if self.snap is not None:
            return True
        return False

    def get_installed_method(self) -> str:
        if self.repository is not None:
            if self.repository.is_installed():
                return "Repository"
        if self.flatpak is not None:
            if self.flatpak.is_installed():
                return "Flatpak"
        if self.snap is not None:
            if self.snap.is_installed():
                return "Snap"
        return "Not"


all_packages: dict[str, dict[str, Package]] = {
    "Server": {
        "Cockpit - Web Interface": Package(Repo("cockpit")),
        "cURL - Client URL": Package(Repo("curl")),
        "htop - Process Reviewer": Package(Repo("htop")),
        "git - Version Control": Package(Repo("git")),
        "MariaDB - Database": Package(Repo("mariadb")),
        "nano - Text Editor": Package(Repo("nano")),
        "ncdu - Disk Usage": Package(Repo("ncdu")),
        "Node.js - JavaScript RE": Package(Repo("node"), None, Snap("node", True, True, "18/stable")),
        "Podman - Containers": Package(Repo("podman")),
        "Rust Language": Package(Repo("rust")),
        "SSH - Secure Shell Protocol": Package(Repo("ssh")),
        "VIM - Text Editor": Package(Repo("vim"))
    },
    "Desktop": {
        "cups - Printer Support": Package(Repo("cups")),
        "ffmpeg - Media Codecs": Package(Repo("ffmpeg")),
        "Vietnamese Keyboard": Package(Repo("ibus-unikey")),
        "MP3 Metadata Editor": Package(Repo("id3v2")),
        "imagemagick": Package(Repo("imagemagick")),
        "LaTex - Compiler": Package(Repo("latex")),
        "qtile - Window Manager": Package(Repo("qtile")),
        "yt-dlp - Download YouTube": Package(Repo("yt-dlp"))
    },
    "Applications": {
        "Cheese - Webcam": Package(Repo("cheese"), Flatpak("org.gnome.Cheese"), None, "gnome"),
        "Deja Dup - Backups": Package(Repo("deja-dup"), Flatpak("org.gnome.DejaDup")),
        "Eye of Gnome - Image Viewer": Package(Repo("eog"), Flatpak("org.gnome.eog"), Snap("eog", True), "gnome"),
        "Evince - Document Viewer": Package(Repo("evince"), Flatpak("org.gnome.Evince"), None, "gnome"),
        "Gnome Books": Package(Repo("gnome-books"), Flatpak("org.gnome.Books"), None, "gnome"),
        "Gnome Boxes - VM Manager": Package(Repo("gnome-boxes"), Flatpak("org.gnome.Boxes"), None, "gnome"),
        "Gnome Calculator": Package(Repo("gnome-calculator"), Flatpak("org.gnome.Calculator"),
                                    Snap("gnome-calculator", True), "gnome"),
        "Gnome Calendar": Package(Repo("gnome-calendar"), Flatpak("org.gnome.Calendar"), None, "gnome"),
        "Gnome Clocks": Package(Repo("gnome-clocks"), Flatpak("org.gnome.clocks"), Snap("gnome-clocks", True), "gnome"),
        "Gnome Connections": Package(Repo("gnome-connections"), Flatpak("org.gnome.Connections"), None, "gnome"),
        "Gnome Contacts": Package(Repo("gnome-contacts"), Flatpak("org.gnome.Contacts"), None, "gnome"),
        "Gnome Maps": Package(Repo("gnome-maps"), Flatpak("org.gnome.Maps"), None, "gnome"),
        "Gnome Password Safe": Package(Repo("gnome-passwordsafe"), Flatpak("org.gnome.PasswordSafe")),
        "Gnome Weather": Package(Repo("gnome-weather"), Flatpak("org.gnome.Weather"), None, "gnome"),
        "GNU Cash - Accounting": Package(Repo("gnucash"), Flatpak("org.gnucash.GnuCash")),
        "Gwenview - Image Viewer": Package(Repo("gwenview"), Flatpak("org.kde.gwenview"), Snap("gwenview", True),
                                           "plasma"),
        "KCalc - Calculator": Package(Repo("kcalc"), Flatpak("org.kde.kcalc"), Snap("kcalc", True), "plasma"),
        "Okular - Document Viewer": Package(Repo("okular"), Flatpak("org.kde.okular"), Snap("ocular", True), "plasma"),
        "Transmission (GTK) - Torrent": Package(Repo("transmission-gtk"), None, None, "gnome"),
        "Transmission (QT) - Torrent": Package(Repo("transmission-qt"), None, None, "plasma"),
        "Virt Manager": Package(Repo("virt-manager")),
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
    menu_entries: list[str] = []
    if package.repository is not None:
        menu_entries.append("[r] repository install")
    if package.flatpak is not None:
        menu_entries.append("[f] flatpak install")
    if package.snap is not None:
        menu_entries.append(f"[s] snap {'official ' if package.snap.is_official else ''}install")
    menu_entries.append("[u] uninstall")
    menu_entries.append("[c] cancel")

    term_menu = TerminalMenu(
        title=f"\n(Press Q or Esc to go back)\nPackage: {pkg}\n{package.get_installed_method()} Installed\n",
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
        package = all_packages[category][pkg]
        if not package.is_available():
            continue

        if package.desktop_environment == "gnome":
            if not has_command("gnome-shell"):
                continue

        if package.desktop_environment == "plasma":
            if not has_command("plasmashell"):
                continue

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
