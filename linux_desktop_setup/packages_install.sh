#!/bin/bash
# Purpose: Install/Remove basic packages for GNU/Linux Desktop
################################################################################

function confirmWhiptail() {
    local height=7
    if [ -n "$2" ]; then
        height=$2
    fi
    whiptail --title "Set up GNU/Linux Desktop" --yesno --defaultno "$1" $height 50
}

function choosePackagesWhiptail() {
    packageSelections=$(whiptail --title "Set up GNU/Linux Desktop" --checklist "Select Packages to Install:" --cancel-button "Cancel" 0 0 0 "${packageOptions[@]}" 3>&1 1>&2 2>&3)
    return $?
}

function chooseCategoryWhiptail() {
    categorySelection=$(whiptail --title "Set up GNU/Linux Desktop" --menu "Select a Category to Find Packages:" --cancel-button "Cancel" --default-item "${defaultCategory}" 0 0 0 "${categoryOptions[@]}" 3>&1 1>&2 2>&3)
    return $?
}

function checkNotInstalled() {
    if ! command -v $1 &> /dev/null; then
        return 0
    fi
    echo "$1 already installed"
    return 1
}

function checkExitStatus() {
    if [ $? -eq 0 ]; then
        echo "Success"
    else
        echo "[ERROR] Process Failed!"
        read -p "Exit? (y/N) " ans
        if [ "$ans" == "y" ]; then
            exit 1
        fi
    fi
}

function update() {
    echo "---------------------------------------------------------------------"
    echo "Update"
    echo "---------------------------------------------------------------------"
    if [ "$pm" == "apt" ]; then
        sudo apt update && sudo apt full-upgrade -y
    elif [ "$pm" == "dnf" ]; then
        sudo dnf upgrade --refresh -y
    elif [ "$pm" == "pacman" ]; then
        sudo pacman -Syyu --noconfirm
    fi
    checkExitStatus
}

function packageManager() {
    local method=$1
    echo "---------------------------------------------------------------------"
    echo "$pm $method ${@:2}"
    echo "---------------------------------------------------------------------"
    if [ "$pm" == "pacman" ]; then
        if [ "$method" == "install" ]; then
            sudo $pm -S ${@:2} --noconfirm --needed
        elif [ "$method" == "remove" ]; then
            sudo $pm -Rsscun ${@:2} --noconfirm
        fi
    elif [ "$method" == "remove" ] && [ "$pm" == "apt" ]; then
        sudo apt-get remove --purge ${@:2} -y
    else
        sudo $pm $method ${@:2} -y
    fi
    checkExitStatus
}

function aurManager() {
    echo "---------------------------------------------------------------------"
    echo "aur install $1"
    echo "---------------------------------------------------------------------"
    sudo -u $SUDO_USER git clone https://aur.archlinux.org/$1.git
    cd $1
    sudo -u $SUDO_USER makepkg -si --noconfirm
    cd ..
    rm -rf $1
    checkExitStatus
}

function snapManager() {
    local method=$1
    echo "---------------------------------------------------------------------"
    echo "snap $method ${@:2}"
    echo "---------------------------------------------------------------------"
    sudo snap $method ${@:2}
    checkExitStatus
}

function flatpakManager() {
    local method=$1
    echo "---------------------------------------------------------------------"
    echo "flatpak $method ${@:2}"
    echo "---------------------------------------------------------------------"
    if [ "$method" == "install" ]; then
        sudo flatpak $method flathub ${@:2} -y
    else
        sudo flatpak $method ${@:2} -y
    fi
    checkExitStatus
}

################################################################################

clear

# Determine distrobution

osName=$(head -n 1 /etc/os-release)
distro=""
pm=""
de=""

if [[ $osName == *"Arch"* ]]; then
    distro="arch"
    pm="pacman"
elif [[ $osName == *"CentOS"* ]]; then
    distro="centos"
    pm="dnf"
    de="gnome"
elif [[ $osName == *"Debian"* ]]; then
    distro="debian"
    pm="apt"
    de="gnome"
elif [[ $osName == *"Fedora"* ]]; then
    distro="fedora"
    pm="dnf"
    de="gnome"
elif [[ $osName == *"LMDE"* ]]; then
    distro="lmde"
    pm="apt"
    de="cinnamon"
elif [[ $osName == *"Mint"* ]]; then
    distro="mint"
    pm="apt"
    de="cinnamon"
elif [[ $osName == *"Pop!_OS"* ]]; then
    distro="pop"
    pm="apt"
    de="gnome"
elif [[ $osName == *"Ubuntu"* ]]; then
    distro="ubuntu"
    pm="apt"
    de="gnome"
else
    echo "---------------------------------------------------------------------"
    echo "Distrobution not recognized"
    echo "---------------------------------------------------------------------"
    exit 1
