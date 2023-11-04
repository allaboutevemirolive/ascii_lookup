mod test;
use std::collections::HashMap;

// INFO
// https://en.wikipedia.org/wiki/Unicode_control_characters
// https://en.wikipedia.org/wiki/ASCII

/// Control codes are non-printing characters in a character
/// set that do not represent a written symbol or part of the
/// normal character set. They are used to control the actions
/// of the physical terminal/display system, to control the
/// interpretation or display of the following characters,
/// or to provide meta-information about a data stream such as a file.
#[derive(Debug)]
pub struct ControlCharacter {
    pub binary: &'static str,
    pub oct: &'static str,
    pub dec: &'static str,
    pub hex: &'static str,
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
        binary: i32,
        oct: i32,
        dec: i32,
        hex: i32,
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
    pub static ref TABLE: HashMap<i32, ControlCharacter> = {
        let mut table = HashMap::new();
        // TODO
        // Check how many places in decimal
        //
        // Key is Decimal
        // Value is ControlCharacter
        table.insert(
            0,
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
            1,
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
            2,
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
            3,
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
            4,
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

        table.insert(
            5,
            ControlCharacter::new(
                000_0101,
                005,
                5,
                05,
                // TODO
                "ENQ",
                '␅',
                "^E",
                "",
                "Enquiry"
            )
        );

        table.insert(
            6,
            ControlCharacter::new(
                000_0110,
                006,
                6,
                06,
                // TODO
                "ACK",
                '␆',
                "^F",
                "",
                "Acknowledgement"
            )
        );

        table.insert(
            7,
            ControlCharacter::new(
                000_0111,
                007,
                7,
                07,
                // TODO
                "BEL",
                '␇',
                "^G",
                "\\a",
                "Bell"
            )
        );

        // ---------------------------------------



        
        // table.insert(
        //     '7',
        //     ControlCharacter::new(
        //         000_0111,
        //         7,
        //         7,
        //         7,
        //         "BELL",
        //         '␇',
        //         "^G",
        //         "\\a",
        //         "Bell"
        //     )
        // );
        
        table.insert(
            8,
            ControlCharacter::new(
                000_1000,
                010,
                8,
                08,
                "BS",
                '␈',
                "^H",
                "\\b",
                "Backspace"
            )
        );
        
        table.insert(
            9,
            ControlCharacter::new(
                000_1001,
                011,
                9,
                09,
                "HT",
                '␉',
                "^I",
                "\\t",
                "Horizontal Tab"
            )
        );
        
        table.insert(
            10,
            ControlCharacter::new(
                000_1010,
                012,
                10,
                10,
                "LF",
                '␊',
                "^J",
                "\\n",
                "Line Feed"
            )
        );
        
        table.insert(
            'B',
            ControlCharacter::new(
                000_1011,
                11,
                11,
                11,
                "VTAB",
                '␋',
                "^K",
                "\\v",
                "Vertical Tab"
            )
        );
        
        table.insert(
            'C',
            ControlCharacter::new(
                000_1100,
                12,
                12,
                12,
                "FF",
                '␌',
                "^L",
                "\\f",
                "Form Feed"
            )
        );
        
        table.insert(
            'D',
            ControlCharacter::new(
                000_1101,
                13,
                13,
                13,
                "CR",
                '␍',
                "^M",
                "\\r",
                "Carriage Return"
            )
        );
        
        table.insert(
            'E',
            ControlCharacter::new(
                000_1110,
                14,
                14,
                14,
                "SO",
                '␎',
                "^N",
                "",
                "Shift Out"
            )
        );
        
        table.insert(
            'F',
            ControlCharacter::new(
                000_1111,
                15,
                15,
                15,
                "SI",
                '␏',
                "^O",
                "",
                "Shift In"
            )
        );
        
        table.insert(
            'P',
            ControlCharacter::new(
                001_0000,
                16,
                16,
                16,
                "DLE",
                '␐',
                "^P",
                "",
                "Data Link Escape"
            )
        );
        
        table.insert(
            'Q',
            ControlCharacter::new(
                001_0001,
                17,
                17,
                17,
                "DC1",
                '␑',
                "^Q",
                "",
                "Device Control 1"
            )
        );
        
        table.insert(
            'R',
            ControlCharacter::new(
                001_0010,
                18,
                18,
                18,
                "DC2",
                '␒',
                "^R",
                "",
                "Device Control 2"
            )
        );
        
        table.insert(
            'S',
            ControlCharacter::new(
                001_0011,
                19,
                19,
                19,
                "DC3",
                '␓',
                "^S",
                "",
                "Device Control 3"
            )
        );
        
        table.insert(
            'T',
            ControlCharacter::new(
                001_0100,
                20,
                20,
                20,
                "DC4",
                '␔',
                "^T",
                "",
                "Device Control 4"
            )
        );
        
        table.insert(
            'U',
            ControlCharacter::new(
                001_0101,
                21,
                21,
                21,
                "NAK",
                '␕',
                "^U",
                "",
                "Negative Acknowledgment"
            )
        );
        
        table.insert(
            'V',
            ControlCharacter::new(
                001_0110,
                22,
                22,
                22,
                "SYN",
                '␖',
                "^V",
                "",
                "Synchronous Idle"
            )
        );
        
        table.insert(
            'W',
            ControlCharacter::new(
                001_0111,
                23,
                23,
                23,
                "ETB",
                '␗',
                "^W",
                "",
                "End of Transmission Block"
            )
        );
        
        table.insert(
            'X',
            ControlCharacter::new(
                001_1000,
                24,
                24,
                24,
                "CAN",
                '␘',
                "^X",
                "",
                "Cancel"
            )
        );
        
        table.insert(
            'Y',
            ControlCharacter::new(
                001_1001,
                25,
                25,
                25,
                "EM",
                '␙',
                "^Y",
                "",
                "End of Medium"
            )
        );
        
        table.insert(
            'Z',
            ControlCharacter::new(
                001_1010,
                26,
                26,
                26,
                "SUB",
                '␚',
                "^Z",
                "",
                "Substitute"
            )
        );
        
        table.insert(
            '[',
            ControlCharacter::new(
                001_1011,
                27,
                27,
                27,
                "ESC",
                '␛',
                "^[",
                "\\e",
                "Escape"
            )
        );
        
        table.insert(
            '\\',
            ControlCharacter::new(
                001_1100,
                28,
                28,
                28,
                "FS",
                '␜',
                "^\\",
                "",
                "File Separator"
            )
        );
        
        table.insert(
            ']',
            ControlCharacter::new(
                001_1101,
                29,
                29,
                29,
                "GS",
                '␝',
                "^]",
                "",
                "Group Separator"
            )
        );
        
        table.insert(
            '^',
            ControlCharacter::new(
                001_1110,
                30,
                30,
                30,
                "RS",
                '␞',
                "^^[",
                "",
                "Record Separator"
            )
        );
        
        table.insert(
            '_',
            ControlCharacter::new(
                001_1111,
                31,
                31,
                31,
                "US",
                '␟',
                "^_",
                "",
                "Unit Separator"
            )
        );
        
        table.insert(
            '',
            ControlCharacter::new(
                111_1111,
                127,
                127,
                7F,
                "DEL",
                '␡',
                "^?",
                "",
                "Delete"
            )
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
