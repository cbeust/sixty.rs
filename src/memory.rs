use crate::constants::ADDRESSING_TYPES;
use std::fs::File;
use std::io::Read;
use std::cmp::max;

const STACK_ADDRESS: usize = 0x100;

pub struct Memory {
    buffer: Vec<u8>,
    _listener: Option<Box<dyn MemoryListener>>,
    pub(crate) stack_pointer: usize,
}

pub trait MemoryListener {
    fn on_read_or_wrote(&mut self, address: usize, value: u8);
}

impl Memory {

    pub fn new_with_file(file_name: &str, listener: Option<Box<dyn MemoryListener>>) -> Memory {
        let mut result = Memory {
            buffer: Vec::new(),
            _listener: listener,
            stack_pointer: 0xff
        };
        result.load(file_name);
        result
    }

    pub fn new_with_vec(buffer: Vec<u8>, listener: Option<Box<dyn MemoryListener>>) -> Memory {
        let actual_buffer =
            if buffer.len() < 0x200 {
                // The memory needs to be at least $200 big since the stack is $100-$1FF,
                // so create a $200 big memory and copy the passed buffer into it
                let mut b = vec![0; 0x200];
                for (i, x) in buffer.iter().enumerate() {
                    b[i] = *x;
                }
                b
            } else {
                // The passed buffer is big enough, use it as is
                buffer
            };
        Memory {
            buffer: actual_buffer,
            _listener: listener,
            stack_pointer: 0xff
        }
    }

    pub(crate) fn get(&self, index: usize) -> u8 {
        self.buffer[index]
    }

    pub(crate) fn set(&mut self, index: usize, value: u8) {
        self.buffer[index] = value
    }

    fn load(&mut self, file_name: &str) {
        let mut f = File::open(file_name).expect("Couldn't find the file");
        f.read_to_end(&mut self.buffer).expect("Could not find file {}");
    }

    pub(crate) fn word(&self, address: usize) -> u16 {
        self.get(address) as u16 | ((self.get(address + 1) as u16) << 8)
    }
    // fn disassemble(&mut self, index: usize) -> (String, usize);

    fn inc(&mut self) {
        if self.stack_pointer == 0xff {
            self.stack_pointer = 0
        } else {
            self.stack_pointer = self.stack_pointer + 1;
        }
    }

    fn dec(&mut self) {
        if self.stack_pointer == 0 {
            self.stack_pointer = 0xff as usize
        } else {
            self.stack_pointer = self.stack_pointer - 1;
        }
    }

    pub(crate) fn push_byte(&mut self, a: u8) {
        self.set(STACK_ADDRESS + self.stack_pointer, a);
        self.dec();
    }

    pub(crate) fn pop_byte(&mut self,) -> u8 {
        self.inc();
        self.get(STACK_ADDRESS + self.stack_pointer)
    }

    pub(crate) fn push_word(&mut self, a: u16) {
        self.set(STACK_ADDRESS + self.stack_pointer, ((a & 0xff00) >> 8) as u8);
        self.dec();
        self.set(STACK_ADDRESS + self.stack_pointer, (a & 0xff) as u8);
        self.dec();
    }

    pub(crate) fn pop_word(&mut self) -> usize {
        self.inc();
        let low = self.get(STACK_ADDRESS + self.stack_pointer) as usize;
        self.inc();
        let high = self.get(STACK_ADDRESS + self.stack_pointer) as usize;
        low | high << 8
    }

    pub fn disassemble(&self, index: usize) -> (String, usize) {
        let opcode = self.get(index) as usize;
        let size: usize = crate::constants::SIZES[opcode];
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

    pub(crate) fn format_stack(&self) -> String {
        let mut result = Vec::new();
        result.push(std::format!("SP={{${:2X} stack:[", self.stack_pointer));
        let down = max(self.stack_pointer + 1, 0xf8);
        let mut i = 0xff;
        if self.stack_pointer < 0xff {
            loop {
                let v = self.get(STACK_ADDRESS + i);
                result.push(std::format!("{:02X}={:02X}", i, v));
                i = i - 1;
                if i < down { break; }
            }
        }
        result.push("]}}".to_string());
        result.join(" ")
    }
}

fn _word(buffer: &Vec<u8>, index: usize) -> u16 {
    return buffer[index + 1] as u16 | ((buffer[index + 2] as u16) << 8);
}

fn word2(b0: u8, b1: u8) -> u16 {
    return b0 as u16 | ((b1 as u16) << 8);
}

fn disassemble3(index: usize, bytes: Vec<u8>) -> (String, usize) {
    let opcode = bytes[0] as usize;
    let name = crate::constants::OPCODE_NAMES[opcode];
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
//     let size: usize = crate::constants::SIZES[opcode];
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
