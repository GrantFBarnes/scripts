#!/usr/bin/env python3

from __future__ import annotations
from helpers.helper_functions import *
import os


def main():
    if os.geteuid() != 0:
        print_error("Must be run as root", True)
        exit()

    distribution: Distribution | None = get_distribution()
    if distribution is None:
        print_error("Distribution not recognized", True)
        exit()

    distribution.repository_update()
    distribution.install_pip()

    run_command("python3 -m pip install -r helpers/requirements.txt")


if __name__ == "__main__":
    main()
