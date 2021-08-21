#!/usr/bin/env python3

import os
import subprocess
from tkinter import *
from tkinter import ttk
from tkinter.scrolledtext import ScrolledText


# Classes
class Distribution:
    def __init__(self, name, repository, package_manager):
        self.name = name
        self.repository = repository
        self.package_manager = package_manager

    def get_installed_repo(self):
        if self.package_manager == "apt":
            return get_command("apt list --installed | sed 's/\// /' | awk '{print $1}'").split("\n")
        if self.package_manager == "dnf":
            return get_command("dnf list installed | sed 's/\./ /' | awk '{print $1}'").split("\n")
        if self.package_manager == "pacman":
            return get_command("pacman -Q | awk '{print $1}'").split("\n")
        return []

    def install_flatpak(self):
        self.install("flatpak")
        run_command("sudo flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo")

    def install_snap(self):
        self.install("snapd")
        if self.package_manager == "dnf":
            run_command("sudo systemctl enable --now snapd.socket")
            run_command("sudo ln -s /var/lib/snapd/snap /snap")

    def update(self):
        if self.package_manager == "apt":
            run_command("sudo apt update && sudo apt upgrade -y")
        elif self.package_manager == "dnf":
            run_command("sudo dnf upgrade --refresh -y")
        elif self.package_manager == "pacman":
            run_command("sudo pacman -Syyu --noconfirm")

    def install(self, pkg):
        if self.package_manager == "apt":
            run_command("sudo apt install " + pkg + " -y")
        elif self.package_manager == "dnf":
            run_command("sudo dnf install " + pkg + " -y")
        elif self.package_manager == "pacman":
            run_command("sudo pacman -S " + pkg + " --noconfirm --needed")

    def uninstall(self, pkg):
        if self.package_manager == "apt":
            run_command("sudo apt remove " + pkg + " -y")
        elif self.package_manager == "dnf":
            run_command("sudo dnf remove " + pkg + " -y")
        elif self.package_manager == "pacman":
            run_command("sudo pacman -Rsun " + pkg + " --noconfirm")

    def autoremove(self):
        if self.package_manager == "apt":
            run_command("sudo apt autoremove -y")
        elif self.package_manager == "dnf":
            run_command("sudo dnf autoremove -y")
        elif self.package_manager == "pacman":
            run_command("sudo pacman -Qdtq | sudo pacman -Rs -")


class Package:
    def __init__(self, name, desc="", group="", repo="", flatpak="", snap="", repo_other=None, snap_classic=False):

        if repo_other is None:
            repo_other = {}

        self.name = name
        self.desc = desc
        self.group = group
        self.repo = repo
        self.flatpak = flatpak
        self.snap = snap
        self.repo_other = repo_other
        self.snap_classic = snap_classic

    def get_repo(self):
        if distribution.name in self.repo_other:
            return self.repo_other[distribution.name]
        if distribution.repository in self.repo_other:
            return self.repo_other[distribution.repository]
        if distribution.package_manager in self.repo_other:
            return self.repo_other[distribution.package_manager]
        return self.repo


# Global Variables
distribution = Distribution("", "", "")
packages = {}
groups = {}
currently_installed = {}
selected_installs = {}


# Functions
def has_command(command):
    return os.system("command -v " + command + " >/dev/null 2>&1") == 0


def get_command(command):
    return subprocess.check_output(["bash", "-c", command]).decode("utf-8").strip()


def run_command(command):
    subprocess.run(["bash", "-c", command])


def uninstall_package(pkg, method):
    package = packages[pkg]
    if method == "repo":
        distribution.uninstall(package.get_repo())
    elif method == "flatpak":
        run_command("flatpak remove " + package.flatpak + " -y")
    elif method == "snap":
        run_command("sudo snap remove " + package.snap)


def install_package(pkg, method):
    package = packages[pkg]
    if method == "repo":
        distribution.install(package.get_repo())
    elif method == "flatpak":
        if not has_command("flatpak"):
            distribution.install_flatpak()
        run_command("flatpak install " + package.flatpak + " -y")
    elif method == "snap":
        if not has_command("snap"):
            distribution.install_snap()
        if package.snap_classic:
            run_command("sudo snap install " + package.snap + " --classic")
        else:
            run_command("sudo snap install " + package.snap)


