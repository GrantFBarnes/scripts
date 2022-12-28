use chrono::prelude::Local;
use regex::Regex;
use std::collections::HashSet;
use std::env::VarError;
use std::ffi::OsString;
use std::fs::{DirEntry, ReadDir};
use std::io::Result;
use std::process::{Command, Output};
use std::string::FromUtf8Error;
use std::{env, fs};

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

fn get_file_diff(file1: &String, file2: &String) -> Option<String> {
    let mut cmd: Command = Command::new("diff");
    cmd.arg(file1);
    cmd.arg(file2);
    get_command_output(cmd)
}

fn get_database_backup(db: &str) -> Option<String> {
    let mut cmd: Command = Command::new("mariadb-dump");
    cmd.arg("--order-by-primary");
    cmd.arg("--extended-insert=FALSE");
    cmd.arg(db);
    get_command_output(cmd)
}

fn write_to_file(path: &String, contents: String) {
    fs::write(path, contents).expect("Failed to write backup to file");
}

fn remove_file(path: &String) {
    match fs::remove_file(path) {
        _ => (),
    }
}

fn create_directory(path: &String) {
    match fs::create_dir_all(path) {
        _ => (),
    }
}

fn get_backup_files(path: &String) -> Vec<String> {
    let mut backup_files: Vec<String> = vec![];

    let file_name_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}-backup-.*\.sql$").unwrap();
    let dir: Result<ReadDir> = fs::read_dir(path);
    if dir.is_ok() {
        let dir: ReadDir = dir.unwrap();
        for entry in dir {
            if entry.is_ok() {
                let entry: DirEntry = entry.unwrap();
                let file_name: OsString = entry.file_name();
                let file_name: Option<&str> = file_name.to_str();
                if file_name.is_some() {
                    let file_name: &str = file_name.unwrap();
                    if file_name_regex.is_match(file_name) {
                        backup_files.push(file_name.to_string());
                    }
                }
            }
        }
    }

    backup_files.sort();
    backup_files.reverse();
    backup_files
}

fn remove_unchanged_backups(backup_files: Vec<String>, db_backup_dir: &String) {
    let mut backups_to_remove: HashSet<String> = HashSet::new();
    for i in 0..backup_files.len() - 1 {
        for j in i + 1..backup_files.len() {
            let file1: String = format!("{}/{}", db_backup_dir, &backup_files[i]);
            let file2: String = format!("{}/{}", db_backup_dir, &backup_files[j]);
            let diff: Option<String> = get_file_diff(&file1, &file2);
            if diff.is_some() {
                let diff: String = diff.unwrap();
                let lines: Vec<&str> = diff.split("\n").collect::<Vec<&str>>();
                if lines.len() <= 5 {
                    backups_to_remove.insert(file2);
                }
            }
        }
    }

    for file in backups_to_remove {
        remove_file(&file);
    }
}

fn remove_excess_backups(backup_files: Vec<String>, db_backup_dir: &String) {
    const COUNT: usize = 30;
    if backup_files.len() > COUNT {
        let files_to_remove: &[String] = &backup_files[COUNT..];
        for file in files_to_remove {
            remove_file(&format!("{}/{}", db_backup_dir, file));
        }
    }
}

fn main() {
    let home_dir: std::result::Result<String, VarError> = env::var("HOME");
    if home_dir.is_err() {
        println!("HOME directory could not be determined");
        return;
    }
    let home_dir: String = home_dir.unwrap();

    let backup_dir: String = format!("{}/backups", home_dir);
    create_directory(&backup_dir);

    let backup_dir: String = format!("{}/databases", backup_dir);
    create_directory(&backup_dir);

    let now: String = Local::now().format("%Y-%m-%d").to_string();

    const DATABASES: [&str; 3] = ["crm", "learn_vietnamese", "tractor_pulling"];
    for db in DATABASES {
        let db_backup_dir: String = format!("{}/{}", backup_dir, db);
        create_directory(&db_backup_dir);

        let file_name: String = format!("{}/{}-backup-{}.sql", db_backup_dir, now, db);

        let backup: Option<String> = get_database_backup(db);
        if backup.is_some() {
            let backup: String = backup.unwrap();
            write_to_file(&file_name, backup);
        }

        let backup_files: Vec<String> = get_backup_files(&db_backup_dir);
        remove_unchanged_backups(backup_files, &db_backup_dir);

        let backup_files: Vec<String> = get_backup_files(&db_backup_dir);
        remove_excess_backups(backup_files, &db_backup_dir);
    }
}
