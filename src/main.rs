use ascii_lookup::*;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    find_dec(input);
}

fn find_dec(input: String) {
    if let Ok(decimal_value) = input.trim().parse::<u8>() {
        if let Some(info) = TABLE.values().find(|&info| info.dec == decimal_value) {
            println!("Binary: {:08}", info.binary);
            println!("Oct: {:03}", info.oct);
            println!("Dec: {}", info.dec);
            println!("Hex: {:02}", info.hex);
            println!("Abbreviation: {}", info.abbreviation);
            println!("Unicode Picture: {}", info.unicode_picture);
            println!("Caret Notation: {}", info.caret_notation);
            println!("C Escape Sequence: {}", info.c_escape_sequence);
            println!("Name (1967): {}", info.name_1967);
        } else {
            println!(
                "No character found with decimal value {} in the table.",
                decimal_value
            );
        }
    } else {
        println!("Invalid input: Please enter a valid decimal value.");
    }
}
