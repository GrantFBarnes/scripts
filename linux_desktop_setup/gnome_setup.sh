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

# Enable Night Shift
gsettings set org.gnome.settings-daemon.plugins.color night-light-enabled true
gsettings set org.gnome.settings-daemon.plugins.color night-light-schedule-automatic false
gsettings set org.gnome.settings-daemon.plugins.color night-light-temperature "uint32 2700"
gsettings set org.gnome.settings-daemon.plugins.color night-light-schedule-from "4.0"
gsettings set org.gnome.settings-daemon.plugins.color night-light-schedule-to "3.9"

# Add WM Buttons
gsettings set org.gnome.desktop.wm.preferences button-layout ":minimize,maximize,close"

# Enable Num Lock
gsettings set org.gnome.desktop.peripherals.keyboard numlock-state true

# Set Nautilus Default View to List
gsettings set org.gnome.nautilus.preferences default-folder-viewer "list-view"

# Set gedit color
gsettings set org.gnome.gedit.preferences.editor scheme "oblivion"

# Set up Touchpad/Mouse
gsettings set org.gnome.desktop.peripherals.touchpad tap-to-click true
gsettings set org.gnome.desktop.peripherals.touchpad natural-scroll false
gsettings set org.gnome.desktop.peripherals.mouse natural-scroll false

# Set up keybindings
gsettings set org.gnome.settings-daemon.plugins.media-keys calculator "['<Super>c']"
gsettings set org.gnome.settings-daemon.plugins.media-keys terminal "['<Primary><Alt>t','<Super>Return']"
gsettings set org.gnome.settings-daemon.plugins.media-keys www "['<Super>b']"

# Set up Notifications
gsettings set org.gnome.desktop.notifications.application:/org/gnome/desktop/notifications/application/org-gnome-software/ enable false

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

exit 0