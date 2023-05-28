use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use figlet_rs::FIGfont;
use owo_colors::OwoColorize;
use std::process::Command;

#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";
pub const PNPM: &'static str = "pnpm.cmd";
pub const YARN: &'static str = "yarn.cmd";
pub const PIP: &'static str = "pip";
pub const RUSTUP: &'static str = "rustup";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    clear_terminal().unwrap();
    display_info().unwrap();

    // Create a selectable options in the terminal
    let items: Vec<&str> = vec!["Upgrade", "Check version"];
    let selection = select_options(&items, "Select an options")?;

    let package_managers: Vec<&str> = vec!["NPM", "Yarn", "PNPM", "Pip", "Rustup"];
    let package_managers_selection = select_options(&package_managers, "Select a package manager")?;

    match selection {
        Some(index) => {
            if cfg!(target_os = "windows") {
                if index == 0 {
                    match package_managers_selection {
                        Some(index) => {
                            upgrade_package_manager(package_managers[index]);
                        }
                        None => println!("User did not select any item"),
                    }
                } else if index == 1 {
                    match package_managers_selection {
                        Some(index) => {
                            check_package_manager_version(package_managers[index]);
                        }
                        None => println!("User did not select any item"),
                    }
                }
            }
        }
        None => println!("User did not select any item"),
    }

    Ok(())
}

fn select_options(
    options: &Vec<&str>,
    prompt: &str,
) -> Result<Option<usize>, Box<dyn std::error::Error>> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(options)
        .default(0)
        .interact_on_opt(&Term::stderr())?;
    Ok(selection)
}

fn clear_terminal() -> Result<(), Box<dyn std::error::Error>> {
    clearscreen::clear().expect("Failed to clear screen");
    Ok(())
}

fn display_info() -> Result<(), Box<dyn std::error::Error>> {
    let ansi_shadow_font = FIGfont::from_file("resources/ANSIShadow.flf").unwrap();
    let figure = ansi_shadow_font.convert("CLIM");
    assert!(figure.is_some());
    print!("{}", figure.unwrap());
    println!("Welcome to CLIM (Command Line Interface Manager)!!");
    println!("Author: AlphaByte-RedTeam <andrew.avv03@gmail.com>");
    println!("Current version: {}\n", env!("CARGO_PKG_VERSION"));

    Ok(())
}

fn upgrade_package_manager(package_manager: &str) {
    let command = match package_manager {
        "NPM" | "Yarn" | "PNPM" => Command::new(NPM)
            .args(&["install", "-g", &package_manager.to_lowercase()])
            .output(),
        "Pip" => Command::new(PIP)
            .args(&["install", "--upgrade", "pip"])
            .output(),
        "Rustup" => Command::new(RUSTUP).args(&["update", "stable"]).output(),
        _ => return,
    };

    let output = command.expect("Failed to execute a process");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    // println!("Successfully upgrade {}", package_manager.green());
    println!(
        "{} {}",
        package_manager.green().bold(),
        "Successfuly Upgraded!!".green()
    );
}

fn check_package_manager_version(package_manager: &str) {
    let command = match package_manager {
        "NPM" => Command::new(NPM).arg("--version").output(),
        "Yarn" => Command::new(YARN).arg("--version").output(),
        "PNPM" => Command::new(PNPM).arg("--version").output(),
        "Pip" => Command::new(PIP).arg("--version").output(),
        "Rustup" => Command::new(RUSTUP).arg("--version").output(),
        _ => return,
    };

    let output = command.expect("Failed to execute a process");
    println!(
        "{} {}{}",
        package_manager.green(),
        "Version: ".green(),
        String::from_utf8_lossy(&output.stdout).green().bold()
    );
}
