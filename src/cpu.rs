use crate::{Memory, constants::*};

pub struct Cpu {
    pub memory: Box<dyn Memory>,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: usize,
    pub sp: usize,
}

impl Cpu {
    pub fn new(mem: Box<dyn Memory>) -> Cpu {
        Cpu {
            memory: mem,
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0,
        }
    }

    pub fn run(&mut self, pc: usize) {
        self.pc = pc;
        loop {
            let mut opcode = self.memory.get(self.pc);
            let (s, size) = self.memory.disassemble(self.pc);
            println!("{}", s);
            // let addressing_type = &ADDRESSING_TYPES[opcode];
            match opcode {
                CLD => { println!("CLD") },
                // BRK => break,
                _ => { println!("Unknown opcode: {:2X}", opcode) }
            }
            self.pc += size;
        }
    }
}