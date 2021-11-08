#!/bin/bash
# Purpose: Setup KDE Desktop
################################################################################
cd $(dirname "$0")
. helper_functions.sh

distro=$(getDistribution)

# Set Themes
lookandfeeltool -a org.kde.breezedark.desktop
