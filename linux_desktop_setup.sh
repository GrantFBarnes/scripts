#!/bin/bash
# Purpose: Setup fresh install of Linux Desktop (Fedora/Mint/Ubuntu)
################################################################################

function confirm() {
    read -p "$1 (y/N) " ans
    if [ "$ans" == "y" ]; then
        echo "confirmed"
    fi
    echo ""
}

function check_exit_status() {
    if [ $? -eq 0 ]; then
        echo "Success"
    else
        echo "[ERROR] Process Failed!"
        if [ $(confirm "Exit script?") ]; then
            exit 1
        fi
    fi
}

function update() {
    echo "---------------------------------------------------------------------"
    echo "Update"
    echo "---------------------------------------------------------------------"
    if [ "$pm" == "dnf" ]; then
        sudo dnf upgrade --refresh -y
    elif [ "$pm" == "apt" ]; then
        sudo apt update && sudo apt full-upgrade -y
    fi
    check_exit_status
}

function package_manager() {
    local method=$1
    echo "---------------------------------------------------------------------"
    echo "sudo $pm $method ${@:2} -y"
    echo "---------------------------------------------------------------------"
    if [ "$method" == "remove" ] && [ "$pm" == "apt" ]; then
        sudo apt-get remove --purge ${@:2} -y
    else
        sudo $pm $method ${@:2} -y
    fi
    check_exit_status
}

function snap_manager() {
    local method=$1
    echo "---------------------------------------------------------------------"
    echo "sudo snap $method ${@:2}"
    echo "---------------------------------------------------------------------"
    sudo snap $method ${@:2}
    check_exit_status
}

function flatpak_manager() {
    local method=$1
    echo "---------------------------------------------------------------------"
    echo "sudo flatpak $method flathub ${@:2} -y"
    echo "---------------------------------------------------------------------"
    if [ "$method" == "install" ]; then
        sudo flatpak $method flathub ${@:2} -y
    else
        sudo flatpak $method ${@:2} -y
    fi
    check_exit_status
}

################################################################################

clear

# Determine distrobution

osName=$(head -n 1 /etc/os-release)
distro=""
pm=""
de=""

if [[ $osName == *"Fedora"* ]]; then
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

echo "---------------------------------------------------------------------"
echo "Distrobution Found: $distro"
echo "---------------------------------------------------------------------"

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
        fi

        update
    fi
elif [ "$distro" == "mint" ]; then
    nosnap=/etc/apt/preferences.d/nosnap.pref
    if [ -f "$nosnap" ]; then
        sudo rm $nosnap
        update
    fi
fi

################################################################################

individual=false
if [ $(confirm "Would you like to install packages individually?") ]; then
    individual=true
fi

srcPref="repo"
if [ $(confirm "Do you favor/prefer snap packages?") ]; then
    srcPref="snap"
fi

################################################################################

# Determine Packages to install and remove

declare -a packagesInstall
declare -a snapsInstall
declare -a flatpaksInstall

declare -a packagesRemove
declare -a snapsRemove
declare -a flatpaksRemove

packagesInstall+=(baobab)
packagesInstall+=(exfat-utils)
packagesInstall+=(firefox)
packagesInstall+=(flatpak)
packagesInstall+=(gedit)
packagesInstall+=(gnome-system-monitor)
packagesInstall+=(gnome-terminal)
packagesInstall+=(gnome-tweaks)
packagesInstall+=(nano)
packagesInstall+=(neofetch)
packagesInstall+=(snapd)

if [ "$pm" == "apt" ]; then
    packagesInstall+=(exfat-fuse)
elif [ "$pm" == "dnf" ]; then
    packagesInstall+=(fuse-exfat)
fi

snapsInstall+=(hello-world)

if [ $(confirm "Used for development?") ]; then
    packagesInstall+=(git)
    packagesInstall+=(meld)
    packagesInstall+=(net-tools)
    packagesInstall+=(nodejs)
    packagesInstall+=(npm)

    snapsInstall+=("code --classic")
fi

