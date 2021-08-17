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
gsettings set org.gnome.clocks world-clocks "[
    {'location': <(uint32 2, <('Honolulu', 'PHNL', true, [(0.37223509621909062, -2.7566263578617853)], [(0.37187632633805073, -2.7551476625596174)])>)>},
    {'location': <(uint32 2, <('Los Angeles', 'KCQT', true, [(0.59370283970450188, -2.0644336110828618)], [(0.59432360095955872, -2.063741622941031)])>)>},
    {'location': <(uint32 2, <('Denver', 'KBKF', true, [(0.69307024596694822, -1.8283729951886007)], [(0.69357907925707463, -1.8323287315783685)])>)>},
    {'location': <(uint32 2, <('Rochester', 'KRST', true, [(0.76627226949544125, -1.6142841198081861)], [(0.76832240304800381, -1.6139041965366119)])>)>},
    {'location': <(uint32 2, <('New York City', 'KNYC', false, [(0.71180344078725644, -1.2909618758762367)], @a(dd) [])>)>},
    {'location': <(uint32 2, <('Oslo', 'ENGM', true, [(1.0506882097005865, 0.19344065294494067)], [(1.0457431159710333, 0.18762289458939041)])>)>},
    {'location': <(uint32 2, <('Ho Chi Minh City', 'VVTS', true, [(0.18878645324181748, 1.8616845412783825)], [(0.18762289458939041, 1.8616845412783825)])>)>}
]"

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

# Set up gedit
gsettings set org.gnome.gedit.preferences.editor scheme "oblivion"
gsettings set org.gnome.gedit.preferences.editor background-pattern "grid"
gsettings set org.gnome.gedit.preferences.editor bracket-matching true
gsettings set org.gnome.gedit.preferences.editor display-line-numbers true
gsettings set org.gnome.gedit.preferences.editor display-right-margin true
gsettings set org.gnome.gedit.preferences.editor right-margin-position 80
gsettings set org.gnome.gedit.preferences.editor search-highlighting true
gsettings set org.gnome.gedit.preferences.editor insert-spaces true
gsettings set org.gnome.gedit.preferences.editor tabs-size 4

# Set up Touchpad/Mouse
gsettings set org.gnome.desktop.peripherals.touchpad tap-to-click true
gsettings set org.gnome.desktop.peripherals.touchpad natural-scroll false
gsettings set org.gnome.desktop.peripherals.mouse natural-scroll false

# Set up keybindings
gsettings set org.gnome.settings-daemon.plugins.media-keys calculator "['<Super>c']"
gsettings set org.gnome.settings-daemon.plugins.media-keys www "['<Super>b']"

# Set up Notifications
gsettings set org.gnome.desktop.notifications.application:/org/gnome/desktop/notifications/application/org-gnome-software/ enable false
gsettings set org.gnome.desktop.notifications.application:/org/gnome/desktop/notifications/application/org-gnome-dejadup/ enable false

# Set Gnome extensions
if [ "$distro" == "ubuntu" ]; then
    gsettings set org.gnome.shell.extensions.desktop-icons show-home false
    gsettings set org.gnome.shell.extensions.desktop-icons show-trash false

    screenDimensions=$(xdpyinfo | awk '/dimensions/{print $2}')
    screenWidth=$(cut -d "x" -f 1 <<<$screenDimensions)
    screenSize="large"
    if [ $screenWidth \< 1800 ]; then
        screenSize="medium"
    elif [ $screenWidth \< 1300 ]; then
        screenSize="small"
    fi

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

# Set App Folders
APP_FOLDERS=org.gnome.desktop.app-folders
APP_FOLDERS_PATH=${APP_FOLDERS}.folder:/org/gnome/desktop/app-folders/folders/

gsettings reset-recursively ${APP_FOLDERS}
gsettings reset-recursively ${APP_FOLDERS_PATH}

gsettings set ${APP_FOLDERS} folder-children "[
    'Apps',
    'Internet',
    'Editors',
    'Office',
    'MultiMedia',
    'Games',
    'System',
    'Settings',
    'Utilities'
]"