fi

################################################################################

# Update and set up repos

update

if [ "$pm" == "dnf" ]; then
    grep -q max_parallel_downloads /etc/dnf/dnf.conf
    if [ $? -eq 1 ]; then
        sudo sh -c 'echo max_parallel_downloads=10 >> /etc/dnf/dnf.conf'
        sudo sh -c 'echo fastestmirror=true >> /etc/dnf/dnf.conf'

        if [ "$distro" == "fedora" ]; then
            sudo dnf install https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm -y
        elif [ "$distro" == "centos" ]; then
            sudo dnf install --nogpgcheck https://dl.fedoraproject.org/pub/epel/epel-release-latest-8.noarch.rpm -y
            sudo dnf install --nogpgcheck https://download1.rpmfusion.org/free/el/rpmfusion-free-release-8.noarch.rpm https://download1.rpmfusion.org/nonfree/el/rpmfusion-nonfree-release-8.noarch.rpm -y
        fi

        update
        packageManager install newt
    fi
elif [ "$distro" == "mint" ]; then
    nosnap=/etc/apt/preferences.d/nosnap.pref
    if [ -f "$nosnap" ]; then
        sudo rm $nosnap
        update
    fi
fi

grep -q EDITOR ~/.bashrc
if [ $? -eq 1 ]; then
    sudo sh -c 'echo export EDITOR="nano" >> ~/.bashrc'
fi

confirmWhiptail "   Distrobution: $distro\n    Desktop Env: $de\nPackage Manager: $pm\n\nWould you like to continue?" 11
if [ $? -eq 1 ]; then
    exit 0
fi

################################################################################

bulkInstallPackages=true
sourcePreference="repo"
preferRepoOverSnap=true
preferRepoOverFlatpak=true
preferFlatpakOverSnap=true

confirmWhiptail "Install packages individually?"
if [ $? -eq 0 ]; then
    bulkInstallPackages=false
fi

confirmWhiptail "Do you prefer snap over repository?"
if [ $? -eq 0 ]; then
    preferRepoOverSnap=false
else
    preferRepoOverSnap=true
fi

confirmWhiptail "Do you prefer flatpak over repository?"
if [ $? -eq 0 ]; then
    preferRepoOverFlatpak=false
else
    preferRepoOverFlatpak=true
fi

if [ "$preferRepoOverSnap" == true ] && [ "$preferRepoOverFlatpak" == true ]; then
    # Prefer repo over both snaps and flatpaks
    sourcePreference="repo"
    confirmWhiptail "Do you prefer snap over flatpak?"
    if [ $? -eq 0 ]; then
        preferFlatpakOverSnap=false
    else
        preferFlatpakOverSnap=true
    fi
elif [ "$preferRepoOverSnap" == true ]; then
    # Prefer repo over snap, but flatpak over repo
    sourcePreference="flatpak"
    preferFlatpakOverSnap=true
elif [ "$preferRepoOverFlatpak" == true ]; then
    # Prefer repo over flatpak, but snap over repo
    sourcePreference="snap"
    preferFlatpakOverSnap=false
else
    # Prefer both snap and flatpak over repo
    confirmWhiptail "Do you prefer snap over flatpak?"
    if [ $? -eq 0 ]; then
        sourcePreference="snap"
        preferFlatpakOverSnap=false
    else
        sourcePreference="flatpak"
        preferFlatpakOverSnap=true
    fi
fi

################################################################################

# Determine Packages to install and remove

defaultCategory="."
categorySelection=""
declare -a categoryOptions

declare -a packageOptions
declare -a packageSelections

declare -a packagesToInstall
declare -a aurToInstall
declare -a snapsToInstall
declare -a flatpaksToInstall

declare -a packagesToRemove
declare -a snapsToRemove
declare -a flatpaksToRemove

# Always install the following packages
packagesToInstall+=(flatpak)
packagesToInstall+=(nano)
packagesToInstall+=(neofetch)

if [ "$pm" == "pacman" ]; then
    checkNotInstalled snap
    if [ $? -eq 0 ]; then
        aurToInstall+=(snapd)
    fi
    packagesToInstall+=(git)
    packagesToInstall+=(base-devel)
else
    packagesToInstall+=(snapd)
fi

confirmWhiptail "Do you want software stores installed?"
if [ $? -eq 0 ]; then
    if [ "$de" == "gnome" ]; then
        if [ "$distro" != "pop" ]; then
            packagesToInstall+=(gnome-software)
            packagesToInstall+=(gnome-software-plugin-flatpak)
        fi
    fi
    snapsToInstall+=(snap-store)
else
    packagesToRemove+=(gnome-software)
    snapsToRemove+=(snap-store)
fi

