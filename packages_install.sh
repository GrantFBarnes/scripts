#!/bin/bash
# Purpose: Install/Remove basic packages for GNU/Linux Desktop
################################################################################
cd $(dirname "$0")
folderLocation=$(pwd)
. helper_functions.sh

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
    if ! command -v $1 &>/dev/null; then
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
            sudo $pm -Rsun ${@:2} --noconfirm
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
    cd /home/$SUDO_USER/aur
    sudo -u $SUDO_USER git clone https://aur.archlinux.org/$1.git
    cd $1
    sudo -u $SUDO_USER makepkg -si --noconfirm
    cd $folderLocation
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

if [ -z "$SUDO_USER" ]; then
    echo "Must be run with sudo"
    exit 1
fi

clear

# Determine distrobution

distro=$(getDistrobution)
pm=$(getPackageManager)

if [ "$distro" == "" ]; then
    echo "---------------------------------------------------------------------"
    echo "Distrobution not recognized"
    echo "---------------------------------------------------------------------"
    exit 1
fi

################################################################################

# Update and set up repos

update

if [ "$pm" == "dnf" ]; then
    packageManager install newt

    grep -q max_parallel_downloads /etc/dnf/dnf.conf
    if [ $? -eq 1 ]; then
        sudo sh -c 'echo max_parallel_downloads=10 >> /etc/dnf/dnf.conf'
        sudo sh -c 'echo fastestmirror=true >> /etc/dnf/dnf.conf'
        update
    fi

    confirmWhiptail "Enable EPEL Repositories?"
    if [ $? -eq 0 ]; then
        if [ "$distro" == "fedora" ]; then
            sudo dnf install https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm -y
        elif [ "$distro" == "centos" ]; then
            sudo dnf install --nogpgcheck https://dl.fedoraproject.org/pub/epel/epel-release-latest-8.noarch.rpm -y
            sudo dnf install --nogpgcheck https://download1.rpmfusion.org/free/el/rpmfusion-free-release-8.noarch.rpm -y
        fi
        confirmWhiptail "Enable Non-Free EPEL Repositories?"
        if [ $? -eq 0 ]; then
            if [ "$distro" == "fedora" ]; then
                sudo dnf install https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm -y
            elif [ "$distro" == "centos" ]; then
                sudo dnf install --nogpgcheck https://download1.rpmfusion.org/nonfree/el/rpmfusion-nonfree-release-8.noarch.rpm -y
            fi
        fi
        update
    fi
elif [ "$distro" == "mint" ]; then
    nosnap=/etc/apt/preferences.d/nosnap.pref
    if [ -f "$nosnap" ]; then
        sudo rm $nosnap
        update
    fi
elif [ "$pm" == "pacman" ]; then
    sudo -u $SUDO_USER mkdir /home/$SUDO_USER/aur
fi

bashrc=/home/$SUDO_USER/.bashrc
if [ ! -f "$bashrc" ]; then
    sudo -u $SUDO_USER touch $bashrc
fi

grep -q EDITOR $bashrc
if [ $? -eq 1 ]; then
    sudo -u $SUDO_USER echo export EDITOR='"/usr/bin/vim"' >>$bashrc
fi

vimrc=/home/$SUDO_USER/.vimrc
if [ ! -f "$vimrc" ]; then
    sudo -u $SUDO_USER touch $vimrc
fi

grep -q "set number relativenumber" $vimrc
if [ $? -eq 1 ]; then
    sudo -u $SUDO_USER echo set number relativenumber >>$vimrc
fi

grep -q "syntax on" $vimrc
if [ $? -eq 1 ]; then
    sudo -u $SUDO_USER echo syntax on >>$vimrc
fi

confirmWhiptail "   Distrobution: $distro\nPackage Manager: $pm\n\nWould you like to continue?" 11
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
packagesToInstall+=(vim)

if [ "$pm" == "pacman" ]; then
    packagesToInstall+=(git)
    packagesToInstall+=(base-devel)
fi

