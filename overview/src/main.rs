use std::env::VarError;
use std::io::Result;
use std::num::ParseFloatError;
use std::process::{Command, Output};
use std::str::Split;
use std::string::FromUtf8Error;
use std::{env, fs};

const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";

fn get_command_output(mut command: Command) -> Option<String> {
    let output: Result<Output> = command.output();
    if output.is_ok() {
        let output: Output = output.unwrap();
        let output: Vec<u8> = output.stdout;
        let output: std::result::Result<String, FromUtf8Error> = String::from_utf8(output);
        if output.is_ok() {
            return Option::from(output.unwrap());
        }
    }
    None
}

fn has_command(command: &str) -> bool {
    match Command::new(command).arg("--version").output() {
        Ok(_) => true,
        _ => false,
    }
}

fn get_file_lines(file: &str) -> Option<Vec<String>> {
    let content: Result<String> = fs::read_to_string(file);
    if content.is_ok() {
        let content: String = content.unwrap();
        let mut lines: Vec<String> = vec![];
        for line in content.split("\n") {
            lines.push(line.to_string());
        }
        return Option::from(lines);
    }
    None
}

fn get_user() -> String {
    let home_dir: std::result::Result<String, VarError> = env::var("HOME");
    if home_dir.is_ok() {
        let home_dir: String = home_dir.unwrap();
        let user: Option<(&str, &str)> = home_dir.rsplit_once("/");
        if user.is_some() {
            let user: (&str, &str) = user.unwrap();
            return user.1.to_string();
        }
    }
    String::from("(Unknown)")
}

fn get_hostname() -> String {
    let hostname: Result<String> = fs::read_to_string("/etc/hostname");
    if hostname.is_ok() {
        return hostname.unwrap().trim().to_string();
    }
    String::from("(Unknown)")
}

fn get_os() -> String {
    const FILES: [&str; 3] = ["/etc/lsb-release", "/usr/lib/os-release", "/etc/os-release"];
    for file in FILES {
        let lines: Option<Vec<String>> = get_file_lines(file);
        if lines.is_some() {
            let lines: Vec<String> = lines.unwrap();
            for line in lines {
                if line.starts_with("PRETTY_NAME") || line.starts_with("DISTRIB_DESCRIPTION") {
                    let line_split: Split<&str> = line.split("\"");
                    let line_split: Vec<&str> = line_split.collect::<Vec<&str>>();
                    if line_split.len() > 1 {
                        return line_split[1].trim().to_string();
                    }
                }
            }
        }
    }
    String::from("(Unknown)")
}

fn get_kernel() -> String {
    let mut cmd: Command = Command::new("uname");
    cmd.arg("-r");
    let output: Option<String> = get_command_output(cmd);
    if output.is_some() {
        let output: String = output.unwrap();
        return output.trim().to_string();
    }
    String::from("(Unknown)")
}

fn get_cpu() -> String {
    let lines: Option<Vec<String>> = get_file_lines("/proc/cpuinfo");
    if lines.is_some() {
        let lines: Vec<String> = lines.unwrap();
        for line in lines {
            if line.starts_with("model name") {
                let line_split: Split<&str> = line.split(": ");
                let line_split: Vec<&str> = line_split.collect::<Vec<&str>>();
                if line_split.len() > 1 {
                    return line_split[1].trim().to_string();
                }
            }
        }
    }
    String::from("(Unknown)")
}

