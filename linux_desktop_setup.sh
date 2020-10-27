#!/bin/bash
# Purpose: Setup fresh install of GNU/Linux Desktop
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
    categorySelection=$(whiptail --title "Set up GNU/Linux Desktop" --menu "Select a Category to Find Packages:" --cancel-button "Cancel" --default-item "." 0 0 0 "${categoryOptions[@]}" 3>&1 1>&2 2>&3)
    return $?
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
    if [ "$pm" == "dnf" ]; then
        sudo dnf upgrade --refresh -y
    elif [ "$pm" == "apt" ]; then
        sudo apt update && sudo apt full-upgrade -y
    fi
    checkExitStatus
}

function packageManager() {
    local method=$1
    echo "---------------------------------------------------------------------"
    echo "sudo $pm $method ${@:2} -y"
    echo "---------------------------------------------------------------------"
    if [ "$method" == "remove" ] && [ "$pm" == "apt" ]; then
        sudo apt-get remove --purge ${@:2} -y
    else
        sudo $pm $method ${@:2} -y
    fi
    checkExitStatus
}

function snapManager() {
    local method=$1
    echo "---------------------------------------------------------------------"
    echo "sudo snap $method ${@:2}"
    echo "---------------------------------------------------------------------"
    sudo snap $method ${@:2}
    checkExitStatus
}

function flatpakManager() {
    local method=$1
    echo "---------------------------------------------------------------------"
    echo "sudo flatpak $method flathub ${@:2} -y"
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

confirmWhiptail "   Distrobution: $distro\n    Desktop Env: $de\nPackage Manager: $pm\n\nWould you like to continue?" 11
if [ $? -eq 1 ]; then
    exit 0
fi

################################################################################

bulkInstallPackages=true
sourcePreference="repo"
preferRepoOverSnap=true
preferRepoOverFlatpak=true
preferSnapOverFlatpak=true

confirmWhiptail "Install packages individually?"
if [ $? -eq 0 ]; then
    bulkInstallPackages=false
fi

if [ "$distro" == "centos" ]; then
    sourcePreference="flatpak"
    preferRepoOverSnap=false
    preferRepoOverFlatpak=false
    preferSnapOverFlatpak=false
else
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
        confirmWhiptail "Do you prefer flatpak over snap?"
        if [ $? -eq 0 ]; then
            preferSnapOverFlatpak=false
        else
            preferSnapOverFlatpak=true
        fi
    elif [ "$preferRepoOverSnap" == true ]; then
        # Prefer repo over snap, but flatpak over repo
        sourcePreference="flatpak"
        preferSnapOverFlatpak=false
    elif [ "$preferRepoOverFlatpak" == true ]; then
        # Prefer repo over flatpak, but snap over repo
        sourcePreference="snap"
        preferSnapOverFlatpak=true
    else
        # Prefer both snap and flatpak over repo
        confirmWhiptail "Do you prefer flatpak over snap?"
        if [ $? -eq 0 ]; then
            sourcePreference="flatpak"
            preferSnapOverFlatpak=false
        else
            sourcePreference="snap"
            preferSnapOverFlatpak=true
        fi
    fi
fi

################################################################################

# Determine Packages to install and remove

declare -a categoryOptions
categorySelection=""

declare -a packageOptions
declare -a packageSelections

declare -a packagesToInstall
declare -a snapsToInstall
declare -a flatpaksToInstall

declare -a packagesToRemove
declare -a snapsToRemove
declare -a flatpaksToRemove

function basePackages() {
    packageOptions=()
    packageOptions+=("baobab" "Disk Usage" on)
    packageOptions+=("firefox" "Firefox Broswer" on)
    packageOptions+=("flatpak" "Flatpak Manager" on)
    packageOptions+=("gedit" "GUI Text Editor" on)

    if [ "$de" == "gnome" ]; then
        packageOptions+=("gnome-system-monitor" "System Monitor" on)
        packageOptions+=("gnome-terminal" "Terminal" on)
        packageOptions+=("gnome-tweaks" "Gnome Tweaks" on)
        if [ "$distro" != "pop" ]; then
            packageOptions+=("gnome-software" "Gnome Software Manager" on)
        fi
    fi

    if [ "$distro" == "ubuntu" ]; then
        packageOptions+=("gnome-software-plugin-flatpak" "Flatpak Support Gnome Software" on)
    fi

    packageOptions+=("nano" "Terminal Text Editor" on)
    packageOptions+=("neofetch" "Displays System Info" on)
    packageOptions+=("snapd" "Snap Daemon" on)

    choosePackagesWhiptail
    if [ $? -eq 1 ]; then
		return
	fi

    for pkg in $packageSelections; do
        pkg=$(echo $pkg | sed 's/"//g')
        case ${pkg} in
            "firefox")
                if [ "$distro" == "debian" ]; then
                    packagesToInstall+=(firefox-esr)
                else
                    packagesToInstall+=(firefox)
                fi
            ;;
            "snapd")
                packagesToInstall+=($pkg)
                snapsToInstall+=(snap-store)
            ;;
            *)
                packagesToInstall+=($pkg)
            ;;
        esac
    done
}