# Gnome extensions
if [ "$distro" == "centos" ]; then
    packagesToInstall+=(gnome-shell-extension-dash-to-dock)
    packagesToInstall+=(gnome-shell-extension-system-monitor-applet)
elif [ "$distro" == "ubuntu" ]; then
    packagesToInstall+=(gnome-shell-extension-caffeine)
    packagesToInstall+=(gnome-shell-extension-system-monitor)
fi

function applicationPackages() {
    packageOptions=()
    packageOptions+=("deja-dup" "Backup Tool" off)
    packageOptions+=("gnome-books" "Gnome Books" off)
    packageOptions+=("gnome-boxes" "Gnome Boxes VM Manager" off)
    packageOptions+=("gnome-calculator" "Gnome Calculator" on)
    packageOptions+=("gnome-calendar" "Gnome Calendar" on)
    packageOptions+=("gnome-clocks" "Gnome Clocks" on)
    packageOptions+=("gnome-photos" "Gnome Photos" off)
    packageOptions+=("gnome-weather" "Gnome Weather" on)
    packageOptions+=("meld" "File Comparitor" off)
    packageOptions+=("transmission-gtk" "Transmission Torrent" off)

    choosePackagesWhiptail
    if [ $? -eq 1 ]; then
		return
	fi

    for pkg in $packageSelections; do
        pkg=$(echo $pkg | sed 's/"//g')
        case ${pkg} in
            "deja-dup")
                if [ "$distro" == "centos" ]; then
                    flatpaksToInstall+=(org.gnome.DejaDup)
                elif [ "$preferRepoOverFlatpak" == true ]; then
                    packagesToInstall+=(deja-dup)
                    flatpaksToRemove+=(org.gnome.DejaDup)
                else
                    flatpaksToInstall+=(org.gnome.DejaDup)
                    packagesToRemove+=(deja-dup)
                fi
            ;;
            "gnome-books")
                if [ "$distro" == "centos" ]; then
                    flatpaksToInstall+=(org.gnome.Books)
                elif [ "$preferRepoOverFlatpak" == true ]; then
                    packagesToInstall+=(gnome-books)
                    flatpaksToRemove+=(org.gnome.Books)
                else
                    flatpaksToInstall+=(org.gnome.Books)
                    packagesToRemove+=(gnome-books)
                fi
            ;;
            "gnome-boxes")
                if [ "$preferRepoOverFlatpak" == true ]; then
                    packagesToInstall+=(gnome-boxes)
                    flatpaksToRemove+=(org.gnome.Boxes)
                else
                    flatpaksToInstall+=(org.gnome.Boxes)
                    packagesToRemove+=(gnome-boxes)
                fi
            ;;
            "gnome-calculator")
                if [ "$preferRepoOverFlatpak" == true ]; then
                    packagesToInstall+=(gnome-calculator)
                    flatpaksToRemove+=(org.gnome.Calculator)
                else
                    flatpaksToInstall+=(org.gnome.Calculator)
                    packagesToRemove+=(gnome-calculator)
                fi
            ;;
            "gnome-calendar")
                if [ "$distro" == "centos" ]; then
                    flatpaksToInstall+=(org.gnome.Calendar)
                elif [ "$preferRepoOverFlatpak" == true ]; then
                    packagesToInstall+=(gnome-calendar)
                    flatpaksToRemove+=(org.gnome.Calendar)
                else
                    flatpaksToInstall+=(org.gnome.Calendar)
                    packagesToRemove+=(gnome-calendar)
                fi
            ;;
            "gnome-clocks")
                if [ "$distro" == "centos" ]; then
                    flatpaksToInstall+=(org.gnome.clocks)
                elif [ "$preferRepoOverFlatpak" == true ]; then
                    packagesToInstall+=(gnome-clocks)
                    flatpaksToRemove+=(org.gnome.clocks)
                else
                    flatpaksToInstall+=(org.gnome.clocks)
                    packagesToRemove+=(gnome-clocks)
                fi
            ;;
            "gnome-photos")
                if [ "$preferRepoOverFlatpak" == true ]; then
                    packagesToInstall+=(gnome-photos)
                    flatpaksToRemove+=(org.gnome.Photos)
                else
                    flatpaksToInstall+=(org.gnome.Photos)
                    packagesToRemove+=(gnome-photos)
                fi
            ;;
            "gnome-weather")
                if [ "$distro" == "centos" ]; then
                    flatpaksToInstall+=(org.gnome.Weather)
                elif [ "$preferRepoOverFlatpak" == true ]; then
                    packagesToInstall+=(gnome-weather)
                    flatpaksToRemove+=(org.gnome.Weather)
                else
                    flatpaksToInstall+=(org.gnome.Weather)
                    packagesToRemove+=(gnome-weather)
                fi
            ;;
            "meld")
                if [ "$preferRepoOverFlatpak" == true ]; then
                    packagesToInstall+=(meld)
                    flatpaksToRemove+=(org.gnome.meld)
                else
                    flatpaksToInstall+=(org.gnome.meld)
                    packagesToRemove+=(meld)
                fi
            ;;
            *)
                packagesToInstall+=($pkg)
            ;;
        esac
    done
}

