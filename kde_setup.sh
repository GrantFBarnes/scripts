#!/bin/bash
# Purpose: Setup KDE Desktop
################################################################################
cd $(dirname "$0")
. helper_functions.sh

distro=$(getDistribution)

# Set Themes
lookandfeeltool -a org.kde.breezedark.desktop

# Set Favorite Applications
kwriteconfig5 --file "kactivitymanagerd-statsrc" --group "Favorites-org.kde.plasma.kickoff.favorites.instance-3-global" --key "ordering" "applications:org.kde.dolphin.desktop,applications:firefox-esr.desktop,applications:org.kde.kate.desktop,applications:org.kde.konsole.desktop"

# Set Pinned Applications
kwriteconfig5 --file "plasma-org.kde.plasma.desktop-appletsrc" --group "Containments" --group "2" --group "Applets" --group "5" --group "Configuration" --group "General" --key "launchers" "applications:org.kde.dolphin.desktop,applications:firefox-esr.desktop,applications:org.kde.kwrite.desktop,applications:org.kde.konsole.desktop"

# Set Clock Format
kwriteconfig5 --file "plasma-org.kde.plasma.desktop-appletsrc" --group "Containments" --group "2" --group "Applets" --group "16" --group "Configuration" --group "Appearance" --key "dateFormat" "isoDate"
kwriteconfig5 --file "plasma-org.kde.plasma.desktop-appletsrc" --group "Containments" --group "2" --group "Applets" --group "16" --group "Configuration" --group "Appearance" --key "showSeconds" true

# Set Night Color
kwriteconfig5 --file "kwinrc" --group "NightColor" --key "Active" true
kwriteconfig5 --file "kwinrc" --group "NightColor" --key "Mode" "Constant"
kwriteconfig5 --file "kwinrc" --group "NightColor" --key "NightTemperature" 2300

# Set NumLock
kwriteconfig5 --file "kcminputrc" --group "Keyboard" --key "NumLock" 0

# Set File Click to Double
kwriteconfig5 --file "kdeglobals" --group "KDE" --key "SingleClick" false

# Set Screen Lock Timeout
kwriteconfig5 --file "kscreenlockerrc" --group "Daemon" --key "Timeout" 15

# Start Empty Session on Login
kwriteconfig5 --file "ksmserverrc" --group "General" --key "loginMode" "emptySession"