def execute_sync_packages():
    global currently_installed
    global selected_installs
    for pkg in selected_installs:
        if selected_installs[pkg].get() != currently_installed[pkg]:
            if currently_installed[pkg]:
                uninstall_package(pkg, currently_installed[pkg])
            if selected_installs[pkg].get():
                install_package(pkg, selected_installs[pkg].get())
            currently_installed[pkg] = selected_installs[pkg].get()


def execute_update():
    distribution.update()
    if has_command("flatpak"):
        run_command("flatpak update -y")
    if has_command("snap"):
        run_command("sudo snap refresh")


def execute_autoremove():
    distribution.autoremove()
    if has_command("flatpak"):
        run_command("flatpak remove --unused -y")


def get_distro():
    for file_name in ["/etc/lsb-release", "/usr/lib/os-release", "/etc/os-release"]:
        if os.path.isfile(file_name):
            file = open(file_name, "r")
            for line in file:
                if line.startswith('PRETTY_NAME="') or line.startswith('DISTRIB_DESCRIPTION="'):
                    return line.split('"')[1]
    return ""


def get_distribution():
    distro = get_distro().lower()
    global distribution
    if "alma" in distro:
        distribution = Distribution("alma", "redhat", "dnf")
    elif "arch" in distro:
        distribution = Distribution("arch", "arch", "pacman")
    elif "centos" in distro:
        distribution = Distribution("centos", "redhat", "dnf")
    elif "debian" in distro:
        distribution = Distribution("debian", "debian", "apt")
    elif "fedora" in distro:
        distribution = Distribution("fedora", "fedora", "dnf")
    elif "lmde" in distro:
        distribution = Distribution("lmde", "debian", "apt")
    elif "manjaro" in distro:
        distribution = Distribution("manjaro", "arch", "pacman")
    elif "mint" in distro:
        distribution = Distribution("mint", "ubuntu", "apt")
    elif "pop" in distro:
        distribution = Distribution("pop", "ubuntu", "apt")
    elif "rocky" in distro:
        distribution = Distribution("rocky", "redhat", "dnf")
    elif "ubuntu" in distro:
        distribution = Distribution("ubuntu", "ubuntu", "apt")
    else:
        print("Linux Distribution not recognized")
        sys.exit(1)