fn get_cpu_speed() -> String {
    let mut cpu_speeds: Vec<f64> = vec![];
    let mut current_speed: f64 = 0.0;
    let mut max_speed: f64 = 0.0;

    let lines: Option<Vec<String>> = get_file_lines("/proc/cpuinfo");
    if lines.is_some() {
        let lines: Vec<String> = lines.unwrap();
        for line in lines {
            if line.starts_with("cpu MHz") {
                let line_split: Split<&str> = line.split(": ");
                let line_split: Vec<&str> = line_split.collect::<Vec<&str>>();
                if line_split.len() > 1 {
                    let speed: std::result::Result<f64, ParseFloatError> =
                        line_split[1].trim().parse::<f64>();
                    if speed.is_ok() {
                        let speed: f64 = speed.unwrap();
                        cpu_speeds.push(speed);
                    }
                }
            }
        }
    }
    if cpu_speeds.len() > 0 {
        let sum: f64 = cpu_speeds.iter().sum::<f64>();
        current_speed = (sum / cpu_speeds.len() as f64) / 1000.0;
    }

    const FILES: [&str; 3] = [
        "/sys/devices/system/cpu/cpu0/cpufreq/bios_limit",
        "/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq",
        "/sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq",
    ];
    for file in FILES {
        let lines: Option<Vec<String>> = get_file_lines(file);
        if lines.is_some() {
            let lines: Vec<String> = lines.unwrap();
            for line in lines {
                let first_char: Option<char> = line.chars().nth(0);
                if first_char.is_some() {
                    let first_char: char = first_char.unwrap();
                    if first_char.is_numeric() {
                        let speed: std::result::Result<f64, ParseFloatError> =
                            line.trim().parse::<f64>();
                        if speed.is_ok() {
                            let speed: f64 = speed.unwrap();
                            max_speed = speed / 1000.0 / 1000.0;
                        }
                    }
                }
            }
        }
    }

    let mut result: String = String::new();
    if current_speed != 0.0 {
        result.push_str(&format!("{:.1} GHz", current_speed));
    }

    if max_speed != 0.0 {
        if current_speed != 0.0 {
            result.push_str(" / ");
        }
        result.push_str(&format!("{} GHz", max_speed));

        let percentage: f64 = (current_speed / max_speed) * 100.0;
        if percentage > 0.0 {
            result.push_str(&format!(" ({:.0}%)", percentage));

            let mut color: &str = GREEN;
            if percentage > 75.0 {
                color = RED;
            } else if percentage > 50.0 {
                color = YELLOW;
            } else if percentage > 25.0 {
                color = RESET;
            }

            result = format!("{}{}{}", color, result, RESET);
        }
    }

    result
}

fn get_memory() -> String {
    let mut total_memory: f64 = 0.0;
    let mut available_memory: f64 = 0.0;

    let lines: Option<Vec<String>> = get_file_lines("/proc/meminfo");
    if lines.is_some() {
        let lines: Vec<String> = lines.unwrap();
        for line in lines {
            if line.starts_with("MemTotal") || line.starts_with("MemAvailable") {
                let line_split: Split<&str> = line.split(" ");
                let line_split: Vec<&str> = line_split.collect::<Vec<&str>>();
                if line_split.len() > 2 {
                    let amount: std::result::Result<f64, ParseFloatError> =
                        line_split[line_split.len() - 2].trim().parse::<f64>();
                    if amount.is_ok() {
                        let amount: f64 = amount.unwrap();
                        if line.starts_with("MemTotal") {
                            total_memory = amount;
                        } else if line.starts_with("MemAvailable") {
                            available_memory = amount;
                        }
                    }
                }
            }
        }
    }

    let used_memory: f64 = total_memory - available_memory;
    let percentage: f64 = (used_memory / total_memory) * 100.0;

    let mut color: &str = GREEN;
    if percentage > 75.0 {
        color = RED;
    } else if percentage > 50.0 {
        color = YELLOW;
    } else if percentage > 25.0 {
        color = RESET;
    }

    format!(
        "{}{:.2}/{:.2} GB ({:.0}%){}",
        color,
        used_memory / 1024.0 / 1024.0,
        total_memory / 1024.0 / 1024.0,
        percentage,
        RESET
    )
}

fn get_uptime() -> String {
    let mut cmd: Command = Command::new("uptime");
    cmd.arg("-p");
    let output: Option<String> = get_command_output(cmd);
    if output.is_some() {
        let output: String = output.unwrap();
        return output.trim().to_string();
    }
    String::from("(Unknown)")
}

