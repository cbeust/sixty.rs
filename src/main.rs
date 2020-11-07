mod constants;
mod cpu;

use crate::cpu::Cpu;
use constants::*;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    sixty();
}

pub trait Memory {
    fn get(&self, index: usize) -> u8;
    fn set(&mut self, index: usize, value: u8);
    fn load(&mut self, file_name: &str);
    fn word(&self, address: usize) -> u16 {
        self.get(address) as u16 | ((self.get(address + 1) as u16) << 8)
    }
    // fn disassemble(&mut self, index: usize) -> (String, usize);
}

impl dyn Memory {
    pub fn disassemble(&mut self, index: usize) -> (String, usize) {
        let opcode = self.get(index) as usize;
        let size: usize = constants::SIZES[opcode];
        let mut bytes = Vec::new();
        bytes.push(opcode as u8);
        if size >= 2 {
            bytes.push(self.get(index + 1));
        }
        if size >= 3 {
            bytes.push(self.get(index + 2));
        }
        return disassemble3(index, bytes);
    }
}

struct SimpleMemory {
    buffer: Vec<u8>
}

impl SimpleMemory {
    fn new(file_name: &str) -> SimpleMemory {
        let mut result = SimpleMemory{
            buffer: Vec::new()
        };
        result.load(file_name);
        result
    }
}

impl Memory for SimpleMemory {
    fn get(&self, index: usize) -> u8 {
        self.buffer[index]
    }

    fn set(&mut self, index: usize, value: u8) {
        self.buffer[index] = value
    }

    fn load(&mut self, file_name: &str) {
        let mut f = File::open(file_name).expect("Couldn't find the file");
        f.read_to_end(&mut self.buffer).expect("Could not find file {}");
    }
}

fn _word(buffer: &Vec<u8>, index: usize) -> u16 {
    return buffer[index + 1] as u16 | ((buffer[index + 2] as u16) << 8);
}

fn word2(b0: u8, b1: u8) -> u16 {
    return b0 as u16 | ((b1 as u16) << 8);
}

fn sixty() {
    let m = SimpleMemory::new("6502_functional_test.bin");
    let mut cpu = Cpu::new(Box::new(m));
    cpu.run(0x400);
}

fn disassemble3(index: usize, bytes: Vec<u8>) -> (String, usize) {
    let opcode = bytes[0] as usize;
    let name = constants::OPCODE_NAMES[opcode];
    let addressing_type = &ADDRESSING_TYPES[opcode];
    let s = match bytes.len() {
        1 => format!("{:04X}: {:02X}         {}", index,
                     opcode,
                     name),
        2 => format!("{:04X}: {:02X} {:02X}      {} {}", index,
                     opcode, bytes[1],
                     name,
                     addressing_type.to_string(index, bytes[1], 0)),
        _ => format!("{:04X}: {:02X} {:02X} {:02X}   {} {}", index,
                     opcode, bytes[1], bytes[2],
                     name,
                     addressing_type.to_string(index, bytes[1], word2(bytes[1], bytes[2])))
    };

    let mut result = String::new();
    result.push_str(& s);
    return (result, bytes.len());
}

// fn disassemble(buffer: &Vec<u8>, index: usize) -> (String, usize) {
//     let opcode = buffer[index] as usize;
//     let size: usize = constants::SIZES[opcode];
//     let mut bytes = Vec::new();
//     bytes.push(opcode as u8);
//     if size >= 2 {
//         bytes.push(buffer[index + 1]);
//     }
//     if size >= 3 {
//         bytes.push(buffer[index + 2]);
//     }
//
//     return disassemble3(index, bytes);
// }
