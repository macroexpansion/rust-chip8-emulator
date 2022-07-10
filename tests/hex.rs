#[test]
fn combine_two_bytes() {
    let a: u8 = 0xA2;
    let b: u8 = 0xF0;

    let opcode: u16 = (a as u16) << 8 | b as u16;

    assert_eq!(opcode, 0xA2F0);
}

#[test]
fn decode() {
    let opcode: u16 = 0xA2F0;

    let i = opcode & 0x0FFF;
    // println!("{i:02X}");

    assert_eq!(i, 0x2F0);
}

#[test]
fn test() {
    let opcode: u16 = 0xA2F0;

    let a = (opcode & 0x000F) >> 12;
    println!("{a:02x}");
}
