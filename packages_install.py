#!/usr/bin/env python3

import os
import subprocess
import sys
from helper_functions import *
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
            return get_command("apt list --installed | awk -F '/' '{print $1}'").split("\n")
        if self.package_manager == "dnf":
            return get_command("dnf list installed | awk -F '.' '{print $1}'").split("\n")
        if self.package_manager == "pacman":
            return get_command("pacman -Q | awk '{print $1}'").split("\n")
        return []

    def setup_flatpak(self):
        if has_command("flatpak"):
            run_command(
                "sudo flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo")
            if self.name == "debian" or self.package_manager == "dnf":
                adwaita_dark = "org.gtk.Gtk3theme.Adwaita-dark"
                has_adwaita_dark = get_command(
                    "flatpak list --columns=application | grep '" + adwaita_dark + "'")
                if not has_adwaita_dark:
                    run_command("flatpak install flathub " + adwaita_dark + " -y")

    def install_flatpak(self):
        self.install(["flatpak"])
        self.setup_flatpak()

    def install_snap(self):
        self.install(["snapd"])
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

    def install(self, pkgs):
        if self.package_manager == "apt":
            for pkg in pkgs:
                run_command("sudo apt install " + pkg + " -y")
        elif self.package_manager == "dnf":
            for pkg in pkgs:
                run_command("sudo dnf install " + pkg + " -y")
        elif self.package_manager == "pacman":
            for pkg in pkgs:
                run_command("sudo pacman -S " + pkg + " --noconfirm --needed")

    def uninstall(self, pkgs):
        if self.package_manager == "apt":
            for pkg in pkgs:
                run_command("sudo apt remove " + pkg + " -y")
        elif self.package_manager == "dnf":
            for pkg in pkgs:
                run_command("sudo dnf remove " + pkg + " -y")
        elif self.package_manager == "pacman":
            for pkg in pkgs:
                run_command("sudo pacman -Rsun " + pkg + " --noconfirm")

    def autoremove(self):
        if self.package_manager == "apt":
            run_command("sudo apt autoremove -y")
        elif self.package_manager == "dnf":
            run_command("sudo dnf autoremove -y")
        elif self.package_manager == "pacman":
            run_command("sudo pacman -Qdtq | sudo pacman -Rs - --noconfirm")


class Package:
    def __init__(self, name, desc="", group="",
                 repo=[], flatpak="", snap="",
                 repo_other=None, snap_official=False, snap_classic=False, de=""):

        if repo_other is None:
            repo_other = {}

        self.name = name
        self.desc = desc
        self.group = group
        self.repo = repo
        self.flatpak = flatpak
        self.snap = snap
        self.repo_other = repo_other
        self.snap_official = snap_official
        self.snap_classic = snap_classic
        self.de = de

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
has_gnome = False
has_kde = False
packages = {}
groups = {}
currently_installed = {}
selected_installs = {}


# Functions
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
        run_command("flatpak install flathub " + package.flatpak + " -y")
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
    print("Sync Complete")


def execute_update():
    distribution.update()
    if has_command("flatpak"):
        run_command("flatpak update -y")
    if has_command("snap"):
        run_command("sudo snap refresh")
    print("Update Complete")


def execute_autoremove():
    distribution.autoremove()
    if has_command("flatpak"):
        run_command("flatpak remove --unused -y")
    print("Auto-Remove Complete")


