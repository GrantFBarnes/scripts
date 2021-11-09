#!/usr/bin/env python3

from helper_functions import *
import os


def convert_group_to_groups(group):
    if group is None or group == "":
        return None
    return group.strip()[1:-1].split("][")


def find_group(path, plugin):
    full_path = os.path.expanduser("~/.config/" + path)
    if not os.path.exists(full_path):
        return None

    lastGroup = ""
    file = open(full_path, "r")
    for line in file:
        if line.startswith("["):
            lastGroup = line
        elif line.startswith("plugin"):
            if plugin in line:
                return lastGroup
    return None


def set_config(file, groups, key, value):
    if groups is None:
        return

    command = 'kwriteconfig5'
    command += ' --file "' + file + '"'
    for group in groups:
        command += ' --group "' + group + '"'
    command += ' --key "' + key + '"'
    command += ' ' + value
    run_command(command)


def main():

    # Configure Clock
    applet_path = "plasma-org.kde.plasma.desktop-appletsrc"
    clock_group = find_group(applet_path, "digitalclock")
    clock_groups = convert_group_to_groups(clock_group)
    set_config(applet_path, clock_groups, "dateFormat", '"isoDate"')
    set_config(applet_path, clock_groups, "showSeconds", 'true')

    # Set Night Color
    set_config("kwinrc", ["NightColor"], "Active", 'true')
    set_config("kwinrc", ["NightColor"], "Mode", '"Constant"')
    set_config("kwinrc", ["NightColor"], "NightTemperature", '2300')

    # Set NumLock
    set_config("kcminputrc", ["Keyboard"], "NumLock", '0')

    # Set File Click to Double
    set_config("kdeglobals", ["KDE"], "SingleClick", 'false')

    # Set Screen Lock Timeout
    set_config("kscreenlockerrc", ["Daemon"], "Timeout", '15')

    # Start Empty Session on Login
    set_config("ksmserverrc", ["General"], "loginMode", '"emptySession"')


if __name__ == "__main__":
    main()
