#!/bin/bash
# Purpose: Setup fresh install of Linux Desktop (Fedora/Mint/Ubuntu)
################################################################################

function confirm() {
    read -p "$1 (y/N) " ans
    if [ "$ans" == "y" ]
    then
        echo "confirmed"
    fi
    echo ""
}

function check_exit_status() {
    if [ $? -eq 0 ]
    then
        echo "Success"
    else
        echo "[ERROR] Process Failed!"
        if [ $(confirm "Exit script?") ]
        then
            exit 1
        fi
    fi
}

function update() {
    echo "---------------------------------------------------------------------"
    echo "Update"
    echo "---------------------------------------------------------------------"
    if [ "$pm" == "dnf" ]; then
        sudo dnf upgrade -y
    elif [ "$pm" == "apt" ]; then
        sudo apt update && sudo apt full-upgrade -y
    fi
    check_exit_status
}

function package_manager() {
    local method=$1
    local package=$2
    echo "---------------------------------------------------------------------"
    echo "sudo $pm $method $package -y"
    echo "---------------------------------------------------------------------"
    if [ "$method" == "remove" ] && [ "$pm" == "apt" ]; then
        sudo apt-get remove --purge $package -y
    else
        sudo $pm $method $package -y
    fi
    check_exit_status
}

function snap_manager() {
    local method=$1
    local package=$2
    local param=$3
    echo "---------------------------------------------------------------------"
    echo "sudo snap $method $package $param"
    echo "---------------------------------------------------------------------"
    sudo snap $method $package $param
    check_exit_status
}

function flatpak_manager() {
    local method=$1
    local location=$2
    local package=$3
    echo "---------------------------------------------------------------------"
    echo "sudo flatpak $method $location $package -y"
    echo "---------------------------------------------------------------------"
    sudo flatpak $method $location $package -y
    check_exit_status
}

function add_rpm_fusion() {
    echo "---------------------------------------------------------------------"
    echo "Add RPM Fusion Repos"
    echo "---------------------------------------------------------------------"
    sudo dnf install https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm -y
    check_exit_status
}

################################################################################

clear

# Determine distrobution

distro=""
pm=""
de=""
grep -q Fedora /etc/os-release
if [ $? -eq 0 ]; then
    distro="fedora"
    pm="dnf"
    de="gnome"
    echo "---------------------------------------------------------------------"
    echo "Distrobution Found: Fedora"
    echo "---------------------------------------------------------------------"
fi

grep -q LMDE /etc/os-release
if [ $? -eq 0 ]; then
    distro="lmde"
    pm="apt"
    de="cinnamon"
    echo "---------------------------------------------------------------------"
    echo "Distrobution Found: LMDE"
    echo "---------------------------------------------------------------------"
fi

grep -q Mint /etc/os-release
if [ $? -eq 0 ]; then
    distro="mint"
    pm="apt"
    de="cinnamon"
    echo "---------------------------------------------------------------------"
    echo "Distrobution Found: Linux Mint"
    echo "---------------------------------------------------------------------"
fi

grep -q Pop!_OS /etc/os-release
if [ $? -eq 0 ]; then
    distro="pop"
    pm="apt"
    de="gnome"
    echo "---------------------------------------------------------------------"
    echo "Distrobution Found: Pop OS"
    echo "---------------------------------------------------------------------"
fi

grep -q Ubuntu /etc/os-release
if [ $? -eq 0 ]; then
    distro="ubuntu"
    pm="apt"
    de="gnome"
    echo "---------------------------------------------------------------------"
    echo "Distrobution Found: Ubuntu"
    echo "---------------------------------------------------------------------"
fi

if [ -z "$distro" ]; then
    echo "---------------------------------------------------------------------"
    echo "Distrobution not recognized"
    echo "---------------------------------------------------------------------"
    exit 1
fi

################################################################################

# Update and set up repos

update

if [ "$distro" == "fedora" ]; then
    add_rpm_fusion
    update
elif [ "$distro" == "mint" ]; then
    sudo rm /etc/apt/preferences.d/nosnap.pref
    update
