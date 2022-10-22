#!/usr/bin/env python3

# Imports
from simple_term_menu import TerminalMenu
from helper_functions import *
import os

# Global Variables
distro: str = ""
pm: str = ""
repo: str = ""


def get_distribution() -> str:
    if not os.path.isfile("/etc/os-release"):
        return ""

    distro_name: str = open("/etc/os-release", "r").readline()

    if "Arch" in distro_name:
        return "arch"
    if "Alma" in distro_name:
        return "alma"
    if "CentOS" in distro_name:
        return "centos"
    if "Debian" in distro_name:
        return "debian"
    if "Fedora" in distro_name:
        return "fedora"
    if "Mint" in distro_name:
        return "mint"
    if "Pop!_OS" in distro_name:
        return "pop"
    if "SUSE" in distro_name:
        return "suse"
    if "Ubuntu" in distro_name:
        return "ubuntu"
    return ""


def get_package_repo() -> str:
    if distro == "arch":
        return "arch"
    if distro == "alma" or distro == "centos":
        return "redhat"
    if distro == "fedora":
        return "fedora"
    if distro == "suse":
        return "suse"
    if distro == "debian":
        return "debian"
    if distro == "mint" or distro == "pop" or distro == "ubuntu":
        return "ubuntu"
    return ""


def get_package_manager() -> str:
    if distro == "arch":
        return "pacman"
    if distro == "alma" or distro == "centos" or distro == "fedora":
        return "dnf"
    if distro == "suse":
        return "zypper"
    if distro == "debian" or distro == "mint" or distro == "pop" or distro == "ubuntu":
        return "apt"
    return ""


def package_manager(action: str, packages: list[str]) -> None:
    if action != "autoremove" and len(packages) == 0:
        return

    packages: str = " ".join(packages)

    print_info(f"{pm} {action} {packages}", True)

    if pm == "pacman":
        if action == "install":
            run_command("sudo pacman -S " + packages + " --noconfirm --needed")
        elif action == "remove":
            run_command("sudo pacman -Rsun " + packages + " --noconfirm")
        elif action == "autoremove":
            orphans: str = get_command("pacman -Qdttq")
            if len(orphans) > 0:
                run_command("sudo pacman -Rs $(pacman -Qdttq) --noconfirm")
    elif pm == "apt" and action == "remove":
        run_command("sudo apt-get remove --purge " + packages + " -y")
    elif action == "module":
        run_command("sudo dnf module enable " + packages + " -y")
    elif pm == "zypper":
        if action == "autoremove":
            run_command(
                "sudo zypper remove --clean-deps --no-confirm $(zypper packages --unneeded | awk -F '|' 'NR==0 || "
                "NR==1 || NR==2 || NR==3 || NR==4 {next} {print $3}')")
        else:
            run_command("sudo zypper " + action + " --no-confirm " + packages)
    else:
        run_command("sudo " + pm + " " + action + " " + packages + " -y")


def update_packages() -> None:
    print_info("Updating Packages", True)

    if pm == "apt":
        run_command("sudo apt update && sudo apt full-upgrade -Vy")
    elif pm == "dnf":
        run_command("sudo dnf upgrade --refresh -y")
    elif pm == "pacman":
        run_command("sudo pacman -Syu --noconfirm")
    elif pm == "zypper":
        run_command("sudo zypper update --no-confirm")


def repository_setup() -> None:
    if pm == "dnf":
        add_to_file_if_not_found("/etc/dnf/dnf.conf", "max_parallel_downloads", "max_parallel_downloads=10")

        confirm_extra_free = input("Enable EPEL/RPM Fusion Repositories? [y/N]: ")
        if confirm_extra_free.lower() == "y":
            distro_version: str = get_command("rpm -E %" + distro)

            if repo == "fedora":
                run_command("sudo dnf install " +
                            "https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-" +
                            distro_version + ".noarch.rpm -y")
            elif repo == "redhat":
                run_command("sudo dnf install --nogpgcheck " +
                            "https://dl.fedoraproject.org/pub/epel/epel-release-latest-" +
                            distro_version + ".noarch.rpm -y")
                run_command("sudo dnf install --nogpgcheck " +
                            "https://download1.rpmfusion.org/free/el/rpmfusion-free-release-" +
                            distro_version + ".noarch.rpm -y")
                run_command("sudo dnf config-manager --set-enabled crb")

            confirm_extra_non_free = input("Enable Non-Free EPEL/RPM Fusion Repositories? [y/N]: ")
            if confirm_extra_non_free.lower() == "y":
                if repo == "fedora":
                    run_command("sudo dnf install " +
                                "https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-" +
                                distro_version + ".noarch.rpm -y")
                elif repo == "redhat":
                    run_command("sudo dnf install --nogpgcheck " +
                                "https://download1.rpmfusion.org/nonfree/el/rpmfusion-nonfree-release-" +
                                distro_version + ".noarch.rpm -y")

            update_packages()


