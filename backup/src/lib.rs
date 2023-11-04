struct ASCIICharacter {
    character: char,
    decimal: u8,
    binary: String,
    hexadecimal: String,
}

impl ASCIICharacter {
    fn new(character: char) -> Self {
        let decimal = character as u8;
        ASCIICharacter {
            character,
            decimal,
            binary: format!("{:08b}", decimal),
            hexadecimal: format!("{:02x}", decimal),
        }
    }
}
