use dialoguer::MultiSelect;
use std::env;
use std::env::VarError;
use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::io::Result;
use std::num::ParseIntError;
use std::path::PathBuf;
use std::process::{Command, ExitStatus, Output};
use std::string::FromUtf8Error;

const ANSI_RESET: &str = "\x1b[0m";
const ANSI_RED: &str = "\x1b[31m";
const ANSI_GREEN: &str = "\x1b[32m";
const ANSI_CYAN: &str = "\x1b[36m";

const MAX_SIZE: u32 = 2400;

fn select_folders(folders: &Vec<String>, prompt: &str) -> Option<Vec<usize>> {
    let selection: Result<Option<Vec<usize>>> = MultiSelect::new()
        .with_prompt(prompt)
        .max_length(10)
        .items(folders)
        .interact_opt();
    if selection.is_ok() {
        let selection: Option<Vec<usize>> = selection.unwrap();
        if selection.is_some() {
            let selection: Vec<usize> = selection.unwrap();
            if !selection.is_empty() {
                return Option::from(selection);
            }
        }
    }
    None
}

fn get_recursive_folders_files(orig_path: &String, path: &String) -> (Vec<String>, Vec<String>) {
    if path.contains(".git") {
        return (vec![], vec![]);
    }

    let dir: Result<ReadDir> = fs::read_dir(path);
    if dir.is_err() {
        return (vec![], vec![]);
    }

    let mut folders: Vec<String> = vec![];
    let mut files: Vec<String> = vec![];

    if orig_path != path {
        folders.push(path[orig_path.len()..].to_string());
    }

    let dir: ReadDir = dir.unwrap();
    for entry in dir {
        if entry.is_err() {
            continue;
        }

        let entry: DirEntry = entry.unwrap();
        let entry_path_buff: PathBuf = entry.path();
        let entry_path: Option<&str> = entry_path_buff.to_str();
        if entry_path.is_none() {
            continue;
        }
        let entry_path: &str = entry_path.unwrap();

        if entry_path_buff.is_file() {
            files.push(entry_path.to_string());
        } else if entry_path_buff.is_dir() {
            let sub_folders_files: (Vec<String>, Vec<String>) =
                get_recursive_folders_files(orig_path, &entry_path.to_string());
            for folder in sub_folders_files.0 {
                folders.push(folder);
            }
            for file in sub_folders_files.1 {
                files.push(file);
            }
        }
    }
    (folders, files)
}

fn remove_file(file: &String) {
    let _ = Command::new("rm")
        .arg("-f")
        .arg(file)
        .status()
        .expect("failed to remove file");
}

fn get_image_dim(file: &String, dim: &str) -> u32 {
    let mut cmd: Command = Command::new("identify");
    cmd.arg("-format");
    cmd.arg(format!("%[{}]", dim));
    cmd.arg(file);
    let output: Result<Output> = cmd.output();
    if output.is_ok() {
        let output: Output = output.unwrap();
        let output: Vec<u8> = output.stdout;
        let output: std::result::Result<String, FromUtf8Error> = String::from_utf8(output);
        if output.is_ok() {
            let output: String = output.unwrap();
            let dim_size: std::result::Result<u32, ParseIntError> = output.parse::<u32>();
            if dim_size.is_ok() {
                return dim_size.unwrap();
            }
        }
    }
    0
}

fn convert_file(old_file: &String, new_file: &String) -> bool {
    let convert_status: Result<ExitStatus> =
        Command::new("convert").arg(old_file).arg(new_file).status();
    if convert_status.is_ok() {
        return convert_status.unwrap().success();
    }
    false
}

fn resize_file(file: &String, dim: &String) {
    let _ = Command::new("convert")
        .arg(file)
        .arg("-resize")
        .arg(dim)
        .arg(file)
        .status()
        .expect("failed to resize file");
}

fn main() {
    let home_dir: std::result::Result<String, VarError> = env::var("HOME");
    if home_dir.is_err() {
        println!("HOME directory could not be determined");
        return;
    }
    let home_dir: String = home_dir.unwrap();
    let pictures_dir: String = format!("{}/Pictures/", &home_dir);

    let folders_files: (Vec<String>, Vec<String>) =
        get_recursive_folders_files(&pictures_dir, &pictures_dir);

    let mut make_small_folders: Vec<&str> = vec![];
    let folder_selection: Option<Vec<usize>> =
        select_folders(&folders_files.0, "Choose folders to make small");
    if folder_selection.is_some() {
        let folder_selection: Vec<usize> = folder_selection.unwrap();
        for idx in folder_selection {
            make_small_folders.push(&folders_files.0[idx]);
        }
    }

    for file_path in &folders_files.1 {
        let extension: Option<(&str, &str)> = file_path.rsplit_once(".");
        if extension.is_none() {
            continue;
        }
        let extension: (&str, &str) = extension.unwrap();
        let file_name_no_extension: &str = extension.0;
        let extension: &str = extension.1;
        let file_name: String = format!("{}.jpeg", file_name_no_extension);
        if extension != "jpeg" {
            println!(
                "Attempting to convert {}{}{} to a jpeg...",
                ANSI_CYAN, file_path, ANSI_RESET
            );
            if convert_file(&file_path, &file_name) {
                println!("    {}Convert Successful{}", ANSI_GREEN, ANSI_RESET);
                remove_file(&file_path);
            } else {
                println!("    {}Failed to Convert{}", ANSI_RED, ANSI_RESET);
                continue;
            }
        }

        for folder in &make_small_folders {
            if file_name.contains(folder) {
                println!(
                    "Checking size of {}{}{}...",
                    ANSI_CYAN, file_name, ANSI_RESET
                );

                let height: u32 = get_image_dim(&file_name, "h");
                let width: u32 = get_image_dim(&file_name, "w");

                if height > MAX_SIZE || width > MAX_SIZE {
                    if height > width {
                        println!(
                            "    Image is {}too tall{} (height: {})",
                            ANSI_RED, ANSI_RESET, height
                        );
                        resize_file(&file_name, &format!("x{}", MAX_SIZE));
                    } else {
                        println!(
                            "    Image is {}too wide{} (width: {})",
                            ANSI_RED, ANSI_RESET, width
                        );
                        println!("    Image is {}too wide{}", ANSI_RED, ANSI_RESET);
                        resize_file(&file_name, &format!("{}", MAX_SIZE));
                    }
                } else {
                    println!("    Image is {}small enough{}", ANSI_GREEN, ANSI_RESET);
                }

                break;
            }
        }
    }

    println!(
        "Finished processing {}{}{} images",
        ANSI_CYAN,
        &folders_files.1.len(),
        ANSI_RESET
    );
}
