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
    get_command(command)


def main():

    applet_path = "plasma-org.kde.plasma.desktop-appletsrc"

    # Configure Clock
    clock_group = find_group(applet_path, "org.kde.plasma.digitalclock")
    clock_groups = convert_group_to_groups(clock_group)
    if clock_groups is not None:
        clock_groups.append("Configuration")
        clock_groups.append("Appearance")
    set_config(applet_path, clock_groups, "dateFormat", '"isoDate"')
    set_config(applet_path, clock_groups, "showSeconds", 'true')

    # Show Battery Percentage
    battery_group = find_group(applet_path, "org.kde.plasma.battery")
    battery_groups = convert_group_to_groups(battery_group)
    if battery_groups is not None:
        battery_groups.append("Configuration")
        battery_groups.append("General")
    set_config(applet_path, battery_groups, "showPercentage", 'true')

    # Set Night Color
    set_config("kwinrc", ["NightColor"], "Active", 'true')
    set_config("kwinrc", ["NightColor"], "Mode", '"Times"')
    set_config("kwinrc", ["NightColor"], "MorningBeginFixed", '0700')
    set_config("kwinrc", ["NightColor"], "EveningBeginFixed", '1900')
    set_config("kwinrc", ["NightColor"], "NightTemperature", '2300')

    # Set NumLock
    set_config("kcminputrc", ["Keyboard"], "NumLock", '0')

    # Set File Click to Double
    set_config("kdeglobals", ["KDE"], "SingleClick", 'false')

    # Set Dolphin to always open home
    set_config("dolphinrc", ["General"], "RememberOpenedTabs", 'false')

    # Set Screen Lock Timeout
    set_config("kscreenlockerrc", ["Daemon"], "Timeout", '15')

    # Start Empty Session on Login
    set_config("ksmserverrc", ["General"], "loginMode", '"emptySession"')

    # Setup Kate
    set_config("katerc", ["General"], "Show Full Path in Title", 'true')
    set_config("katerc", ["General"], "Show Menu Bar", 'true')
    set_config("katerc", ["KTextEditor Renderer"],
               "Show Indentation Lines", 'true')
    set_config("katerc", ["KTextEditor Renderer"],
               "Show Whole Bracket Expression", 'true')
    set_config("katerc", ["KTextEditor Document"], "Show Spaces", '1')
    set_config("katerc", ["KTextEditor View"], "Scroll Past End", 'true')
    set_config("katerc", ["KTextEditor View"], "Show Line Count", 'true')
    set_config("katerc", ["KTextEditor View"], "Show Word Count", 'true')
    set_config("katerc", ["KTextEditor View"], "Line Numbers", 'true')
    set_config("katerc", ["KTextEditor View"], "Smart Copy Cut", 'true')
    set_config("katerc", ["KTextEditor View"], "Input Mode", '1')
    set_config("katerc", ["KTextEditor View"],
               "Vi Input Mode Steal Keys", 'false')


if __name__ == "__main__":
    main()
