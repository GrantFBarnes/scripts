#!/bin/bash
# Purpose: Install/Remove basic packages for GNU/Linux
################################################################################
cd $(dirname "$0")
folderLocation=$(pwd)
. helper_functions.sh

function confirmWhiptail() {
    local height=7
    if [ -n "$2" ]; then
        height=$2
    fi
    whiptail --title "Set up GNU/Linux" --yesno --defaultno "$1" $height 50
}

function choosePackagesWhiptail() {
    packageSelections=$(whiptail --title "Set up GNU/Linux" --checklist "Select Packages to Install:" --cancel-button "Cancel" 0 0 0 "${packageOptions[@]}" 3>&1 1>&2 2>&3)
    return $?
}

function chooseCategoryWhiptail() {
    categorySelection=$(whiptail --title "Set up GNU/Linux" --menu "Select an Action:" --cancel-button "Cancel" --default-item "${defaultCategory}" 0 0 0 "${categoryOptions[@]}" 3>&1 1>&2 2>&3)
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
    elif [ "$method" == "module" ]; then
        sudo dnf module install ${@:2} -y
    else
        sudo $pm $method ${@:2} -y
    fi
    checkExitStatus
}

################################################################################

if [ -z "$SUDO_USER" ]; then
    echo "Must be run with sudo"
    exit 1
fi

# Determine distribution

distro=$(getDistribution)
pm=$(getPackageManager)

if [ "$distro" == "" ]; then
    echo "---------------------------------------------------------------------"
    echo "Distribution not recognized"
    echo "---------------------------------------------------------------------"
    exit 1
fi

# Install newt to get Whiptail to work
checkNotInstalled whiptail
if [ $? -eq 0 ]; then
    if [ "$pm" == "dnf" ]; then
        packageManager install newt
    elif [ "$pm" == "pacman" ]; then
        packageManager install libnewt
    fi
fi

################################################################################

defaultCategory="."
categorySelection=""
declare -a categoryOptions

declare -a packageOptions
declare -a packageSelections

declare -a packagesToInstall
declare -a modulesToInstall

declare -a packagesToRemove

function setupRepository() {
    if [ "$pm" == "dnf" ]; then
        distroVersion=$(rpm -E %${distro})

        grep -q max_parallel_downloads /etc/dnf/dnf.conf
        if [ $? -eq 1 ]; then
            sudo sh -c 'echo max_parallel_downloads=10 >> /etc/dnf/dnf.conf'
            sudo sh -c 'echo fastestmirror=true >> /etc/dnf/dnf.conf'
            update
        fi

        confirmWhiptail "Enable EPEL/RPM Fusion Repositories?"
        if [ $? -eq 0 ]; then
            if [ "$distro" == "fedora" ]; then
                sudo dnf install https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-${distroVersion}.noarch.rpm -y
            elif [ "$distro" == "centos" ]; then
                sudo dnf install --nogpgcheck https://dl.fedoraproject.org/pub/epel/epel-release-latest-${distroVersion}.noarch.rpm -y
                sudo dnf install --nogpgcheck https://download1.rpmfusion.org/free/el/rpmfusion-free-release-${distroVersion}.noarch.rpm -y
                sudo dnf config-manager --set-enabled powertools
            fi
            confirmWhiptail "Enable Non-Free EPEL Repositories?"
            if [ $? -eq 0 ]; then
                if [ "$distro" == "fedora" ]; then
                    sudo dnf install https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-${distroVersion}.noarch.rpm -y
                elif [ "$distro" == "centos" ]; then
                    sudo dnf install --nogpgcheck https://download1.rpmfusion.org/nonfree/el/rpmfusion-nonfree-release-${distroVersion}.noarch.rpm -y
                fi
            fi
            update
        fi
    fi
}

