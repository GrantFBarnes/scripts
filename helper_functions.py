import os
import subprocess

# ANSI Escape Sequences
ansi_reset: str = "\033[0m"
ansi_bold: str = "\033[1m"
ansi_red: str = "\033[31m"
ansi_green: str = "\033[32m"
ansi_yellow: str = "\033[33m"
ansi_cyan: str = "\033[36m"
ansi_red_bg: str = "\033[41m"


# Print Functions

def print_message(message: str, include_separators: bool, label: str, color: str) -> None:
    if include_separators:
        print(f"{color}--------------------------------------------------{ansi_reset}")
    print(f"{ansi_bold}{color}{label}{ansi_reset} {message}")
    if include_separators:
        print(f"{color}--------------------------------------------------{ansi_reset}")


def print_success(message: str, include_separators: bool) -> None:
    print_message(message, include_separators, "Success:", ansi_green)


def print_info(message: str, include_separators: bool) -> None:
    print_message(message, include_separators, "Info:", ansi_cyan)


def print_warning(message: str, include_separators: bool) -> None:
    print_message(message, include_separators, "Warning:", ansi_yellow)


def print_error(message: str, include_separators: bool) -> None:
    print_message(message, include_separators, "Error:", ansi_red)


# Command Line Functions

def has_command(command: str) -> bool:
    return os.system("command -v " + command + " >/dev/null 2>&1") == 0


def run_command(command: str) -> None:
    subprocess.run(["bash", "-c", command])


def get_command(command: str) -> str:
    try:
        return subprocess.check_output(["bash", "-c", command]).decode("utf-8").strip()
    except:
        return ""


# File Functions

def add_to_file_if_not_found(file_path: str, search_str: str, new_line: str) -> None:
    if os.path.isfile(file_path):
        with open(file_path, "r+") as file:
            for line in file:
                if search_str in line:
                    break
            else:
                file.write(new_line)
    else:
        with open(file_path, "w") as file:
            file.write(new_line)