gsettings set ${APP_FOLDERS_PATH}Apps/ name "Apps"
gsettings set ${APP_FOLDERS_PATH}Apps/ apps "[
    'org.gnome.Boxes.desktop',
    'virtualbox.desktop',
    'gnucash.desktop',
    'org.gnucash.GnuCash.desktop',
    'bitwarden_bitwarden.desktop',
    'org.gnome.Calendar.desktop',
    'org.gnome.clocks.desktop',
    'org.gnome.Weather.desktop',
    'org.gnome.Contacts.desktop',
    'org.gnome.Books.desktop',
    'calibre-gui.desktop',
    'calibre-ebook-edit.desktop',
    'calibre-ebook-viewer.desktop',
    'calibre-lrfviewer.desktop',
    'foliate_foliate.desktop',
    'com.github.johnfactotum.Foliate.desktop',
    'org.gnome.Maps.desktop',
    'org.gnome.Cheese.desktop',
    'usb-creator-gtk.desktop',
    'gramps.desktop'
]"

gsettings set ${APP_FOLDERS_PATH}Internet/ name "Internet"
gsettings set ${APP_FOLDERS_PATH}Internet/ apps "[
    'chromium_chromium.desktop',
    'chromium-browser.desktop',
    'chromium.desktop',
    'org.chromium.Chromium.desktop',
    'icecat.desktop',
    'firefox.desktop',
    'firefox-esr.desktop',
    'org.mozilla.firefox.desktop',
    'torbrowser.desktop',
    'com.github.micahflee.torbrowser-launcher.desktop',
    'org.gnome.Epiphany.desktop',
    'thunderbird.desktop',
    'mozilla-thunderbird.desktop',
    'org.mozilla.Thunderbird.desktop',
    'transmission-gtk.desktop'
]"

gsettings set ${APP_FOLDERS_PATH}Editors/ name "Editors"
gsettings set ${APP_FOLDERS_PATH}Editors/ apps "[
    'org.gnome.gedit.desktop',
    'code_code.desktop',
    'com.vscodium.codium.desktop',
    'com.jetbrains.PyCharm-Community.desktop',
    'pycharm-community_pycharm-community.desktop',
    'org.texstudio.TeXstudio.desktop',
    'org.gnome.meld.desktop',
    'vim.desktop'
]"

gsettings set ${APP_FOLDERS_PATH}Office/ name "Office"
gsettings set ${APP_FOLDERS_PATH}Office/ apps "[
    'libreoffice-writer.desktop',
    'libreoffice_writer.desktop',
    'org.libreoffice.LibreOffice.writer.desktop',
    'libreoffice-calc.desktop',
    'libreoffice_calc.desktop',
    'org.libreoffice.LibreOffice.calc.desktop',
    'libreoffice-impress.desktop',
    'libreoffice_impress.desktop',
    'org.libreoffice.LibreOffice.impress.desktop',
    'libreoffice-draw.desktop',
    'libreoffice_draw.desktop',
    'org.libreoffice.LibreOffice.draw.desktop',
    'libreoffice-math.desktop',
    'libreoffice_math.desktop',
    'org.libreoffice.LibreOffice.math.desktop',
    'libreoffice-base.desktop',
    'libreoffice_base.desktop',
    'org.libreoffice.LibreOffice.base.desktop',
    'libreoffice_libreoffice.desktop',
    'org.libreoffice.LibreOffice.desktop',
    'libreoffice-startcenter.desktop'
]"
gsettings set ${APP_FOLDERS_PATH}MultiMedia/ name "Multi Media"
gsettings set ${APP_FOLDERS_PATH}MultiMedia/ apps "[
    'blender.desktop',
    'blender_blender.desktop',
    'org.blender.Blender.desktop',
    'gimp.desktop',
    'org.gimp.GIMP.desktop',
    'org.kde.kdenlive.desktop',
    'rhythmbox.desktop',
    'org.gnome.Rhythmbox3.desktop',
    'org.gnome.Music.desktop',
    'org.gnome.Photos.desktop',
    'shotwell.desktop',
    'org.gnome.Shotwell.desktop',
    'org.gnome.Totem.desktop',
    'vlc.desktop',
    'vlc_vlc.desktop',
    'org.videolan.VLC.desktop',
    'org.gnome.SoundRecorder.desktop',
    'eog.desktop',
    'org.gnome.eog.desktop'
]"

