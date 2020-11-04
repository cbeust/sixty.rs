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
    pub fn new(memory: Box<dyn Memory>) -> Cpu {
        Cpu {
            memory,
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0,
        }
    }

    pub fn run(&mut self, pc: usize) {
        self.pc = pc;
        let max = 10;
        let mut i = 0;
        loop {
            let opcode = self.memory.get(self.pc);
            let (s, size) = self.memory.disassemble(self.pc);
            println!("{}", s);
            // let addressing_type = &ADDRESSING_TYPES[opcode];
            match opcode {
                CLD => { println!("CLD") },
                // BRK => break,
                _ => { println!("Unknown opcode: {:2X}", opcode) }
            }
            self.pc += size;
            i = i + 1;
            if i >= max { break };
        }
    }
}