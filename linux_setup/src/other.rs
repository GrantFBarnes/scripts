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

pub fn install(package: &str, info: &mut Info) {
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
                        .spawn()
                        .unwrap();
                    let _ = Command::new("sh")
                        .stdin(Stdio::from(curl_cmd.stdout.unwrap()))
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()
                        .expect("install rust failed")
                        .wait();
                }
                _ => (),
            }
        }
    }
}

pub fn uninstall(package: &str, info: &mut Info) {
    if is_available(package) {
        if is_installed(package, info) {
            let index: Option<usize> = info.other_installed.iter().position(|x| *x == package);
            if index.is_some() {
                info.other_installed.remove(index.unwrap());
            }

            println!("Uninstalling other {}...", package);

            match package {
                "rust" => {
                    let _ = Command::new("rustup")
                        .arg("self")
                        .arg("uninstall")
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()
                        .expect("uninstall rust failed")
                        .wait();
                }
                _ => (),
            }
        }
    }
}

pub fn update(info: &Info) {
    println!("Update other...");

    for pkg in PACKAGES {
        if is_installed(pkg, info) {
            match pkg {
                "rust" => {
                    let _ = Command::new("rustup")
                        .arg("self")
                        .arg("update")
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()
                        .expect("update rustup failed")
                        .wait();

                    let _ = Command::new("rustup")
                        .arg("update")
                        .arg("stable")
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()
                        .expect("update rust failed")
                        .wait();
                }
                _ => (),
            }
        }
    }
}

pub fn get_installed() -> Vec<String> {
    let mut packages: Vec<String> = vec![];

    for pkg in PACKAGES {
        match pkg {
            "rust" => match Command::new("rustup").arg("--version").output() {
                Ok(_) => packages.push(pkg.to_string()),
                _ => (),
            },
            _ => (),
        }
    }

    return packages;
}