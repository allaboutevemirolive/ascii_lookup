use crate::ControlCharacter;

#[test]
fn invoke_ControlCharacter() {
    let horizontal_tab = ControlCharacter {
        binary: "00001001",
        oct: "011",
        dec: 9,
        hex: "09",
        abbreviation: "HT",
        unicode_picture: '␉',
        caret_notation: "^I",
        c_escape_sequence: "\\t",
        name_1967: "Horizontal Tab",
    };
}
