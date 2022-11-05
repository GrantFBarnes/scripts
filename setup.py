#!/usr/bin/env python3

from __future__ import annotations
from helpers.helper_functions import *
import os


def main():
    distribution: Distribution | None = get_distribution()
    if distribution is None:
        print_error("Distribution not recognized", True)
        exit()

    if os.geteuid() == 0:
        distribution.update()
        distribution.install_pip()

    run_command("python3 -m pip install -r helpers/requirements.txt")


if __name__ == "__main__":
    main()
