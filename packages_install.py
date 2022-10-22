#!/usr/bin/env python3

# Imports
from simple_term_menu import TerminalMenu
from helper_functions import *
import os


class Distribution:
    def __init__(self, name: str, repository: str, package_manager: str):
        self.name = name
        self.repository = repository
        self.package_manager = package_manager

    def repository_get_installed(self) -> list[str]:
        if self.package_manager == "apt":
            return get_command("apt list --installed | awk -F '/' '{print $1}'").split("\n")
        elif self.package_manager == "dnf":
            return get_command("dnf list installed | awk -F '.' '{print $1}'").split("\n")
        elif self.package_manager == "pacman":
            return get_command("pacman -Q | awk '{print $1}'").split("\n")
        elif self.package_manager == "zypper":
            return get_command("zypper packages --installed-only | awk -F '|' '{print $3}'").split("\n")

    def repository_install(self, packages: list[str]) -> None:
        if len(packages) == 0:
            return

        packages: str = " ".join(packages)

        print_info(f"{self.package_manager} install {packages}", True)

        if self.package_manager == "apt":
            run_command("sudo apt install " + packages + " -Vy")
        elif self.package_manager == "dnf":
            run_command("sudo dnf install " + packages + " -y")
        elif self.package_manager == "pacman":
            run_command("sudo pacman -S " + packages + " --noconfirm --needed")
        elif self.package_manager == "zypper":
            run_command("sudo zypper install --no-confirm " + packages)

    def repository_remove(self, packages: list[str]) -> None:
        if len(packages) == 0:
            return

        packages: str = " ".join(packages)

        print_info(f"{self.package_manager} remove {packages}", True)

        if self.package_manager == "apt":
            run_command("sudo apt remove " + packages + " -Vy")
        elif self.package_manager == "dnf":
            run_command("sudo dnf remove " + packages + " -y")
        elif self.package_manager == "pacman":
            run_command("sudo pacman -Rsun " + packages + " --noconfirm")
        elif self.package_manager == "zypper":
            run_command("sudo zypper remove --no-confirm " + packages)

    def repository_autoremove(self) -> None:
        print_info(f"{self.package_manager} autoremove", True)

        if self.package_manager == "apt":
            run_command("sudo apt autoremove -Vy")
        elif self.package_manager == "dnf":
            run_command("sudo dnf autoremove -y")
        elif self.package_manager == "pacman":
            run_command("sudo pacman -Rs $(pacman -Qdtq) --noconfirm")
        elif self.package_manager == "zypper":
            run_command(
                "sudo zypper remove --clean-deps --no-confirm $(zypper packages --unneeded | awk -F '|' 'NR==0 || "
                "NR==1 || NR==2 || NR==3 || NR==4 {next} {print $3}')")

    def repository_update(self) -> None:
        print_info(f"{self.package_manager} update", True)

        if self.package_manager == "apt":
            run_command("sudo apt update && sudo apt upgrade -Vy")
        elif self.package_manager == "dnf":
            run_command("sudo dnf upgrade --refresh -y")
        elif self.package_manager == "pacman":
            run_command("sudo pacman -Syu --noconfirm")
        elif self.package_manager == "zypper":
            run_command("sudo zypper update --no-confirm")

    def repository_module(self, packages: list[str]) -> None:
        if len(packages) == 0:
            return

        packages: str = " ".join(packages)

        print_info(f"{self.package_manager} module {packages}", True)

        if self.package_manager == "dnf":
            run_command("sudo dnf module enable " + packages + " -y")

    def repository_setup(self) -> None:
        print_info(f"{self.package_manager} setup", True)

        if self.package_manager == "dnf":
            add_to_file_if_not_found("/etc/dnf/dnf.conf", "max_parallel_downloads", "max_parallel_downloads=10")

            confirm_extra_free = input("Enable EPEL/RPM Fusion Repositories? [y/N]: ")
            if confirm_extra_free.lower() == "y":
                distro_version: str = get_command("rpm -E %" + distribution.name)

                if self.repository == "fedora":
                    run_command("sudo dnf install " +
                                "https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-" +
                                distro_version + ".noarch.rpm -y")
                elif self.repository == "redhat":
                    run_command("sudo dnf install --nogpgcheck " +
                                "https://dl.fedoraproject.org/pub/epel/epel-release-latest-" +
                                distro_version + ".noarch.rpm -y")
                    run_command("sudo dnf install --nogpgcheck " +
                                "https://download1.rpmfusion.org/free/el/rpmfusion-free-release-" +
                                distro_version + ".noarch.rpm -y")
                    run_command("sudo dnf config-manager --set-enabled crb")

                confirm_extra_non_free = input("Enable Non-Free EPEL/RPM Fusion Repositories? [y/N]: ")
                if confirm_extra_non_free.lower() == "y":
                    if self.repository == "fedora":
                        run_command("sudo dnf install " +
                                    "https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-" +
                                    distro_version + ".noarch.rpm -y")
                    elif self.repository == "redhat":
                        run_command("sudo dnf install --nogpgcheck " +
                                    "https://download1.rpmfusion.org/nonfree/el/rpmfusion-nonfree-release-" +
                                    distro_version + ".noarch.rpm -y")

                self.repository_update()

    def install_flatpak(self) -> None:
        if not has_command("flatpak"):
            self.repository_install(["flatpak"])
        run_command("flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo")

    def install_snap(self) -> None:
        if not has_command("snap"):
            self.repository_install(["snapd"])
            if self.package_manager == "dnf":
                run_command("sudo systemctl enable --now snapd.socket")
                run_command("sudo ln -s /var/lib/snapd/snap /snap")


