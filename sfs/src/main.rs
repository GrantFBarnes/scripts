use dialoguer::Select;
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
const ANSI_MAGENTA: &str = "\x1b[35m";
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

fn get_directory_sizes(path: &String) -> Option<HashMap<String, u64>> {
    let dir: Result<ReadDir> = fs::read_dir(path);
    if dir.is_err() {
        return None;
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

    Some(sizes)
}

fn select_directory(path: &String, sizes: HashMap<String, u64>) {
    let mut sorted_sizes: Vec<(&String, &u64)> = sizes.iter().collect();
    sorted_sizes.sort_by(|a, b| b.1.cmp(a.1));

    let mut options_display: Vec<String> = vec![];
    let mut options_value: Vec<String> = vec![];

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
        options_display.push(format!(
            "{}{}{} {}{}{}",
            path, ANSI_BLUE, entry.0, ANSI_CYAN, size, ANSI_RESET
        ));
        options_value.push(format!("{}{}", path, entry.0));
    }

    if path != "/" {
        let up_dir: Result<PathBuf> = fs::canonicalize(PathBuf::from(format!("{}{}", path, "/..")));
        if up_dir.is_err() {
            return;
        }
        let up_dir: PathBuf = up_dir.unwrap();
        let up_dir: Option<&str> = up_dir.to_str();
        if up_dir.is_none() {
            return;
        }
        let up_dir: &str = up_dir.unwrap();

        options_display.push("..".to_string());
        options_value.push(up_dir.to_string());
    }

    let selection: Result<Option<usize>> = Select::new()
        .with_prompt(format!("  {}{}{}", ANSI_MAGENTA, path, ANSI_RESET))
        .items(&options_display)
        .default(0)
        .max_length(10)
        .interact_opt();
    if selection.is_err() {
        return;
    }
    let selection: Option<usize> = selection.unwrap();
    if selection.is_none() {
        return;
    }
    let selection = selection.unwrap();
    process_directory(&options_value[selection]);
}

fn process_directory(path: &String) {
    let owned_path: String = path.clone();
    let process: JoinHandle<Option<HashMap<String, u64>>> =
        thread::spawn(move || get_directory_sizes(&owned_path));

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

    let process_results: thread::Result<Option<HashMap<String, u64>>> = process.join();
    if process_results.is_err() {
        println!("Thread failed to join");
        return;
    }
    let process_results: Option<HashMap<String, u64>> = process_results.unwrap();
    if process_results.is_none() {
        println!("{}{}{} could not be scanned...", ANSI_RED, path, ANSI_RESET);
        return;
    }
    select_directory(&path, process_results.unwrap());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        print_help();
        return;
    }

    let arg: &String = &args[1];

    if arg == "-h" || arg == "--help" {
        print_help();
        return;
    }

    process_directory(arg);
}