fi

################################################################################

# Install Packages

package_manager install baobab
package_manager install exfat-utils
package_manager install firefox
package_manager install gedit
package_manager install gnome-system-monitor
package_manager install gnome-terminal
package_manager install gnome-tweaks
package_manager install nano
package_manager install neofetch

if [ "$pm" == "apt" ]; then
    package_manager install exfat-fuse
elif [ "$pm" == "dnf" ]; then
    package_manager install fuse-exfat
fi

# Install Flatpak

package_manager install flatpak
sudo flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo

# Install Snap

package_manager install snapd
if [ "$distro" == "fedora" ]; then
    sudo ln -s /var/lib/snapd/snap /snap
fi
snap_manager install hello-world

# Install based on purpose

if [ $(confirm "Used for development?") ]; then
    package_manager install git
    package_manager install meld
    package_manager install net-tools
    package_manager install nodejs
    package_manager install npm

    snap_manager install code --classic
fi

if [ $(confirm "Used for home?") ]; then
    package_manager install deja-dup
    package_manager install gnome-books
    package_manager install gnome-boxes
    package_manager install gnome-calculator
    package_manager install gnome-calendar
    package_manager install gnome-clocks
    package_manager install gnome-weather
    package_manager install libreoffice
    package_manager install simple-scan
    package_manager install thunderbird
    package_manager install transmission-gtk

    snap_manager install slack --classic

    if [ "$distro" == "fedora" ]; then
        package_manager install chromium
        package_manager install fedora-icon-theme
    else
        snap_manager install chromium
    fi

    if [ "$distro" == "ubuntu" ]; then
        package_manager install usb-creator-gtk
        package_manager install virtualbox
    fi
fi

if [ $(confirm "Used for multi media?") ]; then
    package_manager install blender
    package_manager install gimp
    package_manager install ffmpeg
    package_manager install gnome-photos
    package_manager install vlc
    package_manager install youtube-dl
fi

if [ $(confirm "Used for gaming?") ]; then
    flatpak_manager install flathub com.valvesoftware.Steam
fi

################################################################################

# Remove Packages

if [ "$distro" == "mint" ] || [ "$distro" == "lmde" ]; then
    package_manager remove celluloid
    package_manager remove drawing
    package_manager remove hexchat*
    package_manager remove mintbackup
    package_manager remove pix*
    package_manager remove warpinator
elif [ "$distro" == "ubuntu" ]; then
    package_manager remove aisleriot
    package_manager remove gnome-mahjongg
    package_manager remove gnome-mines
    package_manager remove gnome-sudoku
    package_manager remove gnome-todo
    package_manager remove remmina*
    package_manager remove seahorse
    package_manager remove shotwell*
fi

package_manager remove cheese
package_manager remove rhythmbox
package_manager remove totem

if [ "$de" == "gnome" ]; then
    package_manager remove gnome-contacts
    package_manager remove gnome-maps
    package_manager remove gnome-software
fi

package_manager autoremove

snap_manager remove hello-world
snap_manager remove snap-store

################################################################################

if [ "$de" == "gnome" ]; then
    if [ "$distro" == "fedora" ]; then
        # Install Themes
        gsettings set org.gnome.desktop.interface gtk-theme "Adwaita-dark"
        gsettings set org.gnome.desktop.interface icon-theme "Fedora"

        # Add WM Buttons
        gsettings set org.gnome.desktop.wm.preferences button-layout ":minimize,maximize,close"
    elif [ "$distro" == "ubuntu" ]; then
        # Install Themes
        gsettings set org.gnome.desktop.interface gtk-theme "Yaru-dark"
    fi

    # Set Favorites
    # gsettings get org.gnome.shell favorite-apps
    gsettings set org.gnome.shell favorite-apps "['org.gnome.Nautilus.desktop', 'firefox.desktop', 'org.gnome.gedit.desktop', 'org.gnome.Terminal.desktop']"
fi

# Display neofetch to finish
neofetch