def get_desktop_env():
    global has_gnome
    global has_kde
    if has_command("gnome-shell"):
        has_gnome = True
    if has_command("kwriteconfig5"):
        has_kde = True


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
    packages["cheese"] = Package(
        name="Cheese", desc="Webcam", group="Applications",
        repo=["cheese"], flatpak="org.gnome.Cheese", snap="",
        repo_other={})
    packages["deja-dup"] = Package(
        name="Deja-Dup", desc="Backup", group="Applications",
        repo=["deja-dup"], flatpak="org.gnome.DejaDup", snap="deja-dup",
        repo_other={"redhat": []}, snap_classic=True)
    packages["calibre"] = Package(
        name="Calibre", desc="E Book Reader/Editor", group="Applications",
        repo=["calibre"], flatpak="", snap="",
        repo_other={"redhat": []})
    packages["eog"] = Package(
        name="Eye of Gnome", desc="Gnome Image Viewer", group="Applications",
        repo=["eog"], flatpak="org.gnome.eog", snap="eog",
        repo_other={}, snap_official=True, de="gnome")
    packages["evince"] = Package(
        name="Evince", desc="Gnome Document Viewer", group="Applications",
        repo=["evince"], flatpak="org.gnome.Evince", snap="evince",
        repo_other={}, de="gnome")
    packages["foliate"] = Package(
        name="Foliate", desc="E Book Reader", group="Applications",
        repo=[], flatpak="com.github.johnfactotum.Foliate", snap="foliate",
        repo_other={})
    packages["fedora-media-writer"] = Package(
        name="Fedora Media Writer", desc="ISO Writer", group="Applications",
        repo=[], flatpak="org.fedoraproject.MediaWriter", snap="",
        repo_other={"fedora": ["mediawriter"]})
    packages["gnome-books"] = Package(
        name="Gnome Books", desc="", group="Applications",
        repo=["gnome-books"], flatpak="org.gnome.Books", snap="",
        repo_other={"redhat": []}, de="gnome")
    packages["gnome-boxes"] = Package(
        name="Gnome Boxes", desc="Virtual Machine Manager", group="Applications",
        repo=["gnome-boxes"], flatpak="org.gnome.Boxes", snap="",
        repo_other={}, de="gnome")
    packages["gnome-calculator"] = Package(
        name="Gnome Calculator", desc="", group="Applications",
        repo=["gnome-calculator"], flatpak="org.gnome.Calculator", snap="gnome-calculator",
        repo_other={}, snap_official=True, de="gnome")
    packages["gnome-calendar"] = Package(
        name="Gnome Calendar", desc="", group="Applications",
        repo=["gnome-calendar"], flatpak="org.gnome.Calendar", snap="gnome-calendar",
        repo_other={"redhat": []}, snap_official=True, de="gnome")
    packages["gnome-clocks"] = Package(
        name="Gnome Clocks", desc="", group="Applications",
        repo=["gnome-clocks"], flatpak="org.gnome.clocks", snap="gnome-clocks",
        repo_other={"redhat": []}, snap_official=True, de="gnome")
    packages["gnome-connections"] = Package(
        name="Gnome Connections", desc="Network Connection Manager", group="Applications",
        repo=["gnome-connections"], flatpak="org.gnome.Connections", snap="",
        repo_other={"debian": [], "redhat": []}, de="gnome")
    packages["gnome-dialect"] = Package(
        name="Gnome Dialect", desc="", group="Applications",
        repo=[], flatpak="com.github.gi_lom.dialect", snap="",
        repo_other={"fedora": ["dialect"]}, de="gnome")
    packages["gnome-maps"] = Package(
        name="Gnome Maps", desc="", group="Applications",
        repo=["gnome-maps"], flatpak="org.gnome.Maps", snap="",
        repo_other={"redhat": []}, de="gnome")
    packages["gnome-passwordsafe"] = Package(
        name="Gnome Password Safe", desc="", group="Applications",
        repo=["gnome-passwordsafe"], flatpak="org.gnome.PasswordSafe", snap="",
        repo_other={"redhat": []}, de="gnome")
    packages["gnome-weather"] = Package(
        name="Gnome Weather", desc="", group="Applications",
        repo=["gnome-weather"], flatpak="org.gnome.Weather", snap="",
        repo_other={"redhat": []}, de="gnome")
    packages["gnucash"] = Package(
        name="GNU Cash", desc="Accounting Application", group="Applications",
        repo=["gnucash"], flatpak="org.gnucash.GnuCash", snap="",
        repo_other={"redhat": []})
    packages["gramps"] = Package(
        name="GRAMPS", desc="Genealogical Research and Analysis Management Programming System", group="Applications",
        repo=["gramps"], flatpak="org.gramps_project.Gramps", snap="",
        repo_other={"redhat": []})
    packages["gwenview"] = Package(
        name="Gwenview", desc="KDE Image Viewer", group="Applications",
        repo=["gwenview"], flatpak="org.kde.gwenview", snap="gwenview",
        repo_other={"redhat": []}, snap_official=True, de="kde")
    packages["kalendar"] = Package(
        name="Kalendar", desc="KDE Calendar", group="Applications",
        repo=["kalendar"], flatpak="", snap="",
        repo_other={"pacman": [], "redhat": []}, de="kde")
    packages["kcalc"] = Package(
        name="KCalc", desc="KDE Calculator", group="Applications",
        repo=["kcalc"], flatpak="org.kde.kcalc", snap="kcalc",
        repo_other={"redhat": []}, snap_official=True, de="kde")
    packages["okular"] = Package(
        name="Okular", desc="KDE Document Viewer", group="Applications",
        repo=["okular"], flatpak="org.kde.okular", snap="okular",
        repo_other={"redhat": []}, snap_official=True, de="kde")
    packages["transmission-gtk"] = Package(
        name="Transmission (GTK)", desc="Torrent", group="Applications",
        repo=["transmission-gtk"], flatpak="", snap="",
        repo_other={}, de="gnome")
    packages["transmission-qt"] = Package(
        name="Transmission (QT)", desc="Torrent", group="Applications",
        repo=["transmission-qt"], flatpak="", snap="",
        repo_other={"redhat": []}, de="kde")
    packages["virtualbox"] = Package(
        name="Virtual Box", desc="Virtual Machine Manager", group="Applications",
        repo=["virtualbox"], flatpak="", snap="",
        repo_other={"dnf": ["VirtualBox"], "debian": []})

    # Browsers Group
    packages["chromium"] = Package(
        name="Chromium", desc="", group="Browsers",
        repo=["chromium"], flatpak="org.chromium.Chromium", snap="chromium",
        repo_other={"ubuntu": [], "redhat": []}, snap_official=True)
    packages["epiphany"] = Package(
        name="Epiphany", desc="Gnome", group="Browsers",
        repo=["epiphany"], flatpak="org.gnome.Epiphany", snap="",
        repo_other={"redhat": [], "apt": ["epiphany-browser"]}, de="gnome")
    packages["icecat"] = Package(
        name="Icecat", desc="GNU", group="Browsers",
        repo=[], flatpak="", snap="",
        repo_other={"fedora": ["icecat"]})
    packages["firefox"] = Package(
        name="Firefox", desc="", group="Browsers",
        repo=["firefox"], flatpak="org.mozilla.firefox", snap="firefox",
        repo_other={"redhat": [], "debian": []}, snap_official=True)
    packages["firefox-esr"] = Package(
        name="Firefox ESR", desc="Extended Support Release", group="Browsers",
        repo=[], flatpak="", snap="",
        repo_other={"redhat": ["firefox"], "debian": ["firefox-esr"]})
    packages["torbrowser"] = Package(
        name="TOR", desc="The Onion Router", group="Browsers",
        repo=["torbrowser-launcher"], flatpak="com.github.micahflee.torbrowser-launcher", snap="",
        repo_other={"redhat": []})

    # Communication Group
    packages["discord"] = Package(
        name="Discord", desc="Gaming Chat", group="Communication",
        repo=[], flatpak="com.discordapp.Discord", snap="discord",
        repo_other={})
    packages["geary"] = Package(
        name="Geary", desc="Gnome Email", group="Communication",
        repo=["geary"], flatpak="", snap="",
        repo_other={"redhat": []})
    packages["thunderbird"] = Package(
        name="Thunderbird", desc="Email", group="Communication",
        repo=["thunderbird"], flatpak="", snap="thunderbird",
        repo_other={}, snap_official=True)

    # Games Group
    packages["0ad"] = Package(
        name="0 A.D.", desc="Ancient Warfare", group="Games",
        repo=["0ad"], flatpak="com.play0ad.zeroad", snap="0ad",
        repo_other={"redhat": []}, snap_official=True)
    packages["aisleriot"] = Package(
        name="Aisleriot", desc="Solitare", group="Games",
        repo=["aisleriot"], flatpak="org.gnome.Aisleriot", snap="",
        repo_other={"redhat": []}, de="gnome")
    packages["gnome-2048"] = Package(
        name="Gnome 2048", desc="", group="Games",
        repo=["gnome-2048"], flatpak="org.gnome.TwentyFortyEight", snap="",
        repo_other={"redhat": []}, de="gnome")
    packages["gnome-chess"] = Package(
        name="Gnome Chess", desc="", group="Games",
        repo=["gnome-chess"], flatpak="org.gnome.Chess", snap="",
        repo_other={"redhat": []}, de="gnome")
    packages["gnome-mines"] = Package(
        name="Gnome Mines", desc="", group="Games",
        repo=["gnome-mines"], flatpak="org.gnome.Mines", snap="",
        repo_other={"redhat": []}, de="gnome")
    packages["gnome-sudoku"] = Package(
        name="Gnome Sudoku", desc="", group="Games",
        repo=["gnome-sudoku"], flatpak="org.gnome.Sudoku", snap="gnome-sudoku",
        repo_other={"redhat": []}, snap_official=True, de="gnome")
    packages["kmines"] = Package(
        name="KMines", desc="", group="Games",
        repo=["kmines"], flatpak="", snap="kmines",
        repo_other={"redhat": []}, snap_official=True, de="kde")
    packages["knights"] = Package(
        name="KNights", desc="", group="Games",
        repo=["knights"], flatpak="", snap="knights",
        repo_other={"redhat": []}, snap_official=True, de="kde")
    packages["ksudoku"] = Package(
        name="KSudoku", desc="", group="Games",
        repo=["ksudoku"], flatpak="org.kde.ksudoku", snap="ksudoku",
        repo_other={"redhat": []}, snap_official=True, de="kde")
    packages["quadrapassel"] = Package(
        name="Quadrapassel", desc="Gnome Tetris", group="Games",
        repo=["quadrapassel"], flatpak="org.gnome.Quadrapassel", snap="quadrapassel",
        repo_other={"redhat": []}, snap_official=True, de="gnome")
    packages["steam"] = Package(
        name="Steam", desc="", group="Games",
        repo=[], flatpak="com.valvesoftware.Steam", snap="",
        repo_other={})
    packages["supertuxkart"] = Package(
        name="Super Tux Kart", desc="", group="Games",
        repo=["supertuxkart"], flatpak="net.supertuxkart.SuperTuxKart", snap="supertuxkart",
        repo_other={"redhat": []})
    packages["xonotic"] = Package(
        name="Xonotic", desc="FPS", group="Games",
        repo=[], flatpak="org.xonotic.Xonotic", snap="xonotic",
        repo_other={"fedora": ["xonotic"]})

    # Multi Media Group
    packages["blender"] = Package(
        name="Blender", desc="3D Modleler and Video Editor", group="Multi Media",
        repo=["blender"], flatpak="org.blender.Blender", snap="blender",
        repo_other={"redhat": []}, snap_official=True, snap_classic=True)
    packages["elisa"] = Package(
        name="Elisa", desc="KDE Music Player", group="Multi Media",
        repo=["elisa"], flatpak="org.kde.elisa", snap="",
        repo_other={"redhat": []}, de="kde")
    packages["gimp"] = Package(
        name="GIMP", desc="GNU Image Manipulation Program", group="Multi Media",
        repo=["gimp"], flatpak="org.gimp.GIMP", snap="gimp",
        repo_other={})
    packages["gnome-music"] = Package(
        name="Gnome Music", desc="", group="Multi Media",
        repo=["gnome-music"], flatpak="org.gnome.Music", snap="",
        repo_other={"redhat": []}, de="gnome")
    packages["gnome-photos"] = Package(
        name="Gnome Photos", desc="", group="Multi Media",
        repo=["gnome-photos"], flatpak="org.gnome.Photos", snap="",
        repo_other={}, de="gnome")
    packages["gnome-sound-recorder"] = Package(
        name="Gnome Sound Recorder", desc="", group="Multi Media",
        repo=["gnome-sound-recorder"], flatpak="org.gnome.SoundRecorder", snap="",
        repo_other={"redhat": []}, de="gnome")
    packages["kdenlive"] = Package(
        name="KdenLive", desc="KDE Video Editor", group="Multi Media",
        repo=["kdenlive"], flatpak="org.kde.kdenlive", snap="kdenlive",
        repo_other={"redhat": []}, snap_official=True, de="kde")
    packages["rhythmbox"] = Package(
        name="RhythmBox", desc="Music Player", group="Multi Media",
        repo=["rhythmbox"], flatpak="org.gnome.Rhythmbox3", snap="",
        repo_other={}, de="gnome")
    packages["shotwell"] = Package(
        name="Shotwell", desc="Photos", group="Multi Media",
        repo=["shotwell"], flatpak="org.gnome.Shotwell", snap="",
        repo_other={"redhat": []}, de="gnome")
    packages["totem"] = Package(
        name="Totem", desc="Gnome Video Player", group="Multi Media",
        repo=["totem"], flatpak="org.gnome.Totem", snap="",
        repo_other={}, de="gnome")
    packages["vlc"] = Package(
        name="VLC", desc="Media Player", group="Multi Media",
        repo=["vlc"], flatpak="org.videolan.VLC", snap="vlc",
        repo_other={}, snap_official=True)

    # Editors Group
    packages["code"] = Package(
        name="VS Code", desc="Visual Studio Code", group="Editors",
        repo=[], flatpak="", snap="code",
        repo_other={}, snap_official=True, snap_classic=True)
    packages["codium"] = Package(
        name="Codium", desc="FOSS Visual Studio Code", group="Editors",
        repo=[], flatpak="com.vscodium.codium", snap="",
        repo_other={"pacman": ["code"]})
    packages["gedit"] = Package(
        name="gedit", desc="Gnome Text Editor", group="Editors",
        repo=["gedit"], flatpak="org.gnome.gedit", snap="",
        repo_other={}, de="gnome")
    packages["gnome-builder"] = Package(
        name="Gnome Builder", desc="Gnome IDE", group="Editors",
        repo=[], flatpak="org.gnome.Builder", snap="",
        repo_other={}, de="gnome")
    packages["kate"] = Package(
        name="Kate", desc="Text Editor", group="Editors",
        repo=["kate"], flatpak="", snap="",
        repo_other={"redhat": []}, de="kde")
    packages["kwrite"] = Package(
        name="KWrite", desc="KDE Text Editor", group="Editors",
        repo=["kwrite"], flatpak="org.kde.kwrite", snap="",
        repo_other={"redhat": []}, de="kde")
    packages["kdevelop"] = Package(
        name="KDevelop", desc="KDE IDE", group="Editors",
        repo=["kdevelop"], flatpak="org.kde.kdevelop", snap="kdevelop",
        repo_other={"redhat": []}, snap_official=True, de="kde")
    packages["libreoffice"] = Package(
        name="LibreOffice", desc="Office Suite", group="Editors",
        repo=["libreoffice-writer", "libreoffice-calc",
              "libreoffice-impress", "libreoffice-draw", "libreoffice-base"],
        flatpak="org.libreoffice.LibreOffice", snap="libreoffice",
        repo_other={"pacman": ["libreoffice-fresh"]}, snap_official=True)
    packages["texstudio"] = Package(
        name="TeX Studio", desc="LaTex Editor", group="Editors",
        repo=[], flatpak="org.texstudio.TeXstudio", snap="",
        repo_other={"pacman": ["texstudio"]})
    packages["pycharm"] = Package(
        name="PyCharm", desc="JetBrains Python Editor", group="Editors",
        repo=[], flatpak="com.jetbrains.PyCharm-Community", snap="pycharm-community",
        repo_other={"pacman": ["pycharm-community-edition"]}, snap_official=True, snap_classic=True)

    # Software Group
    packages["gnome-software"] = Package(
        name="Gnome Software", desc="", group="Software",
        repo=["gnome-software"], flatpak="", snap="",
        repo_other={"pop": []}, de="gnome")
    packages["plasma-discover"] = Package(
        name="Plasma Discover", desc="", group="Software",
        repo=["plasma-discover"], flatpak="", snap="",
        repo_other={"pacman": ["discover"], "redhat": []}, de="kde")
    packages["snap-store"] = Package(
        name="Snap Store", desc="", group="Software",
        repo=[], flatpak="", snap="snap-store",
        repo_other={}, snap_official=True)
    packages["synaptic"] = Package(
        name="Synaptic", desc="Apt Software Manager", group="Software",
        repo=[], flatpak="", snap="",
        repo_other={"apt": ["synaptic"]})

    # Utilities Group
    packages["ark"] = Package(
        name="Ark", desc="KDE Archiving Tool", group="Utilities",
        repo=["ark"], flatpak="org.kde.ark", snap="ark",
        repo_other={"redhat": []}, snap_official=True, de="kde")
    packages["baobab"] = Package(
        name="Baobab", desc="Gnome Disk Usage", group="Utilities",
        repo=["baobab"], flatpak="", snap="",
        repo_other={}, de="gnome")
    packages["dconf-editor"] = Package(
        name="dconf editor", desc="Gnome Environment Variables", group="Utilities",
        repo=["dconf-editor"], flatpak="", snap="",
        repo_other={}, de="gnome")
    packages["gnome-disk-utility"] = Package(
        name="Gnome Disk Utility", desc="", group="Utilities",
        repo=["gnome-disk-utility"], flatpak="", snap="",
        repo_other={}, de="gnome")
    packages["gnome-system-monitor"] = Package(
        name="Gnome System Monitor", desc="", group="Utilities",
        repo=["gnome-system-monitor"], flatpak="", snap="",
        repo_other={}, de="gnome")
    packages["gnome-tweaks"] = Package(
        name="Gnome Tweaks", desc="", group="Utilities",
        repo=["gnome-tweaks"], flatpak="", snap="",
        repo_other={}, de="gnome")
    packages["ksysguard"] = Package(
        name="KSysGuard", desc="KDE System Monitor", group="Utilities",
        repo=["ksysguard"], flatpak="", snap="",
        repo_other={"redhat": []}, de="kde")
    packages["plasma-systemmonitor"] = Package(
        name="Plasma System Monitor", desc="KDE System Monitor", group="Utilities",
        repo=["plasma-systemmonitor"], flatpak="", snap="",
        repo_other={"redhat": []}, de="kde")
    packages["simple-scan"] = Package(
        name="Simple Scan", desc="", group="Utilities",
        repo=["simple-scan"], flatpak="", snap="",
        repo_other={})
    packages["spectacle"] = Package(
        name="Spectacle", desc="KDE Screenshot", group="Utilities",
        repo=["spectacle"], flatpak="", snap="spectacle",
        repo_other={"redhat": []}, snap_official=True, de="kde")


