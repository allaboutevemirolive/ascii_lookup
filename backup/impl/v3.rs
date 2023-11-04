use std::collections::HashMap;

struct CharInfo {
    character: char,
    ascii_value: u8,
    binary_representation: &'static str,
    decimal_value: u8,
}

impl CharInfo {
    fn new(character: char, ascii_value: u8, binary_representation: &'static str, decimal_value: u8) -> Self {
        CharInfo {
            character,
            ascii_value,
            binary_representation,
            decimal_value,
        }
    }
}

lazy_static::lazy_static! {
    static ref TABLE: HashMap<char, CharInfo> = {
        let mut table = HashMap::new();
        table.insert('0', CharInfo::new('0', 48, "00110000", 30));
        table.insert('1', CharInfo::new('1', 49, "00110001", 31));
        table.insert('2', CharInfo::new('2', 50, "00110010", 32));
        table.insert('3', CharInfo::new('3', 51, "00110011", 33));
        table
    };
}

fn main() {
    println!("Enter a decimal value:");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if let Ok(decimal_value) = input.trim().parse::<u8>() {
        if let Some(info) = TABLE.values().find(|&info| info.decimal_value == decimal_value) {
            println!("Character: {}", info.character);
            println!("ASCII Value: {}", info.ascii_value);
            println!("Binary Representation: {}", info.binary_representation);
        } else {
            println!("No character found with decimal value {} in the table.", decimal_value);
        }
    } else {
        println!("Invalid input: Please enter a valid decimal value.");
    }
}