if [ $(confirm "Used for home?") ]; then
    packagesInstall+=(deja-dup)
    packagesInstall+=(gnome-books)
    packagesInstall+=(gnome-boxes)
    packagesInstall+=(gnome-calculator)
    packagesInstall+=(gnome-calendar)
    packagesInstall+=(gnome-clocks)
    packagesInstall+=(gnome-weather)
    packagesInstall+=(simple-scan)
    packagesInstall+=(thunderbird)
    packagesInstall+=(transmission-gtk)

    snapsInstall+=("slack --classic")

    if [ "$srcPref" == "snap" ]; then
        snapsInstall+=(libreoffice)
        packagesRemove+=(libreoffice)
    else
        packagesInstall+=(libreoffice)
        snapsRemove+=(libreoffice)
    fi

    if [ "$pm" == "dnf" ]; then
        if [ "$srcPref" == "snap" ]; then
            snapsInstall+=(chromium)
            packagesRemove+=(chromium)
        else
            packagesInstall+=(chromium)
            snapsRemove+=(chromium)
        fi

        if [ "$distro" == "fedora" ]; then
            packagesInstall+=(fedora-icon-theme)
        fi
    else
        snapsInstall+=(chromium)
    fi

    if [ "$distro" == "ubuntu" ]; then
        packagesInstall+=(usb-creator-gtk)
        packagesInstall+=(virtualbox)
    fi
fi

if [ $(confirm "Used for multi media?") ]; then
    packagesInstall+=(gimp)
    packagesInstall+=(ffmpeg)
    packagesInstall+=(gnome-photos)
    packagesInstall+=(youtube-dl)

    if [ "$srcPref" == "snap" ]; then
        snapsInstall+=("blender --classic")
        snapsInstall+=(vlc)
        packagesRemove+=(blender)
        packagesRemove+=(vlc)
    else
        packagesInstall+=(blender)
        packagesInstall+=(vlc)
        snapsRemove+=(blender)
        snapsRemove+=(vlc)
    fi
fi

if [ $(confirm "Used for gaming?") ]; then
    flatpaksInstall+=(com.valvesoftware.Steam)
fi

################################################################################

# Install Packages

# Package manager

if [ ${#packagesInstall[@]} -gt 0 ]; then
    if [ "$individual" == true ]; then
        for i in "${packagesInstall[@]}"; do 
            package_manager install $i
        done
    else
        package_manager install ${packagesInstall[*]}
    fi
fi

# Flatpaks

sudo flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo

if [ ${#flatpaksInstall[@]} -gt 0 ]; then
    for i in "${flatpaksInstall[@]}"; do 
        flatpak_manager install $i
    done
fi

# Snaps

if [ "$pm" == "dnf" ]; then
    if [ "$distro" == "centos" ]; then
        sudo systemctl enable --now snapd.socket
    fi
    sudo ln -s /var/lib/snapd/snap /snap
fi

if [ ${#snapsInstall[@]} -gt 0 ]; then
    for i in "${snapsInstall[@]}"; do 
        snap_manager install $i
    done
fi

################################################################################

# Determine Packages to Remove

packagesRemove+=(cheese)
packagesRemove+=(rhythmbox)
packagesRemove+=(totem)

snapsRemove+=(hello-world)
snapsRemove+=(snap-store)

if [ "$distro" == "mint" ] || [ "$distro" == "lmde" ]; then
    packagesRemove+=(celluloid)
    packagesRemove+=(drawing)
    packagesRemove+=(hexchat*)
    packagesRemove+=(mintbackup)
    packagesRemove+=(pix*)
    packagesRemove+=(warpinator)
    packagesRemove+=(xed)
elif [ "$distro" == "ubuntu" ]; then
    packagesRemove+=(aisleriot)
    packagesRemove+=(gnome-mahjongg)
    packagesRemove+=(gnome-mines)
    packagesRemove+=(gnome-sudoku)
    packagesRemove+=(gnome-todo)
    packagesRemove+=(remmina*)
    packagesRemove+=(seahorse)
    packagesRemove+=(shotwell*)
elif [ "$distro" == "pop" ]; then
    packagesRemove+=(geary)
    packagesRemove+=(popsicle)
fi

if [ "$de" == "gnome" ]; then
    packagesRemove+=(gnome-contacts)
    packagesRemove+=(gnome-maps)
    packagesRemove+=(gnome-software)
fi

################################################################################

# Remove Packages

# Package manager

if [ ${#packagesRemove[@]} -gt 0 ]; then
    if [ "$individual" == true ]; then
        for i in "${packagesRemove[@]}"; do 
            package_manager remove $i
        done
    else
        package_manager remove ${packagesRemove[*]}
    fi
fi

package_manager autoremove

# Flatpaks

if [ ${#flatpaksRemove[@]} -gt 0 ]; then
    for i in "${flatpaksRemove[@]}"; do 
        flatpak_manager remove $i
    done
fi

# Snaps

if [ ${#snapsRemove[@]} -gt 0 ]; then
    for i in "${snapsRemove[@]}"; do 
        snap_manager remove $i
    done
fi

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