def define_groups():
    for pkg in packages:
        package = packages[pkg]
        if package.group not in groups:
            groups[package.group] = []
        groups[package.group].append(pkg)


def get_installed_flatpak():
    if has_command("flatpak"):
        return get_command("flatpak list --app | awk -F '\t' '{print $2}'").split("\n")
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
            for r in package.get_repo():
                if r in installed_repo:
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
    Button(root, text="Update", command=execute_update, height=2,
           width=16, fg="black", bg="white").grid(row=0, column=1)
    Button(root, text="Auto-Remove", command=execute_autoremove,
           height=2, width=16, fg="black", bg="white").grid(row=1, column=1)
    Button(root, text="Sync Packages", command=execute_sync_packages,
           height=2, width=16, fg="black", bg="white").grid(row=2, column=1)

    box = ScrolledText(root)
    box.config(height=35, width=105, fg="black", bg="white")
    box.grid(row=0, column=0, rowspan=3)

    for group in groups:
        group_frame = Frame(root, bg="white")

        row = 0
        Label(group_frame, text=group, font=("", 16), fg="black",
              bg="white").grid(row=row, column=0, columnspan=6, pady=10)

        row += 1
        Label(group_frame, text="Package", width=24, fg="black",
              bg="white").grid(row=row, column=0, pady=10)
        Label(group_frame, text="Description", width=40,
              fg="black", bg="white").grid(row=row, column=1)
        Label(group_frame, text="Repo", width=6, fg="black",
              bg="white").grid(row=row, column=2)
        Label(group_frame, text="Flatpak", width=6,
              fg="black", bg="white").grid(row=row, column=3)
        Label(group_frame, text="Snap", width=6, fg="black",
              bg="white").grid(row=row, column=4)
        Label(group_frame, text="Remove", width=7,
              fg="black", bg="white").grid(row=row, column=5)

        for pkg in groups[group]:
            package = packages[pkg]
            has_package = False
            has_flatpak = False
            has_snap = False

            if package.get_repo() and len(package.get_repo()) > 0:
                has_package = True

            if package.flatpak:
                has_flatpak = True

            if package.snap:
                has_snap = True

            if not has_package and not has_flatpak and not has_snap:
                continue

            de_bg = "white"
            if package.de == "gnome":
                if not has_gnome:
                    continue
                de_bg = "#c3d0ff"
            elif package.de == "kde":
                if not has_kde:
                    continue
                de_bg = "#c7ffc3"

            row += 1
            ttk.Separator(group_frame, orient="horizontal").grid(
                row=row, column=0, columnspan=6, sticky="we")

            row += 1
            Label(group_frame, text=package.name, wraplength=180,
                  fg="black", bg=de_bg).grid(row=row, column=0, sticky="e")
            Label(group_frame, text=package.desc, wraplength=280,
                  fg="black", bg="white").grid(row=row, column=1)

            if has_package:
                Radiobutton(
                    group_frame,
                    text="R",
                    variable=selected_installs[pkg],
                    value="repo",
                    fg="black",
                    bg="#99ffb9" if currently_installed[pkg] == "repo" else "#e6ffee"
                ).grid(row=row, column=2)

            if has_flatpak:
                Radiobutton(
                    group_frame,
                    text="F",
                    variable=selected_installs[pkg],
                    value="flatpak",
                    fg="black",
                    bg="#99d6ff" if currently_installed[pkg] == "flatpak" else "#e6f5ff"
                ).grid(row=row, column=3)

            if has_snap:
                snap_bg = ""
                if package.snap_official:
                    snap_bg = "#d699ff" if currently_installed[pkg] == "snap" else "#f5e6ff"
                else:
                    snap_bg = "#ffd699" if currently_installed[pkg] == "snap" else "#fff5e6"

                Radiobutton(
                    group_frame,
                    text="S",
                    variable=selected_installs[pkg],
                    value="snap",
                    fg="black",
                    bg=snap_bg
                ).grid(row=row, column=4)

            Radiobutton(
                group_frame,
                text="X",
                variable=selected_installs[pkg],
                value="",
                fg="black",
                bg="#ff9999" if not currently_installed[pkg] else "#ffe6e6"
            ).grid(row=row, column=5, padx=12)

        row += 1
        ttk.Separator(group_frame, orient="horizontal").grid(
            row=row, column=0, columnspan=6, sticky="we")

        box.window_create(END, window=group_frame)
        box.insert(END, "\n")


def main():
    if os.getuid() != 0:
        print("Must be executed as root")
        sys.exit(1)

    get_desktop_env()
    get_distribution()
    define_packages()
    define_groups()

    distribution.setup_flatpak()

    root = Tk()
    root.title("Package Installer")
    root["bg"] = "white"
    get_packages()
    create_gui(root)
    root.mainloop()


if __name__ == "__main__":
    main()