def define_packages():
    # Applications Group
    packages["cheese"] = Package(name="Cheese", desc="Webcam", group="Applications",
                                 repo="cheese", flatpak="org.gnome.Cheese",
                                 repo_other={"redhat": ""})
    packages["deja-dup"] = Package(name="Deja-Dup", desc="Backup", group="Applications",
                                   repo="deja-dup", flatpak="org.gnome.DejaDup",
                                   repo_other={"redhat": ""})
    packages["calibre"] = Package(name="Calibre", desc="E Book Reader/Editor", group="Applications",
                                  repo="calibre")
    packages["eog"] = Package(name="Eye of Gnome", desc="Gnome Image Viewer", group="Applications",
                              repo="eog", flatpak="org.gnome.eog")
    packages["evince"] = Package(name="Evince", desc="Gnome Document Viewer", group="Applications",
                                 repo="evince", flatpak="org.gnome.Evince")
    packages["foliate"] = Package(name="Foliate", desc="E Book Reader", group="Applications",
                                  flatpak="com.github.johnfactotum.Foliate", snap="foliate")
    packages["gnome-books"] = Package(name="Gnome Books", group="Applications",
                                      repo="gnome-books", flatpak="org.gnome.Books",
                                      repo_other={"redhat": ""})
    packages["gnome-boxes"] = Package(name="Gnome Boxes", desc="Virtual Machine Manager", group="Applications",
                                      repo="gnome-boxes", flatpak="org.gnome.Boxes")
    packages["gnome-calculator"] = Package(name="Gnome Calculator", group="Applications",
                                           repo="gnome-calculator", flatpak="org.gnome.Calculator")
    packages["gnome-calendar"] = Package(name="Gnome Calendar", group="Applications",
                                         repo="gnome-calendar", flatpak="org.gnome.Calendar",
                                         repo_other={"redhat": ""})
    packages["gnome-clocks"] = Package(name="Gnome Clocks", group="Applications",
                                       repo="gnome-clocks", flatpak="org.gnome.clocks",
                                       repo_other={"redhat": ""})
    packages["gnome-photos"] = Package(name="Gnome Photos", group="Applications",
                                       repo="gnome-photos", flatpak="org.gnome.Photos")
    packages["gnome-software"] = Package(name="Gnome Software", group="Applications",
                                         repo="gnome-software",
                                         repo_other={"pop": ""})
    packages["gnome-weather"] = Package(name="Gnome Weather", group="Applications",
                                        repo="gnome-weather", flatpak="org.gnome.Weather",
                                        repo_other={"redhat": ""})
    packages["gnucash"] = Package(name="GNU Cash", desc="Accounting Application", group="Applications",
                                  repo="gnucash", flatpak="org.gnucash.GnuCash")
    packages["gramps"] = Package(name="GRAMPS",
                                 desc="Genealogical Research and Analysis Management Programming System",
                                 group="Applications",
                                 repo="gramps", flatpak="org.gramps_project.Gramps")
    packages["shotwell"] = Package(name="Shotwell", desc="Photos", group="Applications",
                                   repo="shotwell", flatpak="org.gnome.Shotwell")
    packages["snap-store"] = Package(name="Snap Store", group="Applications",
                                     snap="snap-store")
    packages["synaptic"] = Package(name="Synaptic", desc="Apt Software Manager", group="Applications",
                                   repo_other={"apt": "synaptic"})
    packages["transmission-gtk"] = Package(name="Transmission", desc="Torrent", group="Applications",
                                           repo="transmission-gtk")
    packages["virtualbox"] = Package(name="Virtual Box", desc="Virtual Machine Manager", group="Applications",
                                     repo_other={"apt": "virtualbox", "dnf": "VirtualBox"})

    # Browsers Group
    packages["chromium"] = Package(name="Chromium", group="Browsers",
                                   repo="chromium", flatpak="com.chromium.Chromium", snap="chromium",
                                   repo_other={"ubuntu": ""})
    packages["epiphany"] = Package(name="Epiphany", desc="Gnome", group="Browsers",
                                   repo="epiphany", flatpak="org.gnome.Epiphany",
                                   repo_other={"redhat": "", "apt": "epiphany-browser"})
    packages["icecat"] = Package(name="Icecat", desc="GNU", group="Browsers",
                                 repo_other={"fedora": "icecat"})
    packages["firefox"] = Package(name="Firefox", group="Browsers",
                                  repo="firefox", flatpak="org.mozilla.firefox",
                                  repo_other={"redhat": "", "debian": ""})
    packages["firefox-esr"] = Package(name="Firefox ESR", desc="Extended Support Release", group="Browsers",
                                      repo_other={"redhat": "firefox", "debian": "firefox-esr"})
    packages["torbrowser"] = Package(name="TOR", desc="The Onion Router", group="Browsers",
                                     repo="torbrowser-launcher", flatpak="com.github.micahflee.torbrowser-launcher",
                                     repo_other={"redhat": ""})

    # Communication Group
    packages["discord"] = Package(name="Discord", desc="Gaming Chat", group="Communication",
                                  flatpak="com.discordapp.Discord", snap="discord")
    packages["geary"] = Package(name="Geary", desc="Gnome Email", group="Communication",
                                repo="geary")
    packages["thunderbird"] = Package(name="Thunderbird", desc="Email", group="Communication",
                                      repo="thunderbird")

    # Games Group
    packages["0ad"] = Package(name="0 A.D.", desc="Ancient Warfare", group="Games",
                              repo="0ad", flatpak="org.play0ad.zeroad", snap="0ad",
                              repo_other={"redhat": ""})
    packages["aisleriot"] = Package(name="Aisleriot", desc="Solitare", group="Games",
                                    repo="aisleriot", flatpak="org.gnome.Aisleriot")
    packages["gnome-chess"] = Package(name="Gnome Chess", group="Games",
                                      repo="gnome-chess", flatpak="org.gnome.Chess")
    packages["gnome-sudoku"] = Package(name="Gnome Sudoku", group="Games",
                                       repo="gnome-sudoku", flatpak="org.gnome.Sudoku")
    packages["quadrapassel"] = Package(name="Quadrapassel", desc="Gnome Tetris", group="Games",
                                       repo="quadrapassel", flatpak="org.gnome.Quadrapassel")
    packages["steam"] = Package(name="Steam", group="Games",
                                flatpak="com.valvesoftware.Steam")
    packages["supertuxkart"] = Package(name="Super Tux Kart", group="Games",
                                       repo="supertuxkart", flatpak="net.supertuxkart.SuperTuxKart",
                                       snap="supertuxkart",
                                       repo_other={"redhat": ""})
    packages["xonotic"] = Package(name="Xonotic", desc="FPS", group="Games",
                                  flatpak="org.xonotic.Xonotic", snap="xonotic")

    # Multi Media Group
    packages["blender"] = Package(name="Blender", desc="3D Modleler and Video Editor", group="Multi Media",
                                  repo="blender", flatpak="org.blender.Blender", snap="blender",
                                  repo_other={"redhat": ""}, snap_classic=True)
    packages["gimp"] = Package(name="GIMP", desc="GNU Image Manipulation Program", group="Multi Media",
                               repo="gimp", flatpak="org.gimp.GIMP")
    packages["gnome-music"] = Package(name="Gnome Music", group="Multi Media",
                                      repo="gnome-music", flatpak="org.gnome.Music")
    packages["kdenlive"] = Package(name="KdenLive", desc="KDE Video Editor", group="Multi Media",
                                   repo="kdenlive", flatpak="org.kde.kdenlive")
    packages["rhythmbox"] = Package(name="RhythmBox", desc="Music Player", group="Multi Media",
                                    repo="rhythmbox", flatpak="org.gnome.Rhythmbox3")
    packages["totem"] = Package(name="Totem", desc="Gnome Video Player", group="Multi Media",
                                repo="totem", flatpak="org.gnome.Totem")
    packages["vlc"] = Package(name="VLC", desc="Media Player", group="Multi Media",
                              repo="vlc", flatpak="org.videolan.VLC", snap="vlc")

    # Editors Group
    packages["code"] = Package(name="VS Code", desc="Visual Studio Code", group="Editors",
                               snap="code",
                               snap_classic=True)
    packages["codium"] = Package(name="Codium", desc="FOSS Visual Studio Code", group="Editors",
                                 flatpak="com.vscodium.codium")
    packages["gedit"] = Package(name="gedit", desc="Gnome Text Editor", group="Editors",
                                repo="gedit", flatpak="org.gnome.gedit")
    packages["libreoffice"] = Package(name="LibreOffice", desc="Office Suite", group="Editors",
                                      repo="libreoffice", flatpak="org.libreoffice.LibreOffice", snap="libreoffice",
                                      repo_other={"pacman": "libreoffice-fresh"})
    packages["texstudio"] = Package(name="TeX Studio", desc="LaTex Editor", group="Editors",
                                    flatpak="com.texstudio.TeXstudio")
    packages["pycharm"] = Package(name="PyCharm", desc="JetBrains Python Editor", group="Editors",
                                  flatpak="com.jetbrains.PyCharm-Community", snap="pycharm-community",
                                  snap_classic=True)

    # Utilities Group
    packages["baobab"] = Package(name="Baobab", desc="Gnome Disk Usage", group="Utilities",
                                 repo="baobab")
    packages["dconf-editor"] = Package(name="dconf editor", desc="Gnome Environment Variables",
                                       group="Utilities",
                                       repo="dconf-editor")
    packages["gnome-disk-utility"] = Package(name="Gnome Disk Utility", group="Utilities",
                                             repo="gnome-disk-utility")
    packages["gnome-system-monitor"] = Package(name="Gnome System Monitor", group="Utilities",
                                               repo="gnome-system-monitor")
    packages["gnome-tweaks"] = Package(name="Gnome Tweaks", group="Utilities",
                                       repo="gnome-tweaks")
    packages["simple-scan"] = Package(name="Simple Scan", group="Utilities",
                                      repo="simple-scan")