function browserPackages() {
    packageOptions=()
    packageOptions+=("chromium" "Chromium Web Browser" off)
    packageOptions+=("epiphany" "Gnome Web Browser" off)
    packageOptions+=("firefox" "Firefox Broswer" on)
    packageOptions+=("torbrowser-launcher" "TOR Browser" off)

    choosePackagesWhiptail
    if [ $? -eq 1 ]; then
		return
	fi

    for pkg in $packageSelections; do
        pkg=$(echo $pkg | sed 's/"//g')
        case ${pkg} in
            "chromium")
                if [ "$distro" == "ubuntu" ]; then
                    if [ "$preferFlatpakOverSnap" == true ]; then
                        flatpaksToInstall+=(org.chromium.Chromium)
                        snapsToRemove+=(chromium)
                    else
                        snapsToInstall+=(chromium)
                        flatpaksToRemove+=(com.chromium.Chromium)
                    fi
                elif [ "$distro" == "centos" ]; then
                    if [ "$preferRepoOverSnap" == true ]; then
                        packagesToInstall+=(chromium)
                        snapsToRemove+=(chromium)
                    else
                        snapsToInstall+=(chromium)
                        packagesToRemove+=(chromium)
                    fi
                elif [ "$sourcePreference" == "snap" ]; then
                    snapsToInstall+=(chromium)

                    flatpaksToRemove+=(org.chromium.Chromium)
                    packagesToRemove+=(chromium)
                elif [ "$sourcePreference" == "flatpak" ]; then
                    flatpaksToInstall+=(org.chromium.Chromium)

                    snapsToRemove+=(chromium)
                    packagesToRemove+=(chromium)
                else
                    packagesToInstall+=(chromium)

                    flatpaksToRemove+=(org.chromium.Chromium)
                    snapsToRemove+=(chromium)
                fi
            ;;
            "epiphany")
                if [ "$distro" == "centos" ]; then
                    flatpaksToInstall+=(org.gnome.Epiphany)
                elif [ "$preferRepoOverFlatpak" == true ]; then
                    if [ "$pm" == "apt" ]; then
                        packagesToInstall+=(epiphany-browser)
                    else
                        packagesToInstall+=(epiphany)
                    fi
                    flatpaksToRemove+=(org.gnome.Epiphany)
                else
                    flatpaksToInstall+=(org.gnome.Epiphany)
                    if [ "$pm" == "dnf" ]; then
                        packagesToRemove+=(epiphany)
                    else
                        packagesToRemove+=(epiphany-browser)
                    fi
                fi
            ;;
            "firefox")
                if [ "$distro" == "debian" ]; then
                    packagesToInstall+=(firefox-esr)
                else
                    packagesToInstall+=(firefox)
                fi
            ;;
            "torbrowser-launcher")
                if [ "$distro" == "centos" ]; then
                    flatpaksToInstall+=(com.github.micahflee.torbrowser-launcher)
                elif [ "$preferRepoOverFlatpak" == true ]; then
                    packagesToInstall+=(torbrowser-launcher)
                    flatpaksToRemove+=(com.github.micahflee.torbrowser-launcher)
                else
                    flatpaksToInstall+=(com.github.micahflee.torbrowser-launcher)
                    packagesToRemove+=(torbrowser-launcher)
                fi
            ;;
            *)
                packagesToInstall+=($pkg)
            ;;
        esac
    done
}

function communicationPackages() {
    packageOptions=()
    packageOptions+=("discord" "Discord" off)
    if [ "$de" == "gnome" ]; then
        packageOptions+=("geary" "Gnome Email Client" off)
    fi
    packageOptions+=("slack" "Slack" off)
    packageOptions+=("thunderbird" "Thunderbird Email Client" off)

    choosePackagesWhiptail
    if [ $? -eq 1 ]; then
		return
	fi

    for pkg in $packageSelections; do
        pkg=$(echo $pkg | sed 's/"//g')
        case ${pkg} in
            "discord")
                if [ "$preferFlatpakOverSnap" == true ]; then
                    flatpaksToInstall+=(com.discordapp.Discord)
                    snapsToRemove+=(discord)
                else
                    snapsToInstall+=(discord)
                    flatpaksToRemove+=(com.discordapp.Discord)
                fi
            ;;
            "slack")
                if [ "$preferFlatpakOverSnap" == true ]; then
                    flatpaksToInstall+=(com.slack.Slack)
                    snapsToRemove+=(slack)
                else
                    snapsToInstall+=("slack --classic")
                    flatpaksToRemove+=(com.slack.Slack)
                fi
            ;;
            *)
                packagesToInstall+=($pkg)
            ;;
        esac
    done
}

