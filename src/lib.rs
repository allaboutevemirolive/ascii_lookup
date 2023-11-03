mod test;
use std::collections::HashMap;
/// Control codes are non-printing characters in a character
/// set that do not represent a written symbol or part of the
/// normal character set. They are used to control the actions
/// of the physical terminal/display system, to control the
/// interpretation or display of the following characters,
/// or to provide meta-information about a data stream such as a file.
#[derive(Debug)]
pub struct ControlCharacter {
    pub binary: u8,
    pub oct: u8,
    pub dec: u8,
    pub hex: u8,
    // TODO
    // We have 3 abbr, 1963, 1965, 1967.
    // This field need further refinement.
    pub abbreviation: &'static str,
    pub unicode_picture: char,
    pub caret_notation: &'static str,
    pub c_escape_sequence: &'static str,
    pub name_1967: &'static str,
}

impl ControlCharacter {
    fn new(
        binary: u8,
        oct: u8,
        dec: u8,
        hex: u8,
        abbreviation: &'static str,
        unicode_picture: char,
        caret_notation: &'static str,
        c_escape_sequence: &'static str,
        name_1967: &'static str,
    ) -> Self {
        ControlCharacter {
            binary,
            oct,
            dec,
            hex,
            abbreviation,
            unicode_picture,
            caret_notation,
            c_escape_sequence,
            name_1967,
        }
    }
}

lazy_static::lazy_static! {
    pub static ref TABLE: HashMap<char, ControlCharacter> = {
        let mut table = HashMap::new();
        // TODO
        // Check how many places in decimal
        //
        // Key is Decimal
        // Value is ControlCharacter
        table.insert(
            '0',
            ControlCharacter::new(
                0_000_000,
                000,
                0,
                00,
                // TODO
                "NUL",
                '␀',
                "^@",
                "\\0", // "\0"
                "Null" )
        );

        table.insert(
            '1',
            ControlCharacter::new(
                000_0001,
                001,
                1,
                01,
                // TODO
                "SOH",
                '␁',
                "^A",
                "",
                "Start of Heading" )
        );

        table.insert(
            '2',
            ControlCharacter::new(
                000_0010,
                002,
                2,
                02,
                // TODO
                "STX",
                '␂',
                "^B",
                "",
                "Start of Text" )
        );

        table.insert(
            '3',
            ControlCharacter::new(
                000_0011,
                003,
                3,
                03,
                // TODO
                "ETX",
                '␃',
                "^C",
                "",
                "End of Text" )
        );


        table.insert(
            '4',
            ControlCharacter::new(
                000_0100,
                004,
                4,
                04,
                // TODO
                "EOT",
                '␄',
                "^D",
                "",
                "End of Transmission" )
        );





        table
    };
}

// Printable characters are the characters that represent
// written language symbols, punctuation marks, and other
// visual symbols that can be displayed and printed.
// #[derive(Debug)]
// pub struct PrintableCharacter {
//     binary: &'static str,
//     oct: &'static str,
//     dec: u8,
//     hex: &'static str,
//     glyph: char,
// }

// impl PrintableCharacter {
//     fn new(
//         binary: &'static str,
//         oct: &'static str,
//         dec: u8,
//         hex: &'static str,
//         glyph: char,
//     ) -> Self {
//         PrintableCharacter {
//             binary,
//             oct,
//             dec,
//             hex,
//             glyph,
//         }
//     }
// }

// pub struct CharacterSet {
//     cc: ControlCharacter,
//     pc: PrintableCharacter,
// }