def define_groups():
    for pkg in packages:
        package = packages[pkg]
        if package.group not in groups:
            groups[package.group] = []
        groups[package.group].append(pkg)


def get_installed_flatpak():
    if has_command("flatpak"):
        return get_command("flatpak list --app | awk '{print $2}'").split("\n")
    return []


def get_installed_snap():
    if has_command("snap"):
        return get_command("snap list | awk '{print $1}'").split("\n")
    return []


def get_packages():
    installed_repo = distribution.get_installed_repo()
    installed_flatpak = get_installed_flatpak()
    installed_snap = get_installed_snap()
    global currently_installed
    global selected_installs
    for pkg in packages:
        currently_installed[pkg] = ""
        selected_installs[pkg] = StringVar()
        package = packages[pkg]

        if package.get_repo():
            if package.get_repo() in installed_repo:
                currently_installed[pkg] = "repo"
                selected_installs[pkg].set("repo")
        if package.flatpak:
            if package.flatpak in installed_flatpak:
                currently_installed[pkg] = "flatpak"
                selected_installs[pkg].set("flatpak")
        if package.snap:
            if package.snap in installed_snap:
                currently_installed[pkg] = "snap"
                selected_installs[pkg].set("snap")


def create_gui(root):
    Button(root, text="Update", command=execute_update, height=2, width=16).grid(row=0, column=1)
    Button(root, text="Auto-Remove", command=execute_autoremove, height=2, width=16).grid(row=1, column=1)
    Button(root, text="Sync Packages", command=execute_sync_packages, height=2, width=16).grid(row=2, column=1)

    box = ScrolledText(root)
    box.config(height=35, width=92)
    box.grid(row=0, column=0, rowspan=3)

    for group in groups:
        group_frame = Frame(root)

        row = 0
        Label(group_frame, text=group, font=("", 16)).grid(row=row, column=0, columnspan=6, pady=10)

        row += 1
        Label(group_frame, text="Package", width=24).grid(row=row, column=0, pady=10)
        Label(group_frame, text="Description", width=40).grid(row=row, column=1)
        Label(group_frame, text="Repo", width=6).grid(row=row, column=2)
        Label(group_frame, text="Flatpak", width=6).grid(row=row, column=3)
        Label(group_frame, text="Snap", width=6).grid(row=row, column=4)
        Label(group_frame, text="Remove", width=7).grid(row=row, column=5)

        for pkg in groups[group]:
            row += 1
            ttk.Separator(group_frame, orient="horizontal").grid(row=row, column=0, columnspan=6, sticky="we")

            row += 1
            package = packages[pkg]

            Label(group_frame, text=package.name, wraplength=180).grid(row=row, column=0, sticky="e")
            Label(group_frame, text=package.desc, wraplength=280).grid(row=row, column=1)

            if package.get_repo():
                Radiobutton(
                    group_frame,
                    text="R",
                    variable=selected_installs[pkg],
                    value="repo"
                ).grid(row=row, column=2)

            if package.flatpak:
                Radiobutton(
                    group_frame,
                    text="F",
                    variable=selected_installs[pkg],
                    value="flatpak"
                ).grid(row=row, column=3)

            if package.snap:
                Radiobutton(
                    group_frame,
                    text="S",
                    variable=selected_installs[pkg],
                    value="snap"
                ).grid(row=row, column=4)

            Radiobutton(
                group_frame,
                text="X",
                variable=selected_installs[pkg],
                value=""
            ).grid(row=row, column=5, padx=12)

        row += 1
        ttk.Separator(group_frame, orient="horizontal").grid(row=row, column=0, columnspan=6, sticky="we")

        box.window_create(END, window=group_frame)
        box.insert(END, "\n")


def main():
    if os.getuid() != 0:
        print("Must be executed as root")
        sys.exit(1)

    get_distribution()
    define_packages()
    define_groups()

    root = Tk()
    root.title("Package Installer")
    get_packages()
    create_gui(root)
    root.mainloop()


if __name__ == "__main__":
    main()
