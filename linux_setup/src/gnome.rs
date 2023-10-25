use std::process::Command;

use crate::distribution::{Distribution, DistributionName};

fn settings_set<S>(path: S, key: S, value: String)
where
    S: Into<String>,
{
    let _ = Command::new("gsettings")
        .arg("set")
        .arg(path.into())
        .arg(key.into())
        .arg(value)
        .status()
        .expect("failed to set gnome settings");
}

fn settings_reset<S>(path: S)
where
    S: Into<String>,
{
    let _ = Command::new("gsettings")
        .arg("reset-recursively")
        .arg(path.into())
        .status()
        .expect("failed to reset gnome settings");
}

pub fn setup(distribution: &Distribution) {
    // Setup Clock
    settings_set(
        "org.gnome.desktop.interface",
        "clock-format",
        format!("\"{}\"", "12h"),
    );
    settings_set(
        "org.gnome.desktop.interface",
        "clock-show-date",
        true.to_string(),
    );
    settings_set(
        "org.gnome.desktop.interface",
        "clock-show-seconds",
        true.to_string(),
    );
    settings_set(
        "org.gnome.desktop.interface",
        "clock-show-weekday",
        true.to_string(),
    );

    // Show Battery Percentage
    settings_set(
        "org.gnome.desktop.interface",
        "show-battery-percentage",
        true.to_string(),
    );

    // Enable Overview Hot Corner
    settings_set(
        "org.gnome.desktop.interface",
        "enable-hot-corners",
        true.to_string(),
    );

    // Set Blank Screen to 15 min (900 seconds)
    settings_set(
        "org.gnome.desktop.session",
        "idle-delay",
        format!("{}", 900),
    );

    // Enable Num Lock
    settings_set(
        "org.gnome.desktop.peripherals.keyboard",
        "numlock-state",
        true.to_string(),
    );

    // Set up Touchpad/Mouse
    settings_set(
        "org.gnome.desktop.peripherals.touchpad",
        "tap-to-click",
        true.to_string(),
    );
    settings_set(
        "org.gnome.desktop.peripherals.touchpad",
        "natural-scroll",
        false.to_string(),
    );
    settings_set(
        "org.gnome.desktop.peripherals.mouse",
        "natural-scroll",
        false.to_string(),
    );

    // Set Ubuntu Settings
    if distribution.name == DistributionName::Ubuntu {
        settings_set(
            "org.gnome.shell.extensions.ding",
            "show-home",
            false.to_string(),
        );

        settings_set(
            "org.gnome.shell.extensions.dash-to-dock",
            "dash-max-icon-size",
            format!("{}", 28),
        );
        settings_set(
            "org.gnome.shell.extensions.dash-to-dock",
            "show-favorites",
            true.to_string(),
        );
        settings_set(
            "org.gnome.shell.extensions.dash-to-dock",
            "show-mounts",
            true.to_string(),
        );
        settings_set(
            "org.gnome.shell.extensions.dash-to-dock",
            "show-trash",
            false.to_string(),
        );
    }

    // Set App Folders

    const APP_FOLDERS: &str = "org.gnome.desktop.app-folders";
    const APP_FOLDERS_PATH: &str =
        "org.gnome.desktop.app-folders.folder:/org/gnome/desktop/app-folders/folders/";

    settings_reset(APP_FOLDERS);
    settings_reset(APP_FOLDERS_PATH);

    settings_set(
        APP_FOLDERS,
        "folder-children",
        "[
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
        .to_owned(),
    );

    settings_set(
        format!("{}Apps/", APP_FOLDERS_PATH),
        "name".to_string(),
        "Apps".to_string(),
    );
    settings_set(
        format!("{}Apps/", APP_FOLDERS_PATH),
        "apps".to_string(),
        "[
            'bitwarden.desktop',
            'bitwarden_bitwarden.desktop',
            'gnucash.desktop',
            'org.gnucash.GnuCash.desktop',
            'org.gnome.PasswordSafe.desktop',
            'org.gnome.World.Secrets.desktop',
            'org.gnome.Calendar.desktop',
            'org.gnome.Contacts.desktop',
            'org.gnome.clocks.desktop',
            'org.gnome.Weather.desktop',
            'org.gnome.Maps.desktop',
            'org.remmina.Remmina.desktop',
            'org.gnome.Connections.desktop',
            'org.gnome.Boxes.desktop',
            'virt-manager.desktop',
            'org.gnome.Cheese.desktop',
            'org.gnome.Todo.desktop',
            'usb-creator-gtk.desktop',
            'org.fedoraproject.MediaWriter.desktop'
        ]"
        .to_string(),
    );

    settings_set(
        format!("{}Internet/", APP_FOLDERS_PATH),
        "name".to_string(),
        "Internet".to_string(),
    );
    settings_set(
        format!("{}Internet/", APP_FOLDERS_PATH),
        "apps".to_string(),
        "[
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
            'discord.desktop',
            'discord_discord.desktop',
            'com.transmissionbt.Transmission.desktop',
            'transmission-gtk.desktop',
            'transmission-qt.desktop'
        ]"
        .to_string(),
    );

    settings_set(
        format!("{}Editors/", APP_FOLDERS_PATH),
        "name".to_string(),
        "Editors".to_string(),
    );
    settings_set(
        format!("{}Editors/", APP_FOLDERS_PATH),
        "apps".to_string(),
        "[
            'org.gnome.gedit.desktop',
            'org.gnome.TextEditor.desktop',
            'code.desktop',
            'code_code.desktop',
            'com.visualstudio.code.desktop',
            'org.kde.kwrite.desktop',
            'org.kde.kate.desktop',
            'org.gnome.Builder.desktop',
            'jetbrains-fleet.desktop',
            'jetbrains-idea-ce.desktop',
            'idea.desktop',
            'intellij-idea-community_intellij-idea-community.desktop',
            'com.jetbrains.IntelliJ-IDEA-Community.desktop',
            'jetbrains-pycharm-ce.desktop',
            'pycharm.desktop',
            'pycharm-community.desktop',
            'pycharm-community_pycharm-community.desktop',
            'com.jetbrains.PyCharm-Community.desktop',
            'jetbrains-toolbox.desktop',
            'org.kde.kile.desktop',
            'texstudio.desktop',
            'org.texstudio.TeXstudio.desktop',
            'org.gnome.gitg.desktop',
            'vim.desktop'
        ]"
        .to_string(),
    );

    settings_set(
        format!("{}Office/", APP_FOLDERS_PATH),
        "name".to_string(),
        "Office".to_string(),
    );
    settings_set(
        format!("{}Office/", APP_FOLDERS_PATH),
        "apps".to_string(),
        "[
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
        .to_string(),
    );

    settings_set(
        format!("{}MultiMedia/", APP_FOLDERS_PATH),
        "name".to_string(),
        "Multi Media".to_string(),
    );
    settings_set(
        format!("{}MultiMedia/", APP_FOLDERS_PATH),
        "apps".to_string(),
        "[
            'blender.desktop',
            'blender_blender.desktop',
            'org.blender.Blender.desktop',
            'gimp.desktop',
            'org.gimp.GIMP.desktop',
            'org.kde.kdenlive.desktop',
            'rhythmbox.desktop',
            'org.gnome.Rhythmbox3.desktop',
            'org.gnome.Music.desktop',
            'org.kde.elisa.desktop',
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
        .to_string(),
    );

    settings_set(
        format!("{}Games/", APP_FOLDERS_PATH),
        "name".to_string(),
        "Games".to_string(),
    );
    settings_set(
        format!("{}Games/", APP_FOLDERS_PATH),
        "apps".to_string(),
        "[
            'com.valvesoftware.Steam.desktop',
            'steam_steam.desktop',
            'sol.desktop',
            'org.gnome.Aisleriot.desktop',
            'org.gnome.Chess.desktop',
            'org.gnome.TwentyFortyEight.desktop',
            'org.gnome.Sudoku.desktop',
            'org.gnome.Mines.desktop',
            'org.gnome.Quadrapassel.desktop',
            'org.gnome.Mahjongg.desktop',
            'org.kde.kmines.desktop',
            'org.kde.knights.desktop',
            'org.kde.ksudoku.desktop',
            '0ad.desktop',
            'supertuxkart.desktop',
            'xonotic.desktop',
            'xonotic-glx.desktop',
            'xonotic-sdl.desktop'
        ]"
        .to_string(),
    );

    settings_set(
        format!("{}System/", APP_FOLDERS_PATH),
        "name".to_string(),
        "System".to_string(),
    );
    settings_set(
        format!("{}System/", APP_FOLDERS_PATH),
        "apps".to_string(),
        "[
            'org.gnome.Nautilus.desktop',
            'org.kde.dolphin.desktop',
            'org.gnome.Terminal.desktop',
            'org.gnome.Console.desktop',
            'org.kde.konsole.desktop',
            'gnome-system-monitor.desktop',
            'org.gnome.baobab.desktop',
            'org.gnome.DiskUtility.desktop',
            'gparted.desktop',
            'org.kde.ksysguard.desktop',
            'org.kde.filelight.desktop',
            'org.kde.partitionmanager.desktop',
            'org.kde.plasma-systemmonitor.desktop',
            'org.gnome.DejaDup.desktop',
            'org.gnome.seahorse.Application.desktop',
            'htop.desktop'
        ]"
        .to_string(),
    );

    settings_set(
        format!("{}Settings/", APP_FOLDERS_PATH),
        "name".to_string(),
        "Settings".to_string(),
    );
    settings_set(
        format!("{}Settings/", APP_FOLDERS_PATH),
        "apps".to_string(),
        "[
            'org.gnome.Settings.desktop',
            'gnome-control-center.desktop',
            'org.gnome.tweaks.desktop',
            'org.gnome.Extensions.desktop',
            'com.mattjakeman.ExtensionManager.desktop',
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
        .to_string(),
    );

    settings_set(
        format!("{}Utilities/", APP_FOLDERS_PATH),
        "name".to_string(),
        "Utilities".to_string(),
    );
    settings_set(
        format!("{}Utilities/", APP_FOLDERS_PATH),
        "apps".to_string(),
        "[
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
            'cups.desktop',
            'electron18.desktop',
            'electron19.desktop',
            'electron.desktop',
            'flutter_openurl.desktop',
            'setroubleshoot.desktop',
            'org.cockpit_project.CockpitClient.desktop',
            'org.gnome.font-viewer.desktop',
            'org.gnome.Characters.desktop',
            'org.gnome.Firmware.desktop',
            'org.gnome.Shotwell-Profile-Browser.desktop',
            'remote-viewer.desktop',
            'org.kde.kwalletmanager5.desktop',
            'org.kde.klipper.desktop',
            'org.kde.kdeconnect.app.desktop',
            'org.kde.kdeconnect.nonplasma.desktop',
            'org.kde.kdeconnect.settings.desktop',
            'org.kde.kdeconnect-settings.desktop',
            'org.kde.kdeconnect.sms.desktop',
            'assistant.desktop',
            'designer.desktop',
            'linguist.desktop',
            'qdbusviewer.desktop',
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
            'jconsole-java11-openjdk.desktop',
            'jshell-java11-openjdk.desktop',
            'qv4l2.desktop',
            'org.kde.kuserfeedback-console.desktop',
            'xdvi.desktop',
            'org.kde.drkonqi.coredump.gui.desktop',
            'torbrowser-settings.desktop',
            'com.github.micahflee.torbrowser-launcher.settings.desktop'
        ]"
        .to_string(),
    );

    // Set App Folder Layout
    settings_set(
        "org.gnome.shell",
        "app-picker-layout",
        "[
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
        .to_string(),
    );

    // Set Favorites
    settings_set(
        "org.gnome.shell",
        "favorite-apps",
        "[
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
            'org.gnome.TextEditor.desktop',
            'org.gnome.Terminal.desktop'
        ]"
        .to_string(),
    );
}
