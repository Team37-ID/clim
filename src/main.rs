use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use figlet_rs::FIGfont;

fn main() -> std::io::Result<()> {
    // Clear the terminal before displaying the CLIM ASCII art
    clearscreen::clear().expect("Failed to clear screen");

    // Display the CLIM ASCII art in ANSI Shadow font
    let ansi_shadow_font = FIGfont::from_file("resources/ANSIShadow.flf").unwrap();
    let figure = ansi_shadow_font.convert("CLIM");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());

    // Create a selectable options in the terminal
    let items: Vec<&str> = vec!["Install", "Upgrade", "Uninstall", "Check version"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an item")
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => {
            println!("User selected item: {}", items[index]);
        }
        None => println!("User did not select any item"),
    }

    Ok(())
}
