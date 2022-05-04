#!/bin/bash
# Purpose: Define Bash Helper Functions to be used by other scripts
################################################################################

function getDistribution() {
    distro=$(head -n 1 /etc/os-release)
    if [[ $distro == *"Arch"* ]]; then
        echo "arch"
    elif [[ $distro == *"CentOS"* ]]; then
        echo "centos"
    elif [[ $distro == *"Debian"* ]]; then
        echo "debian"
    elif [[ $distro == *"Fedora"* ]]; then
        echo "fedora"
    elif [[ $distro == *"LMDE"* ]]; then
        echo "lmde"
    elif [[ $distro == *"Manjaro"* ]]; then
        echo "manjaro"
    elif [[ $distro == *"Mint"* ]]; then
        echo "mint"
    elif [[ $distro == *"Pop!_OS"* ]]; then
        echo "pop"
    elif [[ $distro == *"SUSE"* ]]; then
        echo "suse"
    elif [[ $distro == *"Ubuntu"* ]]; then
        echo "ubuntu"
    else
        echo ""
    fi
}

function getPackageManager() {
    distro=$(head -n 1 /etc/os-release)
    if [[ $distro == *"Arch"* ]] || [[ $distro == *"Manjaro"* ]]; then
        echo "pacman"
    elif [[ $distro == *"CentOS"* ]] || [[ $distro == *"Fedora"* ]]; then
        echo "dnf"
    elif [[ $distro == *"SUSE"* ]]; then
        echo "zypper"
    elif [[ $distro == *"Debian"* ]] || [[ $distro == *"LMDE"* ]] || [[ $distro == *"Mint"* ]] || [[ $distro == *"Pop!_OS"* ]] || [[ $distro == *"Ubuntu"* ]]; then
        echo "apt"
    else
        echo ""
    fi
}

function confirmWhiptail() {
    local height=7
    if [ -n "$2" ]; then
        height=$2
    fi
    whiptail --title "Confirmation" --yesno --defaultno "$1" $height 50
}

function checkNotInstalled() {
    if ! command -v $1 &>/dev/null; then
        return 0
    fi
    return 1
}