function developmentPackages() {
    packageOptions=()
    packageOptions+=("curl" "Curl Command" on)
    if [ "$pm" != "pacman" ]; then
        packageOptions+=("git" "Git" on)
    fi
    packageOptions+=("net-tools" "Network Packages" off)
    packageOptions+=("nodejs" "NodeJS" off)
    packageOptions+=("npm" "Node Package Manager" off)
    packageOptions+=("ssh" "SSH" on)
    packageOptions+=("youtube-dl" "Command Line YT Downloader" off)

    choosePackagesWhiptail
    if [ $? -eq 1 ]; then
		return
	fi

    for pkg in $packageSelections; do
        pkg=$(echo $pkg | sed 's/"//g')
        case ${pkg} in
            "ssh")
                if [ "$pm" == "pacman" ]; then
                    packagesToInstall+=(openssh)
                else
                    packagesToInstall+=(ssh)
                fi
            ;;
            *)
                packagesToInstall+=($pkg)
            ;;
        esac
    done
}

function mediaPackages() {
    packageOptions=()
    packageOptions+=("blender" "3D Modleler and Video Editor" off)
    packageOptions+=("gimp" "GNU Image Manipulation Program" off)
    packageOptions+=("rhythmbox" "Rhythmbox Music" off)
    packageOptions+=("spotify" "Spotify" off)
    packageOptions+=("vlc" "Media Player" off)

    choosePackagesWhiptail
    if [ $? -eq 1 ]; then
		return
	fi

    for pkg in $packageSelections; do
        pkg=$(echo $pkg | sed 's/"//g')
        case ${pkg} in
            "blender")
                if [ "$distro" == "centos" ]; then
                    if [ "$preferFlatpakOverSnap" == true ]; then
                        flatpaksToInstall+=(org.blender.Blender)
                        snapsToRemove+=(blender)
                    else
                        snapsToInstall+=("blender --classic")
                        flatpaksToRemove+=(org.blender.Blender)
                    fi
                elif [ "$sourcePreference" == "snap" ]; then
                    snapsToInstall+=("blender --classic")

                    flatpaksToRemove+=(org.blender.Blender)
                    packagesToRemove+=(blender)
                elif [ "$sourcePreference" == "flatpak" ]; then
                    flatpaksToInstall+=(org.blender.Blender)

                    snapsToRemove+=(blender)
                    packagesToRemove+=(blender)
                else
                    packagesToInstall+=(blender)

                    flatpaksToRemove+=(org.blender.Blender)
                    snapsToRemove+=(blender)
                fi
            ;;
            "gimp")
                if [ "$preferRepoOverFlatpak" == true ]; then
                    packagesToInstall+=(gimp)
                    flatpaksToRemove+=(org.gimp.GIMP)
                else
                    flatpaksToInstall+=(org.gimp.GIMP)
                    packagesToRemove+=(gimp)
                fi
            ;;
            "spotify")
                snapsToInstall+=(spotify)
            ;;
            "vlc")
                if [ "$sourcePreference" == "snap" ]; then
                    snapsToInstall+=(vlc)

                    flatpaksToRemove+=(org.videolan.VLC)
                    packagesToRemove+=(vlc)
                elif [ "$sourcePreference" == "flatpak" ]; then
                    flatpaksToInstall+=(org.videolan.VLC)

                    snapsToRemove+=(vlc)
                    packagesToRemove+=(vlc)
                else
                    packagesToInstall+=(vlc)

                    flatpaksToRemove+=(org.videolan.VLC)
                    snapsToRemove+=(vlc)
                fi
            ;;
            *)
                packagesToInstall+=($pkg)
            ;;
        esac
    done
}