fn get_packages() -> String {
    let mut dpkg: usize = 0;
    let mut pacman: usize = 0;
    let mut rpm: usize = 0;

    let mut flatpak: usize = 0;
    let mut snap: usize = 0;

    if has_command("dpkg") {
        let mut cmd: Command = Command::new("dpkg");
        cmd.arg("--list");
        let output: Option<String> = get_command_output(cmd);
        if output.is_some() {
            let output: String = output.unwrap();
            let pkgs: Split<&str> = output.split("\n");
            let pkgs: Vec<&str> = pkgs.collect::<Vec<&str>>();
            dpkg = pkgs.len() - 1;
        }
    }
    if has_command("pacman") {
        let mut cmd: Command = Command::new("pacman");
        cmd.arg("-Q");
        let output: Option<String> = get_command_output(cmd);
        if output.is_some() {
            let output: String = output.unwrap();
            let pkgs: Split<&str> = output.split("\n");
            let pkgs: Vec<&str> = pkgs.collect::<Vec<&str>>();
            pacman = pkgs.len() - 1;
        }
    }
    if has_command("rpm") {
        let mut cmd: Command = Command::new("rpm");
        cmd.arg("-qa");
        let output: Option<String> = get_command_output(cmd);
        if output.is_some() {
            let output: String = output.unwrap();
            let pkgs: Split<&str> = output.split("\n");
            let pkgs: Vec<&str> = pkgs.collect::<Vec<&str>>();
            rpm = pkgs.len() - 1;
        }
    }

    if has_command("flatpak") {
        let mut cmd: Command = Command::new("flatpak");
        cmd.arg("list");
        let output: Option<String> = get_command_output(cmd);
        if output.is_some() {
            let output: String = output.unwrap();
            let pkgs: Split<&str> = output.split("\n");
            let pkgs: Vec<&str> = pkgs.collect::<Vec<&str>>();
            flatpak = pkgs.len() - 1;
        }
    }
    if has_command("snap") {
        let mut cmd: Command = Command::new("snap");
        cmd.arg("list");
        let output: Option<String> = get_command_output(cmd);
        if output.is_some() {
            let output: String = output.unwrap();
            let pkgs: Split<&str> = output.split("\n");
            let pkgs: Vec<&str> = pkgs.collect::<Vec<&str>>();
            snap = pkgs.len() - 1;
        }
    }

    let mut packages: String = String::new();
    if dpkg > 0 {
        packages.push_str(&format!("{} (dpkg), ", dpkg));
    }
    if pacman > 0 {
        packages.push_str(&format!("{} (pacman), ", pacman));
    }
    if rpm > 0 {
        packages.push_str(&format!("{} (rpm), ", rpm));
    }

    if flatpak > 0 {
        packages.push_str(&format!("{} (flatpak), ", flatpak));
    }
    if snap > 0 {
        packages.push_str(&format!("{} (snap), ", snap));
    }

    packages[..packages.len() - 2].to_string()
}

fn main() {
    println!("{}-------------------------------{}", CYAN, RESET);
    println!("{}    User{}: {}", CYAN, RESET, get_user());
    println!("{}Hostname{}: {}", CYAN, RESET, get_hostname());
    println!("{}      OS{}: {}", CYAN, RESET, get_os());
    println!("{}  Kernel{}: {}", CYAN, RESET, get_kernel());
    println!("{}     CPU{}: {}", CYAN, RESET, get_cpu());
    println!("{}   Speed{}: {}", CYAN, RESET, get_cpu_speed());
    println!("{}  Memory{}: {}", CYAN, RESET, get_memory());
    println!("{}  Uptime{}: {}", CYAN, RESET, get_uptime());
    println!("{}Packages{}: {}", CYAN, RESET, get_packages());
    println!("{}-------------------------------{}", CYAN, RESET);
}
