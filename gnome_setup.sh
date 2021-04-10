#!/bin/bash
# Purpose: Setup environment of GNU/Linux Desktop
################################################################################
cd $(dirname "$0")
. helper_functions.sh

distro=$(getDistrobution)

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
gsettings set org.gnome.settings-daemon.plugins.media-keys www "['<Super>b']"

# Set up Notifications
gsettings set org.gnome.desktop.notifications.application:/org/gnome/desktop/notifications/application/org-gnome-software/ enable false

# Set Gnome extensions
if [ "$distro" == "ubuntu" ]; then
    gsettings set org.gnome.shell.extensions.desktop-icons show-home false
    gsettings set org.gnome.shell.extensions.desktop-icons show-trash false
fi

# Set Favorites
gsettings set org.gnome.shell favorite-apps "['org.gnome.Nautilus.desktop', 'icecat.desktop', 'firefox.desktop', 'mozilla-thunderbird.desktop', 'com.vscodium.codium.desktop', 'org.gnome.gedit.desktop', 'org.gnome.Terminal.desktop']"

# Set App Folders
gsettings reset-recursively org.gnome.desktop.app-folders
gsettings set org.gnome.desktop.app-folders folder-children "['Apps', 'Internet', 'Editors', 'Office', 'MultiMedia', 'Games', 'System', 'Settings', 'Utilities']"

gsettings set org.gnome.desktop.app-folders.folder:/org/gnome/desktop/app-folders/folders/Apps/ name "Apps"
gsettings set org.gnome.desktop.app-folders.folder:/org/gnome/desktop/app-folders/folders/Internet/ name "Internet"
gsettings set org.gnome.desktop.app-folders.folder:/org/gnome/desktop/app-folders/folders/Editors/ name "Editors"
gsettings set org.gnome.desktop.app-folders.folder:/org/gnome/desktop/app-folders/folders/Office/ name "Office"
gsettings set org.gnome.desktop.app-folders.folder:/org/gnome/desktop/app-folders/folders/MultiMedia/ name "Multi Media"
gsettings set org.gnome.desktop.app-folders.folder:/org/gnome/desktop/app-folders/folders/Games/ name "Games"
gsettings set org.gnome.desktop.app-folders.folder:/org/gnome/desktop/app-folders/folders/System/ name "System"
gsettings set org.gnome.desktop.app-folders.folder:/org/gnome/desktop/app-folders/folders/Settings/ name "Settings"
gsettings set org.gnome.desktop.app-folders.folder:/org/gnome/desktop/app-folders/folders/Utilities/ name "Utilities"

gsettings set org.gnome.desktop.app-folders.folder:/org/gnome/desktop/app-folders/folders/Apps/ apps "['org.gnome.Boxes.desktop', 'virtualbox.desktop', 'gnucash.desktop', 'org.gnome.Calendar.desktop', 'org.gnome.clocks.desktop', 'org.gnome.Weather.desktop', 'com.github.johnfactotum.Foliate.desktop', 'org.gnome.Maps.desktop', 'org.gnome.Cheese.desktop', 'gramps.desktop']"
gsettings set org.gnome.desktop.app-folders.folder:/org/gnome/desktop/app-folders/folders/Internet/ apps "['icecat.desktop', 'firefox.desktop', 'torbrowser.desktop', 'mozilla-thunderbird.desktop', 'transmission-gtk.desktop']"
gsettings set org.gnome.desktop.app-folders.folder:/org/gnome/desktop/app-folders/folders/Editors/ apps "['org.gnome.gedit.desktop', 'com.vscodium.codium.desktop', 'com.jetbrains.PyCharm-Community.desktop', 'org.texstudio.TeXstudio.desktop']"
gsettings set org.gnome.desktop.app-folders.folder:/org/gnome/desktop/app-folders/folders/Office/ apps "['libreoffice-writer.desktop', 'libreoffice-calc.desktop', 'libreoffice-impress.desktop']"
gsettings set org.gnome.desktop.app-folders.folder:/org/gnome/desktop/app-folders/folders/MultiMedia/ apps "['blender.desktop', 'gimp.desktop', 'rhythmbox.desktop', 'org.gnome.Photos.desktop', 'org.gnome.Totem.desktop', 'vlc.desktop', 'org.gnome.eog.desktop']"
gsettings set org.gnome.desktop.app-folders.folder:/org/gnome/desktop/app-folders/folders/Games/ apps "['sol.desktop', 'org.gnome.Chess.desktop', 'org.gnome.Sudoku.desktop']"
gsettings set org.gnome.desktop.app-folders.folder:/org/gnome/desktop/app-folders/folders/System/ apps "['org.gnome.Nautilus.desktop', 'org.gnome.Terminal.desktop', 'gnome-system-monitor.desktop', 'org.gnome.baobab.desktop', 'org.gnome.DiskUtility.desktop', 'timeshift-gtk.desktop', 'org.gnome.DejaDup.desktop', 'htop.desktop']"
gsettings set org.gnome.desktop.app-folders.folder:/org/gnome/desktop/app-folders/folders/Settings/ apps "['gnome-control-center.desktop', 'org.gnome.tweaks.desktop', 'org.gnome.Software.desktop', 'ca.desrt.dconf-editor.desktop']"
gsettings set org.gnome.desktop.app-folders.folder:/org/gnome/desktop/app-folders/folders/Utilities/ apps "['org.gnome.Calculator.desktop', 'simple-scan.desktop', 'org.gnome.Evince.desktop', 'org.gnome.Screenshot.desktop', 'org.gnome.Logs.desktop', 'org.gnome.FileRoller.desktop', 'org.gnome.font-viewer.desktop', 'org.gnome.Characters.desktop', 'yelp.desktop', 'org.freedesktop.GnomeAbrt.desktop', 'torbrowser-settings.desktop']"

# Set Layout
gsettings set org.gnome.shell app-picker-layout "[{'Apps': <{'position': <0>}>, 'Internet': <{'position': <1>}>, 'Editors': <{'position': <2>}>, 'Office': <{'position': <3>}>, 'MultiMedia': <{'position': <4>}>, 'Games': <{'position': <5>}>, 'System': <{'position': <6>}>, 'Settings': <{'position': <7>}>, 'Utilities': <{'position': <8>}>}]"

exit 0