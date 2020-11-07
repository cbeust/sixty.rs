#![allow(unused)]
#![allow(warnings)]

use crate::{Memory, constants::*, word2};
use std::fmt;

pub struct Cpu {
    pub memory: Box<dyn Memory>,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: usize,
    pub sp: usize,
    pub p: StatusFlags
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let registers = std::format!("A={:02X} X={:02X} Y={:02X} S={:02X}",
            self.a, self.x, self.y, self.sp);
        write!(f, "{} {}", registers, self.p)
    }
}

pub struct StatusFlags {
    pub value: u8
}

impl StatusFlags {
    fn new() -> StatusFlags {
        StatusFlags { value: 0 }
    }

    fn set(&mut self, v: u8) {
        self.value = v;
    }

    fn get_bit(&self, bit: u8) -> bool {
        self.value & (1 << bit) != 0
    }

    fn set_bit(&mut self, f: bool, bit: u8) {
        if f { self.value |= 1 << bit }
        else { self.value &= !(1 << bit) }
    }

    fn n(&self) -> bool { self.get_bit(7) }
    fn set_n(&mut self, f: bool) { self.set_bit(f, 7) }
    fn v(&self) -> bool { self.get_bit(6) }
    fn set_v(&mut self, f: bool) { self.set_bit(f, 6) }
    fn b(&self) -> bool { self.get_bit(4) }
    fn set_b(&mut self, f: bool) { self.set_bit(f, 4) }
    fn d(&self) -> bool { self.get_bit(3) }
    fn set_d(&mut self, f: bool) { self.set_bit(f, 3) }
    fn i(&self) -> bool { self.get_bit(2) }
    fn set_i(&mut self, f: bool) { self.set_bit(f, 2) }
    fn z(&self) -> bool { self.get_bit(1) }
    fn set_z(&mut self, f: bool) { self.set_bit(f, 1) }
    fn c(&self) -> bool { self.get_bit(0) }
    fn set_c(&mut self, f: bool) { self.set_bit(f, 0) }

    fn set_nz_flags(&mut self, reg: u8) {
        self.set_z(reg == 0);
        self.set_n(reg & 0x80 != 0);
    }
}

impl fmt::Display for StatusFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn s(n: &str, v: bool) -> &str {
            if v {n} else {"-".as_ref()}
        }

        write!(f, "{{P:${:02X} {}{}-{}{}{}{}{}}}", self.value,
               s("N", self.n()),
               s("V", self.v()),
               s("B", self.b()),
               s("D", self.d()),
               s("I", self.i()),
               s("Z", self.z()),
               s("C", self.c()))
    }
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
            p: StatusFlags::new()
        }
    }

    pub fn run(&mut self, pc: usize) {
        self.p.set(0xff);
        println!("Current p: {}", self.p);
        self.pc = pc;
        let max = 10;
        let mut i = 0;
        let byte = self.memory.get(pc + 1);
        let word = word2(byte, self.memory.get(pc + 2));
        let mut timing = 0;
        // let content = || -> u8 { self.memory.get(word as usize) };
        loop {
            // let mut bm = Box::new(&self.memory);
            let opcode = self.memory.get(self.pc);
            let addressing_type = &ADDRESSING_TYPES[opcode as usize];
            match opcode {
                ADC_IMM => self.adc(byte),
                ADC_ZP| ADC_ZP_X| ADC_ABS| ADC_ABS_X| ADC_ABS_Y| ADC_IND_X| ADC_IND_Y => {
                    let (effectiveAddress, content) =
                        addressing_type.dereference(&self.memory, pc, self);
                    self.adc(content);
                    match opcode {
                        ADC_IND_Y => {
                            timing += self.page_crossed(
                                self.memory.word(self.memory.word(pc - 1) as usize),
                                effectiveAddress);
                        },
                        ADC_ABS_X | ADC_ABS_Y => {
                            timing += self.page_crossed(
                                self.memory.word(pc - 2 as usize),
                                effectiveAddress);
                        },
                        _ => { /* ignore */ }
                    }
                }
                CLC => self.p.set_c(false),
                SEC => self.p.set_c(true),
                CLI => self.p.set_i(false),
                SEI => self.p.set_i(true),
                CLD => self.p.set_d(false),
                SED => self.p.set_d(true),
                CLV => self.p.set_v(false),
                // BRK => break,
                _ => {}// println!("***** Unknown opcode: {:2X}", opcode) }
            }
            let (s, size) = self.memory.disassemble(self.pc);
            println!("{:<30} {}", s, self);
            self.pc += size;
            i = i + 1;
            if i >= max { break };
        }
    }

    fn page_crossed(&self, old: u16, new: u16) -> u8 {
        if ((old ^ new) & 0xff00) > 0 { 1 } else { 0 }
    }

    fn adc(&mut self, v: u8) {
        if self.p.d() {
            unimplemented!("ADD with decimal mode not implemented")
        } else {
            self.add(v);
        }
    }

    fn add(&mut self, v: u8) {
        let result: u16 = self.a as u16 + self.p.v() as u16 + self.p.c() as u16;
        let carry6 = self.a & 0x7f + self.p.v() as u8 & 0x7f + self.p.c() as u8;
        self.p.set_c(result & 0x100 != 0);
        self.p.set_v(self.p.c() ^ (carry6 & 0x80 != 0));
        let result2 = result as u8;
        self.p.set_nz_flags(result2);
        self.a = result2;
    }
}