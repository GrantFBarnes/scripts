extern crate rust_cli;

use rust_cli::ansi::font;
use rust_cli::ansi::Color;
use rust_cli::ansi::Font;
use rust_cli::prompts::Select;

use std::cmp;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::{DirEntry, FileType, Metadata, ReadDir};
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::Duration;

fn print_help() {
    println!("SFS (Scan File System)");
    println!("Command line program that finds the disk usage of files/folders in specified path");
    println!();
    print!("Usage: ");
    font::text_color(Color::Blue);
    print!("sfs <PATH>");
    font::reset();
    println!();
    println!();
    println!("Options:");
    font::text_color(Color::Cyan);
    print!("  <PATH>");
    font::reset();
    print!("      Path of folder to scan");
    println!();
    font::text_color(Color::Cyan);
    print!("  -h, --help");
    font::reset();
    print!("  Print help information");
    println!();
}

fn get_dir_size(path: &String) -> u64 {
    let dir: io::Result<ReadDir> = fs::read_dir(path);
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
        total_size += get_entry_size(&entry);
    }
    total_size
}

fn get_entry_size(entry: &DirEntry) -> u64 {
    let file_type: io::Result<FileType> = entry.file_type();
    if file_type.is_err() {
        return 0;
    }
    let file_type: FileType = file_type.unwrap();

    if file_type.is_file() {
        let meta_data: io::Result<Metadata> = entry.metadata();
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
    let dir: io::Result<ReadDir> = fs::read_dir(path);
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
        if entry_path == "/proc" {
            continue;
        }
        let entry_path: String = entry_path.replacen(path, "", 1);

        let entry_size: u64 = get_entry_size(&entry);

        sizes.insert(entry_path, entry_size);
    }

    Some(sizes)
}

fn select_directory(path: &String, sizes: HashMap<String, u64>) {
    let mut max_path: usize = 0;
    let mut max_size: u64 = 0;
    for entry in &sizes {
        let full_path: String = format!(
            "{}{}{}{}",
            path,
            Color::Blue.as_str(),
            entry.0,
            Font::Reset.as_str()
        );
        if full_path.len() > max_path {
            max_path = full_path.len();
        }
        if entry.1 > &max_size {
            max_size = entry.1.to_owned();
        }
    }

    let mut sorted_sizes: Vec<(&String, &u64)> = sizes.iter().collect();
    sorted_sizes.sort_by(|a, b| b.1.cmp(a.1));

    let mut options_display: Vec<String> = vec![];
    let mut options_value: Vec<String> = vec![];

    for entry in sorted_sizes {
        let full_path: String = format!(
            "{}{}{}{}",
            path,
            Color::Blue.as_str(),
            entry.0,
            Font::Reset.as_str()
        );

        let percent_bar_len: usize = 20;
        let percent: f64 = entry.1.to_owned() as f64 / max_size as f64;
        let percent_bar: f64 = percent * percent_bar_len as f64;
        let percent_bar: usize = percent_bar as usize;
        let full_percentage: String = format!(
            "[{}{}]",
            "#".repeat(percent_bar),
            ".".repeat(percent_bar_len - percent_bar)
        );

        let mut size: String = format!("{} B ", (entry.1));
        if entry.1 > &(1024 * 1024 * 1024 * 1024) {
            size = format!("{} TB", (entry.1 / (1024 * 1024 * 1024 * 1024)));
        } else if entry.1 > &(1024 * 1024 * 1024) {
            size = format!("{} GB", (entry.1 / (1024 * 1024 * 1024)));
        } else if entry.1 > &(1024 * 1024) {
            size = format!("{} MB", (entry.1 / (1024 * 1024)));
        } else if entry.1 > &1024 {
            size = format!("{} KB", (entry.1 / 1024));
        }
        let full_size: String = format!(
            "{}{: >7}{}",
            Color::Cyan.as_str(),
            size,
            Font::Reset.as_str()
        );

        options_display.push(format!(
            "{:width$} {} {}",
            full_path,
            full_percentage,
            full_size,
            width = max_path
        ));
        options_value.push(format!("{}{}", path, entry.0));
    }

    if path != "/" {
        let up_dir = fs::canonicalize(PathBuf::from(format!("{}{}", path, "/..")));
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

    let selection = Select::new()
        .title("Select Directory to Scan")
        .options(&options_display)
        .run_select_index();
    if selection.is_err() {
        return;
    }
    let selection = selection.unwrap();
    if selection.is_none() {
        return;
    }

    if process_directory(&options_value[selection.unwrap()]).is_err() {
        rust_cli::ansi::cursor::previous_lines(cmp::min(sizes.len(), 15) + 2);
        select_directory(path, sizes);
    }
}

fn process_directory(path: &String) -> Result<(), &str> {
    let owned_path: String = path.clone();
    let process: JoinHandle<Option<HashMap<String, u64>>> =
        thread::spawn(move || get_directory_sizes(&owned_path));

    let min_dot_count: usize = 0;
    let max_dot_count: usize = 10;
    let mut dot_count: usize = 0;
    while !process.is_finished() {
        print!("\rScanning Files{}", ".".repeat(dot_count));
        match io::stdout().flush() {
            Err(_) => return Err("io stdout flush failed"),
            _ => (),
        };
        sleep(Duration::from_millis(250));
        dot_count += 1;
        if dot_count == max_dot_count {
            dot_count = min_dot_count;
        }
    }
    rust_cli::ansi::erase::line();
    rust_cli::ansi::cursor::line_start();

    let process_results: thread::Result<Option<HashMap<String, u64>>> = process.join();
    if process_results.is_err() {
        return Err("thread failed to join");
    }
    let process_results: HashMap<String, u64> = process_results
        .unwrap()
        .ok_or("selection could not be scanned")?;
    select_directory(&path, process_results);
    return Ok(());
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

    let _ = process_directory(arg);
}
