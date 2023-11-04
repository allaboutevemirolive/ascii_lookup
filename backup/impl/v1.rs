use std::collections::HashMap;

struct CharInfo {
    ascii_value: u8,
    binary_representation: &'static str,
    decimal_value: u8,
}

impl CharInfo {
    fn new(ascii_value: u8, binary_representation: &'static str, decimal_value: u8) -> Self {
        CharInfo {
            ascii_value,
            binary_representation,
            decimal_value,
        }
    }
}

fn main() {
    let mut table = HashMap::new();
    table.insert('0', CharInfo::new(48, "00110000", 30));
    table.insert('1', CharInfo::new(49, "00110001", 31));
    table.insert('2', CharInfo::new(50, "00110010", 32));
    table.insert('3', CharInfo::new(51, "00110011", 33));

    println!("Enter a sequence of characters (e.g., '123'):");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Find character
    for character in input.trim().chars() {
        if let Some(info) = table.get(&character) {
            println!("Character: {}", character);
            println!("ASCII Value: {}", info.ascii_value);
            println!("Binary Representation: {}", info.binary_representation);
            println!("Decimal Value: {}", info.decimal_value);
            println!(); // Add a newline for separation
        } else {
            println!("Character '{}' not found in the table.", character);
        }
    }
}
