#!/bin/bash
# Purpose: Setup fresh install of Linux Desktop (Fedora/Ubuntu)
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
elif [[ $osName == *"CentOS"* ]]; then
    distro="centos"
    pm="dnf"
    de="gnome"
elif [[ $osName == *"Debian"* ]]; then
    distro="debian"
    pm="apt"
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

individual=false
srcPref="repo"
repoOverSnap=true
repoOverFlatpak=true
snapOverFlatpak=true

if [ $(confirm "Would you like to install packages individually?") ]; then
    individual=true
fi

if [ "$distro" == "centos" ]; then
    srcPref="flatpak"
    repoOverSnap=false
    repoOverFlatpak=false
    snapOverFlatpak=false
else
    if [ $(confirm "Do you prefer snap over repo?") ]; then
        repoOverSnap=false
    else
        repoOverSnap=true
    fi

    if [ $(confirm "Do you prefer flatpak over repo?") ]; then
        repoOverFlatpak=false
    else
        repoOverFlatpak=true
    fi

    if [ "$repoOverSnap" == true ] && [ "$repoOverFlatpak" == true ]; then
        # Prefer repo over both snaps and flatpaks
        srcPref="repo"
        if [ $(confirm "Do you prefer flatpak over snap?") ]; then
            snapOverFlatpak=false
        else
            snapOverFlatpak=true
        fi
    elif [ "$repoOverSnap" == true ]; then
        # Prefer repo over snap, but flatpak over repo
        srcPref="flatpak"
        snapOverFlatpak=false
    elif [ "$repoOverFlatpak" == true ]; then
        # Prefer repo over flatpak, but snap over repo
        srcPref="snap"
        snapOverFlatpak=true
    else 
        # Prefer both snap and flatpak over repo
        if [ $(confirm "Do you prefer flatpak over snap?") ]; then
            srcPref="flatpak"
            snapOverFlatpak=false
        else
            srcPref="snap"
            snapOverFlatpak=true
        fi
    fi
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
packagesInstall+=(flatpak)
packagesInstall+=(gedit)
packagesInstall+=(gnome-system-monitor)
packagesInstall+=(gnome-terminal)
packagesInstall+=(nano)
packagesInstall+=(neofetch)
packagesInstall+=(snapd)

snapsInstall+=(hello-world)
snapsInstall+=(snap-store)

if [ "$distro" == "debian" ]; then
    packagesInstall+=(firefox-esr)
else
    packagesInstall+=(firefox)
fi

if [ "$de" == "gnome" ]; then
    packagesInstall+=(gnome-tweaks)
    if [ "$distro" != "pop" ]; then
        packagesInstall+=(gnome-software)
    fi
fi

if [ "$pm" == "apt" ]; then
    packagesInstall+=(exfat-fuse)
elif [ "$pm" == "dnf" ]; then
    packagesInstall+=(fuse-exfat)
fi

if [ "$distro" == "fedora" ]; then
    packagesInstall+=(fedora-icon-theme)
elif [ "$distro" == "ubuntu" ]; then
    packagesInstall+=(gnome-software-plugin-flatpak)
fi

if [ "$srcPref" == "snap" ]; then
    snapsInstall+=(libreoffice)

    flatpaksRemove+=(org.libreoffice.LibreOffice)
    packagesRemove+=(libreoffice*)
elif [ "$srcPref" == "flatpak" ]; then
    flatpaksInstall+=(org.libreoffice.LibreOffice)

    snapsRemove+=(libreoffice)
    packagesRemove+=(libreoffice*)
else
    packagesInstall+=(libreoffice)

    flatpaksRemove+=(org.libreoffice.LibreOffice)
    snapsRemove+=(libreoffice)
fi

if [ $(confirm "Used for development?") ]; then
    packagesInstall+=(git)
    packagesInstall+=(net-tools)
    packagesInstall+=(nodejs)
    packagesInstall+=(npm)

    snapsInstall+=("code --classic")

    if [ "$repoOverFlatpak" == true ]; then
        packagesInstall+=(meld)
        flatpaksRemove+=(org.gnome.meld)
    else
        flatpaksInstall+=(org.gnome.meld)
        packagesRemove+=(meld)
    fi
