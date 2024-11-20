use std::io;
use std::process::Command;

use crate::distribution::{Distribution, Repository};

fn settings_set(path: &str, key: &str, value: &str) -> Result<(), io::Error> {
    Command::new("gsettings")
        .arg("set")
        .arg(path)
        .arg(key)
        .arg(value)
        .status()?;
    Ok(())
}

fn settings_reset(path: &str) -> Result<(), io::Error> {
    Command::new("gsettings")
        .arg("reset-recursively")
        .arg(path)
        .status()?;
    Ok(())
}

pub fn setup(distribution: &Distribution) -> Result<(), io::Error> {
    // Setup Clock
    settings_set(
        "org.gnome.desktop.interface",
        "clock-format",
        format!("\"{}\"", "12h").as_str(),
    )?;
    settings_set(
        "org.gnome.desktop.interface",
        "clock-show-date",
        true.to_string().as_str(),
    )?;
    settings_set(
        "org.gnome.desktop.interface",
        "clock-show-seconds",
        true.to_string().as_str(),
    )?;
    settings_set(
        "org.gnome.desktop.interface",
        "clock-show-weekday",
        true.to_string().as_str(),
    )?;

    // Show Battery Percentage
    settings_set(
        "org.gnome.desktop.interface",
        "show-battery-percentage",
        true.to_string().as_str(),
    )?;

    // Enable Overview Hot Corner
    settings_set(
        "org.gnome.desktop.interface",
        "enable-hot-corners",
        true.to_string().as_str(),
    )?;

    // Set Blank Screen to 15 min (900 seconds)
    settings_set(
        "org.gnome.desktop.session",
        "idle-delay",
        format!("{}", 900).as_str(),
    )?;

    // Enable Num Lock
    settings_set(
        "org.gnome.desktop.peripherals.keyboard",
        "numlock-state",
        true.to_string().as_str(),
    )?;

    // Set up Touchpad/Mouse
    settings_set(
        "org.gnome.desktop.peripherals.touchpad",
        "tap-to-click",
        true.to_string().as_str(),
    )?;
    settings_set(
        "org.gnome.desktop.peripherals.touchpad",
        "natural-scroll",
        false.to_string().as_str(),
    )?;
    settings_set(
        "org.gnome.desktop.peripherals.mouse",
        "natural-scroll",
        false.to_string().as_str(),
    )?;

    // Set Ubuntu Settings
    if distribution.repository == Repository::Ubuntu {
        settings_set(
            "org.gnome.shell.extensions.ding",
            "show-home",
            false.to_string().as_str(),
        )?;

        settings_set(
            "org.gnome.shell.extensions.dash-to-dock",
            "dash-max-icon-size",
            format!("{}", 28).as_str(),
        )?;
        settings_set(
            "org.gnome.shell.extensions.dash-to-dock",
            "show-favorites",
            true.to_string().as_str(),
        )?;
        settings_set(
            "org.gnome.shell.extensions.dash-to-dock",
            "show-mounts",
            true.to_string().as_str(),
        )?;
        settings_set(
            "org.gnome.shell.extensions.dash-to-dock",
            "show-trash",
            false.to_string().as_str(),
        )?;
    }

    // Set App Folders

    const APP_FOLDERS: &str = "org.gnome.desktop.app-folders";
    const APP_FOLDERS_PATH: &str =
        "org.gnome.desktop.app-folders.folder:/org/gnome/desktop/app-folders/folders/";

    settings_reset(APP_FOLDERS)?;
    settings_reset(APP_FOLDERS_PATH)?;

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
        ]",
    )?;

    settings_set(
        format!("{}Apps/", APP_FOLDERS_PATH).as_str(),
        "name",
        "Apps",
    )?;
    settings_set(
        format!("{}Apps/", APP_FOLDERS_PATH).as_str(),
        "apps",
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
            'org.gnome.Snapshot.desktop',
            'org.gnome.Todo.desktop',
            'usb-creator-gtk.desktop',
            'org.fedoraproject.MediaWriter.desktop'
        ]",
    )?;

    settings_set(
        format!("{}Internet/", APP_FOLDERS_PATH).as_str(),
        "name",
        "Internet",
    )?;
    settings_set(
        format!("{}Internet/", APP_FOLDERS_PATH).as_str(),
        "apps",
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
            'org.mozilla.thunderbird.desktop',
            'mozilla-thunderbird.desktop',
            'org.mozilla.Thunderbird.desktop',
            'discord.desktop',
            'discord_discord.desktop',
            'com.discordapp.Discord.desktop',
            'com.transmissionbt.Transmission.desktop',
            'transmission-gtk.desktop',
            'transmission-qt.desktop'
        ]",
    )?;

    settings_set(
        format!("{}Editors/", APP_FOLDERS_PATH).as_str(),
        "name",
        "Editors",
    )?;
    settings_set(
        format!("{}Editors/", APP_FOLDERS_PATH).as_str(),
        "apps",
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
            'rider_rider.desktop',
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
            'nvim.desktop',
            'vim.desktop'
        ]",
    )?;

    settings_set(
        format!("{}Office/", APP_FOLDERS_PATH).as_str(),
        "name",
        "Office",
    )?;
    settings_set(
        format!("{}Office/", APP_FOLDERS_PATH).as_str(),
        "apps",
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
        ]",
    )?;

    settings_set(
        format!("{}MultiMedia/", APP_FOLDERS_PATH).as_str(),
        "name",
        "Multi Media",
    )?;
    settings_set(
        format!("{}MultiMedia/", APP_FOLDERS_PATH).as_str(),
        "apps",
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
            'org.gnome.Loupe.desktop',
            'eog.desktop',
            'org.gnome.eog.desktop',
            'org.kde.gwenview.desktop'
        ]",
    )?;

    settings_set(
        format!("{}Games/", APP_FOLDERS_PATH).as_str(),
        "name",
        "Games",
    )?;
    settings_set(
        format!("{}Games/", APP_FOLDERS_PATH).as_str(),
        "apps",
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
        ]",
    )?;

    settings_set(
        format!("{}System/", APP_FOLDERS_PATH).as_str(),
        "name",
        "System",
    )?;
    settings_set(
        format!("{}System/", APP_FOLDERS_PATH).as_str(),
        "apps",
        "[
            'org.gnome.Nautilus.desktop',
            'org.kde.dolphin.desktop',
            'org.gnome.Terminal.desktop',
            'org.gnome.Console.desktop',
            'org.kde.konsole.desktop',
            'org.gnome.SystemMonitor.desktop',
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
        ]",
    )?;

    settings_set(
        format!("{}Settings/", APP_FOLDERS_PATH).as_str(),
        "name",
        "Settings",
    )?;
    settings_set(
        format!("{}Settings/", APP_FOLDERS_PATH).as_str(),
        "apps",
        "[
            'org.gnome.Settings.desktop',
            'gnome-control-center.desktop',
            'org.gnome.tweaks.desktop',
            'org.gnome.Extensions.desktop',
            'com.mattjakeman.ExtensionManager.desktop',
            'ca.desrt.dconf-editor.desktop',
            'org.freedesktop.MalcontentControl.desktop',
            'snap-store_snap-store.desktop',
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
        ]",
    )?;

    settings_set(
        format!("{}Utilities/", APP_FOLDERS_PATH).as_str(),
        "name",
        "Utilities",
    )?;
    settings_set(
        format!("{}Utilities/", APP_FOLDERS_PATH).as_str(),
        "apps",
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
            'firmware-updater_firmware-updater.desktop',
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
        ]",
    )?;

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
        ]",
    )?;

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
        ]",
    )?;
    Ok(())
}
