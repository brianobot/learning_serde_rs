#![allow(dead_code,unused_variables)]

const BYTES_PER_LINE: usize = 16;
// multiline string written as raw string literal r## does not need quote escapping
// the preceeding b indicates to the compiler to treat the value as bytes (&[u8]) and not utf-8 text (&str)
const INPUT: &'static [u8] = br#"
    fn main() {
        println!("Hello World");
    }
"#;

pub fn hex_dump() -> Result<(), ()> {
    let buffer = INPUT.to_vec();

    let mut position_in_input = 0;
    for line in buffer.chunks(BYTES_PER_LINE) {
        print!("[0x{:08x}]", position_in_input);
        for byte in line {
            print!("{:02x}", byte);
        }
        println!(""); // shortcut to print a newline to 
        position_in_input += BYTES_PER_LINE;
    }

    Ok(())
}