function developmentPackages() {
    packageOptions=()
    packageOptions+=("code" "Visual Studio Code" off)
    packageOptions+=("git" "Git" off)
    packageOptions+=("meld" "Gnome Meld File Comparitor" off)
    packageOptions+=("net-tools" "Network Packages" off)
    packageOptions+=("nodejs" "NodeJS" off)
    packageOptions+=("npm" "Node Package Manager" off)

    choosePackagesWhiptail
    if [ $? -eq 1 ]; then
		return
	fi

    for pkg in $packageSelections; do
        pkg=$(echo $pkg | sed 's/"//g')
        case ${pkg} in
            "code")
                snapsToInstall+=("code --classic")
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

function homePackages() {
    packageOptions=()
    packageOptions+=("chromium" "Chromium Web Browser" off)
    packageOptions+=("epiphany" "Gnome Web Browser" off)
    packageOptions+=("deja-dup" "Backup Tool" off)
    packageOptions+=("exfat" "ExFat Format Support" off)
    packageOptions+=("gnome-books" "Gnome Books" off)
    packageOptions+=("gnome-boxes" "Gnome Boxes VM Manager" off)
    packageOptions+=("gnome-calculator" "Gnome Calculator" off)
    packageOptions+=("gnome-calendar" "Gnome Calendar" off)
    packageOptions+=("gnome-clocks" "Gnome Clocks" off)
    packageOptions+=("gnome-photos" "Gnome Photos" off)
    packageOptions+=("gnome-weather" "Gnome Weather" off)
    packageOptions+=("imagemagick" "Image Magick" off)
    packageOptions+=("libreoffice" "LibreOffice Suite" off)
    packageOptions+=("slack" "Slack" off)
    packageOptions+=("simple-scan" "Scanner Application" off)
    packageOptions+=("spotify" "Spotify" off)
    packageOptions+=("texworks" "LaTeX Editor" off)
    packageOptions+=("thunderbird" "Thunderbird Email Client" off)

    if [ "$distro" != "centos" ]; then
        packageOptions+=("torbrowser-launcher" "TOR Browser" off)
    fi

    packageOptions+=("transmission-gtk" "Transmission Torrent" off)

    if [ "$distro" == "ubuntu" ]; then
        packageOptions+=("virtualbox" "Virtual Box VM Manager" off)
        packageOptions+=("usb-creator-gtk" "USB Creator" off)
    fi

    choosePackagesWhiptail
    if [ $? -eq 1 ]; then
		return
	fi

    for pkg in $packageSelections; do
        pkg=$(echo $pkg | sed 's/"//g')
        case ${pkg} in
            "chromium")
                if [ "$pm" == "dnf" ]; then
                    if [ "$preferRepoOverSnap" == true ]; then
                        packagesToInstall+=(chromium)
                        snapsToRemove+=(chromium)
                    else
                        snapsToInstall+=(chromium)
                        packagesToRemove+=(chromium)
                    fi
                else
                    snapsToInstall+=(chromium)
                fi
            ;;
            "epiphany")
                if [ "$preferRepoOverFlatpak" == true ]; then
                    if [ "$pm" == "dnf" ]; then
                        packagesToInstall+=(epiphany)
                    else
                        packagesToInstall+=(epiphany-browser)
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
            "imagemagick")
                if [ "$pm" == "dnf" ]; then
                    packagesToInstall+=(ImageMagick)
                elif [ "$pm" == "apt" ]; then
                    packagesToInstall+=(imagemagick)
                fi
            ;;
            "deja-dup")
                if [ "$preferRepoOverFlatpak" == true ]; then
                    packagesToInstall+=(deja-dup)
                    flatpaksToRemove+=(org.gnome.DejaDup)
                else
                    flatpaksToInstall+=(org.gnome.DejaDup)
                    packagesToRemove+=(deja-dup)
                fi
            ;;
            "exfat")
                packagesToInstall+=(exfat-utils)
                if [ "$pm" == "apt" ]; then
                    packagesToInstall+=(exfat-fuse)
                elif [ "$pm" == "dnf" ]; then
                    packagesToInstall+=(fuse-exfat)
                fi
            ;;
            "gnome-books")
                if [ "$preferRepoOverFlatpak" == true ]; then
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
                if [ "$preferRepoOverFlatpak" == true ]; then
                    packagesToInstall+=(gnome-calendar)
                    flatpaksToRemove+=(org.gnome.Calendar)
                else
                    flatpaksToInstall+=(org.gnome.Calendar)
                    packagesToRemove+=(gnome-calendar)
                fi
            ;;
            "gnome-clocks")
                if [ "$preferRepoOverFlatpak" == true ]; then
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
                if [ "$preferRepoOverFlatpak" == true ]; then
                    packagesToInstall+=(gnome-weather)
                    flatpaksToRemove+=(org.gnome.Weather)
                else
                    flatpaksToInstall+=(org.gnome.Weather)
                    packagesToRemove+=(gnome-weather)
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
                    packagesToInstall+=(libreoffice)

                    flatpaksToRemove+=(org.libreoffice.LibreOffice)
                    snapsToRemove+=(libreoffice)
                fi
            ;;
            "texworks")
                flatpaksToInstall+=(org.tug.texworks)
            ;;
            "slack")
                if [ "$preferSnapOverFlatpak" == true ]; then
                    snapsToInstall+=("slack --classic")
                    flatpaksToRemove+=(com.slack.Slack)
                else
                    flatpaksToInstall+=(com.slack.Slack)
                    snapsToRemove+=(slack)
                fi
            ;;
            "spotify")
                if [ "$preferSnapOverFlatpak" == true ]; then
                    snapsToInstall+=(spotify)
                    flatpaksToRemove+=(com.spotify.Client)
                else
                    flatpaksToInstall+=(com.spotify.Client)
                    snapsToRemove+=(spotify)
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
    packageOptions+=("vlc" "Media Player" off)

    if [ "$distro" != "centos" ]; then
        packageOptions+=("ffmpeg" "ffmpeg to watch videos" off)
    fi

    choosePackagesWhiptail
    if [ $? -eq 1 ]; then
		return
	fi

    for pkg in $packageSelections; do
        pkg=$(echo $pkg | sed 's/"//g')
        case ${pkg} in
            "blender")
                if [ "$sourcePreference" == "snap" ]; then
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
    packageOptions+=("steam" "Steam" off)
    packageOptions+=("xonotic" "Open Source FPS" off)

    choosePackagesWhiptail
    if [ $? -eq 1 ]; then
		return
	fi

    for pkg in $packageSelections; do
        pkg=$(echo $pkg | sed 's/"//g')
        case ${pkg} in
            "steam")
                flatpaksToInstall+=(com.valvesoftware.Steam)
            ;;
            "xonotic")
                snapsToInstall+=(xonotic)
            ;;
            *)
                packagesToInstall+=($pkg)
            ;;
        esac
    done
}

