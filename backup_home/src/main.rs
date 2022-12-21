use dialoguer::Confirm;
use dialoguer::MultiSelect;
use dialoguer::Password;
use std::env;
use std::env::VarError;
use std::fs;
use std::io::Result;
use std::process::{Command, Stdio};

fn select_folders(folders: &Vec<&str>, prompt: &str) -> Option<Vec<usize>> {
    let selection: Result<Option<Vec<usize>>> = MultiSelect::new()
        .with_prompt(prompt)
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

fn remove_file(file: &String) {
    let _ = Command::new("rm")
        .arg("-f")
        .arg(file)
        .status()
        .expect("failed to remove file");
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

    let backup_dir: String = format!("{}/home", backup_dir);
    create_directory(&backup_dir);

    let all_folders: Vec<&str> = vec!["Documents", "Music", "Pictures", "Videos"];

    let backup_folders_selection: Option<Vec<usize>> =
        select_folders(&all_folders, "Select folders to backup");
    if backup_folders_selection.is_none() {
        println!("No folders were chosen to backup");
        return;
    }
    let backup_folders_selection: Vec<usize> = backup_folders_selection.unwrap();

    let mut backup_folders: Vec<&str> = vec![];
    for idx in backup_folders_selection {
        backup_folders.push(all_folders[idx]);
    }

    let mut encrypt_folders: Vec<&str> = vec![];
    let mut passphrase: String = String::new();
    if Confirm::new()
        .with_prompt("Do you want to encrypt backups?")
        .interact()
        .expect("failed to confirm encryption")
    {
        let encrypt_folders_selection: Option<Vec<usize>> =
            select_folders(&backup_folders, "Select folders to encrypt");
        if encrypt_folders_selection.is_some() {
            let encrypt_folders_selection: Vec<usize> = encrypt_folders_selection.unwrap();
            for idx in encrypt_folders_selection {
                encrypt_folders.push(backup_folders[idx]);
            }
            passphrase = Password::new()
                .with_prompt("Encryption Passphrase")
                .with_confirmation("Confirm passphrase", "Passphrases mismatching")
                .interact()
                .expect("failed to collect passphrase");
        }
    }

    for folder in backup_folders {
        let tar_file: String = format!("{backup_dir}/{folder}.tar.gz");
        let crypt_file: String = format!("{backup_dir}/{folder}.tar.gz.gpg");

        remove_file(&tar_file);
        remove_file(&crypt_file);

        println!("Compressing {}...", &folder);
        let _ = Command::new("tar")
            .arg("--exclude-vcs")
            .arg("-cvzf")
            .arg(&tar_file)
            .arg(&folder)
            .current_dir(&home_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("tar command failed")
            .wait();

        if encrypt_folders.contains(&folder) {
            println!("Encrypting {}...", &folder);
            let _ = Command::new("gpg")
                .arg("--batch")
                .arg("-c")
                .arg("--passphrase")
                .arg(&passphrase)
                .arg(&tar_file)
                .current_dir(&home_dir)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("gpg command failed")
                .wait();

            remove_file(&tar_file);
        }
    }
}
