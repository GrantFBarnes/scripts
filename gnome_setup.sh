#!/bin/bash
# Purpose: Setup Gnome Desktop
################################################################################
cd $(dirname "$0")
. helper_functions.sh

distro=$(getDistribution)

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

# Enable Animations
gsettings set org.gnome.desktop.interface enable-animations true

# Set Blank Screen to 15 min (900 seconds)
gsettings set org.gnome.desktop.session idle-delay 900

# Enable Night Shift
gsettings set org.gnome.settings-daemon.plugins.color night-light-enabled true
gsettings set org.gnome.settings-daemon.plugins.color night-light-schedule-automatic false
gsettings set org.gnome.settings-daemon.plugins.color night-light-temperature "uint32 2700"
gsettings set org.gnome.settings-daemon.plugins.color night-light-schedule-from "19"
gsettings set org.gnome.settings-daemon.plugins.color night-light-schedule-to "7"

# Set WM Buttons
gsettings set org.gnome.desktop.wm.preferences button-layout ":close"
if [ "$distro" == "ubuntu" ]; then
    gsettings set org.gnome.desktop.wm.preferences button-layout ":minimize,maximize,close"
fi

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
    gsettings set org.gnome.shell.extensions.ding show-home false

    screenDimensions=$(xdpyinfo | awk '/dimensions/{print $2}')
    screenWidth=$(cut -d "x" -f 1 <<<$screenDimensions)
    screenSize="large"
    if [ $screenWidth \< 1800 ]; then
        screenSize="medium"
    elif [ $screenWidth \< 1300 ]; then
        screenSize="small"
    fi

    if [ $screenSize == "small" ]; then
        gsettings set org.gnome.shell.extensions.dash-to-dock dock-fixed false
    else
        gsettings set org.gnome.shell.extensions.dash-to-dock dock-fixed true
    fi
    gsettings set org.gnome.shell.extensions.dash-to-dock dash-max-icon-size 28
    gsettings set org.gnome.shell.extensions.dash-to-dock show-favorites true
    gsettings set org.gnome.shell.extensions.dash-to-dock show-mounts true
    gsettings set org.gnome.shell.extensions.dash-to-dock show-trash false
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
    'bitwarden_bitwarden.desktop',
    'gnucash.desktop',
    'org.gnucash.GnuCash.desktop',
    'org.gnome.PasswordSafe.desktop',
    'org.gnome.Calendar.desktop',
    'org.gnome.clocks.desktop',
    'org.gnome.Weather.desktop',
    'org.gnome.Maps.desktop',
    'org.gnome.Books.desktop',
    'org.gnome.Boxes.desktop',
    'org.gnome.Connections.desktop',
    'virt-manager.desktop',
    'org.gnome.Cheese.desktop',
    'usb-creator-gtk.desktop',
    'org.fedoraproject.MediaWriter.desktop'
]"

gsettings set ${APP_FOLDERS_PATH}Internet/ name "Internet"
gsettings set ${APP_FOLDERS_PATH}Internet/ apps "[
    'chromium_chromium.desktop',
    'chromium-browser.desktop',
    'chromium.desktop',
    'org.chromium.Chromium.desktop',
    'icecat.desktop',
    'firefox.desktop',
    'firefox_firefox.desktop',
    'firefox-esr.desktop',
    'org.mozilla.firefox.desktop',
    'brave_brave.desktop',
    'torbrowser.desktop',
    'com.github.micahflee.torbrowser-launcher.desktop',
    'org.gnome.Epiphany.desktop',
    'thunderbird.desktop',
    'thunderbird_thunderbird.desktop',
    'mozilla-thunderbird.desktop',
    'org.mozilla.Thunderbird.desktop',
    'transmission-gtk.desktop',
    'transmission-qt.desktop'
]"

gsettings set ${APP_FOLDERS_PATH}Editors/ name "Editors"
gsettings set ${APP_FOLDERS_PATH}Editors/ apps "[
    'org.gnome.gedit.desktop',
    'org.gnome.TextEditor.desktop',
    'code_code.desktop',
    'com.vscodium.codium.desktop',
    'org.kde.kwrite.desktop',
    'org.kde.kate.desktop',
    'org.gnome.Builder.desktop',
    'intellij-idea-community_intellij-idea-community.desktop',
    'pycharm-community_pycharm-community.desktop',
    'org.kde.kile.desktop',
    'org.texstudio.TeXstudio.desktop',
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
    'org.gnome.eog.desktop',
    'org.kde.gwenview.desktop'
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
    'org.kde.kmines.desktop',
    'org.kde.knights.desktop',
    'org.kde.ksudoku.desktop',
    '0ad.desktop',
    'supertuxkart.desktop',
    'xonotic.desktop'
]"

gsettings set ${APP_FOLDERS_PATH}System/ name "System"
gsettings set ${APP_FOLDERS_PATH}System/ apps "[
    'org.gnome.Nautilus.desktop',
    'org.kde.dolphin.desktop',
    'org.gnome.Terminal.desktop',
    'org.gnome.Console.desktop',
    'org.kde.konsole.desktop',
    'gnome-system-monitor.desktop',
    'org.gnome.baobab.desktop',
    'org.gnome.DiskUtility.desktop',
    'org.kde.ksysguard.desktop',
    'org.kde.filelight.desktop',
    'org.kde.partitionmanager.desktop',
    'org.kde.plasma-systemmonitor.desktop',
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
    'org.kde.discover.desktop',
    'synaptic.desktop',
    'snap-store_ubuntu-software.desktop',
    'software-properties-gnome.desktop',
    'software-properties-gtk.desktop',
    'update-manager.desktop',
    'software-properties-livepatch.desktop',
    'gnome-session-properties.desktop',
    'kdesystemsettings.desktop',
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
    'org.gnome.Tour.desktop',
    'org.gnome.PowerStats.desktop',
    'org.gnome.Logs.desktop',
    'org.gnome.FileRoller.desktop',
    'org.kde.okular.desktop',
    'org.kde.ark.desktop',
    'org.kde.kcalc.desktop',
    'org.kde.spectacle.desktop',
    'system-config-printer.desktop',
    'setroubleshoot.desktop',
    'org.gnome.font-viewer.desktop',
    'org.gnome.Characters.desktop',
    'org.gnome.Firmware.desktop',
    'remote-viewer.desktop',
    'org.kde.kwalletmanager5.desktop',
    'org.kde.klipper.desktop',
    'org.kde.kdeconnect.app.desktop',
    'org.kde.kdeconnect.nonplasma.desktop',
    'org.kde.kdeconnect.settings.desktop',
    'org.kde.kdeconnect-settings.desktop',
    'org.kde.kdeconnect.sms.desktop',
    'texdoctk.desktop',
    'yelp.desktop',
    'org.freedesktop.GnomeAbrt.desktop',
    'im-config.desktop',
    'nm-connection-editor.desktop',
    'gnome-language-selector.desktop',
    'display-im6.q16.desktop',
    'avahi-discover.desktop',
    'bssh.desktop',
    'bvnc.desktop',
    'lstopo.desktop',
    'qv412.desktop',
    'qvidcap.desktop',
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
    'firefox.desktop',
    'firefox_firefox.desktop',
    'firefox-esr.desktop',
    'org.mozilla.firefox.desktop',
    'thunderbird.desktop',
    'thunderbird_thunderbird.desktop',
    'mozilla-thunderbird.desktop',
    'org.mozilla.Thunderbird.desktop',
    'org.gnome.gedit.desktop',
    'org.gnome.Terminal.desktop'
]"

exit 0
