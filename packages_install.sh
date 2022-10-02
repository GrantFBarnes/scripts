#!/bin/bash
# Purpose: Install/Remove basic packages for GNU/Linux
################################################################################
cd $(dirname "$0")
folderLocation=$(pwd)
. helper_functions.sh

function choosePackagesWhiptail() {
    packageSelections=$(whiptail --title "Set up GNU/Linux" --checklist "Select Packages to Install:" --cancel-button "Cancel" 0 0 0 "${packageOptions[@]}" 3>&1 1>&2 2>&3)
    return $?
}

function chooseCategoryWhiptail() {
    categorySelection=$(whiptail --title "Set up GNU/Linux" --menu "Select an Action:" --cancel-button "Cancel" --default-item "${defaultCategory}" 0 0 0 "${categoryOptions[@]}" 3>&1 1>&2 2>&3)
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
    if [ "$pm" == "apt" ]; then
        sudo apt update && sudo apt full-upgrade -y
    elif [ "$pm" == "dnf" ]; then
        sudo dnf upgrade --refresh -y
    elif [ "$pm" == "pacman" ]; then
        sudo pacman -Syyu --noconfirm
    elif [ "$pm" == "zypper" ]; then
        sudo zypper update --no-confirm
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
            sudo pacman -S ${@:2} --noconfirm --needed
        elif [ "$method" == "remove" ]; then
            sudo pacman -Rsun ${@:2} --noconfirm
        elif [ "$method" == "autoremove" ]; then
            pacman -Qdttq > pacmanorphans
            if [[ $(wc -l < pacmanorphans) -gt 0 ]]; then
                sudo pacman -Rs $(pacman -Qdttq) --noconfirm
            fi
            rm pacmanorphans
        fi
    elif [ "$method" == "remove" ] && [ "$pm" == "apt" ]; then
        sudo apt-get remove --purge ${@:2} -y
    elif [ "$method" == "module" ]; then
        sudo dnf module enable ${@:2} -y
    elif [ "$pm" == "zypper" ]; then
        if [ "$method" == "autoremove" ]; then
            sudo zypper remove --clean-deps --no-confirm $(zypper packages --unneeded | awk -F '|' 'NR==0 || NR==1 || NR==2 || NR==3 || NR==4 {next} {print $3}')
        else
            sudo zypper $method --no-confirm ${@:2}
        fi
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
    if [ "$pm" == "dnf" ] || [ "$pm" == "zypper" ]; then
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

declare -a modulesToEnable
declare -a packagesToInstall

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

    grep -q SQL_TU_PASSWORD $bashrc
    if [ $? -eq 1 ]; then
        sudo -u $SUDO_USER echo export SQL_TU_PASSWORD='""' >>$bashrc
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

    grep -q "set number" $vimrc
    if [ $? -eq 1 ]; then
        sudo -u $SUDO_USER echo set number >>$vimrc
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

function installServerPackages() {

    packageOptions=()
    packageOptions+=("bash-completion" "Bash Completion" off)
    packageOptions+=("cockpit" "Cockpit" off)
    packageOptions+=("curl" "Curl Command" off)
    packageOptions+=("htop" "Process Reviewer" off)
    packageOptions+=("git" "Git" off)
    packageOptions+=("mariadb-server" "MariaDB Server" off)
    packageOptions+=("nano" "nano" off)
    packageOptions+=("ncdu" "Command Line Disk Usage" off)
    packageOptions+=("node" "Node.js and NPM" off)
    packageOptions+=("pip" "Python PIP" off)
    packageOptions+=("podman" "Podman Containers" off)
    packageOptions+=("rust" "Rust Language" off)
    packageOptions+=("ssh" "SSH" off)
    packageOptions+=("vim" "VIM" on)

    choosePackagesWhiptail
    if [ $? -eq 1 ]; then
        return
    fi

    for pkg in $packageSelections; do
        pkg=$(echo $pkg | sed 's/"//g')
        case ${pkg} in
        "node")
            if [ "$pm" == "dnf" ]; then
                modulesToEnable+=(nodejs:18)
            fi

            if [ "$pm" == "zypper" ]; then
                packagesToInstall+=(nodejs16)
                packagesToInstall+=(npm16)
            else
                packagesToInstall+=(nodejs)
                packagesToInstall+=(npm)
            fi
            ;;
        "mariadb-server")
            if [ "$pm" == "pacman" ] || [ "$pm" == "zypper" ]; then
                packagesToInstall+=(mariadb)
            else
                packagesToInstall+=($pkg)
            fi
            ;;
        "pip")
            if [ "$pm" == "pacman" ]; then
                packagesToInstall+=(python-pip)
            elif [ "$pm" == "zypper" ]; then
                packagesToInstall+=(python38-pip)
            else
                packagesToInstall+=(python3-pip)
            fi
            ;;
        "rust")
            if [ "$pm" == "pacman" ]; then
                packagesToInstall+=(rustup)
            else
                packagesToInstall+=(rust)
                packagesToInstall+=(rustfmt)
                packagesToInstall+=(cargo)
            fi
            ;;
        "ssh")
            if [ "$pm" == "apt" ]; then
                packagesToInstall+=(ssh)
            elif [ "$pm" == "zypper" ]; then
                packagesToInstall+=(libssh4)
                packagesToInstall+=(openssh)
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

    if [ ${#modulesToEnable[@]} -gt 0 ]; then
        for i in "${modulesToEnable[@]}"; do
            packageManager module $i
        done
    fi

    if [ ${#packagesToInstall[@]} -gt 0 ]; then
        for i in "${packagesToInstall[@]}"; do
            checkNotInstalled $i
            if [ $? -eq 0 ]; then
                packageManager install $i
            fi
        done
    fi
}

function installDesktopPackages() {

    packageOptions=()
    packageOptions+=("cups" "Printer Support" off)
    packageOptions+=("ffmpeg" "ffmpeg to watch videos" off)
    if [ "$distro" != "centos" ]; then
        packageOptions+=("ibus-unikey" "Vietnamese Unikey" off)
        packageOptions+=("id3v2" "Modify MP3 Meta Data" off)
    fi
    packageOptions+=("imagemagick" "Image Magick" off)
    packageOptions+=("latex" "LaTeX CLI" off)
    if [ "$distro" != "centos" ]; then
        packageOptions+=("yt-dlp" "Command Line YT Downloader" off)
    fi

    choosePackagesWhiptail
    if [ $? -eq 1 ]; then
        return
    fi

    for pkg in $packageSelections; do
        pkg=$(echo $pkg | sed 's/"//g')
        case ${pkg} in
        "ffmpeg")
            if [ "$pm" == "zypper" ]; then
                packagesToInstall+=(ffmpeg-4)
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
        "latex")
            if [ "$pm" == "apt" ]; then
                packagesToInstall+=(texlive-latex-base)
                packagesToInstall+=(texlive-latex-extra)
            elif [ "$pm" == "dnf" ]; then
                packagesToInstall+=(texlive-latex)
                if [ "$distro" == "fedora" ]; then
                    packagesToInstall+=(texlive-collection-latexextra)
                fi
            elif [ "$pm" == "pacman" ]; then
                packagesToInstall+=(texlive-core)
                packagesToInstall+=(texlive-latexextra)
            fi
            ;;
        *)
            packagesToInstall+=($pkg)
            ;;
        esac
    done

    if [ ${#modulesToEnable[@]} -gt 0 ]; then
        for i in "${modulesToEnable[@]}"; do
            packageManager module $i
        done
    fi

    if [ ${#packagesToInstall[@]} -gt 0 ]; then
        for i in "${packagesToInstall[@]}"; do
            checkNotInstalled $i
            if [ $? -eq 0 ]; then
                packageManager install $i
            fi
        done
    fi
}

function removePackages() {
    packagesToRemove+=(akregator)
    packagesToRemove+=(evolution)
    packagesToRemove+=(konqueror)
    packagesToRemove+=(kmail)
    packagesToRemove+=(mpv)

    if [ "$distro" == "mint" ]; then
        packagesToRemove+=(celluloid)
        packagesToRemove+=(drawing)
        packagesToRemove+=(hexchat*)
        packagesToRemove+=(mintbackup)
        packagesToRemove+=(pix*)
        packagesToRemove+=(xed)
    elif [ "$distro" == "ubuntu" ] || [ "$distro" == "debian" ]; then
        packagesToRemove+=(gnome-mahjongg)
        packagesToRemove+=(gnome-todo)
        packagesToRemove+=(remmina*)
        packagesToRemove+=(seahorse)

        if [ "$distro" == "debian" ]; then
            packagesToRemove+=(five-or-more)
            packagesToRemove+=(four-in-a-row)
            packagesToRemove+=(gnome-klotski)
            packagesToRemove+=(gnome-nibbles)
            packagesToRemove+=(gnome-robots)
            packagesToRemove+=(gnome-taquin)
            packagesToRemove+=(gnome-tetravex)
            packagesToRemove+=(iagno)
            packagesToRemove+=(lightsoff)
            packagesToRemove+=(anthy*)
            packagesToRemove+=(fcitx*)
            packagesToRemove+=(goldendict)
            packagesToRemove+=(hitori)
            packagesToRemove+=(hdate-applet)
            packagesToRemove+=(*mozc*)
            packagesToRemove+=(mlterm*)
            packagesToRemove+=(malcontent)
            packagesToRemove+=(swell-foop)
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
    fi

    # Remove Packages

    if [ ${#packagesToRemove[@]} -gt 0 ]; then
        for i in "${packagesToRemove[@]}"; do
            checkNotInstalled $i
            if [ $? -eq 1 ]; then
                packageManager remove $i
            fi
        done
    fi

    packageManager autoremove
}

function chooseUsage() {
    categoryOptions=()
    categoryOptions+=("Update" "Packages")
    categoryOptions+=("Repository" "Setup")
    categoryOptions+=("Environment" "Setup")
    categoryOptions+=("Server" "Packages")
    categoryOptions+=("Desktop" "Packages")
    categoryOptions+=("Remove" "Packages")
    categoryOptions+=("Exit" "")

    packagesToInstall=()

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
        defaultCategory="Server"
        ;;
    "Server")
        installServerPackages
        defaultCategory="Desktop"
        ;;
    "Desktop")
        installDesktopPackages
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
