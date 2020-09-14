#!/bin/bash
# Purpose: Setup fresh install of Linux Desktop (Fedora/CentOS/Mint/Ubuntu)
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
    echo "sudo snap $method ${@:2} --classic"
    echo "---------------------------------------------------------------------"
    if [ "$method" == "install" ]; then
        sudo snap $method ${@:2} --classic
    else
        sudo snap $method ${@:2}
    fi
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
elif [[ $osName == *"CentOs"* ]]; then
    distro="centos"
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
        elif [ "$distro" == "centos" ]; then
            sudo dnf install --nogpgcheck https://dl.fedoraproject.org/pub/epel/epel-release-latest-8.noarch.rpm -y
            sudo dnf install --nogpgcheck https://download1.rpmfusion.org/free/el/rpmfusion-free-release-8.noarch.rpm https://download1.rpmfusion.org/nonfree/el/rpmfusion-nonfree-release-8.noarch.rpm -y
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

# Determine Packages to install

declare -a packages
declare -a snaps
declare -a flatpaks

packages+=(baobab)
packages+=(exfat-utils)
packages+=(firefox)
packages+=(flatpak)
packages+=(gedit)
packages+=(gnome-system-monitor)
packages+=(gnome-terminal)
packages+=(gnome-tweaks)
packages+=(nano)
packages+=(neofetch)
packages+=(snapd)

if [ "$pm" == "apt" ]; then
    packages+=(exfat-fuse)
elif [ "$pm" == "dnf" ]; then
    packages+=(fuse-exfat)
fi

snaps+=(hello-world)

if [ $(confirm "Used for development?") ]; then
    packages+=(git)
    packages+=(meld)
    packages+=(net-tools)
    packages+=(nodejs)
    packages+=(npm)

    snaps+=(code)
fi

if [ $(confirm "Used for home?") ]; then
    packages+=(deja-dup)
    packages+=(gnome-books)
    packages+=(gnome-boxes)
    packages+=(gnome-calculator)
    packages+=(gnome-calendar)
    packages+=(gnome-clocks)
    packages+=(gnome-weather)
    packages+=(libreoffice)
    packages+=(simple-scan)
    packages+=(thunderbird)
    packages+=(transmission-gtk)

    snaps+=(slack)

    if [ "$pm" == "dnf" ]; then
        packages+=(chromium)
        if [ "$distro" == "fedora" ]; then
            packages+=(fedora-icon-theme)
        fi
    else
        snaps+=(chromium)
    fi

    if [ "$distro" == "ubuntu" ]; then
        packages+=(usb-creator-gtk)
        packages+=(virtualbox)
    fi
fi

if [ $(confirm "Used for multi media?") ]; then
    packages+=(blender)
    packages+=(gimp)
    packages+=(ffmpeg)
    packages+=(gnome-photos)
    packages+=(vlc)
    packages+=(youtube-dl)
fi

if [ $(confirm "Used for gaming?") ]; then
    flatpaks+=(com.valvesoftware.Steam)
fi

################################################################################

individual=false
if [ $(confirm "Would you like to install packages individually?") ]; then
    individual=true
fi

################################################################################

# Install Packages

# Package manager

if [ ${#packages[@]} -gt 0 ]; then
    if [ "$individual" == true ]; then
        for i in "${packages[@]}"; do 
            package_manager install $i
        done
    else
        package_manager install ${packages[*]}
    fi
fi

# Flatpaks

sudo flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo

if [ ${#flatpaks[@]} -gt 0 ]; then
    for i in "${flatpaks[@]}"; do 
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

if [ ${#snaps[@]} -gt 0 ]; then
    for i in "${snaps[@]}"; do 
        snap_manager install $i
    done
fi

################################################################################

# Determine Packages to Remove

packages=()
snaps=()
flatpaks=()

packages+=(cheese)
packages+=(rhythmbox)
packages+=(totem)

snaps+=(hello-world)
snaps+=(snap-store)

if [ "$distro" == "mint" ] || [ "$distro" == "lmde" ]; then
    packages+=(celluloid)
    packages+=(drawing)
    packages+=(hexchat*)
    packages+=(mintbackup)
    packages+=(pix*)
    packages+=(warpinator)
    packages+=(xed)
elif [ "$distro" == "ubuntu" ]; then
    packages+=(aisleriot)
    packages+=(gnome-mahjongg)
    packages+=(gnome-mines)
    packages+=(gnome-sudoku)
    packages+=(gnome-todo)
    packages+=(remmina*)
    packages+=(seahorse)
    packages+=(shotwell*)
elif [ "$distro" == "pop" ]; then
    packages+=(geary)
    packages+=(popsicle)
fi

if [ "$de" == "gnome" ]; then
    packages+=(gnome-contacts)
    packages+=(gnome-maps)
    packages+=(gnome-software)
fi

################################################################################

# Remove Packages

# Package manager

if [ ${#packages[@]} -gt 0 ]; then
    if [ "$individual" == true ]; then
        for i in "${packages[@]}"; do 
            package_manager remove $i
        done
    else
        package_manager remove ${packages[*]}
    fi
fi

package_manager autoremove

# Flatpaks

if [ ${#flatpaks[@]} -gt 0 ]; then
    for i in "${flatpaks[@]}"; do 
        flatpak_manager remove $i
    done
fi

# Snaps

if [ ${#snaps[@]} -gt 0 ]; then
    for i in "${snaps[@]}"; do 
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
