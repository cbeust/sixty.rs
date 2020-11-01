mod constants;

fn main() {
    sixty();
}

use std::io::prelude::*;
use std::fs::File;

fn word(buffer: &Vec<u8>, index: usize) -> u16 {
    return buffer[index + 1] as u16 | ((buffer[index + 2] as u16) << 8);
}

fn sixty() {
    let mut buffer: Vec<u8> = Vec::new();
    let mut f = File::open("6502_functional_test.bin").expect("Couldn't find the file");
    f.read_to_end(&mut buffer);

    let mut i: u16 = 0x400;
    while i < 0x600 {
        let index = i as usize;
        let opcode = buffer[index] as usize;
        let name = constants::OPCODE_NAMES[opcode];
        let size = constants::SIZES[opcode];
        if size == 1 {
            println!("{:04X}: {}", i, name);
        } else if size == 2 {
            println!("{:04X}: {} {:02X}", i, name, buffer[index + 1]);
        } else {
            println!("{:04X}: {} {:X}", i, name, word(&buffer, index));
        }
        i = i + size as u16;
    }
    println!("Read file");
}
