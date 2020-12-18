#!/bin/bash
# Purpose: Setup environment of GNU/Linux Desktop
################################################################################

distro=$1

# Set Themes
if [ "$distro" == "arch" ] || [ "$distro" == "centos" ] || [ "$distro" == "debian" ] || [ "$distro" == "fedora" ]; then
    gsettings set org.gnome.desktop.interface gtk-theme "Adwaita-dark"
elif [ "$distro" == "ubuntu" ]; then
    gsettings set org.gnome.desktop.interface gtk-theme "Yaru-dark"
fi

# Set Icons
if [ "$distro" == "arch" ] || [ "$distro" == "centos" ] || [ "$distro" == "debian" ] || [ "$distro" == "fedora" ]; then
    mkdir ~/.local/share/icons
    cp -r Yaru-Blue/ ~/.local/share/icons/Yaru-Blue
    gsettings set org.gnome.desktop.interface icon-theme "Yaru-Blue"
fi

# Setup Clock
gsettings set org.gnome.desktop.interface clock-format "12h"
gsettings set org.gnome.desktop.interface clock-show-date true
gsettings set org.gnome.desktop.interface clock-show-seconds true
gsettings set org.gnome.desktop.interface clock-show-weekday true

# Show Battery Percentage
gsettings set org.gnome.desktop.interface show-battery-percentage true

# Enable Overview Hot Corner
gsettings set org.gnome.desktop.interface enable-hot-corners true

# Disable Animations
gsettings set org.gnome.desktop.interface enable-animations false

# Set Blank Screen to 15 min (900 seconds)
gsettings set org.gnome.desktop.session idle-delay 900

# Add WM Buttons
gsettings set org.gnome.desktop.wm.preferences button-layout ":minimize,maximize,close"

# Set Nautilus Default View to List
gsettings set org.gnome.nautilus.preferences default-folder-viewer "list-view"

# Set Touchpad Tap to Click
gsettings set org.gnome.desktop.peripherals.touchpad tap-to-click true

# Set Gnome extensions
gnome-extensions info caffeine@patapon.info
if [ $? -eq 0 ]; then
    gnome-extensions enable caffeine@patapon.info
fi

dashToDock=false

gnome-extensions info dash-to-dock@micxgx.gmail.com
if [ $? -eq 0 ]; then
    gnome-extensions enable dash-to-dock@micxgx.gmail.com
    dashToDock=true
fi

gnome-extensions info ubuntu-dock@ubuntu.com
if [ $? -eq 0 ]; then
    gnome-extensions enable ubuntu-dock@ubuntu.com
    dashToDock=true
fi

if [ $dashToDock == true ]; then
    gsettings set org.gnome.shell.extensions.dash-to-dock background-color "#000000"
    gsettings set org.gnome.shell.extensions.dash-to-dock background-opacity 0.5
    gsettings set org.gnome.shell.extensions.dash-to-dock custom-background-color true
    gsettings set org.gnome.shell.extensions.dash-to-dock custom-theme-shrink true
    gsettings set org.gnome.shell.extensions.dash-to-dock dash-max-icon-size 28
    gsettings set org.gnome.shell.extensions.dash-to-dock dock-fixed true
    gsettings set org.gnome.shell.extensions.dash-to-dock extend-height true
    gsettings set org.gnome.shell.extensions.dash-to-dock hot-keys false
    gsettings set org.gnome.shell.extensions.dash-to-dock icon-size-fixed true
    gsettings set org.gnome.shell.extensions.dash-to-dock intellihide-mode "MAXIMIZED_WINDOWS"
    gsettings set org.gnome.shell.extensions.dash-to-dock running-indicator-dominant-color true
    gsettings set org.gnome.shell.extensions.dash-to-dock running-indicator-style "DASHES"
    gsettings set org.gnome.shell.extensions.dash-to-dock preferred-monitor 0
    gsettings set org.gnome.shell.extensions.dash-to-dock show-favorites true
    gsettings set org.gnome.shell.extensions.dash-to-dock show-mounts true
    gsettings set org.gnome.shell.extensions.dash-to-dock show-trash false
    gsettings set org.gnome.shell.extensions.dash-to-dock transparency-mode "FIXED"
fi

gnome-extensions info system-monitor@paradoxxx.zero.gmail.com
if [ $? -eq 0 ]; then
    gnome-extensions enable system-monitor@paradoxxx.zero.gmail.com
fi

exit 0