function setupEnvironment() {

    # Setup up bashrc
    bashrc=/home/$SUDO_USER/.bashrc
    if [ ! -f "$bashrc" ]; then
        sudo -u $SUDO_USER touch $bashrc
    fi

    grep -q codium $bashrc
    if [ $? -eq 1 ]; then
        sudo -u $SUDO_USER echo alias codium='"flatpak run com.vscodium.codium --no-sandbox"' >>$bashrc
        sudo -u $SUDO_USER echo '' >>$bashrc
    fi

    grep -q EDITOR $bashrc
    if [ $? -eq 1 ]; then
        sudo -u $SUDO_USER echo export EDITOR='"/usr/bin/vim"' >>$bashrc
        sudo -u $SUDO_USER echo '' >>$bashrc
    fi

    grep -q NODE_OPTIONS $bashrc
    if [ $? -eq 1 ]; then
        sudo -u $SUDO_USER echo export NODE_OPTIONS='--max_old_space_size=8192' >>$bashrc
        sudo -u $SUDO_USER echo '' >>$bashrc
    fi

    grep -q GFB_HOSTING_ENV $bashrc
    if [ $? -eq 1 ]; then
        sudo -u $SUDO_USER echo export GFB_HOSTING_ENV='"dev"' >>$bashrc
    fi

    grep -q GFB_EDIT_SECRET $bashrc
    if [ $? -eq 1 ]; then
        sudo -u $SUDO_USER echo export GFB_EDIT_SECRET='""' >>$bashrc
    fi

    grep -q JWT_SECRET $bashrc
    if [ $? -eq 1 ]; then
        sudo -u $SUDO_USER echo export JWT_SECRET='""' >>$bashrc
    fi

    grep -q MYSQL_TU_PASSWORD $bashrc
    if [ $? -eq 1 ]; then
        sudo -u $SUDO_USER echo export MYSQL_TU_PASSWORD='""' >>$bashrc
        sudo -u $SUDO_USER echo '' >>$bashrc
    fi

    # Setup up vimrc
    vimrc=/home/$SUDO_USER/.vimrc
    if [ ! -f "$vimrc" ]; then
        sudo -u $SUDO_USER touch $vimrc
    fi

    grep -q "syntax on" $vimrc
    if [ $? -eq 1 ]; then
        sudo -u $SUDO_USER echo syntax on >>$vimrc
    fi

    grep -q "filetype plugin indent on" $vimrc
    if [ $? -eq 1 ]; then
        sudo -u $SUDO_USER echo filetype plugin indent on >>$vimrc
    fi

    grep -q "set scrolloff" $vimrc
    if [ $? -eq 1 ]; then
        sudo -u $SUDO_USER echo set scrolloff=10 >>$vimrc
    fi

    grep -q "set number relativenumber" $vimrc
    if [ $? -eq 1 ]; then
        sudo -u $SUDO_USER echo set number relativenumber >>$vimrc
    fi

    grep -q "set ignorecase smartcase" $vimrc
    if [ $? -eq 1 ]; then
        sudo -u $SUDO_USER echo set ignorecase smartcase >>$vimrc
    fi

    grep -q "set incsearch hlsearch" $vimrc
    if [ $? -eq 1 ]; then
        sudo -u $SUDO_USER echo set incsearch hlsearch >>$vimrc
    fi
}

