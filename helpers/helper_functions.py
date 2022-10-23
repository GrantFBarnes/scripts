from __future__ import annotations
import os
import subprocess

# ANSI Escape Sequences
ansi_reset: str = "\033[0m"
ansi_bold: str = "\033[1m"
ansi_red: str = "\033[31m"
ansi_green: str = "\033[32m"
ansi_yellow: str = "\033[33m"
ansi_cyan: str = "\033[36m"
ansi_red_bg: str = "\033[41m"


# Print Functions

def print_message(message: str, include_separators: bool, label: str, color: str) -> None:
    if include_separators:
        print(f"{color}--------------------------------------------------{ansi_reset}")
    print(f"{ansi_bold}{color}{label}{ansi_reset} {message}")
    if include_separators:
        print(f"{color}--------------------------------------------------{ansi_reset}")


def print_success(message: str, include_separators: bool) -> None:
    print_message(message, include_separators, "Success:", ansi_green)


def print_info(message: str, include_separators: bool) -> None:
    print_message(message, include_separators, "Info:", ansi_cyan)


def print_warning(message: str, include_separators: bool) -> None:
    print_message(message, include_separators, "Warning:", ansi_yellow)


def print_error(message: str, include_separators: bool) -> None:
    print_message(message, include_separators, "Error:", ansi_red)


# Command Line Functions

def has_command(command: str) -> bool:
    return os.system("command -v " + command + " >/dev/null 2>&1") == 0


def run_command(command: str) -> None:
    subprocess.run(["bash", "-c", command])


def get_command(command: str) -> str:
    try:
        return subprocess.check_output(["bash", "-c", command]).decode("utf-8").strip()
    except (Exception,):
        return ""


# File Functions

def add_to_file_if_not_found(file_path: str, search_str: str, new_line: str) -> None:
    if os.path.isfile(file_path):
        with open(file_path, "r+") as file:
            for line in file:
                if search_str in line:
                    break
            else:
                file.write(new_line)
    else:
        with open(file_path, "w") as file:
            file.write(new_line)


# Linux Distributions

class Distribution:
    def __init__(self, name: str, repository: str, package_manager: str):
        self.name: str = name
        self.repository: str = repository
        self.package_manager: str = package_manager

    def repository_get_installed(self) -> list[str]:
        if self.package_manager == "apt":
            return get_command("apt list --installed | awk -F '/' '{print $1}'").split("\n")
        elif self.package_manager == "dnf":
            return get_command("dnf list installed | awk -F '.' '{print $1}'").split("\n")
        elif self.package_manager == "pacman":
            return get_command("pacman -Q | awk '{print $1}'").split("\n")
        elif self.package_manager == "zypper":
            return get_command("zypper packages --installed-only | awk -F '|' '{print $3}'").split("\n")

    def repository_get_package_names(self, package: str) -> list[str]:
        # Check distribution exceptions
        if self.name == "":
            return []

        # Check repository exceptions
        if self.repository == "redhat":
            if package == "ibus-unikey":
                return ["https://rpmfind.net/linux/fedora/linux/releases/34/Everything/x86_64/os/Packages"
                        "/i/ibus-unikey-0.6.1-26.20190311git46b5b9e.fc34.x86_64.rpm"]
            if package == "id3v2":
                return []
        elif self.repository == "debian":
            if package == "yt-dlp":
                return []

        # Check package manager exceptions
        if self.package_manager == "dnf":
            if package == "imagemagick":
                return ["ImageMagick"]
        elif self.package_manager == "zypper":
            if package == "ffmpeg":
                return ["ffmpeg-4"]

        # Check package exceptions
        if package == "latex":
            if self.repository == "fedora":
                return ["texlive-latex", "texlive-collection-latexextra"]
            if self.package_manager == "dnf":
                return ["texlive-latex"]
            elif self.package_manager == "apt":
                return ["texlive-latex-base", "texlive-latex-extra"]
            elif self.package_manager == "pacman":
                return ["texlive-core", "texlive-latexextra"]
        elif package == "mariadb":
            if self.package_manager == "pacman" or self.package_manager == "zypper":
                return ["mariadb"]
            else:
                return ["mariadb-server"]
        elif package == "node":
            if self.package_manager == "zypper":
                return ["nodejs16", "npm16"]
            else:
                return ["nodejs", "npm"]
        elif package == "rust":
            if self.package_manager == "pacman":
                return ["rustup"]
            else:
                return ["rust", "rustfmt", "cargo"]
        elif package == "ssh":
            if self.package_manager == "apt":
                return ["ssh"]
            elif self.package_manager == "zypper":
                return ["libssh4", "openssh"]
            else:
                return ["libssh", "openssh"]
        elif package == "qtile":
            if self.package_manager == "arch":
                return ["qtile", "alacritty", "rofi", "numlockx", "playerctl"]
            else:
                return []

        return [package]

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
                distro_version: str = get_command("rpm -E %" + self.name)

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

    def install_pip(self) -> None:
        if not has_command("pip3"):
            if self.package_manager == "apt":
                self.repository_install(["python3-pip"])
            elif self.package_manager == "dnf":
                self.repository_install(["python3-pip"])
            elif self.package_manager == "pacman":
                self.repository_install(["python-pip"])
            elif self.package_manager == "zypper":
                self.repository_install(["python38-pip"])


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
    if "Rocky" in distro_name:
        return Distribution("rocky", "redhat", "dnf")
    if "SUSE" in distro_name:
        return Distribution("suse", "suse", "zypper")
    if "Ubuntu" in distro_name:
        return Distribution("ubuntu", "ubuntu", "apt")
    return None
