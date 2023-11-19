extern crate rust_cli;

use rust_cli::commands::Operation;

use chrono::prelude::Local;
use chrono::{Duration, NaiveDateTime, ParseResult};
use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;

fn get_database_backup(db: &str) -> Result<String, io::Error> {
    Operation::new()
        .command(format!(
            "mariadb-dump --order-by-primary --extended-insert=FALSE {}",
            db
        ))
        .run_output()
}

fn get_backup_files(path: &String) -> Result<Vec<String>, io::Error> {
    let mut backup_files: Vec<String> = vec![];

    let file_name_regex: Regex = Regex::new(r"^\d{8}_\d{6}_backup_.*\.sql$").unwrap();
    let dir = fs::read_dir(path)?;
    for entry in dir {
        if entry.is_ok() {
            let entry = entry.unwrap();
            let file_name = entry.file_name();
            let file_name: Option<&str> = file_name.to_str();
            if file_name.is_some() {
                let file_name: &str = file_name.unwrap();
                if file_name_regex.is_match(file_name) {
                    backup_files.push(file_name.to_string());
                }
            }
        }
    }

    backup_files.sort();
    backup_files.reverse();
    Ok(backup_files)
}

fn remove_latest_backup(
    backup_files: Vec<String>,
    db_backup_dir: &String,
) -> Result<bool, io::Error> {
    let latest: Option<&String> = backup_files.get(0);
    let last: Option<&String> = backup_files.get(1);
    if latest.is_some() && last.is_some() {
        let latest: String = format!("{}/{}", db_backup_dir, latest.unwrap());
        let last: String = format!("{}/{}", db_backup_dir, last.unwrap());

        let diff: String = Operation::new()
            .command(format!("diff {} {}", &latest, &last))
            .run_output()?;
        let lines: Vec<&str> = diff.split("\n").collect();
        if lines.len() <= 5 {
            fs::remove_file(&latest)?;
            return Ok(true);
        }
    }
    Ok(false)
}

fn remove_old_backups(backup_files: Vec<String>, db_backup_dir: &String) -> Result<(), io::Error> {
    let mut backups_to_remove: HashSet<String> = HashSet::new();

    let mut days_backed_up: HashSet<i64> = HashSet::new();
    let mut weeks_backed_up: HashSet<i64> = HashSet::new();
    let mut months_backed_up: HashSet<i64> = HashSet::new();

    let fmt: &str = "%Y-%m-%d %H:%M:%S";
    let now: String = Local::now().format(fmt).to_string();
    let now = NaiveDateTime::parse_from_str(&now, fmt);
    if now.is_err() {
        return Err(io::Error::other("failed to parse date"));
    }
    let now: NaiveDateTime = now.unwrap();

    for file in &backup_files {
        let file_date: ParseResult<NaiveDateTime> = NaiveDateTime::parse_from_str(
            &format!(
                "{}-{}-{} {}:{}:{}",
                &file[0..4],
                &file[4..6],
                &file[6..8],
                &file[9..11],
                &file[11..13],
                &file[13..15]
            ),
            fmt,
        );
        if file_date.is_err() {
            continue;
        }
        let file_date: NaiveDateTime = file_date.unwrap();

        let diff: Duration = now - file_date;

        let days_old: i64 = diff.num_days();
        if days_old < 7 {
            continue;
        }
        if days_backed_up.contains(&days_old) {
            backups_to_remove.insert(file.to_string());
            continue;
        }
        days_backed_up.insert(days_old);

        let weeks_old: i64 = diff.num_weeks();
        if weeks_old < 4 {
            continue;
        }
        if weeks_backed_up.contains(&weeks_old) {
            backups_to_remove.insert(file.to_string());
            continue;
        }
        weeks_backed_up.insert(weeks_old);

        let months_old: i64 = diff.num_weeks() / 4;
        if months_old < 6 {
            continue;
        }
        if months_backed_up.contains(&months_old) {
            backups_to_remove.insert(file.to_string());
            continue;
        }
        months_backed_up.insert(months_old);
    }

    for file in backups_to_remove {
        fs::remove_file(&format!("{}/{}", db_backup_dir, file))?;
    }
    Ok(())
}

fn remove_excess_backups(
    backup_files: Vec<String>,
    db_backup_dir: &String,
) -> Result<(), io::Error> {
    const COUNT: usize = 100;
    if backup_files.len() > COUNT {
        let files_to_remove: &[String] = &backup_files[COUNT..];
        for file in files_to_remove {
            fs::remove_file(&format!("{}/{}", db_backup_dir, file))?;
        }
    }
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let home_dir = env::var("HOME");
    if home_dir.is_err() {
        return Err(io::Error::other("HOME directory could not be determined"));
    }
    let home_dir: String = home_dir.unwrap();

    let backup_dir: String = format!("{}/backups", home_dir);
    fs::create_dir_all(&backup_dir)?;

    let backup_dir: String = format!("{}/databases", backup_dir);
    fs::create_dir_all(&backup_dir)?;

    let now: String = Local::now().format("%Y%m%d_%H%M%S").to_string();

    const DATABASES: [&str; 3] = ["crm", "learn_vietnamese", "tractor_pulling"];
    for db in DATABASES {
        let db_backup_dir: String = format!("{}/{}", backup_dir, db);
        fs::create_dir_all(&db_backup_dir)?;

        let file_name: String = format!("{}/{}_backup_{}.sql", db_backup_dir, now, db);

        let backup: String = get_database_backup(db)?;
        fs::write(&file_name, backup)?;

        let backup_files: Vec<String> = get_backup_files(&db_backup_dir)?;
        if backup_files.len() > 0 {
            if remove_latest_backup(backup_files, &db_backup_dir)? {
                continue;
            }
        }

        let backup_files: Vec<String> = get_backup_files(&db_backup_dir)?;
        if backup_files.len() > 0 {
            remove_old_backups(backup_files, &db_backup_dir)?;
        }

        let backup_files: Vec<String> = get_backup_files(&db_backup_dir)?;
        if backup_files.len() > 0 {
            remove_excess_backups(backup_files, &db_backup_dir)?;
        }
    }
    Ok(())
}
