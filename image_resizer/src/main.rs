extern crate rust_cli;

use rust_cli::ansi::Color;
use rust_cli::ansi::Font;
use rust_cli::commands::Operation;
use rust_cli::prompts::select::Select;

use std::env;
use std::env::VarError;
use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::io;
use std::path::PathBuf;

const MAX_SIZE: u32 = 2400;

fn get_recursive_folders_files(orig_path: &String, path: &String) -> (Vec<String>, Vec<String>) {
    if path.contains(".git") {
        return (vec![], vec![]);
    }

    let dir = fs::read_dir(path);
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

fn get_image_dim(file: &String, dim: &str) -> Result<u32, io::Error> {
    Ok(
        Operation::new(format!("identify -format %[{}] {}", dim, file))
            .output()?
            .parse::<u32>()
            .unwrap_or(0),
    )
}

fn convert_file(old_file: &String, new_file: &String) -> Result<bool, io::Error> {
    Ok(
        Operation::new(format!("convert {} {} ", old_file, new_file))
            .run()?
            .success(),
    )
}

fn main() -> Result<(), io::Error> {
    let home_dir: Result<String, VarError> = env::var("HOME");
    if home_dir.is_err() {
        return Err(io::Error::other("HOME directory could not be determined"));
    }
    let home_dir: String = home_dir.unwrap();
    let pictures_dir: String = format!("{}/Pictures/", &home_dir);

    let folders_files: (Vec<String>, Vec<String>) =
        get_recursive_folders_files(&pictures_dir, &pictures_dir);

    let make_small_folders: Vec<String> = Select::new()
        .title("Choose folders to make small")
        .options(&folders_files.0)
        .run_multi_select()?
        .iter()
        .map(|t| t.1.to_string())
        .collect();

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
                Color::Cyan.as_str(),
                file_path,
                Font::Reset.as_str()
            );
            if convert_file(&file_path, &file_name).is_ok_and(|x| x) {
                println!(
                    "    {}Convert Successful{}",
                    Color::Green.as_str(),
                    Font::Reset.as_str()
                );
                Operation::new(format!("rm -f {}", &file_path)).run()?;
            } else {
                println!(
                    "    {}Failed to Convert{}",
                    Color::Red.as_str(),
                    Font::Reset.as_str()
                );
                continue;
            }
        }

        for folder in &make_small_folders {
            if file_name.contains(folder) {
                println!(
                    "Checking size of {}{}{}...",
                    Color::Cyan.as_str(),
                    file_name,
                    Font::Reset.as_str()
                );

                let height: u32 = get_image_dim(&file_name, "h")?;
                let width: u32 = get_image_dim(&file_name, "w")?;

                if height > MAX_SIZE || width > MAX_SIZE {
                    if height > width {
                        println!(
                            "    Image is {}too tall{} (height: {})",
                            Color::Red.as_str(),
                            Font::Reset.as_str(),
                            height
                        );
                        Operation::new(format!(
                            "convert {} -resize x{} {}",
                            &file_name, MAX_SIZE, &file_name
                        ))
                        .run()?;
                    } else {
                        println!(
                            "    Image is {}too wide{} (width: {})",
                            Color::Red.as_str(),
                            Font::Reset.as_str(),
                            width
                        );
                        println!(
                            "    Image is {}too wide{}",
                            Color::Red.as_str(),
                            Font::Reset.as_str()
                        );
                        Operation::new(format!(
                            "convert {} -resize {} {}",
                            &file_name, MAX_SIZE, &file_name
                        ))
                        .run()?;
                    }
                } else {
                    println!(
                        "    Image is {}small enough{}",
                        Color::Green.as_str(),
                        Font::Reset.as_str()
                    );
                }

                break;
            }
        }
    }

    println!(
        "Finished processing {}{}{} images",
        Color::Cyan.as_str(),
        &folders_files.1.len(),
        Font::Reset.as_str()
    );

    Ok(())
}
