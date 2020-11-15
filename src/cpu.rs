#![allow(unused)]
#![allow(warnings)]

use crate::{Memory, constants::*, word2, StackPointer};
use std::fmt;
use std::cell::{RefCell, RefMut};
use std::borrow::BorrowMut;

const DEBUG_ASM: bool = false;
const DEBUG_PC: usize = 0x10000; // 0x670;
const DEBUG_CYCLES: u64 = 0x14700;

pub struct StatusFlags {
    _value: u8
}

impl StatusFlags {
    fn new() -> StatusFlags {
        StatusFlags { _value: 0x20 /* reserved to true by default */ }
    }

    fn set_value(&mut self, value: u8) {
        self._value = value | 1<<5;  // always set the reserved bit
    }

    fn value(&self) -> u8 { self._value }

    fn get_bit(&self, bit: u8) -> bool {
        self._value & (1 << bit) != 0
    }

    fn set_bit(&mut self, f: bool, bit: u8) {
        if f { self._value |= 1 << bit }
        else { self._value &= !(1 << bit) }
    }

    fn n(&self) -> bool { self.get_bit(7) }
    fn set_n(&mut self, f: bool) { self.set_bit(f, 7) }
    fn v(&self) -> bool { self.get_bit(6) }
    fn set_v(&mut self, f: bool) { self.set_bit(f, 6) }
    fn reserved(&self) -> bool { true }
    // fn set_reserved(&mut self, f: bool) {
    //     if ! f {
    //         println!("PROBLEM");
    //     }
    //     self.set_bit(f, 5)
    // }
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

        write!(f, "{{P:${:02X} {}{}{}{}{}{}{}{}}}", self.value(),
               s("N", self.n()),
               s("V", self.v()),
               s("r", self.reserved()),
               s("B", self.b()),
               s("D", self.d()),
               s("I", self.i()),
               s("Z", self.z()),
               s("C", self.c()))
    }
}

pub struct Cpu<'a> {
    pub memory: &'a RefCell<dyn Memory>,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: usize,
    pub sp: StackPointer<'a>,
    pub p: StatusFlags,

    pub cycles: u64
}

impl fmt::Display for Cpu<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let registers = std::format!("A={:02X} X={:02X} Y={:02X} S={:02X}",
                                     self.a, self.x, self.y, self.sp.s as u8);
        // Desired format:
        // 00000000| 05E0: D0 FE      BNE  $05E0       (2) A=AA X=FF Y=00 S=FD P=03 PC=$5E2 P=$03 {---- --ZC} SP={$FD stack:[$1FF:$55 $1FE:$AA ]}
        write!(f, "{} {} {}", registers, self.p, self.sp)
    }
}

