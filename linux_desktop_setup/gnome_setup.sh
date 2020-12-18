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

screenDimensions=$(xdpyinfo | awk '/dimensions/{print $2}')
screenWidth=$(cut -d "x" -f 1 <<< $screenDimensions)
screenSize="large"
if [ $screenWidth \< 1800 ]; then
    screenSize="medium"
elif [ $screenWidth \< 1300 ]; then
    screenSize="small"
fi

gsettings set org.gnome.shell.extensions.desktop-icons show-home false
gsettings set org.gnome.shell.extensions.desktop-icons show-trash false

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
    if [ $screenSize == "small" ]; then
        gsettings set org.gnome.shell.extensions.dash-to-dock dock-fixed false
    else
        gsettings set org.gnome.shell.extensions.dash-to-dock dock-fixed true
    fi
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

    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor center-display false
    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor compact-display true
    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor icon-display false

    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor cpu-display true
    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor cpu-show-menu true
    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor cpu-show-text false
    if [ $screenSize == "small" ]; then
        gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor cpu-style "graph"
        gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor cpu-graph-width 50
    elif [ $screenSize == "medium" ]; then
        gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor cpu-style "graph"
        gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor cpu-graph-width 100
    else
        gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor cpu-style "both"
        gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor cpu-graph-width 100
    fi

    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor disk-display true
    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor disk-show-menu true
    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor disk-show-text false
    if [ $screenSize == "small" ]; then
        gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor disk-style "graph"
        gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor disk-graph-width 50
    elif [ $screenSize == "medium" ]; then
        gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor disk-style "graph"
        gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor disk-graph-width 100
    else
        gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor disk-style "both"
        gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor disk-graph-width 100
    fi

    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor memory-display true
    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor memory-show-menu true
    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor memory-show-text false
    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor memory-buffer-color "#00000000"
    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor memory-cache-color "#00000000"
    if [ $screenSize == "small" ]; then
        gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor memory-style "graph"
        gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor memory-graph-width 50
    elif [ $screenSize == "medium" ]; then
        gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor memory-style "graph"
        gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor memory-graph-width 100
    else
        gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor memory-style "both"
        gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor memory-graph-width 100
    fi

    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor net-display true
    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor net-show-menu true
    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor net-show-text false
    if [ $screenSize == "small" ]; then
        gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor net-style "graph"
        gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor net-graph-width 50
    elif [ $screenSize == "medium" ]; then
        gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor net-style "graph"
        gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor net-graph-width 100
    else
        gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor net-style "both"
        gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor net-graph-width 100
    fi

    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor battery-display false
    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor battery-show-menu false
    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor battery-show-text false

    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor fan-display false
    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor fan-show-menu false
    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor fan-show-text false

    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor freq-display false
    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor freq-show-menu false
    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor freq-show-text false

    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor gpu-display false
    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor gpu-show-menu false
    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor gpu-show-text false

    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor swap-display false
    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor swap-show-menu false
    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor swap-show-text false

    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor thermal-display false
    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor thermal-show-menu false
    gsettings --schemadir /usr/share/gnome-shell/extensions/system-monitor@paradoxxx.zero.gmail.com/schemas set org.gnome.shell.extensions.system-monitor thermal-show-text false
fi

exit 0