use crate::ControlCharacter;

#[test]
fn invoke_ControlCharacter() {
    let horizontal_tab = ControlCharacter::new(
        000_0110,
        006,
        6,
        06,
        // TODO
        "ACK",
        '␆',
        "^F",
        "",
        "Acknowledgement",
    );
}