function applicationPackages() {
    packageOptions=()
    packageOptions+=("cheese" "Webcam Application" off)
    packageOptions+=("deja-dup" "Backup Tool" off)
    packageOptions+=("calibre" "E Book Reader/Editor" off)
    packageOptions+=("foliate" "E Book Reader" off)
    packageOptions+=("eog" "Eye of Gnome" off)
    packageOptions+=("evince" "Gnome Document Viewer" off)
    packageOptions+=("gnome-books" "Gnome Books" off)
    packageOptions+=("gnome-boxes" "Gnome Boxes VM Manager" off)
    packageOptions+=("gnome-calculator" "Gnome Calculator" off)
    packageOptions+=("gnome-calendar" "Gnome Calendar" off)
    packageOptions+=("gnome-clocks" "Gnome Clocks" off)
    packageOptions+=("gnome-photos" "Gnome Photos" off)
    if [ "$distro" != "pop" ]; then
        packageOptions+=("gnome-software" "Gnome Software" off)
    fi
    packageOptions+=("gnome-weather" "Gnome Weather" off)
    packageOptions+=("gnucash" "Finance Program" off)
    packageOptions+=("gramps" "Genealogical Research and Analysis Management Programming System" off)
    packageOptions+=("snap-store" "Snap Store" off)
    packageOptions+=("transmission-gtk" "Transmission Torrent" off)

    choosePackagesWhiptail
    if [ $? -eq 1 ]; then
        return
    fi

    for pkg in $packageSelections; do
        pkg=$(echo $pkg | sed 's/"//g')
        case ${pkg} in
        "cheese")
            if [ "$distro" == "centos" ]; then
                flatpaksToInstall+=(org.gnome.Cheese)
            elif [ "$preferRepoOverFlatpak" == true ]; then
                packagesToInstall+=(cheese)
                flatpaksToRemove+=(org.gnome.Cheese)
            else
                flatpaksToInstall+=(org.gnome.Cheese)
                packagesToRemove+=(cheese)
            fi
            ;;
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
        "foliate")
            if [ "$preferFlatpakOverSnap" == true ]; then
                flatpaksToInstall+=(com.github.johnfactotum.Foliate)
                snapsToRemove+=(foliate)
            else
                snapsToInstall+=(foliate)
                flatpaksToRemove+=(com.github.johnfactotum.Foliate)
            fi
            ;;
        "eog")
            if [ "$preferRepoOverFlatpak" == true ]; then
                packagesToInstall+=(eog)
                flatpaksToRemove+=(org.gnome.eog)
            else
                flatpaksToInstall+=(org.gnome.eog)
                packagesToRemove+=(eog)
            fi
            ;;
        "evince")
            if [ "$preferRepoOverFlatpak" == true ]; then
                packagesToInstall+=(evince)
                flatpaksToRemove+=(org.gnome.Evince)
            else
                flatpaksToInstall+=(org.gnome.Evince)
                packagesToRemove+=(evince)
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
        "gnucash")
            if [ "$preferRepoOverFlatpak" == true ]; then
                packagesToInstall+=(gnucash)
                flatpaksToRemove+=(org.gnucash.GnuCash)
            else
                flatpaksToInstall+=(org.gnucash.GnuCash)
                packagesToRemove+=(gnucash)
            fi
            ;;
        "gramps")
            if [ "$preferRepoOverFlatpak" == true ]; then
                packagesToInstall+=(gramps)
                flatpaksToRemove+=(org.gramps_project.Gramps)
            else
                flatpaksToInstall+=(org.gramps_project.Gramps)
                packagesToRemove+=(gramps)
            fi
            ;;
        "snap-store")
            snapsToInstall+=(snap-store)
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
    if [ "$distro" == "fedora" ]; then
        packageOptions+=("icecat" "GNU IceCat Broswer" off)
    fi
    packageOptions+=("firefox" "Firefox Broswer" off)
    if [ "$distro" == "centos" ] || [ "$distro" == "debian" ]; then
        packageOptions+=("firefox-esr" "Firefox ESR Broswer" off)
    fi
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
            if [ "$distro" == "centos" ] || [ "$distro" == "debian" ]; then
                flatpaksToInstall+=(org.mozilla.firefox)
            else
                packagesToInstall+=(firefox)
            fi
            ;;
        "firefox-esr")
            if [ "$distro" == "centos" ]; then
                packagesToInstall+=(firefox)
            elif [ "$distro" == "debian" ]; then
                packagesToInstall+=(firefox-esr)
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
    packageOptions+=("geary" "Gnome Email Client" off)
    packageOptions+=("skype" "Skype" off)
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
        "skype")
            if [ "$preferFlatpakOverSnap" == true ]; then
                flatpaksToInstall+=(com.skype.Client)
                snapsToRemove+=(skype)
            else
                snapsToInstall+=("skype --classic")
                flatpaksToRemove+=(com.skype.Client)
            fi
            ;;
        "thunderbird")
            if [ "$preferRepoOverFlatpak" == true ]; then
                packagesToInstall+=(thunderbird)
                flatpaksToRemove+=(org.mozilla.Thunderbird)
            else
                flatpaksToInstall+=(org.mozilla.Thunderbird)
                packagesToRemove+=(thunderbird)
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
    packageOptions+=("id3v2" "Modify MP3 Meta Data" off)
    if [ "$pm" != "pacman" ]; then
        packageOptions+=("git" "Git" on)
    fi
    packageOptions+=("mysql-server" "MySQL Server" off)
    packageOptions+=("nano" "nano" on)
    packageOptions+=("net-tools" "Network Packages" off)
    packageOptions+=("node" "Node.js and NPM" off)
    packageOptions+=("python3-pip" "Python PIP" off)
    packageOptions+=("ssh" "SSH" on)
    packageOptions+=("youtube-dl" "Command Line YT Downloader" off)

    choosePackagesWhiptail
    if [ $? -eq 1 ]; then
        return
    fi

    for pkg in $packageSelections; do
        pkg=$(echo $pkg | sed 's/"//g')
        case ${pkg} in
        "node")
            packagesToInstall+=(nodejs)
            packagesToInstall+=(npm)
            grep -q NODE_OPTIONS $bashrc
            if [ $? -eq 1 ]; then
                sudo -u $SUDO_USER echo export NODE_OPTIONS='--max_old_space_size=8192' >>$bashrc
            fi
            ;;
        "mysql-server")
            if [ "$distro" == "fedora" ]; then
                packagesToInstall+=(mariadb-server)
            else
                packagesToInstall+=($pkg)
            fi
            ;;
        "ssh")
            if [ "$pm" == "apt" ]; then
                packagesToInstall+=(ssh)
            else
                packagesToInstall+=(libssh)
                packagesToInstall+=(openssh)
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
    packageOptions+=("kdenlive" "KDE Video Editor" off)
    packageOptions+=("rhythmbox" "Rhythmbox Music" off)
    packageOptions+=("spotify" "Spotify" off)
    packageOptions+=("totem" "Gnome Video" off)
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
        "kdenlive")
            flatpaksToInstall+=(org.kde.kdenlive)
            ;;
        "rhythmbox")
            if [ "$preferRepoOverFlatpak" == true ]; then
                packagesToInstall+=(rhythmbox)
                flatpaksToRemove+=(org.gnome.Rhythmbox3)
            else
                flatpaksToInstall+=(org.gnome.Rhythmbox3)
                packagesToRemove+=(rhythmbox)
            fi
            ;;
        "spotify")
            snapsToInstall+=(spotify)
            ;;
        "totem")
            if [ "$preferRepoOverFlatpak" == true ]; then
                packagesToInstall+=(totem)
                flatpaksToRemove+=(org.gnome.Totem)
            else
                flatpaksToInstall+=(org.gnome.Totem)
                packagesToRemove+=(totem)
            fi
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
    packageOptions+=("aisleriot" "Solitare" off)
    packageOptions+=("gnome-chess" "Chess" off)
    packageOptions+=("gnome-sudoku" "Sudoku" off)
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
        "aisleriot")
            if [ "$preferRepoOverFlatpak" == true ]; then
                packagesToInstall+=(aisleriot)
                flatpaksToRemove+=(org.gnome.Aisleriot)
            else
                flatpaksToInstall+=(org.gnome.Aisleriot)
                packagesToRemove+=(aisleriot)
            fi
            ;;
        "gnome-chess")
            if [ "$preferRepoOverFlatpak" == true ]; then
                packagesToInstall+=(gnome-chess)
                flatpaksToRemove+=(org.gnome.Chess)
            else
                flatpaksToInstall+=(org.gnome.Chess)
                packagesToRemove+=(gnome-chess)
            fi
            ;;
        "gnome-sudoku")
            if [ "$preferRepoOverFlatpak" == true ]; then
                packagesToInstall+=(gnome-sudoku)
                flatpaksToRemove+=(org.gnome.Sudoku)
            else
                flatpaksToInstall+=(org.gnome.Sudoku)
                packagesToRemove+=(gnome-sudoku)
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
    packageOptions+=("codium" "Visual Studio Codium" off)
    packageOptions+=("gedit" "GUI Text Editor" off)
    packageOptions+=("libreoffice" "LibreOffice Suite" off)
    packageOptions+=("pycharm" "PyCharm Python Editor" off)
    packageOptions+=("texstudio" "LaTeX Editor" off)

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
        "codium")
            flatpaksToInstall+=(com.vscodium.codium)
            grep -q codium $bashrc
            if [ $? -eq 1 ]; then
                sudo -u $SUDO_USER echo alias codium='"flatpak run com.vscodium.codium"' >>$bashrc
            fi
            ;;
        "gedit")
            if [ "$preferRepoOverFlatpak" == true ]; then
                packagesToInstall+=(gedit)
                flatpaksToRemove+=(org.gnome.gedit)
            else
                flatpaksToInstall+=(org.gnome.gedit)
                packagesToRemove+=(gedit)
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
                    packagesToInstall+=(libreoffice-base)
                fi

                flatpaksToRemove+=(org.libreoffice.LibreOffice)
                snapsToRemove+=(libreoffice)
            fi
            ;;
        "texstudio")
            flatpaksToInstall+=(org.texstudio.TeXstudio)
            ;;
        "pycharm")
            if [ "$preferFlatpakOverSnap" == true ]; then
                flatpaksToInstall+=(com.jetbrains.PyCharm-Community)
                grep -q pycharm $bashrc
                if [ $? -eq 1 ]; then
                    sudo -u $SUDO_USER echo alias pycharm='"flatpak run com.jetbrains.PyCharm-Community"' >>$bashrc
                fi

                snapsToRemove+=(pycharm-community)
            else
                snapsToInstall+=("pycharm-community --classic")
                flatpaksToRemove+=(com.jetbrains.PyCharm-Community)
            fi
            ;;
        *)
            packagesToInstall+=($pkg)
            ;;
        esac
    done
}

