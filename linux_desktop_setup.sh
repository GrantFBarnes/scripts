#!/bin/bash
# Purpose: Setup fresh install of GNU/Linux Desktop
################################################################################

function confirm() {
    local height=7
    if [ -n "$2" ]; then
        height=$2
    fi
    whiptail --title "Set up GNU/Linux Desktop" --yesno --defaultno "$1" $height 50
}

function check_exit_status() {
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

confirm "   Distrobution: $distro\n    Desktop Env: $de\nPackage Manager: $pm\n\nWould you like to continue?" 11
if [ $? -eq 1 ]; then
    exit 0
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
        package_manager install newt
    fi
elif [ "$distro" == "mint" ]; then
    nosnap=/etc/apt/preferences.d/nosnap.pref
    if [ -f "$nosnap" ]; then
        sudo rm $nosnap
        update
    fi
fi

################################################################################

individual=true
srcPref="repo"
repoOverSnap=true
repoOverFlatpak=true
snapOverFlatpak=true

confirm "Would you like to bulk install packages?"
if [ $? -eq 0 ]; then
    individual=false
fi

if [ "$distro" == "centos" ]; then
    srcPref="flatpak"
    repoOverSnap=false
    repoOverFlatpak=false
    snapOverFlatpak=false
else
    confirm "Do you prefer snap over repository?"
    if [ $? -eq 0 ]; then
        repoOverSnap=false
    else
        repoOverSnap=true
    fi

    confirm "Do you prefer flatpak over repository?"
    if [ $? -eq 0 ]; then
        repoOverFlatpak=false
    else
        repoOverFlatpak=true
    fi

    if [ "$repoOverSnap" == true ] && [ "$repoOverFlatpak" == true ]; then
        # Prefer repo over both snaps and flatpaks
        srcPref="repo"
        confirm "Do you prefer flatpak over snap?"
        if [ $? -eq 0 ]; then
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
        confirm "Do you prefer flatpak over snap?"
        if [ $? -eq 0 ]; then
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

function basePackages() {
    options=()
    options+=("baobab" "Disk Usage" on)
    options+=("firefox" "Firefox Broswer" on)
    options+=("flatpak" "Flatpak Manager" on)
    options+=("gedit" "GUI Text Editor" on)

    if [ "$de" == "gnome" ]; then
        options+=("gnome-system-monitor" "System Monitor" on)
        options+=("gnome-terminal" "Terminal" on)
        options+=("gnome-tweaks" "Gnome Tweaks" on)
        if [ "$distro" != "pop" ]; then
            options+=("gnome-software" "Gnome Software Manager" on)
        fi
    fi

    if [ "$distro" == "ubuntu" ]; then
        options+=("gnome-software-plugin-flatpak" "Flatpak Support Gnome Software" on)
    fi

    options+=("nano" "Terminal Text Editor" on)
    options+=("neofetch" "Displays System Info" on)
    options+=("snapd" "Snap Daemon" on)

    selection=$(whiptail --title "Set up GNU/Linux Desktop" --checklist "Select Packages to Install:" --cancel-button "Cancel" --default-item "." 0 0 0 "${options[@]}" 3>&1 1>&2 2>&3)
    if [ $? -eq 1 ]; then
		return
	fi

    for pkg in $selection; do
        pkg=$(echo $pkg | sed 's/"//g')
        case ${pkg} in
            "firefox")
                if [ "$distro" == "debian" ]; then
                    packagesInstall+=(firefox-esr)
                else
                    packagesInstall+=(firefox)
                fi
            ;;
            "snapd")
                packagesInstall+=($pkg)
                snapsInstall+=(snap-store)
            ;;
            *)
                packagesInstall+=($pkg)
            ;;
        esac
    done
}

function developmentPackages() {
    options=()
    options+=("code" "Visual Studio Code" off)
    options+=("git" "Git" off)
    options+=("meld" "Gnome Meld File Comparitor" off)
    options+=("net-tools" "Network Packages" off)
    options+=("nodejs" "NodeJS" off)
    options+=("npm" "Node Package Manager" off)

    selection=$(whiptail --title "Set up GNU/Linux Desktop" --checklist "Select Packages to Install:" --cancel-button "Cancel" --default-item "." 0 0 0 "${options[@]}" 3>&1 1>&2 2>&3)
    if [ $? -eq 1 ]; then
		return
	fi

    for pkg in $selection; do
        pkg=$(echo $pkg | sed 's/"//g')
        case ${pkg} in
            "code")
                snapsInstall+=("code --classic")
            ;;
            "meld")
                if [ "$repoOverFlatpak" == true ]; then
                    packagesInstall+=(meld)
                    flatpaksRemove+=(org.gnome.meld)
                else
                    flatpaksInstall+=(org.gnome.meld)
                    packagesRemove+=(meld)
                fi
            ;;
            *)
                packagesInstall+=($pkg)
            ;;
        esac
    done
}