fi

if [ $(confirm "Used for home?") ]; then
    flatpaksInstall+=(org.gnome.Epiphany)
    flatpaksInstall+=(org.tug.texworks)
    packagesInstall+=(simple-scan)
    packagesInstall+=(thunderbird)
    packagesInstall+=(transmission-gtk)

    if [ "$snapOverFlatpak" == true ]; then
        snapsInstall+=("slack --classic")
        snapsInstall+=(spotify)

        flatpaksRemove+=(com.slack.Slack)
        flatpaksRemove+=(com.spotify.Client)
    else
        flatpaksInstall+=(com.slack.Slack)
        flatpaksInstall+=(com.spotify.Client)

        snapsRemove+=(slack)
        snapsRemove+=(spotify)
    fi

    if [ "$repoOverFlatpak" == true ]; then
        packagesInstall+=(deja-dup)
        packagesInstall+=(gnome-books)
        packagesInstall+=(gnome-boxes)
        packagesInstall+=(gnome-calculator)
        packagesInstall+=(gnome-calendar)
        packagesInstall+=(gnome-clocks)
        packagesInstall+=(gnome-photos)
        packagesInstall+=(gnome-weather)

        flatpaksRemove+=(org.gnome.DejaDup)
        flatpaksRemove+=(org.gnome.Books)
        flatpaksRemove+=(org.gnome.Boxes)
        flatpaksRemove+=(org.gnome.Calculator)
        flatpaksRemove+=(org.gnome.Calendar)
        flatpaksRemove+=(org.gnome.clocks)
        flatpaksRemove+=(org.gnome.Photos)
        flatpaksRemove+=(org.gnome.Weather)
    else
        flatpaksInstall+=(org.gnome.DejaDup)
        flatpaksInstall+=(org.gnome.Books)
        flatpaksInstall+=(org.gnome.Boxes)
        flatpaksInstall+=(org.gnome.Calculator)
        flatpaksInstall+=(org.gnome.Calendar)
        flatpaksInstall+=(org.gnome.clocks)
        flatpaksInstall+=(org.gnome.Photos)
        flatpaksInstall+=(org.gnome.Weather)

        packagesRemove+=(deja-dup)
        packagesRemove+=(gnome-books)
        packagesRemove+=(gnome-boxes)
        packagesRemove+=(gnome-calculator)
        packagesRemove+=(gnome-calendar)
        packagesRemove+=(gnome-clocks)
        packagesRemove+=(gnome-photos)
        packagesRemove+=(gnome-weather)
    fi

    if [ "$distro" == "ubuntu" ]; then
        packagesInstall+=(virtualbox)
        packagesInstall+=(usb-creator-gtk)
    fi

    if [ "$pm" == "dnf" ]; then
        if [ "$repoOverSnap" == true ]; then
            packagesInstall+=(chromium)
            snapsRemove+=(chromium)
        else
            snapsInstall+=(chromium)
            packagesRemove+=(chromium)
        fi
    else
        snapsInstall+=(chromium)
    fi

    if [ "$pm" == "dnf" ]; then
        packagesInstall+=(ImageMagick)
    elif [ "$pm" == "apt" ]; then
        packagesInstall+=(imagemagick)
    fi
fi

if [ $(confirm "Used for multi media?") ]; then
    packagesInstall+=(youtube-dl)

    if [ "$distro" != "centos" ]; then
        packagesInstall+=(ffmpeg)
    fi

    if [ "$repoOverFlatpak" == true ]; then
        packagesInstall+=(gimp)
        flatpaksRemove+=(org.gimp.GIMP)
    else
        flatpaksInstall+=(org.gimp.GIMP)
        packagesRemove+=(gimp)
    fi

    if [ "$srcPref" == "snap" ]; then
        snapsInstall+=("blender --classic")
        snapsInstall+=(vlc)

        flatpaksRemove+=(org.blender.Blender)
        flatpaksRemove+=(org.videolan.VLC)
        packagesRemove+=(blender)
        packagesRemove+=(vlc)
    elif [ "$srcPref" == "flatpak" ]; then
        flatpaksInstall+=(org.blender.Blender)
        flatpaksInstall+=(org.videolan.VLC)

        snapsRemove+=(blender)
        snapsRemove+=(vlc)
        packagesRemove+=(blender)
        packagesRemove+=(vlc)
    else
        packagesInstall+=(blender)
        packagesInstall+=(vlc)

        flatpaksRemove+=(org.blender.Blender)
        flatpaksRemove+=(org.videolan.VLC)
        snapsRemove+=(blender)
        snapsRemove+=(vlc)
    fi
