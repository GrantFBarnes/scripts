#!/bin/bash
# Purpose: Setup environment of GNU/Linux Desktop
################################################################################

# Determine distrobution

osName=$(head -n 1 /etc/os-release)
distro=""
de=""

if [[ $osName == *"Fedora"* ]]; then
    distro="fedora"
elif [[ $osName == *"CentOS"* ]]; then
    distro="centos"
elif [[ $osName == *"Debian"* ]]; then
    distro="debian"
elif [[ $osName == *"Pop!_OS"* ]]; then
    distro="pop"
elif [[ $osName == *"Ubuntu"* ]]; then
    distro="ubuntu"
else
    exit 0
fi

# Set Themes
if [ "$distro" == "fedora" ] || [ "$distro" == "centos" ] || [ "$distro" == "debian" ]; then
    gsettings set org.gnome.desktop.interface gtk-theme "Adwaita-dark"
elif [ "$distro" == "ubuntu" ]; then
    gsettings set org.gnome.desktop.interface gtk-theme "Yaru-dark"
fi

# Set Icons
if [ "$distro" == "fedora" ] || [ "$distro" == "centos" ] || [ "$distro" == "debian" ]; then
    mkdir ~/.local/share/icons
    cp -r Yaru-Blue/ ~/.local/share/icons/Yaru-Blue
    gsettings set org.gnome.desktop.interface icon-theme "Yaru-Blue"
fi

# Set up clock
gsettings set org.gnome.desktop.interface clock-format "12h"
gsettings set org.gnome.desktop.interface clock-show-date true
gsettings set org.gnome.desktop.interface clock-show-seconds true

# Set blank screen to 15 min (900 seconds)
gsettings set org.gnome.desktop.session idle-delay 900

# Add WM Buttons
gsettings set org.gnome.desktop.wm.preferences button-layout ":minimize,maximize,close"

exit 0