def environment_setup() -> None:
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


def server_packages() -> None:
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
            if pm == "dnf":
                modules_to_enable.append("nodejs:18")

            if pm == "zypper":
                packages_to_install.append("nodejs16")
                packages_to_install.append("npm16")
            else:
                packages_to_install.append("nodejs")
                packages_to_install.append("npm")

        elif pkg == "mariadb-server":
            if pm == "pacman" or pm == "zypper":
                packages_to_install.append("mariadb")
            else:
                packages_to_install.append(pkg)

        elif pkg == "pip":
            if pm == "pacman":
                packages_to_install.append("python-pip")
            elif pm == "zypper":
                packages_to_install.append("python38-pip")
            else:
                packages_to_install.append("python3-pip")

        elif pkg == "rust":
            if pm == "pacman":
                packages_to_install.append("rustup")
            else:
                packages_to_install.append("rust")
                packages_to_install.append("rustfmt")
                packages_to_install.append("cargo")

        elif pkg == "ssh":
            if pm == "apt":
                packages_to_install.append("ssh")
            elif pm == "zypper":
                packages_to_install.append("libssh4")
                packages_to_install.append("openssh")
            else:
                packages_to_install.append("libssh")
                packages_to_install.append("openssh")

        else:
            packages_to_install.append(pkg)

    package_manager("module", modules_to_enable)
    package_manager("install", packages_to_install)


def desktop_packages() -> None:
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

    if distro == "centos":
        menu_entries.remove("id3v2")
    if distro == "debian":
        menu_entries.remove("yt-dlp")
    if distro != "arch":
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
            if pm == "zypper":
                packages_to_install.append("ffmpeg-4")
            else:
                packages_to_install.append(pkg)

        elif pkg == "ibus-unikey":
            if distro == "centos":
                packages_to_install.append(
                    "https://rpmfind.net/linux/fedora/linux/releases/34/Everything/x86_64/os/Packages/"
                    "i/ibus-unikey-0.6.1-26.20190311git46b5b9e.fc34.x86_64.rpm")
            else:
                packages_to_install.append(pkg)

        elif pkg == "imagemagick":
            if pm == "dnf":
                packages_to_install.append("ImageMagick")
            elif pm == "apt":
                packages_to_install.append("imagemagick")

        elif pkg == "latex":
            if pm == "apt":
                packages_to_install.append("texlive-latex-base")
                packages_to_install.append("texlive-latex-extra")
            elif pm == "dnf":
                packages_to_install.append("texlive-latex")
                if repo == "fedora":
                    packages_to_install.append("texlive-collection-latexextra")
            elif pm == "pacman":
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

    package_manager("install", packages_to_install)


def remove_packages() -> None:
    packages_to_remove: list[str] = [
        "akregator",
        "evolution",
        "konqueror",
        "kmail",
        "mpv"
    ]

    if distro == "mint":
        packages_to_remove.append("celluloid")
        packages_to_remove.append("drawing")
        packages_to_remove.append("hexchat*")
        packages_to_remove.append("mintbackup")
        packages_to_remove.append("pix*")
        packages_to_remove.append("xed")
    elif distro == "ubuntu" or distro == "debian":
        packages_to_remove.append("gnome-mahjongg")
        packages_to_remove.append("gnome-todo")
        packages_to_remove.append("remmina*")
        packages_to_remove.append("seahorse")

        if distro == "debian":
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

    package_manager("remove", packages_to_remove)
    package_manager("autoremove", [])


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
            update_packages()
            cursor_index = 1
        elif menu_selection_idx == 1:
            repository_setup()
            cursor_index = 2
        elif menu_selection_idx == 2:
            environment_setup()
            cursor_index = 3
        elif menu_selection_idx == 3:
            server_packages()
            cursor_index = 4
        elif menu_selection_idx == 4:
            desktop_packages()
            cursor_index = 5
        elif menu_selection_idx == 5:
            remove_packages()
            cursor_index = 7
        else:
            break


def main() -> None:
    if os.geteuid() != 0:
        print_error("Must be run as root", True)
        exit()

    global distro
    distro = get_distribution()
    if distro == "":
        print_error("Distribution not recognized", True)
        exit()

    global pm
    pm = get_package_manager()
    if pm == "":
        print_error("Package Manager not known", True)
        exit()

    global repo
    repo = get_package_repo()
    if repo == "":
        print_error("Package Repository not known", True)
        exit()

    run()


if __name__ == "__main__":
    main()