function homePackages() {
    options=()
    options+=("chromium" "Chromium Web Browser" off)
    options+=("epiphany" "Gnome Web Browser" off)
    options+=("deja-dup" "Backup Tool" off)
    options+=("exfat" "ExFat Format Support" off)
    options+=("gnome-books" "Gnome Books" off)
    options+=("gnome-boxes" "Gnome Boxes VM Manager" off)
    options+=("gnome-calculator" "Gnome Calculator" off)
    options+=("gnome-calendar" "Gnome Calendar" off)
    options+=("gnome-clocks" "Gnome Clocks" off)
    options+=("gnome-photos" "Gnome Photos" off)
    options+=("gnome-weather" "Gnome Weather" off)
    options+=("imagemagick" "Image Magick" off)
    options+=("libreoffice" "LibreOffice Suite" off)
    options+=("slack" "Slack" off)
    options+=("simple-scan" "Scanner Application" off)
    options+=("spotify" "Spotify" off)
    options+=("texworks" "LaTeX Editor" off)
    options+=("thunderbird" "Thunderbird Email Client" off)
    options+=("torbrowser-launcher" "TOR Browser" off)
    options+=("transmission-gtk" "Transmission Torrent" off)

    if [ "$distro" == "ubuntu" ]; then
        options+=("virtualbox" "Virtual Box VM Manager" off)
        options+=("usb-creator-gtk" "USB Creator" off)
    fi

    selection=$(whiptail --title "Set up GNU/Linux Desktop" --checklist "Select Packages to Install:" --cancel-button "Cancel" --default-item "." 0 0 0 "${options[@]}" 3>&1 1>&2 2>&3)
    if [ $? -eq 1 ]; then
		return
	fi

    for pkg in $selection; do
        pkg=$(echo $pkg | sed 's/"//g')
        case ${pkg} in
            "chromium")
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
            ;;
            "imagemagick")
                if [ "$pm" == "dnf" ]; then
                    packagesInstall+=(ImageMagick)
                elif [ "$pm" == "apt" ]; then
                    packagesInstall+=(imagemagick)
                fi
            ;;
            "deja-dup")
                if [ "$repoOverFlatpak" == true ]; then
                    packagesInstall+=(deja-dup)
                    flatpaksRemove+=(org.gnome.DejaDup)
                else
                    flatpaksInstall+=(org.gnome.DejaDup)
                    packagesRemove+=(deja-dup)
                fi
            ;;
            "exfat")
                packagesInstall+=(exfat-utils)
                if [ "$pm" == "apt" ]; then
                    packagesInstall+=(exfat-fuse)
                elif [ "$pm" == "dnf" ]; then
                    packagesInstall+=(fuse-exfat)
                fi
            ;;
            "gnome-books")
                if [ "$repoOverFlatpak" == true ]; then
                    packagesInstall+=(gnome-books)
                    flatpaksRemove+=(org.gnome.Books)
                else
                    flatpaksInstall+=(org.gnome.Books)
                    packagesRemove+=(gnome-books)
                fi
            ;;
            "gnome-boxes")
                if [ "$repoOverFlatpak" == true ]; then
                    packagesInstall+=(gnome-boxes)
                    flatpaksRemove+=(org.gnome.Boxes)
                else
                    flatpaksInstall+=(org.gnome.Boxes)
                    packagesRemove+=(gnome-boxes)
                fi
            ;;
            "gnome-calculator")
                if [ "$repoOverFlatpak" == true ]; then
                    packagesInstall+=(gnome-calculator)
                    flatpaksRemove+=(org.gnome.Calculator)
                else
                    flatpaksInstall+=(org.gnome.Calculator)
                    packagesRemove+=(gnome-calculator)
                fi
            ;;
            "gnome-calendar")
                if [ "$repoOverFlatpak" == true ]; then
                    packagesInstall+=(gnome-calendar)
                    flatpaksRemove+=(org.gnome.Calendar)
                else
                    flatpaksInstall+=(org.gnome.Calendar)
                    packagesRemove+=(gnome-calendar)
                fi
            ;;
            "gnome-clocks")
                if [ "$repoOverFlatpak" == true ]; then
                    packagesInstall+=(gnome-clocks)
                    flatpaksRemove+=(org.gnome.clocks)
                else
                    flatpaksInstall+=(org.gnome.clocks)
                    packagesRemove+=(gnome-clocks)
                fi
            ;;
            "gnome-photos")
                if [ "$repoOverFlatpak" == true ]; then
                    packagesInstall+=(gnome-photos)
                    flatpaksRemove+=(org.gnome.Photos)
                else
                    flatpaksInstall+=(org.gnome.Photos)
                    packagesRemove+=(gnome-photos)
                fi
            ;;
            "gnome-weather")
                if [ "$repoOverFlatpak" == true ]; then
                    packagesInstall+=(gnome-weather)
                    flatpaksRemove+=(org.gnome.Weather)
                else
                    flatpaksInstall+=(org.gnome.Weather)
                    packagesRemove+=(gnome-weather)
                fi
            ;;
            "libreoffice")
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
            ;;
            "slack")
                if [ "$snapOverFlatpak" == true ]; then
                    snapsInstall+=("slack --classic")
                    flatpaksRemove+=(com.slack.Slack)
                else
                    flatpaksInstall+=(com.slack.Slack)
                    snapsRemove+=(slack)
                fi
            ;;
            "spotify")
                if [ "$snapOverFlatpak" == true ]; then
                    snapsInstall+=(spotify)
                    flatpaksRemove+=(com.spotify.Client)
                else
                    flatpaksInstall+=(com.spotify.Client)
                    snapsRemove+=(spotify)
                fi
            ;;
            *)
                packagesInstall+=($pkg)
            ;;
        esac
    done
}

