use figlet_rs::FIGfont;

fn main() {
    // Display the CLIM ASCII art in ANSI Shadow font
    let ansi_shadow_font = FIGfont::from_file("resources/ANSIShadow.flf").unwrap();
    let figure = ansi_shadow_font.convert("CLIM");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
}
