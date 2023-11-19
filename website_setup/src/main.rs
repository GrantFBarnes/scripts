extern crate rust_cli;

use rust_cli::commands::Operation;

use std::env;
use std::fs;
use std::io;

const ANSI_RESET: &str = "\x1b[0m";
const ANSI_CYAN: &str = "\x1b[36m";

fn main() -> Result<(), io::Error> {
    let home_dir = env::var("HOME");
    if home_dir.is_err() {
        return Err(io::Error::other("HOME directory could not be determined"));
    }
    let home_dir: String = home_dir.unwrap();

    let projects: Vec<String> = rust_cli::prompts::Select::new()
        .title("Select projects")
        .options(&vec![
            "home-page",
            "crm",
            "learn-vietnamese",
            "tractor-pulling",
            "vehicle-ownership-cost",
        ])
        .run_multi_select_values()?;
    if projects.is_empty() {
        println!("No projects were chosen");
        return Ok(());
    }

    let git_dir: String = format!("{}/git/grantfbarnes", &home_dir);
    if fs::read_dir(&git_dir).is_err() {
        Operation::new()
            .command(format!("mkdir -p {}", &git_dir))
            .run()?;
    }

    let tar_dir: String = format!("{}/website_deployment_files", &git_dir);
    if fs::read_dir(&tar_dir).is_err() {
        Operation::new()
            .command(format!("mkdir -p {}", &tar_dir))
            .run()?;
    }

    for project in projects {
        println!("Working on {}{}{}...", ANSI_CYAN, project, ANSI_RESET);

        let project_dir: String = format!("{}/{}", &git_dir, project);

        if fs::read_dir(&project_dir).is_err() {
            Operation::new()
                .command(format!(
                    "git clone git@github.com:GrantFBarnes/{}.git",
                    project
                ))
                .directory(&git_dir)
                .show_output(true)
                .run()?;
        }

        Operation::new()
            .command(format!("rm -rf {}/node_modules", &project_dir))
            .run()?;
        Operation::new()
            .command(format!("rm -rf {}/dist", &project_dir))
            .run()?;

        Operation::new()
            .command("npm install")
            .directory(&project_dir)
            .show_output(true)
            .run()?;
        Operation::new()
            .command("npm run build")
            .directory(&project_dir)
            .show_output(true)
            .run()?;

        let tar_file: String = format!("{}/{}.tar.gz", &tar_dir, project);
        Operation::new()
            .command(format!("rm -rf {}", &tar_file))
            .run()?;
        Operation::new()
            .command(format!(
                "tar --exclude-vcs -cvzf {} {}",
                &tar_file, &project
            ))
            .directory(&git_dir)
            .run()?;
    }
    Ok(())
}