def get_distribution() -> Distribution | None:
    if not os.path.isfile("/etc/os-release"):
        return None

    distro_name: str = open("/etc/os-release", "r").readline()

    if "Arch" in distro_name:
        return Distribution("arch", "arch", "pacman")
    if "Alma" in distro_name:
        return Distribution("alma", "redhat", "dnf")
    if "CentOS" in distro_name:
        return Distribution("centos", "redhat", "dnf")
    if "Debian" in distro_name:
        return Distribution("debian", "debian", "apt")
    if "Fedora" in distro_name:
        return Distribution("fedora", "fedora", "dnf")
    if "Mint" in distro_name:
        return Distribution("mint", "ubuntu", "apt")
    if "Pop!_OS" in distro_name:
        return Distribution("pop", "ubuntu", "apt")
    if "SUSE" in distro_name:
        return Distribution("suse", "suse", "zypper")
    if "Ubuntu" in distro_name:
        return Distribution("ubuntu", "ubuntu", "apt")
    return None


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


def install_server_packages() -> None:
    menu_entries: list[str] = [
        "bash-completion",
        "cockpit",
        "curl",
        "htop",
        "git",
        "mariadb-server",
        "nano",
        "ncdu",
        "node",
        "pip",
        "podman",
        "rust",
        "ssh",
        "vim"
    ]
    term_menu = TerminalMenu(
        title="\nServer Packages (Press Q or Esc to quit)\n",
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

            if distribution.package_manager == "zypper":
                packages_to_install.append("nodejs16")
                packages_to_install.append("npm16")
            else:
                packages_to_install.append("nodejs")
                packages_to_install.append("npm")

        elif pkg == "mariadb-server":
            if distribution.package_manager == "pacman" or distribution.package_manager == "zypper":
                packages_to_install.append("mariadb")
            else:
                packages_to_install.append(pkg)

        elif pkg == "pip":
            if distribution.package_manager == "pacman":
                packages_to_install.append("python-pip")
            elif distribution.package_manager == "zypper":
                packages_to_install.append("python38-pip")
            else:
                packages_to_install.append("python3-pip")

        elif pkg == "rust":
            if distribution.package_manager == "pacman":
                packages_to_install.append("rustup")
            else:
                packages_to_install.append("rust")
                packages_to_install.append("rustfmt")
                packages_to_install.append("cargo")

        elif pkg == "ssh":
            if distribution.package_manager == "apt":
                packages_to_install.append("ssh")
            elif distribution.package_manager == "zypper":
                packages_to_install.append("libssh4")
                packages_to_install.append("openssh")
            else:
                packages_to_install.append("libssh")
                packages_to_install.append("openssh")

        else:
            packages_to_install.append(pkg)

    distribution.repository_module(modules_to_enable)
    distribution.repository_install(packages_to_install)


def install_desktop_packages() -> None:
    menu_entries: list[str] = [
        "cups",
        "ffmpeg",
        "ibus-unikey",
        "id3v2",
        "imagemagick",
        "latex",
        "qtile",
        "yt-dlp"
    ]

    if distribution == "centos":
        menu_entries.remove("id3v2")
    if distribution == "debian":
        menu_entries.remove("yt-dlp")
    if distribution != "arch":
        menu_entries.remove("qtile")

    term_menu = TerminalMenu(
        title="\nDesktop Packages (Press Q or Esc to quit)\n",
        menu_entries=menu_entries,
        cycle_cursor=True,
        multi_select=True,
        show_multi_select_hint=True,
        clear_screen=False
    )

    term_menu.show()
    if term_menu.chosen_menu_entries is None:
        return

    packages_to_install: list[str] = []

    for pkg in term_menu.chosen_menu_entries:
        if pkg == "ffmpeg":
            if distribution.package_manager == "zypper":
                packages_to_install.append("ffmpeg-4")
            else:
                packages_to_install.append(pkg)

        elif pkg == "ibus-unikey":
            if distribution == "centos":
                packages_to_install.append(
                    "https://rpmfind.net/linux/fedora/linux/releases/34/Everything/x86_64/os/Packages/"
                    "i/ibus-unikey-0.6.1-26.20190311git46b5b9e.fc34.x86_64.rpm")
            else:
                packages_to_install.append(pkg)

        elif pkg == "imagemagick":
            if distribution.package_manager == "dnf":
                packages_to_install.append("ImageMagick")
            elif distribution.package_manager == "apt":
                packages_to_install.append("imagemagick")

        elif pkg == "latex":
            if distribution.package_manager == "apt":
                packages_to_install.append("texlive-latex-base")
                packages_to_install.append("texlive-latex-extra")
            elif distribution.package_manager == "dnf":
                packages_to_install.append("texlive-latex")
                if distribution.repository == "fedora":
                    packages_to_install.append("texlive-collection-latexextra")
            elif distribution.package_manager == "pacman":
                packages_to_install.append("texlive-core")
                packages_to_install.append("texlive-latexextra")

        elif pkg == "qtile":
            packages_to_install.append("qtile")
            packages_to_install.append("alacritty")
            packages_to_install.append("rofi")
            packages_to_install.append("numlockx")
            packages_to_install.append("playerctl")

        else:
            packages_to_install.append(pkg)

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
    cursor_index = 0
    while True:
        menu_selection_idx: int = TerminalMenu(
            title="\n(Press Q or Esc to quit)\n",
            menu_entries=[
                "Update Packages",
                "Repository Setup",
                "Environment Setup",
                "Server Packages",
                "Desktop Packages",
                "Remove Packages",
                None,
                "Exit"
            ],
            cycle_cursor=False,
            clear_screen=False,
            cursor_index=cursor_index
        ).show()

        if menu_selection_idx == 0:
            distribution.repository_update()
            cursor_index = 1
        elif menu_selection_idx == 1:
            distribution.repository_setup()
            cursor_index = 2
        elif menu_selection_idx == 2:
            setup_environment()
            cursor_index = 3
        elif menu_selection_idx == 3:
            install_server_packages()
            cursor_index = 4
        elif menu_selection_idx == 4:
            install_desktop_packages()
            cursor_index = 5
        elif menu_selection_idx == 5:
            remove_packages()
            cursor_index = 7
        else:
            break


# Global Variables

distribution: Distribution | None = None


def main() -> None:
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
