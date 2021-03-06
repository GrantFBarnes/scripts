#!/bin/bash
# Purpose: Define Bash Helper Functions to be used by other scripts
################################################################################

function getDistrobution() {
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
    elif [[ $distro == *"Debian"* ]] || [[ $distro == *"LMDE"* ]] || [[ $distro == *"Mint"* ]] || [[ $distro == *"Pop!_OS"* ]] || [[ $distro == *"Ubuntu"* ]]; then
        echo "apt"
    else
        echo ""
    fi
}
