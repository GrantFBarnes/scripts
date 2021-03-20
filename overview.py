import os
import subprocess


def has_command(command):
    return os.system("command -v " + command + " >/dev/null 2>&1") == 0


def run_command(command):
    return subprocess.check_output(['bash', '-c', command]).decode("utf-8").strip()


def get_distro():
    for fileName in ["/etc/lsb-release", "/usr/lib/os-release", "/etc/os-release"]:
        if os.path.isfile(fileName):
            file = open(fileName, "r")
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
    maxSpeed = 0
    percent = 0

    file = open("/proc/cpuinfo", "r")
    for line in file:
        if line.startswith("cpu MHz"):
            speeds.append(float(line.split(": ")[1].strip()))
    speed = (sum(speeds) / len(speeds)) / 1000

    sDir = "/sys/devices/system/cpu/cpu0/cpufreq/"
    for fileName in [sDir + "bios_limit", sDir + "cpuinfo_max_freq", sDir + "scaling_max_freq"]:
        if os.path.isfile(fileName):
            file = open(fileName, "r")
            for line in file:
                if line[0].isdigit():
                    maxSpeed = int(line) / 1000 / 1000

    if speed != 0 and maxSpeed != 0:
        percent = int((speed / maxSpeed) * 100)
    return "{:.3f}".format(speed) + " GHz / " + "{:.3f}".format(maxSpeed) + " GHz (" + str(percent) + "%)"


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
    percent = (used * 100) // total
    return str(used) + " MB / " + str(total) + " MB (" + str(percent) + "%)"


def get_uptime():
    boot = int(run_command('date -d"$(uptime -s)" +%s'))
    now = int(run_command("date +%s"))
    uptimeSeconds = now - boot

    secondsCount = int(uptimeSeconds % 60)
    minutesCount = int(uptimeSeconds / 60 % 60)
    hoursCount = int(uptimeSeconds / 60 / 60 % 24)
    daysCount = int(uptimeSeconds / 60 / 60 / 24 % 365)
    yearsCount = int(uptimeSeconds / 60 / 60 / 24 / 365)

    seconds = str(secondsCount) + " seconds"
    minutes = str(minutesCount) + " minutes"
    hours = str(hoursCount) + " hours"
    days = str(daysCount) + " days"
    years = str(yearsCount) + " years"

    if secondsCount == 1:
        seconds = seconds[:-1]
    if minutesCount == 1:
        minutes = minutes[:-1]
    if hoursCount == 1:
        hours = hours[:-1]
    if daysCount == 1:
        days = days[:-1]
    if yearsCount == 1:
        years = years[:-1]

    uptime = ""
    if yearsCount != 0:
        uptime += years + ", "
    if daysCount != 0:
        uptime += days + ", "
    if hoursCount != 0:
        uptime += hours + ", "
    if minutesCount != 0:
        uptime += minutes + ", "
    uptime += seconds
    return uptime


def get_packages():
    dpkg = "0"
    dnf = "0"
    pacman = "0"

    snap = "0"
    flatpak = "0"

    if has_command("dpkg"):
        dpkg = run_command("dpkg --list | wc -l")
    if has_command("dnf"):
        dnf = run_command("dnf list installed | wc -l")
    if has_command("pacman"):
        pacman = run_command("pacman -Q | wc -l")

    if has_command("snap"):
        snap = run_command("snap list | wc -l")
    if has_command("flatpak"):
        flatpak = run_command("flatpak list | wc -l")

    packages = ""
    if dpkg != "0":
        packages += dpkg + " (dpkg), "
    if dnf != "0":
        packages += dnf + " (dnf), "
    if pacman != "0":
        packages += pacman + " (pacman), "

    if snap != "0":
        packages += snap + " (snap), "
    if flatpak != "0":
        packages += flatpak + " (flatpak), "

    packages = packages[:-2]
    return packages

blue = '\033[94m'
stop = '\033[0m'
bold = '\033[1m'

print(f"{blue}-------------------------------{stop}")
print(f"{bold}{blue}    User{stop}: " + run_command("echo $USER"))
print(f"{bold}{blue}Hostname{stop}: " + run_command("echo $HOSTNAME"))
print(f"{bold}{blue}  Distro{stop}: " + get_distro())
print(f"{bold}{blue}  Kernel{stop}: " + run_command("uname -rm"))
print(f"{bold}{blue}     CPU{stop}: " + get_cpu())
print(f"{bold}{blue}   Speed{stop}: " + get_cpu_speed())
print(f"{bold}{blue}  Memory{stop}: " + get_memory())
print(f"{bold}{blue}  Uptime{stop}: " + get_uptime())
print(f"{bold}{blue}Packages{stop}: " + get_packages())
print(f"{blue}-------------------------------{stop}")