function gamingPackages() {
    packageOptions=()
    packageOptions+=("0ad" "0 A.D. Ancient Warfare" off)
    packageOptions+=("parsec" "Streaming App" off)
    packageOptions+=("steam" "Steam" off)
    packageOptions+=("supertuxkart" "Tux Kart Racer" off)
    packageOptions+=("xonotic" "Xonotic FPS" off)

    choosePackagesWhiptail
    if [ $? -eq 1 ]; then
		return
	fi

    for pkg in $packageSelections; do
        pkg=$(echo $pkg | sed 's/"//g')
        case ${pkg} in
            "0ad")
                if [ "$distro" == "centos" ]; then
                    if [ "$preferFlatpakOverSnap" == true ]; then
                        flatpaksToInstall+=(com.play0ad.zeroad)
                        snapsToRemove+=(0ad)
                    else
                        snapsToInstall+=(0ad)
                        flatpaksToRemove+=(com.play0ad.zeroad)
                    fi
                elif [ "$sourcePreference" == "snap" ]; then
                    snapsToInstall+=(0ad)

                    flatpaksToRemove+=(com.play0ad.zeroad)
                    packagesToRemove+=(0ad)
                elif [ "$sourcePreference" == "flatpak" ]; then
                    flatpaksToInstall+=(com.play0ad.zeroad)

                    snapsToRemove+=(0ad)
                    packagesToRemove+=(0ad)
                else
                    packagesToInstall+=(0ad)

                    flatpaksToRemove+=(com.play0ad.zeroad)
                    snapsToRemove+=(0ad)
                fi
            ;;
            "parsec")
                flatpaksToInstall+=(com.parsecgaming.parsec)
            ;;
            "steam")
                if [ "$distro" == "ubuntu" ]; then
                    if [ "$preferRepoOverFlatpak" == true ]; then
                        packagesToInstall+=(steam)
                        flatpaksToRemove+=(com.valvesoftware.Steam)
                    else
                        flatpaksToInstall+=(com.valvesoftware.Steam)
                        packagesToRemove+=(steam)
                    fi
                else
                    packagesToInstall+=(com.valvesoftware.Steam)
                fi
            ;;
            "supertuxkart")
                if [ "$distro" == "centos" ]; then
                    if [ "$preferFlatpakOverSnap" == true ]; then
                        flatpaksToInstall+=(net.supertuxkart.SuperTuxKart)
                        snapsToRemove+=(supertuxkart)
                    else
                        snapsToInstall+=(supertuxkart)
                        flatpaksToRemove+=(net.supertuxkart.SuperTuxKart)
                    fi
                elif [ "$sourcePreference" == "snap" ]; then
                    snapsToInstall+=(supertuxkart)

                    flatpaksToRemove+=(net.supertuxkart.SuperTuxKart)
                    packagesToRemove+=(supertuxkart)
                elif [ "$sourcePreference" == "flatpak" ]; then
                    flatpaksToInstall+=(net.supertuxkart.SuperTuxKart)

                    snapsToRemove+=(supertuxkart)
                    packagesToRemove+=(supertuxkart)
                else
                    packagesToInstall+=(supertuxkart)

                    flatpaksToRemove+=(net.supertuxkart.SuperTuxKart)
                    snapsToRemove+=(supertuxkart)
                fi
            ;;
            "xonotic")
                if [ "$preferFlatpakOverSnap" == true ]; then
                    flatpaksToInstall+=(org.xonotic.Xonotic)
                    snapsToRemove+=(xonotic)
                else
                    snapsToInstall+=(xonotic)
                    flatpaksToRemove+=(org.xonotic.Xonotic)
                fi
            ;;
            *)
                packagesToInstall+=($pkg)
            ;;
        esac
    done
}

function textPackages() {
    packageOptions=()
    packageOptions+=("code" "Visual Studio Code" off)
    packageOptions+=("gedit" "GUI Text Editor" on)
    packageOptions+=("libreoffice" "LibreOffice Suite" off)
    packageOptions+=("texworks" "LaTeX Editor" off)

    choosePackagesWhiptail
    if [ $? -eq 1 ]; then
		return
	fi

    for pkg in $packageSelections; do
        pkg=$(echo $pkg | sed 's/"//g')
        case ${pkg} in
            "code")
                if [ "$pm" == "apt" ]; then
                    snapsToInstall+=("code --classic")
                elif [ "$pm" == "dnf" ]; then
                    if [ "$preferRepoOverSnap" == true ]; then
                        sudo rpm --import https://packages.microsoft.com/keys/microsoft.asc
                        sudo sh -c 'echo -e "[code]\nname=Visual Studio Code\nbaseurl=https://packages.microsoft.com/yumrepos/vscode\nenabled=1\ngpgcheck=1\ngpgkey=https://packages.microsoft.com/keys/microsoft.asc" > /etc/yum.repos.d/vscode.repo'
                        sudo dnf check-update

                        packagesToInstall+=(code)
                        snapsToRemove+=(code)
                    else
                        sudo rm -rf /etc/yum.repos.d/vscode.repo

                        snapsToInstall+=("code --classic")
                        packagesToRemove+=(code)
                    fi
                fi
            ;;
            "libreoffice")
                if [ "$sourcePreference" == "snap" ]; then
                    snapsToInstall+=(libreoffice)

                    flatpaksToRemove+=(org.libreoffice.LibreOffice)
                    packagesToRemove+=(libreoffice*)
                elif [ "$sourcePreference" == "flatpak" ]; then
                    flatpaksToInstall+=(org.libreoffice.LibreOffice)

                    snapsToRemove+=(libreoffice)
                    packagesToRemove+=(libreoffice*)
                else
                    if [ "$pm" == "pacman" ]; then
                        packagesToInstall+=(libreoffice-fresh)
                    else
                        packagesToInstall+=(libreoffice-writer)
                        packagesToInstall+=(libreoffice-calc)
                        packagesToInstall+=(libreoffice-impress)
                    fi

                    flatpaksToRemove+=(org.libreoffice.LibreOffice)
                    snapsToRemove+=(libreoffice)
                fi
            ;;
            "texworks")
                flatpaksToInstall+=(org.tug.texworks)
            ;;
            *)
                packagesToInstall+=($pkg)
            ;;
        esac
    done
}

