use rand::Rng;

fn main() {
    let control_characters: [char; 33] = [
        '\u{0000}', '\u{0001}', '\u{0002}', '\u{0003}', '\u{0004}', '\u{0005}', '\u{0006}', '\u{0007}',
        '\u{0008}', '\u{0009}', '\u{000A}', '\u{000B}', '\u{000C}', '\u{000D}', '\u{000E}', '\u{000F}',
        '\u{0010}', '\u{0011}', '\u{0012}', '\u{0013}', '\u{0014}', '\u{0015}', '\u{0016}', '\u{0017}',
        '\u{0018}', '\u{0019}', '\u{001A}', '\u{001B}', '\u{001C}', '\u{001D}', '\u{001E}', '\u{001F}',
        '\u{007F}'
    ];

    // let mut rng = rand::thread_rng();

    // for _ in 0..100 {
    //     let random_index = rng.gen_range(0..32);
    //     print!("{}", control_characters[random_index]);
    // }
    // println!();

    // ---------------------

    let control_characters_binary: [&str; 33] = [
        "00000000", "00000001", "00000010", "00000011", "00000100", "00000101", "00000110", "00000111",
        "00001000", "00001001", "00001010", "00001011", "00001100", "00001101", "00001110", "00001111",
        "00100000", "00100001", "00100010", "00100011", "00100100", "00100101", "00100110", "00100111",
        "00101000", "00101001", "00101010", "00101011", "00101100", "00101101", "00101110", "00101111",
        "11111111"
    ];

    for (i, binary) in control_characters_binary.iter().enumerate() {
        println!("{:03} {} {}", i, binary, i);
    }

    // ---------------------

    let control_characters_octal: [&str; 33] = [
        "000", "001", "002", "003", "004", "005", "006", "007",
        "010", "011", "012", "013", "014", "015", "016", "017",
        "020", "021", "022", "023", "024", "025", "026", "027",
        "030", "031", "032", "033", "034", "035", "036", "037",
        "177"
    ];

    for (i, octal) in control_characters_octal.iter().enumerate() {
        println!("{:03} {} {}", i, octal, i);
    }

    // ---------------------

    let control_characters_dec: [u32; 33] = [
        0, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31,
        127
    ];

    for (i, dec) in control_characters_dec.iter().enumerate() {
        println!("{:03} {}", i, dec);
    }

    // ----------------------

    let control_characters_hex: [&str; 33] = [
        "00", "01", "02", "03", "04", "05", "06", "07",
        "08", "09", "0A", "0B", "0C", "0D", "0E", "0F",
        "10", "11", "12", "13", "14", "15", "16", "17",
        "18", "19", "1A", "1B", "1C", "1D", "1E", "1F",
        "7F"
    ];

    for (i, hex) in control_characters_hex.iter().enumerate() {
        println!("{:03} 0x{}", i, hex);
    }

    // -----------------------

    let control_characters_unicode: [&str; 33] = [
        "␀", "␁", "␂", "␃", "␄", "␅", "␆", "␇",
        "␈", "␉", "␊", "␋", "␌", "␍", "␎", "␏",
        "␐", "␑", "␒", "␓", "␔", "␕", "␖", "␗",
        "␘", "␙", "␚", "␛", "␜", "␝", "␞", "␟",
        "␡"
    ];

    for (i, unicode) in control_characters_unicode.iter().enumerate() {
        println!("{:03} {}", i, unicode);
    }

    // ----------------------

    let control_characters_caret: [&str; 33] = [
        "^@", "^A", "^B", "^C", "^D", "^E", "^F", "^G",
        "^H", "^I", "^J", "^K", "^L", "^M", "^N", "^O",
        "^P", "^Q", "^R", "^S", "^T", "^U", "^V", "^W",
        "^X", "^Y", "^Z", "^[", "^\\", "^]", "^^[", "^_",
        "^?"
    ];

    for (i, caret) in control_characters_caret.iter().enumerate() {
        println!("{:03} {}", i, caret);
    }

    // ---------------------- 

    let control_characters_c_escape: [&str; 33] = [
        "\\0", "\\x01", "\\x02", "\\x03", "\\x04", "\\x05", "\\x06", "\\a",
        "\\b", "\\t", "\\n", "\\v", "\\f", "\\r", "\\x0E", "\\x0F",
        "\\x10", "\\x11", "\\x12", "\\x13", "\\x14", "\\x15", "\\x16", "\\x17",
        "\\x18", "\\x19", "\\x1A", "\\e", "\\x1C", "\\x1D", "\\x1E", "\\x1F",
        "\\x7F"
    ];

    for (i, c_escape) in control_characters_c_escape.iter().enumerate() {
        println!("{:03} {}", i, c_escape);
    }

    // ---------------------- 

    let control_characters_names_1967: [&str; 33] = [
        "Null", "Start of Heading", "Start of Text", "End of Text",
        "End of Transmission", "Enquiry", "Acknowledgement", "Bell",
        "Backspace", "Horizontal Tab", "Line Feed", "Vertical Tab",
        "Form Feed", "Carriage Return", "Shift Out", "Shift In",
        "Data Link Escape", "Device Control 1 (often XON)", "Device Control 2",
        "Device Control 3 (often XOFF)", "Device Control 4", "Negative Acknowledgement",
        "Synchronous Idle", "End of Transmission Block", "Cancel", "End of Medium",
        "Substitute", "Escape", "File Separator", "Group Separator",
        "Record Separator", "Unit Separator", "Delete"
    ];

    for (i, name) in control_characters_names_1967.iter().enumerate() {
        println!("{:03} {}", i, name);
    }
}
