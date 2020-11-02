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

    let mut i: usize = 0x600;
    while i < 0x700 {
        let (s, size) = disassemble(&buffer, i);
        println!("{}", s);
        i += size;
    }
}

use constants::*;

fn disassemble(buffer: &Vec<u8>, index: usize) -> (String, usize) {
    let opcode = buffer[index] as usize;
    let name = constants::OPCODE_NAMES[opcode];
    let size: usize = constants::SIZES[opcode];
    let addressingType = &ADDRESSING_TYPES[opcode];

    let result: String;
    if size == 1 {
        result = format!("{:04X}: {}", index, name);
    } else if size == 2 {
        result = format!("{:04X}: {} {:}", index, name,
             addressingType.to_string(index, buffer[index + 1], 0))
    } else {
        result = format!("{:04X}: {} {}", index, name,
            addressingType.to_string(index, buffer[index + 1], word(&buffer, index)));
    };

    return (result, size);
}
