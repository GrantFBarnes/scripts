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
    if len(speeds) != 0:
        speed = (sum(speeds) / len(speeds)) / 1000

    sDir = "/sys/devices/system/cpu/cpu0/cpufreq/"
    for fileName in [sDir + "bios_limit", sDir + "cpuinfo_max_freq", sDir + "scaling_max_freq"]:
        if os.path.isfile(fileName):
            file = open(fileName, "r")
            for line in file:
                if line[0].isdigit():
                    maxSpeed = int(line) / 1000 / 1000
                    break
        if maxSpeed != 0:
            break

    result = ""
    if speed != 0:
        result += "{:.3f}".format(speed) + " GHz"

    if maxSpeed != 0:
        if speed != 0:
            result += " / "
        result += "{:.3f}".format(maxSpeed) + " GHz"
        percent = int((speed / maxSpeed) * 100)
        if percent != 0:
            result += " (" + str(percent) + "%)"

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

    return result


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


def main():
    bold = "\033[1m"
    cyan = "\033[36m"
    reset = "\033[0m"

    print(f"{cyan}-------------------------------{reset}")
    print(f"{bold}{cyan}    User{reset}: " + run_command("echo $USER"))
    print(f"{bold}{cyan}Hostname{reset}: " + run_command("echo $HOSTNAME"))
    print(f"{bold}{cyan}  Distro{reset}: " + get_distro())
    print(f"{bold}{cyan}  Kernel{reset}: " + run_command("uname -rm"))
    print(f"{bold}{cyan}     CPU{reset}: " + get_cpu())
    print(f"{bold}{cyan}   Speed{reset}: " + get_cpu_speed())
    print(f"{bold}{cyan}  Memory{reset}: " + get_memory())
    print(f"{bold}{cyan}  Uptime{reset}: " + get_uptime())
    print(f"{bold}{cyan}Packages{reset}: " + get_packages())
    print(f"{cyan}-------------------------------{reset}")


if __name__ == "__main__":
    main()
