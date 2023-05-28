use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use figlet_rs::FIGfont;
use std::process::{Command, Stdio};

#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";
pub const PNPM: &'static str = "pnpm.cmd";
pub const YARN: &'static str = "yarn.cmd";
pub const PIP: &'static str = "pip";
pub const RUSTUP: &'static str = "rustup";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    clear_terminal();
    display_info();

    // Create a selectable options in the terminal
    let items: Vec<&str> = vec!["Upgrade", "Check version"];
    let selection = select_options(&items, "Select an options")?;

    let package_managers: Vec<&str> = vec!["NPM", "Yarn", "PNPM", "Pip", "Rustup"];
    let package_managers_selection = select_options(&package_managers, "Select a package manager")?;

    match selection {
        Some(index) => {
            if index == 0 {
                match package_managers_selection {
                    Some(index) => {
                        if cfg!(target_os = "windows") {
                            if index == 0 || index == 1 || index == 2 {
                                let command = Command::new(NPM)
                                    .args(&[
                                        "install",
                                        "-g",
                                        &package_managers[index].to_lowercase(),
                                    ])
                                    .output()
                                    .expect("Failed to execute process");

                                println!("{}", String::from_utf8_lossy(&command.stdout));

                                println!(
                                    "{} has been upgraded successfully!!",
                                    package_managers[index]
                                );
                            }

                            if index == 3 {
                                let command = Command::new(PIP)
                                    .args(&[
                                        "install",
                                        "--upgrade",
                                        &package_managers[index].to_lowercase(),
                                    ])
                                    .output()
                                    .expect("Failed to execute process");

                                println!("{}", String::from_utf8_lossy(&command.stdout));

                                println!(
                                    "{} has been upgraded successfully!!",
                                    package_managers[index]
                                );
                            }

                            if index == 4 {
                                let command = Command::new(RUSTUP)
                                    .args(&["update", "stable"])
                                    .output()
                                    .expect("Failed to execute process");

                                println!("{}", String::from_utf8_lossy(&command.stdout));

                                println!(
                                    "{} has been upgraded successfully!!",
                                    package_managers[index]
                                );
                            }
                        };
                    }

                    None => println!("User did not select any item"),
                }
            } else if index == 1 {
                match package_managers_selection {
                    Some(index) => {
                        if cfg!(target_os = "windows") {
                            if index == 0 {
                                let command = Command::new(NPM)
                                    .args(&["--version"])
                                    .output()
                                    .expect("Failed to execute process");

                                println!("{}", String::from_utf8_lossy(&command.stdout));
                            } else if index == 1 {
                                let command = Command::new(YARN)
                                    .args(&["--version"])
                                    .output()
                                    .expect("Failed to execute process");

                                println!("{}", String::from_utf8_lossy(&command.stdout));
                            } else if index == 2 {
                                let command = Command::new(PNPM)
                                    .args(&["--version"])
                                    .output()
                                    .expect("Failed to execute process");

                                println!("{}", String::from_utf8_lossy(&command.stdout));
                            } else if index == 3 {
                                let command = Command::new(PIP)
                                    .args(&["--version"])
                                    .output()
                                    .expect("Failed to execute process");

                                println!("{}", String::from_utf8_lossy(&command.stdout));
                            } else if index == 4 {
                                let command = Command::new(RUSTUP)
                                    .args(&["--version"])
                                    .output()
                                    .expect("Failed to execute process");

                                println!("{}", String::from_utf8_lossy(&command.stdout));
                            }
                        };
                    }

                    None => println!("User did not select any item"),
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
