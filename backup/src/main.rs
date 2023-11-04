#[derive(Debug)]
pub struct ControlCharacter {
    abbreviation: Abbreviation,
    name_1967: String,
    decimal: i32,
}

impl ControlCharacter {
    fn new(abbreviation: Abbreviation, name_1967: &str, decimal: i32) -> Self {
        Self {
            abbreviation,
            name_1967: name_1967.to_string(),
            decimal,
        }
    }

    fn get_abbreviation(&self) -> &Abbreviation {
        &self.abbreviation
    }

    fn get_name_1967(&self) -> &String {
        &self.name_1967
    }

    fn get_decimal(&self) -> &i32 {
        &self.decimal
    }
}

#[derive(Debug)]
pub struct Abbreviation {
    y1963: String,
    y1965: String,
    y1967: String,
}

fn main() {
    let control_characters: Vec<ControlCharacter> = vec![
        ControlCharacter::new(
            Abbreviation {
                y1963: "NULL".to_string(),
                y1965: "NUL".to_string(),
                y1967: "NUL".to_string(),
            },
            "Null",
            0,
        ),
        ControlCharacter::new(
            Abbreviation {
                y1963: "SOM".to_string(),
                y1965: "SOH".to_string(),
                y1967: "SOH".to_string(),
            },
            "Start of Heading",
            1,
        ),
        ControlCharacter::new(
            Abbreviation {
                y1963: "EOA".to_string(),
                y1965: "STX".to_string(),
                y1967: "STX".to_string(),
            },
            "Start of Text",
            2,
        ),
        ControlCharacter::new(
            Abbreviation {
                y1963: "EOM".to_string(),
                y1965: "ETX".to_string(),
                y1967: "ETX".to_string(),
            },
            "End of Text",
            3,
        ),
        ControlCharacter::new(
            Abbreviation {
                y1963: "EOT".to_string(),
                y1965: "EOT".to_string(),
                y1967: "EOT".to_string(),
            },
            "End of Transmission",
            4,
        ),
        ControlCharacter::new(
            Abbreviation {
                y1963: "WRU".to_string(),
                y1965: "ENQ".to_string(),
                y1967: "ENQ".to_string(),
            },
            "Enquiry",
            5,
        ),
        ControlCharacter::new(
            Abbreviation {
                y1963: "RU".to_string(),
                y1965: "ACK".to_string(),
                y1967: "ACK".to_string(),
            },
            "Acknowledgement",
            6,
        ),
        ControlCharacter::new(
            Abbreviation {
                y1963: "BELL".to_string(),
                y1965: "BEL".to_string(),
                y1967: "BEL".to_string(),
            },
            "Bell",
            7,
        ),
        ControlCharacter::new(
            Abbreviation {
                y1963: "FE0".to_string(),
                y1965: "BS".to_string(),
                y1967: "BS".to_string(),
            },
            "Backspace",
            8,
        ),
        ControlCharacter::new(
            Abbreviation {
                y1963: "HT/SK".to_string(),
                y1965: "HT".to_string(),
                y1967: "HT".to_string(),
            },
            "Horizontal Tab",
            9,
        ),
        ControlCharacter::new(
            Abbreviation {
                y1963: "LF".to_string(),
                y1965: "LF".to_string(),
                y1967: "LF".to_string(),
            },
            "Line Feed",
            10,
        ),


        // ControlCharacter::new(
        //     Abbreviation("SOH".to_string(), "".to_string(), "".to_string()),
        //     "Start of Heading",
        // ),
        // ControlCharacter::new(
        //     Abbreviation("STX".to_string(), "".to_string(), "".to_string()),
        //     "Start of Text",
        // ),
        // ControlCharacter::new(
        //     Abbreviation("ETX".to_string(), "".to_string(), "".to_string()),
        //     "End of Text",
        // ),
        // ... (add all the control characters)
    ];

    for character in &control_characters {
        // println!(
        //     "Decimal: {} Name (1967): {} Abbreviation: {:?}  ",
        //     character.get_decimal(),
        //     character.get_name_1967(),
        //     character.get_abbreviation(),
        // );

        println!("{:?}", character);
        println!();
    }
}
