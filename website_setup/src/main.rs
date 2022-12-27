use dialoguer::MultiSelect;
use std::env::VarError;
use std::io::Result;
use std::process::{Command, Stdio};
use std::{env, fs};

const ANSI_RESET: &str = "\x1b[0m";
const ANSI_CYAN: &str = "\x1b[36m";

fn select_options(options: &Vec<&str>, prompt: &str) -> Option<Vec<usize>> {
    let selection: Result<Option<Vec<usize>>> = MultiSelect::new()
        .with_prompt(prompt)
        .items(options)
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

fn git_clone(project: &str, directory: &String) {
    let _ = Command::new("git")
        .arg("clone")
        .arg(format!("git@github.com:GrantFBarnes/{}.git", project))
        .current_dir(directory)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to git clone")
        .wait();
}

fn make_directory(folder: &String) {
    let _ = Command::new("mkdir")
        .arg("-p")
        .arg(folder)
        .status()
        .expect("failed to make directory");
}

fn remove_folder(folder: &String) {
    let _ = Command::new("rm")
        .arg("-rf")
        .arg(folder)
        .status()
        .expect("failed to remove folder");
}

fn npm_install(directory: &String) {
    let _ = Command::new("npm")
        .arg("install")
        .current_dir(directory)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to npm install")
        .wait();
}

fn npm_build(directory: &String) {
    let _ = Command::new("npm")
        .arg("run")
        .arg("build")
        .current_dir(directory)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to npm build")
        .wait();
}

fn tar_folder(file: &String, folder: &String, directory: &String) {
    let _ = Command::new("tar")
        .arg("--exclude-vcs")
        .arg("-cvzf")
        .arg(file)
        .arg(folder)
        .current_dir(directory)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to tar folder")
        .wait();
}

fn main() {
    let all_projects: Vec<&str> = vec![
        "home-page",
        "crm",
        "learn-vietnamese",
        "tractor-pulling",
        "vehicle-ownership-cost",
    ];

    let project_selection: Option<Vec<usize>> = select_options(&all_projects, "Select projects");
    if project_selection.is_none() {
        println!("No projects were chosen");
        return;
    }
    let project_selection: Vec<usize> = project_selection.unwrap();

    let mut projects: Vec<&str> = vec![];
    for idx in project_selection {
        projects.push(all_projects[idx]);
    }

    let home_dir: std::result::Result<String, VarError> = env::var("HOME");
    if home_dir.is_err() {
        println!("HOME directory could not be determined");
        return;
    }
    let home_dir: String = home_dir.unwrap();

    let git_dir: String = format!("{}/git", &home_dir);
    match fs::read_dir(&git_dir) {
        Err(_) => make_directory(&git_dir),
        _ => (),
    }

    let tar_dir: String = format!("{}/website_deployment_files", &git_dir);
    match fs::read_dir(&tar_dir) {
        Err(_) => make_directory(&tar_dir),
        _ => (),
    }

    for project in projects {
        println!("Working on {}{}{}...", ANSI_CYAN, project, ANSI_RESET);

        let project_dir: String = format!("{}/{}", &git_dir, project);

        match fs::read_dir(&project_dir) {
            Err(_) => git_clone(project, &git_dir),
            _ => (),
        }

        remove_folder(&format!("{}/node_modules", &project_dir));
        remove_folder(&format!("{}/dist", &project_dir));

        npm_install(&project_dir);
        npm_build(&project_dir);

        let tar_file: String = format!("{}/{}.tar.gz", &tar_dir, project);
        remove_folder(&tar_file);
        tar_folder(&tar_file, &project.to_string(), &git_dir);
    }
}
