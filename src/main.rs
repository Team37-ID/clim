use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use figlet_rs::FIGfont;
use std::process::Command;

#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";
pub const PNPM: &'static str = "pnpm.cmd";
pub const YARN: &'static str = "yarn.cmd";
pub const PIP: &'static str = "pip";
pub const RUSTUP: &'static str = "rustup";

fn main() -> std::io::Result<()> {
    // Clear the terminal before displaying the CLIM ASCII art
    clearscreen::clear().expect("Failed to clear screen");

    // Display the CLIM ASCII art in ANSI Shadow font
    let ansi_shadow_font = FIGfont::from_file("resources/ANSIShadow.flf").unwrap();
    let figure = ansi_shadow_font.convert("CLIM");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());

    // Create a selectable options in the terminal
    let items: Vec<&str> = vec!["Upgrade", "Check version"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an item")
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    let upgrade_items: Vec<&str> = vec!["NPM", "Yarn", "PNPM", "Pip", "Rustup"];
    let upgrade_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select which package manager to upgrade")
        .items(&upgrade_items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => {
            if index == 0 {
                match upgrade_selection {
                    Some(index) => {
                        if cfg!(target_os = "windows") {
                            if index == 0 || index == 1 || index == 2 {
                                let command = Command::new(NPM)
                                    .args(&["install", "-g", &upgrade_items[index].to_lowercase()])
                                    .output()
                                    .expect("Failed to execute process");

                                println!("{}", String::from_utf8_lossy(&command.stdout));

                                println!(
                                    "{} has been upgraded successfully!!",
                                    upgrade_items[index]
                                );
                            }

                            if index == 3 {
                                let command = Command::new(PIP)
                                    .args(&[
                                        "install",
                                        "--upgrade",
                                        &upgrade_items[index].to_lowercase(),
                                    ])
                                    .output()
                                    .expect("Failed to execute process");

                                println!("{}", String::from_utf8_lossy(&command.stdout));

                                println!(
                                    "{} has been upgraded successfully!!",
                                    upgrade_items[index]
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
                                    upgrade_items[index]
                                );
                            }
                        };
                    }

                    None => println!("User did not select any item"),
                }
            }

            if index == 1 {
                // TODO: Add check package manager version
            }
        }
        None => println!("User did not select any item"),
    }

    Ok(())
}
