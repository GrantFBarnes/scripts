#!/usr/bin/env python3

from __future__ import annotations
from helpers.helper_functions import *
import os


def set_alias():
    bashrc = os.path.expanduser("~") + "/.bashrc"
    add_to_file_if_not_found(bashrc, "alias ov=", 'alias ov="python3 ' + os.path.abspath(__file__) + '"\n')


def get_distro():
    for file_name in ["/etc/lsb-release", "/usr/lib/os-release", "/etc/os-release"]:
        if os.path.isfile(file_name):
            file = open(file_name, "r")
            for line in file:
                if line.startswith('PRETTY_NAME="') or line.startswith('DISTRIB_DESCRIPTION="'):
                    return line.split('"')[1]
    return "(Unknown)"


def get_cpu():
    file = open("/proc/cpuinfo", "r")
    for line in file:
        if line.startswith("model name"):
            return line.split(": ")[1].strip()
    return "(Unknown)"


def get_cpu_speed():
    speeds = []
    speed = 0
    max_speed = 0

    file = open("/proc/cpuinfo", "r")
    for line in file:
        if line.startswith("cpu MHz"):
            speeds.append(float(line.split(": ")[1].strip()))
    if len(speeds) != 0:
        speed = (sum(speeds) / len(speeds)) / 1000

    cpu_dir = "/sys/devices/system/cpu/cpu0/cpufreq/"
    for file_name in [cpu_dir + "bios_limit", cpu_dir + "cpuinfo_max_freq", cpu_dir + "scaling_max_freq"]:
        if os.path.isfile(file_name):
            file = open(file_name, "r")
            for line in file:
                if line[0].isdigit():
                    max_speed = int(line) / 1000 / 1000
                    break
        if max_speed != 0:
            break

    result = ""
    if speed != 0:
        result += "{:.3f}".format(speed) + " GHz"

    if max_speed != 0:
        if speed != 0:
            result += " / "
        result += "{:.3f}".format(max_speed) + " GHz"
        percent = int((speed / max_speed) * 100)
        if percent != 0:
            result += " (" + str(percent) + "%)"
            if percent < 25:
                result = f"{ansi_green}" + result + f"{ansi_reset}"
            elif percent < 50:
                result = f"{ansi_yellow}" + result + f"{ansi_reset}"
            elif percent < 75:
                result = f"{ansi_red}" + result + f"{ansi_reset}"
            else:
                result = f"{ansi_red_bg}" + result + f"{ansi_reset}"

    return result


def get_memory():
    file = open("/proc/meminfo", "r")
    total = 0
    available = 0
    for line in file:
        if line.startswith("MemTotal"):
            total = int(line.split(" ")[-2]) // 1024
        if line.startswith("MemAvailable"):
            available = int(line.split(" ")[-2]) // 1024
        if total and available:
            break

    used = total - available

    result = ""
    if used != 0:
        result += str(used) + " MB"

    if total != 0:
        if used != 0:
            result += " / "
        result += str(total) + " MB"
        percent = (used * 100) // total
        if percent != 0:
            result += " (" + str(percent) + "%)"
            if percent < 25:
                result = f"{ansi_green}" + result + f"{ansi_reset}"
            elif percent < 50:
                result = f"{ansi_yellow}" + result + f"{ansi_reset}"
            elif percent < 75:
                result = f"{ansi_red}" + result + f"{ansi_reset}"
            else:
                result = f"{ansi_red_bg}" + result + f"{ansi_reset}"

    return result


def get_uptime():
    if "SUSE" in get_distro():
        return "NA"
    boot = int(get_command('date -d"$(uptime -s)" +%s'))
    now = int(get_command("date +%s"))
    uptime_count = now - boot

    seconds_count = int(uptime_count % 60)
    minutes_count = int(uptime_count / 60 % 60)
    hours_count = int(uptime_count / 60 / 60 % 24)
    days_count = int(uptime_count / 60 / 60 / 24 % 365)
    years_count = int(uptime_count / 60 / 60 / 24 / 365)

    seconds = str(seconds_count) + " seconds"
    minutes = str(minutes_count) + " minutes"
    hours = str(hours_count) + " hours"
    days = str(days_count) + " days"
    years = str(years_count) + " years"

    if seconds_count == 1:
        seconds = seconds[:-1]
    if minutes_count == 1:
        minutes = minutes[:-1]
    if hours_count == 1:
        hours = hours[:-1]
    if days_count == 1:
        days = days[:-1]
    if years_count == 1:
        years = years[:-1]

    uptime = ""
    if years_count != 0:
        uptime += years + ", "
    if days_count != 0:
        uptime += days + ", "
    if hours_count != 0:
        uptime += hours + ", "
    if minutes_count != 0:
        uptime += minutes + ", "
    uptime += seconds
    return uptime


def get_packages():
    dpkg = "0"
    pacman = "0"
    rpm = "0"

    snap = "0"
    flatpak = "0"

    if has_command("dpkg"):
        dpkg = get_command("dpkg --list | wc -l")
    if has_command("pacman"):
        pacman = get_command("pacman -Q | wc -l")
    if has_command("rpm"):
        rpm = get_command("rpm -qa | wc -l")

    if has_command("snap"):
        snap = get_command("snap list | wc -l")
    if has_command("flatpak"):
        flatpak = get_command("flatpak list | wc -l")

    packages = ""
    if dpkg != "0":
        packages += dpkg + " (dpkg), "
    if pacman != "0":
        packages += pacman + " (pacman), "
    if rpm != "0":
        packages += rpm + " (rpm), "

    if snap != "0":
        packages += snap + " (snap), "
    if flatpak != "0":
        packages += flatpak + " (flatpak), "

    packages = packages[:-2]
    return packages


def main():
    set_alias()

    print(f"{ansi_cyan}-------------------------------{ansi_reset}")
    print(f"{ansi_bold}{ansi_cyan}    User{ansi_reset}: " + get_command("echo $USER"))
    print(f"{ansi_bold}{ansi_cyan}Hostname{ansi_reset}: " + get_command("echo $HOSTNAME"))
    print(f"{ansi_bold}{ansi_cyan}  Distro{ansi_reset}: " + get_distro())
    print(f"{ansi_bold}{ansi_cyan}  Kernel{ansi_reset}: " + get_command("uname -rm"))
    print(f"{ansi_bold}{ansi_cyan}     CPU{ansi_reset}: " + get_cpu())
    print(f"{ansi_bold}{ansi_cyan}   Speed{ansi_reset}: " + get_cpu_speed())
    print(f"{ansi_bold}{ansi_cyan}  Memory{ansi_reset}: " + get_memory())
    print(f"{ansi_bold}{ansi_cyan}  Uptime{ansi_reset}: " + get_uptime())
    print(f"{ansi_bold}{ansi_cyan}Packages{ansi_reset}: " + get_packages())
    print(f"{ansi_cyan}-------------------------------{ansi_reset}")


if __name__ == "__main__":
    main()