function utilityPackages() {
    packageOptions=()
    packageOptions+=("baobab" "Disk Usage" off)
    packageOptions+=("dconf-editor" "dconf Editor" off)
    packageOptions+=("exfat" "ExFat Format Support" off)
    packageOptions+=("ffmpeg" "ffmpeg to watch videos" off)
    packageOptions+=("gnome-disk-utility" "Disk Utility" off)
    packageOptions+=("gnome-system-monitor" "System Monitor" off)
    packageOptions+=("gnome-tweaks" "Gnome Tweaks" off)
    packageOptions+=("htop" "Process Reviewer" off)
    packageOptions+=("ibus-unikey" "Vietnamese Unikey" off)
    packageOptions+=("imagemagick" "Image Magick" off)
    packageOptions+=("neofetch" "neofetch overview display" off)
    packageOptions+=("ncdu" "Command Line Disk Usage" off)
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
                packagesToInstall+=(http://rpmfind.net/linux/epel/7/x86_64/Packages/s/SDL2-2.0.14-2.el7.x86_64.rpm)
                packagesToInstall+=(ffmpeg)
                packagesToInstall+=(ffmpeg-devel)
            else
                packagesToInstall+=(ffmpeg)
            fi
            ;;
        "ibus-unikey")
            if [ "$distro" == "centos" ]; then
                packagesToInstall+=(http://rpmfind.net/linux/fedora/linux/releases/34/Everything/x86_64/os/Packages/i/ibus-unikey-0.6.1-26.20190311git46b5b9e.fc34.x86_64.rpm)
            else
                packagesToInstall+=($pkg)
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

# Decide if flatpak or snapd need to be installed

if [ ${#flatpaksToInstall[@]} -gt 0 ]; then
    packagesToInstall+=(flatpak)
fi

if [ ${#snapsToInstall[@]} -gt 0 ]; then
    if [ "$pm" == "pacman" ]; then
        checkNotInstalled snap
        if [ $? -eq 0 ]; then
            aurToInstall+=(snapd)
        fi
    else
        packagesToInstall+=(snapd)
    fi
fi

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

if [ ${#flatpaksToInstall[@]} -gt 0 ]; then
    sudo flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo

    for i in "${flatpaksToInstall[@]}"; do
        flatpakManager install $i
    done
fi

# Snaps

if [ ${#snapsToInstall[@]} -gt 0 ]; then
    if [ "$pm" == "dnf" ] || [ "$pm" == "pacman" ]; then
        sudo systemctl enable --now snapd.socket
        sudo ln -s /var/lib/snapd/snap /snap
    fi

    for i in "${snapsToInstall[@]}"; do
        snapManager install $i
    done
fi

################################################################################

# Determine Packages to Remove

packagesToRemove+=(evolution)
packagesToRemove+=(mpv)

if [ "$distro" == "mint" ] || [ "$distro" == "lmde" ]; then
    packagesToRemove+=(celluloid)
    packagesToRemove+=(drawing)
    packagesToRemove+=(hexchat*)
    packagesToRemove+=(mintbackup)
    packagesToRemove+=(pix*)
    packagesToRemove+=(xed)
elif [ "$distro" == "ubuntu" ] || [ "$distro" == "debian" ]; then
    packagesToRemove+=(five-or-more)
    packagesToRemove+=(four-in-a-row)
    packagesToRemove+=(gnome-klotski)
    packagesToRemove+=(gnome-mahjongg)
    packagesToRemove+=(gnome-music)
    packagesToRemove+=(gnome-nibbles)
    packagesToRemove+=(gnome-robots)
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
elif [ "$distro" == "fedora" ]; then
    packagesToRemove+=(gnome-tour)
elif [ "$distro" == "centos" ]; then
    packagesToRemove+=(pidgin)
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

echo "---------------------------------------------------------------------"
echo "Finished"
echo "---------------------------------------------------------------------"

exit 0