function chooseUsage() {
    categoryOptions=()
    categoryOptions+=("Base" "")
    categoryOptions+=("Development" "")
    categoryOptions+=("Home" "")
    categoryOptions+=("Multi Media" "")
    categoryOptions+=("Gaming" "")
    categoryOptions+=("" "")
    categoryOptions+=("Install" "")

    # Remove duplicate values in install arrays
    packagesToInstall=($(echo "${packagesToInstall[@]}" | tr ' ' '\n' | sort -u | tr '\n' ' '))

    chooseCategoryWhiptail
    if [ $? -eq 1 ]; then
		return 1
	fi

    case ${categorySelection} in
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

if [ ${#packagesToInstall[@]} -gt 0 ]; then
    if [ "$bulkInstallPackages" == true ]; then
        packageManager install ${packagesToInstall[*]}
    else
        for i in "${packagesToInstall[@]}"; do
            packageManager install $i
        done
    fi
fi

# Flatpaks

sudo flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo

if [ ${#flatpaksToInstall[@]} -gt 0 ]; then
    for i in "${flatpaksToInstall[@]}"; do
        flatpakManager install $i
    done
fi

# Snaps

if [ "$pm" == "dnf" ]; then
    if [ "$distro" == "centos" ]; then
        sudo systemctl enable --now snapd.socket
    fi
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
packagesToRemove+=(rhythmbox)
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
    packagesToRemove+=(geary)
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
fi

# Display neofetch to finish
neofetch

exit 0
