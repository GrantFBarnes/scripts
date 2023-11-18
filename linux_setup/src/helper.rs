use std::fs;
use std::io;
use std::process::Command;
use std::string::FromUtf8Error;

const ANSI_RESET: &str = "\x1b[0m";

const ANSI_BLACK: &str = "\x1b[30m";
const ANSI_RED: &str = "\x1b[31m";
const ANSI_GREEN: &str = "\x1b[32m";
const ANSI_YELLOW: &str = "\x1b[33m";
const ANSI_BLUE: &str = "\x1b[34m";
const ANSI_MAGENTA: &str = "\x1b[35m";
const ANSI_CYAN: &str = "\x1b[36m";
const ANSI_WHITE: &str = "\x1b[37m";

pub fn get_colored_string<S: Into<String>>(string: S, color: &str) -> String {
    match color {
        "black" => format!("{}{}{}", ANSI_BLACK, string.into(), ANSI_RESET),
        "red" => format!("{}{}{}", ANSI_RED, string.into(), ANSI_RESET),
        "green" => format!("{}{}{}", ANSI_GREEN, string.into(), ANSI_RESET),
        "yellow" => format!("{}{}{}", ANSI_YELLOW, string.into(), ANSI_RESET),
        "blue" => format!("{}{}{}", ANSI_BLUE, string.into(), ANSI_RESET),
        "magenta" => format!("{}{}{}", ANSI_MAGENTA, string.into(), ANSI_RESET),
        "cyan" => format!("{}{}{}", ANSI_CYAN, string.into(), ANSI_RESET),
        "white" => format!("{}{}{}", ANSI_WHITE, string.into(), ANSI_RESET),
        _ => string.into(),
    }
}

pub fn get_command_output(mut command: Command) -> Result<String, io::Error> {
    let output: Vec<u8> = command.output()?.stdout;
    let output: Result<String, FromUtf8Error> = String::from_utf8(output);
    if output.is_err() {
        return Err(io::Error::other("failed to convert output to string"));
    }
    Ok(output.unwrap())
}

fn append_to_file(path: &String, value: &str, sudo: bool) -> Result<(), io::Error> {
    if sudo {
        Command::new("sudo")
            .arg("sh")
            .arg("-c")
            .arg(format!("echo \'{}\' >> {}", value, path))
            .spawn()?;
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(format!("echo \'{}\' >> {}", value, path))
            .spawn()?;
    }
    Ok(())
}

pub fn append_to_file_if_not_found(
    path: &String,
    find_value: &str,
    add_value: &str,
    sudo: bool,
) -> Result<(), io::Error> {
    match fs::read_to_string(path) {
        Ok(content) => {
            if !content.contains(find_value) {
                append_to_file(path, add_value, sudo)?;
            }
        }
        Err(_) => append_to_file(path, add_value, sudo)?,
    }
    Ok(())
}
