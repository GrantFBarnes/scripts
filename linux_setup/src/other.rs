use std::io;
use std::process::{Command, Stdio};

use crate::Info;

const PACKAGES: [&str; 1] = ["rust"];

pub fn is_available(package: &str) -> bool {
    PACKAGES.contains(&package)
}

pub fn is_installed(package: &str, info: &Info) -> bool {
    if info.other_installed.contains(&package.to_string()) {
        return true;
    }
    false
}

pub fn install(package: &str, info: &mut Info) -> Result<(), io::Error> {
    if is_available(package) {
        if !is_installed(package, info) {
            info.other_installed.push(package.to_owned());

            println!("Installing other {}...", package);

            match package {
                "rust" => {
                    let curl_cmd = Command::new("curl")
                        .arg("'=https'")
                        .arg("--tlsv1.2")
                        .arg("-sSf")
                        .arg("https://sh.rustup.rs")
                        .stdout(Stdio::piped())
                        .spawn()?;
                    Command::new("sh")
                        .stdin(Stdio::from(curl_cmd.stdout.unwrap()))
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()?
                        .wait()?;
                }
                _ => (),
            }
        }
    }
    Ok(())
}

pub fn uninstall(package: &str, info: &mut Info) -> Result<(), io::Error> {
    if is_available(package) {
        if is_installed(package, info) {
            let index: Option<usize> = info.other_installed.iter().position(|x| *x == package);
            if index.is_some() {
                info.other_installed.remove(index.unwrap());
            }

            println!("Uninstalling other {}...", package);

            match package {
                "rust" => {
                    rust_cli::commands::Operation::new()
                        .command("rustup self uninstall")
                        .show_output(true)
                        .run()?;
                }
                _ => (),
            }
        }
    }
    Ok(())
}

pub fn update(info: &Info) -> Result<(), io::Error> {
    println!("Update other...");

    for pkg in PACKAGES {
        if is_installed(pkg, info) {
            match pkg {
                "rust" => {
                    rust_cli::commands::Operation::new()
                        .command("rustup self update")
                        .show_output(true)
                        .run()?;
                    rust_cli::commands::Operation::new()
                        .command("rustup update stable")
                        .show_output(true)
                        .run()?;
                }
                _ => (),
            }
        }
    }
    Ok(())
}

pub fn get_installed() -> Result<Vec<String>, io::Error> {
    let mut packages: Vec<String> = vec![];

    for pkg in PACKAGES {
        match pkg {
            "rust" => match rust_cli::commands::Operation::new()
                .command("rustup --version")
                .run()
            {
                Ok(_) => packages.push(pkg.to_string()),
                _ => (),
            },
            _ => (),
        }
    }

    Ok(packages)
}