function installPackages() {

    packageOptions=()
    packageOptions+=("curl" "Curl Command" off)
    packageOptions+=("exfat" "ExFat Format Support" off)
    packageOptions+=("ffmpeg" "ffmpeg to watch videos" off)
    packageOptions+=("htop" "Process Reviewer" off)
    packageOptions+=("ibus-unikey" "Vietnamese Unikey" off)
    packageOptions+=("id3v2" "Modify MP3 Meta Data" off)
    packageOptions+=("imagemagick" "Image Magick" off)
    packageOptions+=("git" "Git" off)
    packageOptions+=("mysql-server" "MySQL Server" off)
    packageOptions+=("nano" "nano" off)
    packageOptions+=("ncdu" "Command Line Disk Usage" off)
    packageOptions+=("neofetch" "neofetch overview display" off)
    packageOptions+=("net-tools" "Network Packages" off)
    packageOptions+=("node" "Node.js and NPM" off)
    packageOptions+=("pip" "Python PIP" off)
    packageOptions+=("ssh" "SSH" off)
    packageOptions+=("tkinter" "Python Tkinter" off)
    packageOptions+=("unzip" "Unzip zip files" off)
    packageOptions+=("vim" "VIM" on)
    packageOptions+=("youtube-dl" "Command Line YT Downloader" off)

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
        "node")
            if [ "$distro" == "fedora" ]; then
                modulesToInstall+=(nodejs:14/default)
            elif [ "$distro" == "centos" ]; then
                modulesToInstall+=(nodejs:14)
            else
                packagesToInstall+=(nodejs)
                packagesToInstall+=(npm)
            fi
            ;;
        "mysql-server")
            if [ "$pm" == "pacman" ]; then
                packagesToInstall+=(mariadb)
            elif [ "$distro" == "debian" ] || [ "$distro" == "fedora" ]; then
                packagesToInstall+=(mariadb-server)
            else
                packagesToInstall+=($pkg)
            fi
            ;;
        "pip")
            if [ "$pm" == "pacman" ]; then
                packagesToInstall+=(python-pip)
            else
                packagesToInstall+=(python3-pip)
            fi
            ;;
        "tkinter")
            if [ "$pm" == "apt" ]; then
                packagesToInstall+=(python3-tk)
            elif [ "$pm" == "dnf" ]; then
                packagesToInstall+=(python3-tkinter)
            elif [ "$pm" == "pacman" ]; then
                packagesToInstall+=(tk)
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

    if [ ${#packagesToInstall[@]} -gt 0 ]; then
        confirmWhiptail "Install packages individually?"
        if [ $? -eq 0 ]; then
            for i in "${packagesToInstall[@]}"; do
                packageManager install $i
            done
        else
            packageManager install ${packagesToInstall[*]}
        fi
    fi

    if [ ${#modulesToInstall[@]} -gt 0 ]; then
        for i in "${modulesToInstall[@]}"; do
            packageManager module $i
        done
    fi
}

function removePackages() {
    packagesToRemove+=(akregator)
    packagesToRemove+=(evolution)
    packagesToRemove+=(gnome-contacts)
    packagesToRemove+=(konqueror)
    packagesToRemove+=(kmail)
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
        packagesToRemove+=(gnome-nibbles)
        packagesToRemove+=(gnome-robots)
        packagesToRemove+=(gnome-taquin)
        packagesToRemove+=(gnome-tetravex)
        packagesToRemove+=(gnome-todo)
        packagesToRemove+=(iagno)
        packagesToRemove+=(lightsoff)
        packagesToRemove+=(remmina*)
        packagesToRemove+=(seahorse)
        packagesToRemove+=(swell-foop)

        if [ "$distro" == "debian" ]; then
            packagesToRemove+=(anthy*)
            packagesToRemove+=(fcitx*)
            packagesToRemove+=(goldendict)
            packagesToRemove+=(hitori)
            packagesToRemove+=(hdate-applet)
            packagesToRemove+=(*mozc*)
            packagesToRemove+=(mlterm*)
            packagesToRemove+=(malcontent)
            packagesToRemove+=(tali)
            packagesToRemove+=(xiterm*)
            packagesToRemove+=(xterm)

            # Remove Languages
            packagesToRemove+=(firefox-esr-l10n-*)
            packagesToRemove+=(libreoffice-l10n-*)
            packagesToRemove+=(hunspell-*)
            packagesToRemove+=(aspell-*)
            packagesToRemove+=(task-*-desktop)
        fi
    elif [ "$distro" == "centos" ]; then
        packagesToRemove+=(pidgin)
    fi

    # Remove Packages

    if [ ${#packagesToRemove[@]} -gt 0 ]; then
        if [ "$pm" == "pacman" ]; then
            for i in "${packagesToRemove[@]}"; do
                packageManager remove $i
            done
        else
            packageManager remove ${packagesToRemove[*]}
        fi
    fi

    packageManager autoremove
}

function chooseUsage() {
    categoryOptions=()
    categoryOptions+=("Update" "Packages")
    categoryOptions+=("Repository" "Setup")
    categoryOptions+=("Environment" "Setup")
    categoryOptions+=("Install" "Packages")
    categoryOptions+=("Remove" "Packages")
    categoryOptions+=("Exit" "")

    # Remove duplicate values in install arrays
    packagesToInstall=($(echo "${packagesToInstall[@]}" | tr ' ' '\n' | sort -u | tr '\n' ' '))

    chooseCategoryWhiptail
    if [ $? -eq 1 ]; then
        return 1
    fi

    case ${categorySelection} in
    "Update")
        update
        defaultCategory="Repository"
        ;;
    "Repository")
        setupRepository
        defaultCategory="Environment"
        ;;
    "Environment")
        setupEnvironment
        defaultCategory="Install"
        ;;
    "Install")
        installPackages
        defaultCategory="Remove"
        ;;
    "Remove")
        removePackages
        defaultCategory="Exit"
        ;;
    "Exit")
        return
        ;;
    esac
    chooseUsage
}

chooseUsage

exit 0