function utilityPackages() {
    packageOptions=()
    packageOptions+=("baobab" "Disk Usage" on)
    if [ "$de" == "gnome" ]; then
        packageOptions+=("dconf-editor" "dconf Editor" off)
    fi
    packageOptions+=("exfat" "ExFat Format Support" off)
    packageOptions+=("ffmpeg" "ffmpeg to watch videos" on)
    packageOptions+=("glances" "Monitoring Tool" off)
    if [ "$de" == "gnome" ]; then
        packageOptions+=("gnome-system-monitor" "System Monitor" on)
        packageOptions+=("gnome-tweaks" "Gnome Tweaks" on)
    fi
    packageOptions+=("htop" "Process Reviewer" off)
    packageOptions+=("imagemagick" "Image Magick" on)
    packageOptions+=("simple-scan" "Scanner Application" off)
    packageOptions+=("timeshift" "Backup Tool" off)
    packageOptions+=("virtualbox" "Virtual Box VM Manager" off)

    choosePackagesWhiptail
    if [ $? -eq 1 ]; then
		return
	fi

    for pkg in $packageSelections; do
        pkg=$(echo $pkg | sed 's/"//g')
        case ${pkg} in
            "exfat")
                packagesToInstall+=(exfat-utils)
                if [ "$pm" == "apt" ]; then
                    packagesToInstall+=(exfat-fuse)
                elif [ "$pm" == "dnf" ]; then
                    packagesToInstall+=(fuse-exfat)
                fi
            ;;
            "ffmpeg")
                if [ "$distro" == "centos" ]; then
                    packagesToInstall+=(http://rpmfind.net/linux/epel/7/x86_64/Packages/s/SDL2-2.0.10-1.el7.x86_64.rpm)
                    packagesToInstall+=(ffmpeg)
                    packagesToInstall+=(ffmpeg-devel)
                else
                    packagesToInstall+=(ffmpeg)
                fi
            ;;
            "imagemagick")
                if [ "$pm" == "dnf" ]; then
                    packagesToInstall+=(ImageMagick)
                elif [ "$pm" == "apt" ]; then
                    packagesToInstall+=(imagemagick)
                fi
            ;;
            "timeshift")
                if [ "$pm" == "pacman" ]; then
                    checkNotInstalled timeshift
                    if [ $? -eq 0 ]; then
                        aurToInstall+=(timeshift)
                    fi
                else
                    packagesToInstall+=(timeshift)
                fi
            ;;
            "virtualbox")
                if [ "$pm" == "dnf" ]; then
                    packagesToInstall+=(VirtualBox)
                elif [ "$pm" == "apt" ]; then
                    packagesToInstall+=(virtualbox)
                fi
            ;;
            *)
                packagesToInstall+=($pkg)
            ;;
        esac
    done
}

function chooseUsage() {
    categoryOptions=()
    categoryOptions+=("Applications" "Various Desktop Applications")
    categoryOptions+=("Browsers" "Web Browsers")
    categoryOptions+=("Communication" "Communication Applications")
    categoryOptions+=("Development" "Development Packages")
    categoryOptions+=("Media" "Multi Media Applications")
    categoryOptions+=("Gaming" "Gaming Applications")
    categoryOptions+=("Text" "Text Applications")
    categoryOptions+=("Utility" "Utility Applications and Packages")
    categoryOptions+=("" "")
    categoryOptions+=("Install" "")

    # Remove duplicate values in install arrays
    packagesToInstall=($(echo "${packagesToInstall[@]}" | tr ' ' '\n' | sort -u | tr '\n' ' '))

    chooseCategoryWhiptail
    if [ $? -eq 1 ]; then
		return 1
	fi

    case ${categorySelection} in
        "Applications")
            applicationPackages
            defaultCategory="Browsers"
        ;;
        "Browsers")
            browserPackages
            defaultCategory="Communication"
        ;;
        "Communication")
            communicationPackages
            defaultCategory="Development"
        ;;
        "Development")
            developmentPackages
            defaultCategory="Media"
        ;;
        "Media")
            mediaPackages
            defaultCategory="Gaming"
        ;;
        "Gaming")
            gamingPackages
            defaultCategory="Text"
        ;;
        "Text")
            textPackages
            defaultCategory="Utility"
        ;;
        "Utility")
            utilityPackages
            defaultCategory="Install"
        ;;
        "Install")
            return
        ;;
    esac
    chooseUsage
}

