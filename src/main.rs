mod constants;

use constants::*;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    sixty();
}

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

fn disassemble(buffer: &Vec<u8>, index: usize) -> (String, usize) {
    let opcode = buffer[index] as usize;
    let name = constants::OPCODE_NAMES[opcode];
    let size: usize = constants::SIZES[opcode];
    let addressing_type = &ADDRESSING_TYPES[opcode];

    let result: String = match size {
        1 => format!("{:04X}: {:02X}         {}", index,
                     opcode,
                     name),
        2 => format!("{:04X}: {:02X} {:02X}      {} {}", index,
                     opcode, buffer[index + 1],
                     name,
                     addressing_type.to_string(index, buffer[index + 1], 0)),
        _ => format!("{:04X}: {:02X} {:02X} {:02X}   {} {}", index,
                     opcode, buffer[index + 1], buffer[index + 2],
                     name,
                     addressing_type.to_string(index, buffer[index + 1], word(&buffer, index)))
    };

    return (result, size);
}