gsettings set ${APP_FOLDERS_PATH}Games/ name "Games"
gsettings set ${APP_FOLDERS_PATH}Games/ apps "[
    'sol.desktop',
    'org.gnome.Aisleriot.desktop',
    'org.gnome.Chess.desktop',
    'org.gnome.TwentyFortyEight.desktop',
    'org.gnome.Sudoku.desktop',
    'org.gnome.Mines.desktop',
    'org.gnome.Quadrapassel.desktop',
    'org.gnome.Reversi.desktop',
    'org.gnome.SwellFoop.desktop',
    'org.gnome.Taquin.desktop',
    'org.gnome.LightsOff.desktop'
]"

gsettings set ${APP_FOLDERS_PATH}System/ name "System"
gsettings set ${APP_FOLDERS_PATH}System/ apps "[
    'org.gnome.Nautilus.desktop',
    'org.gnome.Terminal.desktop',
    'gnome-system-monitor.desktop',
    'org.gnome.baobab.desktop',
    'org.gnome.DiskUtility.desktop',
    'timeshift-gtk.desktop',
    'org.gnome.DejaDup.desktop',
    'htop.desktop'
]"

gsettings set ${APP_FOLDERS_PATH}Settings/ name "Settings"
gsettings set ${APP_FOLDERS_PATH}Settings/ apps "[
    'gnome-control-center.desktop',
    'org.gnome.tweaks.desktop',
    'org.gnome.Extensions.desktop',
    'ca.desrt.dconf-editor.desktop',
    'org.freedesktop.MalcontentControl.desktop',
    'org.gnome.Software.desktop',
    'synaptic.desktop',
    'snap-store_ubuntu-software.desktop',
    'software-properties-gnome.desktop',
    'software-properties-gtk.desktop',
    'update-manager.desktop',
    'software-properties-livepatch.desktop',
    'gnome-session-properties.desktop',
    'software-properties-drivers.desktop'
]"

gsettings set ${APP_FOLDERS_PATH}Utilities/ name "Utilities"
gsettings set ${APP_FOLDERS_PATH}Utilities/ apps "[
    'org.gnome.Calculator.desktop',
    'simple-scan.desktop',
    'evince.desktop',
    'org.gnome.Evince.desktop',
    'org.gnome.Documents.desktop',
    'org.gnome.Screenshot.desktop',
    'org.gnome.PowerStats.desktop',
    'org.gnome.Logs.desktop',
    'org.gnome.FileRoller.desktop',
    'setroubleshoot.desktop',
    'org.gnome.font-viewer.desktop',
    'org.gnome.Characters.desktop',
    'org.gnome.Firmware.desktop',
    'yelp.desktop',
    'org.freedesktop.GnomeAbrt.desktop',
    'im-config.desktop',
    'nm-connection-editor.desktop',
    'gnome-language-selector.desktop',
    'display-im6.q16.desktop',
    'torbrowser-settings.desktop',
    'com.github.micahflee.torbrowser-launcher.settings.desktop'
]"

# Set Layout
gsettings set org.gnome.shell app-picker-layout "[
    {
        'Apps': <{'position': <0>}>,
        'Internet': <{'position': <1>}>,
        'Editors': <{'position': <2>}>,
        'Office': <{'position': <3>}>,
        'MultiMedia': <{'position': <4>}>,
        'Games': <{'position': <5>}>,
        'System': <{'position': <6>}>,
        'Settings': <{'position': <7>}>,
        'Utilities': <{'position': <8>}>
    }
]"

# Set Favorites
gsettings set org.gnome.shell favorite-apps "[
    'org.gnome.Nautilus.desktop',
    'icecat.desktop',
    'firefox.desktop',
    'firefox-esr.desktop',
    'org.mozilla.firefox.desktop',
    'thunderbird.desktop',
    'mozilla-thunderbird.desktop',
    'org.mozilla.Thunderbird.desktop',
    'code_code.desktop',
    'com.vscodium.codium.desktop',
    'org.gnome.gedit.desktop',
    'org.gnome.Terminal.desktop',
    'gnucash.desktop',
    'bitwarden_bitwarden.desktop'
]"

exit 0