impl <'a> Cpu<'a> {
    pub fn new(mut memory: &'a RefCell<dyn Memory>) -> Cpu<'a> {
        Cpu {
            memory,
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: StackPointer { s: 0, memory },
            p: StatusFlags::new(),
            cycles: 0
        }
    }

    pub fn run(&mut self, start_pc: usize) {
        self.pc = start_pc;
        let mut previous_pc = 0;
        loop {
            if previous_pc != 0 && previous_pc == self.pc {
                let memory = self.memory.borrow();
                println!("Infinite loop at PC {:2X} {}", self.pc, self);
                println!("");
            } else if self.pc == 0x346c || self.pc == 0x3469 {
                println!("ALL TESTS PASSED!");
            } else {
                previous_pc = self.pc;
                let opcode = self.memory.borrow().get(self.pc);
                self.pc += SIZES[opcode as usize];
                self.cycles = self.cycles + self.next_instruction(previous_pc);
            }
        }
    }

    pub fn next_instruction(&mut self, pc: usize) -> u64 {
        let max = 10;
        let mut i = 0;
        // let byte = self.self.memory.borrow().get(self.pc + 1);
        // let word = word2(byte, self.self.memory.borrow().get(self.pc + 2));

        // let mut bm = Box::new(&self.memory);
        let opcode = self.memory.borrow().get(pc);
        let addressing_type = &ADDRESSING_TYPES[opcode as usize];
        let mut cycles = TIMINGS[opcode as usize];

        fn runInst(opcode: u8, cpu: &mut Cpu, pc: usize, address: usize,
                   ind_y: u8, abs_x: u8, abs_y: u8) -> u8 {
            cpu.p.set_nz_flags(cpu.a);
            let memory = cpu.memory.borrow();
            let result =
                if opcode == ind_y {
                    cpu.page_crossed(memory.word(memory.get(pc - 1) as usize), address)
                } else if opcode == abs_x || opcode == abs_y {
                    cpu.page_crossed(memory.word(pc - 2), address)
                } else {
                    0
                };
            result
        }

        // if pc == 0x4e5 {
        //     println!("BREAKPOINT");
        // }
        match opcode {
            ADC_IMM => self.adc(self.memory.borrow().get(pc + 1)),
            ADC_ZP| ADC_ZP_X| ADC_ABS| ADC_ABS_X| ADC_ABS_Y| ADC_IND_X| ADC_IND_Y => {
                let address = addressing_type.address(pc, self);
                self.adc(self.memory.borrow().get(address));
                cycles += runInst(opcode, self, pc, address, ADC_IND_Y, ADC_ABS_X, ADC_ABS_Y);
            },
            AND_IMM => {
                self.a = self.a & self.memory.borrow().get(pc + 1);
                self.p.set_nz_flags(self.a);
            },
            AND_ZP | AND_ZP_X | AND_ABS | AND_ABS_X | AND_ABS_Y | AND_IND_X | AND_IND_Y => {
                let address = addressing_type.address(pc, self);
                let content = self.memory.borrow().get(address);
                self.a &= content;
                cycles += runInst(opcode, self, pc, address, AND_IND_Y, AND_ABS_X, AND_ABS_Y);
            },
            ASL => self.a = self.asl(self.a),
            ASL_ZP | ASL_ZP_X | ASL_ABS | ASL_ABS_X => {
                let address = addressing_type.address(pc, self);
                let result = self.asl(self.memory.borrow().get(address));
                self.memory.borrow_mut().set(address, result);
            },
            BIT_ZP | BIT_ABS => {
                let address = addressing_type.address(pc, self);
                let content = self.memory.borrow().get(address);
                let v = self.p.v() as u8;
                self.p.set_z(v & self.a == 0);
                self.p.set_n(v & 0x80 != 0);
                self.p.set_v(v & 0x40 != 0);
            },
            BPL => { cycles += self.branch(self.memory.borrow().get(pc + 1), ! self.p.n()) },
            BMI => { cycles += self.branch(self.memory.borrow().get(pc + 1), self.p.n()) },
            BNE => { cycles += self.branch(self.memory.borrow().get(pc + 1), ! self.p.z()) },
            BEQ => { cycles += self.branch(self.memory.borrow().get(pc + 1), self.p.z()) },
            BCC => { cycles += self.branch(self.memory.borrow().get(pc + 1), ! self.p.c()) },
            BCS => { cycles += self.branch(self.memory.borrow().get(pc + 1), self.p.c()) },
            BVC => { cycles += self.branch(self.memory.borrow().get(pc + 1), ! self.p.v()) },
            BVS => { cycles += self.branch(self.memory.borrow().get(pc + 1), self.p.v()) },
            BRK => self.handle_interrupt(true, IRQ_VECTOR_H, IRQ_VECTOR_L),
            CMP_IMM => self.cmp(self.a, self.memory.borrow().get(pc + 1)),
            CMP_ZP| CMP_ZP_X| CMP_ABS| CMP_ABS_X| CMP_ABS_Y| CMP_IND_X| CMP_IND_Y => {
                self.cmp(self.a, self.memory.borrow().get(
                    addressing_type.address(pc, self)));
            },
            CPX_IMM => self.cmp(self.x, self.memory.borrow().get(pc + 1)),
            CPX_ZP | CPX_ABS => {
                self.cmp(self.x, self.memory.borrow().get(
                    addressing_type.address(pc, self)));
            },
            CPY_IMM => self.cmp(self.y, self.memory.borrow().get(pc + 1)),
            CPY_ZP | CPY_ABS => {
                self.cmp(self.y, self.memory.borrow().get(
                    addressing_type.address(pc, self)));
            },
            DEC_ZP| DEC_ZP_X| DEC_ABS| DEC_ABS_X => {
                let address = addressing_type.address(pc, self);
                let new_value = self.memory.borrow().get(address) - 1;
                self.memory.borrow_mut().set(address, new_value);
                self.p.set_nz_flags(new_value);
            },
            EOR_IMM => {
                self.a = self.a ^ self.memory.borrow().get(pc + 1);
                self.p.set_nz_flags(self.a);
            },
            EOR_ZP| EOR_ZP_X| EOR_ABS| EOR_ABS_X| EOR_ABS_Y| EOR_IND_Y| EOR_IND_X => {
                let address = addressing_type.address(pc, self);
                self.a = self.a ^ self.memory.borrow().get(address);
                self.p.set_nz_flags(self.a);
            },
            CLC => self.p.set_c(false),
            SEC => self.p.set_c(true),
            CLI => self.p.set_i(false),
            SEI => self.p.set_i(true),
            CLD => self.p.set_d(false),
            SED => self.p.set_d(true),
            CLV => self.p.set_v(false),
            INC_ZP | INC_ZP_X | INC_ABS | INC_ABS_X => {
                let address = addressing_type.address(pc, self);
                let content = self.memory.borrow().get(address);
                let word = self.memory.borrow().word(pc + 1);
                let new_value = content + 1;
                self.memory.borrow_mut().set(address, content);
                self.p.set_nz_flags(new_value);
            },
            JMP => self.pc = self.memory.borrow().word(pc + 1) as usize,
            JMP_IND => {
                self.pc = self.memory.borrow().word(addressing_type.address(pc, self))
                    as usize
            },
            JSR => {
                self.sp.push_word(pc as u16 + 2);
                self.pc = self.memory.borrow().word(pc + 1) as usize;
            },
            LDX_IMM => {
                self.x = self.memory.borrow().get(pc + 1);
                self.p.set_nz_flags(self.x);
            },
            LDA_IMM => {
                self.a = self.memory.borrow().get(pc + 1);
                self.p.set_nz_flags(self.a);
            },
            LDA_ZP| LDA_ZP_X| LDA_ABS| LDA_ABS_X| LDA_ABS_Y| LDA_IND_X| LDA_IND_Y => {
                let address = addressing_type.address(pc, self);
                self.a = self.memory.borrow().get(address);
                self.p.set_nz_flags(self.a);
            },
            LDX_IMM => {
                self.x = self.memory.borrow().get(pc + 1);
                self.p.set_nz_flags(self.x);
            },
            LDX_ZP | LDX_ZP_Y | LDX_ABS | LDX_ABS_Y => {
                let address = addressing_type.address(pc, self);
                let content = self.memory.borrow().get(address);
                self.x = content;
                self.p.set_nz_flags(self.x);
            },
            LDY_IMM => {
                self.y = self.memory.borrow().get(pc + 1);
                self.p.set_nz_flags(self.y);
            },
            LDY_ZP | LDY_ZP_X | LDY_ABS | LDY_ABS_X => {
                let address = addressing_type.address(pc, self);
                let content = self.memory.borrow().get(address);
                self.y = content;
                self.p.set_nz_flags(self.y);
            },
            LSR => self.a = self.lsr(self.a),
            LSR_ZP| LSR_ZP_X| LSR_ABS| LSR_ABS_X => {
                let address = addressing_type.address(pc, self);
                let new_value = self.lsr(self.memory.borrow().get(address));
                self.memory.borrow_mut().set(address, new_value);
            },
            NOP => {},
            ORA_IMM => {
                self.a = self.a | self.memory.borrow().get(pc + 1);
                self.p.set_nz_flags(self.a);
            },
            ORA_ZP| ORA_ZP_X| ORA_ABS| ORA_ABS_X| ORA_ABS_Y| ORA_IND_X| ORA_IND_Y => {
                let address = addressing_type.address(pc, self);
                self.a = self.a | self.memory.borrow().get(address);
                self.p.set_nz_flags(self.a);
            },
            TAX => {
                self.x = self.a;
                self.p.set_nz_flags(self.x);
            },
            TXA => {
                self.a = self.x;
                self.p.set_nz_flags(self.a);
            },
            DEX => {
                self.x -= 1;
                self.p.set_nz_flags(self.x);
            },
            INX => {
                self.x += 1;
                self.p.set_nz_flags(self.x);
            },
            TAY => {
                self.y = self.a;
                self.p.set_nz_flags(self.y);
            },
            TYA => {
                self.a = self.y;
                self.p.set_nz_flags(self.a);
            },
            DEY => {
                self.y -= 1;
                self.p.set_nz_flags(self.y);
            },
            INY => {
                self.y += 1;
                self.p.set_nz_flags(self.y);
            },
            ROL => {
                self.a = self.rol(self.a);
            },
            ROL_ZP | ROL_ZP_X | ROL_ABS | ROL_ABS_X => {
                let address = addressing_type.address(pc, self);
                let new_value = self.rol(self.memory.borrow().get(address));
                self.memory.borrow_mut().set(address, new_value);
            },
            ROR => {
                self.a = self.ror(self.a);
            },
            ROR_ZP | ROR_ZP_X | ROR_ABS | ROR_ABS_X => {
                let address = addressing_type.address(pc, self);
                let new_value = self.ror(self.memory.borrow().get(address));
                self.memory.borrow_mut().set(address, new_value);
            },
            RTI => {
                self.p.set_value(self.sp.pop_byte());
                self.pc = self.sp.pop_word();
            },
            RTS => {
                self.pc = self.sp.pop_word() + 1;
            },
            SBC_IMM => {
                self.pc = self.sp.pop_word() + 1;
            },
            SBC_ZP |  SBC_ZP_X | SBC_ABS | SBC_ABS_X | SBC_ABS_Y | SBC_IND_X | SBC_IND_Y =>{
                let address = addressing_type.address(pc, self);
                self.memory.borrow_mut().set(address, self.a);
            },
            STA_ZP | STA_ZP_X | STA_ABS | STA_ABS_X | STA_ABS_Y | STA_IND_X | STA_IND_Y => {
                let address = addressing_type.address(pc, self);
                self.memory.borrow_mut().set(address, self.a);
            },
            TXS => self.sp.s = self.x as usize,
            TSX => {
                self.x = self.sp.s as u8;
                self.p.set_nz_flags(self.x);
            },
            PHA => self.sp.push_byte(self.a),
            PLA => {
                self.a = self.sp.pop_byte();
                self.p.set_nz_flags(self.a);
            },
            PHP => {
                self.p.set_b(true);
                // self.p.set_reserved(true);
                self.sp.push_byte(self.p.value());
            },
            PLP => self.p.set_value(self.sp.pop_byte()),
            STX_ZP | STX_ZP_Y | STX_ABS => {
                let address = addressing_type.address(pc, self);
                self.memory.borrow_mut().set(address, self.x);
            },
            STY_ZP | STY_ZP_X | STY_ABS => {
                let address = addressing_type.address(pc, self);
                self.memory.borrow_mut().set(address, self.y);
            },
            _ => {
                panic!("Unknown opcode");
            }
        }
        if self.pc == DEBUG_PC {
            println!("BREAKPOINT DEBUG_PC");
        }
        if DEBUG_ASM || self.pc > DEBUG_PC - 100 || self.cycles > DEBUG_CYCLES {
            let (s, size) = self.memory.borrow().disassemble(pc);
            println!("{:08X}| {:<30} {}", self.cycles, s, self);
        }
        // i = i + 1;
        // if i >= max { break };
        return cycles as u64;
    }

    fn ror(&mut self, v: u8) -> u8 {
        let bit0 = v & 1;
        let result = (v >> 1) | (self.p.c() as u8) << 7;
        self.p.set_nz_flags(result);
        self.p.set_c(bit0 != 0);
        result
    }

    fn rol(&mut self, v: u8) -> u8 {
        let result = (v << 1) | self.p.c() as u8;
        self.p.set_c(v & 0x80 != 0);
        self.p.set_nz_flags(result);
        result
    }

    fn lsr(&mut self, v: u8) -> u8 {
        let bit0 = v & 1;
        self.p.set_c(bit0 != 0);
        let result = v >> 1;
        self.p.set_nz_flags(result);
        result
    }

    fn cmp(&mut self, register: u8, v: u8) {
        // let tmp: i8 = 0;
        let tmp: i8 = (register as i16 - v as i16) as i8;
        self.p.set_c(register >= v);
        self.p.set_z(tmp == 0);
        self.p.set_n(tmp < 0);
    }

    fn handle_interrupt(&mut self, brk: bool, vector_high: usize, vector_low: usize) {
        println!("Current P value: {}", self.p);
        self.p.set_b(brk);
        self.sp.push_word((self.pc + 1) as u16);
        self.sp.push_byte(self.p.value());
        self.p.set_i(true);
        let memory = self.memory.borrow();
        let new_pc = (self.memory.borrow().get(vector_high) as u16) << 8 | self.memory.borrow().get(vector_low) as u16;
        self.pc = new_pc as usize;
    }

    fn branch(&mut self, byte: u8, condition: bool) -> u8 {
        let mut result = 0;
        if condition {
            let old = self.pc;
            self.pc += byte as usize;
            if byte >= 0x80 {
                self.pc -= 0x100 as usize
            }
            result += 1 + self.page_crossed(old as u16, self.pc);
        }
        result
    }

    fn page_crossed(&self, old: u16, new: usize) -> u8 {
        if ((old ^ new as u16) & 0xff00) > 0 { 1 } else { 0 }
    }

    fn asl(&mut self, v: u8) -> u8 {
        self.p.set_c(self.p.v() as u8 & 0x80 != 0);
        let result: u8 = (self.p.v() as u8) << 1;
        self.p.set_nz_flags(result);
        return result;
    }

    fn adc(&mut self, v: u8) {
        if self.p.d() {
            unimplemented!("ADD with decimal mode not implemented")
        } else {
            self.add(v);
        }
    }

    fn add(&mut self, v: u8) {
        let result: u16 = self.a as u16 + v as u16 + self.p.c() as u16;
        let carry6 = self.a & 0x7f + v as u8 & 0x7f + self.p.c() as u8;
        self.p.set_c(result & 0x100 != 0);
        self.p.set_v(self.p.c() ^ (carry6 & 0x80 != 0));
        let result2 = result as u8;
        self.p.set_nz_flags(result2);
        self.a = result2;
    }
}
