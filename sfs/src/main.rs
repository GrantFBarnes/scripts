use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::{DirEntry, FileType, Metadata, ReadDir};
use std::io;
use std::io::{Result, Write};
use std::ops::Add;
use std::path::PathBuf;
use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::Duration;

const ANSI_RESET: &str = "\x1b[0m";
const ANSI_RED: &str = "\x1b[31m";
const ANSI_BLUE: &str = "\x1b[34m";
const ANSI_CYAN: &str = "\x1b[36m";

fn print_help() {
    println!(
        "
SFS (Scan File System)
Command line program that finds the disk usage of files/folders in specified path

Usage: {}sfs <PATH>{}

Options:
  {}<PATH>{}        Path of folder to scan
  {}-h, --help{}    Print help information
        ",
        ANSI_BLUE, ANSI_RESET, ANSI_CYAN, ANSI_RESET, ANSI_CYAN, ANSI_RESET
    );
}

fn get_dir_size(path: &String) -> u64 {
    let dir: Result<ReadDir> = fs::read_dir(path);
    if dir.is_err() {
        return 0;
    }

    let mut total_size: u64 = 0;

    let dir: ReadDir = dir.unwrap();
    for entry in dir {
        if entry.is_err() {
            continue;
        }

        let entry: DirEntry = entry.unwrap();
        total_size = total_size.add(get_entry_size(&entry));
    }
    total_size
}

fn get_entry_size(entry: &DirEntry) -> u64 {
    let file_type: Result<FileType> = entry.file_type();
    if file_type.is_err() {
        return 0;
    }
    let file_type: FileType = file_type.unwrap();

    if file_type.is_file() {
        let meta_data: Result<Metadata> = entry.metadata();
        if meta_data.is_err() {
            return 0;
        }
        return meta_data.unwrap().len();
    } else if file_type.is_dir() {
        let path: PathBuf = entry.path();
        let path: Option<&str> = path.to_str();
        if path.is_none() {
            return 0;
        }
        return get_dir_size(&path.unwrap().to_string());
    }
    0
}

fn get_directory_sizes(path: &String) -> Vec<String> {
    let dir: Result<ReadDir> = fs::read_dir(path);
    if dir.is_err() {
        return vec![format!(
            "Could not find folder {}{}{}",
            ANSI_RED, path, ANSI_RESET
        )];
    }

    let mut sizes: HashMap<String, u64> = HashMap::new();

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
        let entry_path: String = entry_path.replace(path, "");

        let entry_size: u64 = get_entry_size(&entry);

        sizes.insert(entry_path, entry_size);
    }

    let mut sorted_sizes: Vec<(&String, &u64)> = sizes.iter().collect();
    sorted_sizes.sort_by(|a, b| b.1.cmp(a.1));

    let mut result: Vec<String> = vec![];
    for entry in sorted_sizes {
        let mut size: String = format!("{} B", (entry.1));
        if entry.1 > &(1024 * 1024 * 1024 * 1024) {
            size = format!("{} TB", (entry.1 / (1024 * 1024 * 1024 * 1024)));
        } else if entry.1 > &(1024 * 1024 * 1024) {
            size = format!("{} GB", (entry.1 / (1024 * 1024 * 1024)));
        } else if entry.1 > &(1024 * 1024) {
            size = format!("{} MB", (entry.1 / (1024 * 1024)));
        } else if entry.1 > &1024 {
            size = format!("{} KB", (entry.1 / 1024));
        }
        result.push(format!(
            "{}{}{} {}{}{}",
            path, ANSI_BLUE, entry.0, ANSI_CYAN, size, ANSI_RESET
        ));
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        print_help();
        return;
    }

    let arg: String = args[1].to_string();

    if arg == "-h" || arg == "--help" {
        print_help();
        return;
    }

    let process: JoinHandle<Vec<String>> = thread::spawn(move || get_directory_sizes(&arg));

    let min_dot_count: usize = 0;
    let max_dot_count: usize = 10;
    let mut dot_count: usize = 0;
    while !process.is_finished() {
        print!("\rProcessing Files{}", ".".repeat(dot_count));
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(250));
        dot_count += 1;
        if dot_count == max_dot_count {
            dot_count = min_dot_count;
        }
    }
    println!(
        "{}Processing Complete",
        ".".repeat(max_dot_count - dot_count)
    );

    let results: thread::Result<Vec<String>> = process.join();
    if results.is_ok() {
        let results: Vec<String> = results.unwrap();
        for line in results {
            println!("{}", line);
        }
    }
}