fi

if [ $(confirm "Used for gaming?") ]; then
    flatpaksInstall+=(com.valvesoftware.Steam)
    snapsInstall+=(xonotic)
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
packagesRemove+=(evolution)
packagesRemove+=(mpv)
packagesRemove+=(rhythmbox)
packagesRemove+=(totem)

snapsRemove+=(hello-world)

if [ "$distro" == "mint" ] || [ "$distro" == "lmde" ]; then
    packagesRemove+=(celluloid)
    packagesRemove+=(drawing)
    packagesRemove+=(hexchat*)
    packagesRemove+=(mintbackup)
    packagesRemove+=(pix*)
    packagesRemove+=(warpinator)
    packagesRemove+=(xed)
elif [ "$distro" == "ubuntu" ] || [ "$distro" == "debian" ]; then
    packagesRemove+=(aisleriot)
    packagesRemove+=(five-or-more)
    packagesRemove+=(four-in-a-row)
    packagesRemove+=(gnome-chess)
    packagesRemove+=(gnome-klotski)
    packagesRemove+=(gnome-mahjongg)
    packagesRemove+=(gnome-mines)
    packagesRemove+=(gnome-music)
    packagesRemove+=(gnome-nibbles)
    packagesRemove+=(gnome-robots)
    packagesRemove+=(gnome-sudoku)
    packagesRemove+=(gnome-tetravex)
    packagesRemove+=(gnome-todo)
    packagesRemove+=(remmina*)
    packagesRemove+=(seahorse)
    packagesRemove+=(shotwell*)

    if [ "$distro" == "debian" ]; then
        packagesRemove+=(anthy*)
        packagesRemove+=(fcitx*)
        packagesRemove+=(goldendict)
        packagesRemove+=(hitori)
        packagesRemove+=(tali)
        packagesRemove+=(quadrapassel)
        packagesRemove+=(xterm)
    fi
elif [ "$distro" == "pop" ]; then
    packagesRemove+=(geary)
    packagesRemove+=(popsicle)
elif [ "$distro" == "centos" ]; then
    packagesRemove+=(pidgin)
fi

if [ "$de" == "gnome" ]; then
    packagesRemove+=(gnome-contacts)
    packagesRemove+=(gnome-maps)
    packagesRemove+=(gnome-software-plugin-snap)
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

sudo flatpak remove --unused

# Snaps

if [ ${#snapsRemove[@]} -gt 0 ]; then
    for i in "${snapsRemove[@]}"; do
        snap_manager remove $i
    done
fi

LANG=en_US.UTF-8 snap list --all | awk '/disabled/{print $1, $3}' |
    while read snapname revision; do
        sudo snap remove "$snapname" --revision="$revision"
    done

################################################################################

if [ "$de" == "gnome" ]; then
    if [ "$distro" == "fedora" ]; then
        # Install Themes
        gsettings set org.gnome.desktop.interface gtk-theme "Adwaita-dark"
        gsettings set org.gnome.desktop.interface icon-theme "Fedora"
    elif [ "$distro" == "ubuntu" ]; then
        # Install Themes
        gsettings set org.gnome.desktop.interface gtk-theme "Yaru-dark"
    fi

    # Add WM Buttons
    gsettings set org.gnome.desktop.wm.preferences button-layout ":minimize,maximize,close"

    # Set Favorites
    # gsettings get org.gnome.shell favorite-apps
    gsettings set org.gnome.shell favorite-apps "['org.gnome.Nautilus.desktop', 'org.gnome.gedit.desktop', 'org.gnome.Terminal.desktop']"
fi

# Display neofetch to finish
neofetch
