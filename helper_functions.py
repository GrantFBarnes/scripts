import os
import subprocess


def has_command(command):
    return os.system("command -v " + command + " >/dev/null 2>&1") == 0


def run_command(command):
    return subprocess.check_output(['bash', '-c', command]).decode("utf-8").strip()