chooseUsage
if [ $? -eq 1 ]; then
    exit 0
fi

################################################################################

# Install Packages

# Package manager

if [ ${#packagesToInstall[@]} -gt 0 ]; then
    if [ "$bulkInstallPackages" == true ]; then
        packageManager install ${packagesToInstall[*]}
    else
        for i in "${packagesToInstall[@]}"; do
            packageManager install $i
        done
    fi
fi

# AUR
if [ ${#aurToInstall[@]} -gt 0 ]; then
    for i in "${aurToInstall[@]}"; do
        aurManager $i
    done
fi

# Flatpaks

sudo flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo

if [ ${#flatpaksToInstall[@]} -gt 0 ]; then
    for i in "${flatpaksToInstall[@]}"; do
        flatpakManager install $i
    done
fi

# Snaps

if [ "$pm" == "dnf" ] || [ "$pm" == "pacman" ]; then
    sudo systemctl enable --now snapd.socket
    sudo ln -s /var/lib/snapd/snap /snap
fi

if [ ${#snapsToInstall[@]} -gt 0 ]; then
    for i in "${snapsToInstall[@]}"; do
        snapManager install $i
    done
fi

################################################################################

# Determine Packages to Remove

packagesToRemove+=(cheese)
packagesToRemove+=(evolution)
packagesToRemove+=(mpv)
packagesToRemove+=(totem)

if [ "$distro" == "mint" ] || [ "$distro" == "lmde" ]; then
    packagesToRemove+=(celluloid)
    packagesToRemove+=(drawing)
    packagesToRemove+=(hexchat*)
    packagesToRemove+=(mintbackup)
    packagesToRemove+=(pix*)
    packagesToRemove+=(warpinator)
    packagesToRemove+=(xed)
elif [ "$distro" == "ubuntu" ] || [ "$distro" == "debian" ]; then
    packagesToRemove+=(aisleriot)
    packagesToRemove+=(five-or-more)
    packagesToRemove+=(four-in-a-row)
    packagesToRemove+=(gnome-chess)
    packagesToRemove+=(gnome-klotski)
    packagesToRemove+=(gnome-mahjongg)
    packagesToRemove+=(gnome-mines)
    packagesToRemove+=(gnome-music)
    packagesToRemove+=(gnome-nibbles)
    packagesToRemove+=(gnome-robots)
    packagesToRemove+=(gnome-sudoku)
    packagesToRemove+=(gnome-tetravex)
    packagesToRemove+=(gnome-todo)
    packagesToRemove+=(remmina*)
    packagesToRemove+=(seahorse)
    packagesToRemove+=(shotwell*)

    if [ "$distro" == "debian" ]; then
        packagesToRemove+=(anthy*)
        packagesToRemove+=(fcitx*)
        packagesToRemove+=(goldendict)
        packagesToRemove+=(hitori)
        packagesToRemove+=(tali)
        packagesToRemove+=(quadrapassel)
        packagesToRemove+=(xterm)
    fi
elif [ "$distro" == "pop" ]; then
    packagesToRemove+=(popsicle)
elif [ "$distro" == "centos" ]; then
    packagesToRemove+=(pidgin)
fi

if [ "$de" == "gnome" ]; then
    packagesToRemove+=(gnome-contacts)
    packagesToRemove+=(gnome-maps)
    packagesToRemove+=(gnome-software-plugin-snap)
fi

################################################################################

# Remove Packages

# Package manager

if [ ${#packagesToRemove[@]} -gt 0 ]; then
    if [ "$bulkInstallPackages" == true ]; then
        packageManager remove ${packagesToRemove[*]}
    else
        for i in "${packagesToRemove[@]}"; do
            packageManager remove $i
        done
    fi
fi

packageManager autoremove

# Flatpaks

if [ ${#flatpaksToRemove[@]} -gt 0 ]; then
    for i in "${flatpaksToRemove[@]}"; do
        flatpakManager remove $i
    done
fi

sudo flatpak remove --unused

# Snaps

if [ ${#snapsToRemove[@]} -gt 0 ]; then
    for i in "${snapsToRemove[@]}"; do
        snapManager remove $i
    done
fi

LANG=en_US.UTF-8 snap list --all | awk '/disabled/{print $1, $3}' |
    while read snapname revision; do
        sudo snap remove "$snapname" --revision="$revision"
    done

################################################################################

neofetch

exit 0