function mediaPackages() {
    options=()
    options+=("blender" "3D Modleler and Video Editor" off)
    options+=("gimp" "GNU Image Manipulation Program" off)
    options+=("vlc" "Media Player" off)

    if [ "$distro" != "centos" ]; then
        options+=("ffmpeg" "ffmpeg to watch videos" off)
    fi

    selection=$(whiptail --title "Set up GNU/Linux Desktop" --checklist "Select Packages to Install:" --cancel-button "Cancel" --default-item "." 0 0 0 "${options[@]}" 3>&1 1>&2 2>&3)
    if [ $? -eq 1 ]; then
		return
	fi

    for pkg in $selection; do
        pkg=$(echo $pkg | sed 's/"//g')
        case ${pkg} in
            "blender")
                if [ "$srcPref" == "snap" ]; then
                    snapsInstall+=("blender --classic")

                    flatpaksRemove+=(org.blender.Blender)
                    packagesRemove+=(blender)
                elif [ "$srcPref" == "flatpak" ]; then
                    flatpaksInstall+=(org.blender.Blender)

                    snapsRemove+=(blender)
                    packagesRemove+=(blender)
                else
                    packagesInstall+=(blender)

                    flatpaksRemove+=(org.blender.Blender)
                    snapsRemove+=(blender)
                fi
            ;;
            "gimp")
                if [ "$repoOverFlatpak" == true ]; then
                    packagesInstall+=(gimp)
                    flatpaksRemove+=(org.gimp.GIMP)
                else
                    flatpaksInstall+=(org.gimp.GIMP)
                    packagesRemove+=(gimp)
                fi
            ;;
            "vlc")
                if [ "$srcPref" == "snap" ]; then
                    snapsInstall+=(vlc)

                    flatpaksRemove+=(org.videolan.VLC)
                    packagesRemove+=(vlc)
                elif [ "$srcPref" == "flatpak" ]; then
                    flatpaksInstall+=(org.videolan.VLC)

                    snapsRemove+=(vlc)
                    packagesRemove+=(vlc)
                else
                    packagesInstall+=(vlc)

                    flatpaksRemove+=(org.videolan.VLC)
                    snapsRemove+=(vlc)
                fi
            ;;
            *)
                packagesInstall+=($pkg)
            ;;
        esac
    done
}

function gamingPackages() {
    options=()
    options+=("steam" "Steam" off)
    options+=("xonotic" "Open Source FPS" off)

    selection=$(whiptail --title "Set up GNU/Linux Desktop" --checklist "Select Packages to Install:" --cancel-button "Cancel" --default-item "." 0 0 0 "${options[@]}" 3>&1 1>&2 2>&3)
    if [ $? -eq 1 ]; then
		return
	fi

    for pkg in $selection; do
        pkg=$(echo $pkg | sed 's/"//g')
        case ${pkg} in
            "steam")
                flatpaksInstall+=(com.valvesoftware.Steam)
            ;;
            "xonotic")
                snapsInstall+=(xonotic)
            ;;
            *)
                packagesInstall+=($pkg)
            ;;
        esac
    done
}

function chooseUsage() {
    options=()
    options+=("Base" "")
    options+=("Development" "")
    options+=("Home" "")
    options+=("Multi Media" "")
    options+=("Gaming" "")
    options+=("" "")
    options+=("Install" "")

    # Remove duplicate values in install arrays
    packagesInstall=($(echo "${packagesInstall[@]}" | tr ' ' '\n' | sort -u | tr '\n' ' '))
    snapsInstall=($(echo "${snapsInstall[@]}" | tr ' ' '\n' | sort -u | tr '\n' ' '))
    flatpaksInstall=($(echo "${flatpaksInstall[@]}" | tr ' ' '\n' | sort -u | tr '\n' ' '))

    packageCount=$((${#packagesInstall[@]} + ${#snapsInstall[@]} + ${#flatpaksInstall[@]}))
    selection=$(whiptail --backtitle "Packages Selected to Install: ${packageCount}" --title "Set up GNU/Linux Desktop" --menu "Find Packages to Install by Category:" --cancel-button "Cancel" --default-item "." 0 0 0 "${options[@]}" 3>&1 1>&2 2>&3)
    if [ $? -eq 1 ]; then
		return 1
	fi

    case ${selection} in
        "Base")
            basePackages
        ;;
        "Development")
            developmentPackages
        ;;
        "Home")
            homePackages
        ;;
        "Multi Media")
            mediaPackages
        ;;
        "Gaming")
            gamingPackages
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

exit 0
