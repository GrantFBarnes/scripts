extern crate rust_cli;

use rust_cli::commands::Operation;
use rust_cli::prompts::Confirm;
use rust_cli::prompts::Select;
use rust_cli::prompts::Text;

use std::env;
use std::env::VarError;
use std::fs;
use std::io::Error;

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

    let all_folders: Vec<&str> = vec!["Documents", "Music", "Pictures", "Videos"];
    let backup_folders: Vec<String> = Select::new()
        .title("Select folders to backup")
        .options(&all_folders)
        .run_multi_select_values()?;

    if backup_folders.is_empty() {
        return Err(Error::other("no folders selected to backup"));
    }

    let mut encrypt_folders: Vec<String> = vec![];
    let mut passphrase: String = String::new();
    if Confirm::new()
        .message("Do you want to encrypt backups?")
        .run()?
    {
        encrypt_folders = Select::new()
            .title("Select folders to encrypt")
            .options(&backup_folders)
            .run_multi_select_values()?;

        if encrypt_folders.is_empty() {
            return Err(Error::other("no folders selected to encrypt"));
        }

        passphrase = Text::new()
            .message("Encryption Passphrase:")
            .required(true)
            .secret(true)
            .confirm(true)
            .run()?;
    }

    for folder in backup_folders {
        let tar_file: String = format!("{backup_dir}/{folder}.tar.gz");
        let crypt_file: String = format!("{backup_dir}/{folder}.tar.gz.gpg");

        Operation::new()
            .command(&format!("rm -f {}", &tar_file))
            .run()?;
        Operation::new()
            .command(&format!("rm -f {}", &crypt_file))
            .run()?;

        println!("Compressing {}...", &folder);
        Operation::new()
            .command(format!("tar --exclude-vcs -cvzf {} {}", &tar_file, &folder))
            .directory(&home_dir)
            .run()?;

        if encrypt_folders.contains(&folder) {
            println!("Encrypting {}...", &folder);
            Operation::new()
                .command(format!(
                    "gpg --batch -c --passphrase {} {}",
                    &passphrase, &tar_file
                ))
                .directory(&home_dir)
                .run()?;

            Operation::new()
                .command(&format!("rm -f {}", &tar_file))
                .run()?;
        }
    }

    Ok(())
}
