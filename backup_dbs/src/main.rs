use chrono::prelude::Local;
use std::env::VarError;
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

fn create_directory(path: &String) {
    match fs::create_dir_all(path) {
        _ => (),
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

    const DATABASES: [&str; 3] = ["crm", "learn-vietnamese", "tractor-pulling"];
    for db in DATABASES {
        let db_backup_dir: String = format!("{}/{}", backup_dir, db);
        create_directory(&db_backup_dir);

        let file_name: String = format!("{}/{}-backup-{}.sql", db_backup_dir, now, db);

        let backup: Option<String> = get_database_backup(db);
        if backup.is_some() {
            let backup: String = backup.unwrap();
            write_to_file(&file_name, backup);
        }
    }
}
