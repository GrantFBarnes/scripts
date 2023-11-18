extern crate rust_cli;

use std::env;
use std::env::VarError;
use std::fs;
use std::io::Error;
use std::process::{Command, Stdio};

fn main() -> Result<(), Error> {
    let home_dir: Result<String, VarError> = env::var("HOME");
    if home_dir.is_err() {
        return Err(Error::other("HOME directory could not be determined"));
    }
    let home_dir: String = home_dir.unwrap();

    let backup_dir: String = format!("{}/backups", home_dir);
    fs::create_dir_all(&backup_dir)?;

    let backup_dir: String = format!("{}/home", backup_dir);
    fs::create_dir_all(&backup_dir)?;

    let all_folders: Vec<String> = vec![
        String::from("Documents"),
        String::from("Music"),
        String::from("Pictures"),
        String::from("Videos"),
    ];
    let backup_folders: Vec<String> = rust_cli::prompts::Select::new()
        .title("Select folders to backup")
        .options(&all_folders)
        .prompt_for_values()?;

    if backup_folders.is_empty() {
        return Err(Error::other("no folders selected to backup"));
    }

    let mut encrypt_folders: Vec<String> = vec![];
    let mut passphrase: String = String::new();
    if rust_cli::prompts::Confirm::new()
        .message("Do you want to encrypt backups?")
        .confirm()?
    {
        encrypt_folders = rust_cli::prompts::Select::new()
            .title("Select folders to encrypt")
            .options(&backup_folders)
            .prompt_for_values()?;

        if encrypt_folders.is_empty() {
            return Err(Error::other("no folders selected to encrypt"));
        }

        passphrase = rust_cli::prompts::Text::new()
            .message("Encryption Passphrase:")
            .required(true)
            .secret(true)
            .confirm(true)
            .prompt()?;
    }

    for folder in backup_folders {
        let tar_file: String = format!("{backup_dir}/{folder}.tar.gz");
        let crypt_file: String = format!("{backup_dir}/{folder}.tar.gz.gpg");

        rust_cli::commands::Operation::new()
            .command(&format!("rm -f {}", &tar_file))
            .run()?;
        rust_cli::commands::Operation::new()
            .command(&format!("rm -f {}", &crypt_file))
            .run()?;

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

            rust_cli::commands::Operation::new()
                .command(&format!("rm -f {}", &tar_file))
                .run()?;
        }
    }

    Ok(())
}
