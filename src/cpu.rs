#![allow(unused)]
#![allow(warnings)]

use crate::{Memory, constants::*};
use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;
use std::borrow::BorrowMut;

const DEBUG_ASM: bool = false;
const DEBUG_PC: usize = 0x20000; // 0x670;
const DEBUG_CYCLES: u64 = u64::max_value(); // 0x4FC1A00

pub struct StatusFlags {
    _value: u8
}

impl StatusFlags {
    fn new() -> StatusFlags {
        StatusFlags { _value: 0x20 /* reserved to true by default */ }
    }

    fn set_value(&mut self, value: u8) {
        self._value = value | 1 << 4 | 1 << 5;  // always set the B and reserved flags
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
    fn reserved(&self) -> bool { true }  // reserved always true
    fn b(&self) -> bool { true } // b always true
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

        write!(f, "P=${:02X} {{{}{}{}{}{}{}{}{}}}", self.value(),
               s("N", self.n()),
               s("V", self.v()),
               "-",
               s("B", self.b()),
               s("D", self.d()),
               s("I", self.i()),
               s("Z", self.z()),
               s("C", self.c()))
    }
}

// type CpuListener = Fn(Cpu) -> bool;

pub trait CpuListener {
    /// return Ok() if the execution should continue and Err() if it should stop, in which
    /// case the String will give the reason for the stop.
    fn on_pc_changed(&mut self, cpu: &Cpu) -> RunStatus;
}

pub struct Cpu {
    pub memory: Memory,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: usize,
    pub p: StatusFlags,

    pub cycles: u64,

    pub listener: RefCell<Option<Box<dyn CpuListener>>>
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sp = self.memory.format_stack();
        let registers = std::format!("A={:02X} X={:02X} Y={:02X} S={:02X}",
                                     self.a, self.x, self.y, self.memory.stack_pointer);
        // Desired format:
        // 00000000| 05E0: D0 FE      BNE  $05E0       (2) A=AA X=FF Y=00 S=FD P=03 PC=$5E2 P=$03 {---- --ZC} SP={$FD stack:[$1FF:$55 $1FE:$AA ]}
        write!(f, "{} {} {}", registers, self.p, sp)
    }
}

pub enum RunStatus {
    Continue,
    Stop(bool, String) // If bool is true, stopping with no error + reason for stopping
}

impl Cpu {
    pub fn new(mut memory: Memory, listener: Option<Box<dyn CpuListener>>) -> Cpu {
        Cpu {
            memory,
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            p: StatusFlags::new(),
            cycles: 0,
            listener: RefCell::new(listener)
        }
    }

    pub fn run(&mut self, start_pc: usize) -> RunStatus {
        self.pc = start_pc;
        let mut result = RunStatus::Continue;
        loop {
            let previous_pc = self.pc;
            let opcode = self.memory.get(self.pc);
            self.pc += SIZES[opcode as usize];
            self.cycles = self.cycles + self.next_instruction(previous_pc);

            let stop = if let Some(l) = self.listener.borrow_mut().as_mut() {
                l.on_pc_changed(self)
            } else {
                RunStatus::Continue
            };

            match stop {
                RunStatus::Stop(success, ref reason) => {
                    result = RunStatus::Stop(success, reason.to_string());
                    println!("{}", reason.as_str());
                    break;
                },
                _ => {}
            }
        }
        return result;
    }

    pub fn next_instruction(&mut self, pc: usize) -> u64 {
        let max = 10;
        let mut i = 0;

        // let mut bm = Box::new(&self.memory);
        let opcode = self.memory.get(pc);
        let addressing_type = &ADDRESSING_TYPES[opcode as usize];
        let mut cycles = TIMINGS[opcode as usize];

        fn run_inst(opcode: u8, cpu: &mut Cpu, pc: usize, address: usize,
                    ind_y: u8, abs_x: u8, abs_y: u8) -> u8 {
            cpu.p.set_nz_flags(cpu.a);
            let memory = & cpu.memory;
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
            ADC_IMM => self.adc(self.memory.get(pc + 1)),
            ADC_ZP| ADC_ZP_X| ADC_ABS| ADC_ABS_X| ADC_ABS_Y| ADC_IND_X| ADC_IND_Y => {
                let address = addressing_type.address(pc, self);
                self.adc(self.memory.get(address));
                cycles += run_inst(opcode, self, pc, address, ADC_IND_Y, ADC_ABS_X, ADC_ABS_Y);
            },
            AND_IMM => {
                self.a = self.a & self.memory.get(pc + 1);
                self.p.set_nz_flags(self.a);
            },
            AND_ZP | AND_ZP_X | AND_ABS | AND_ABS_X | AND_ABS_Y | AND_IND_X | AND_IND_Y => {
                let address = addressing_type.address(pc, self);
                let content = self.memory.get(address);
                self.a &= content;
                cycles += run_inst(opcode, self, pc, address, AND_IND_Y, AND_ABS_X, AND_ABS_Y);
            },
            ASL => self.a = self.asl(self.a),
            ASL_ZP | ASL_ZP_X | ASL_ABS | ASL_ABS_X => {
                let address = addressing_type.address(pc, self);
                let result = self.asl(self.memory.get(address));
                self.memory.set(address, result);
            },
            BIT_ZP | BIT_ABS => {
                let address = addressing_type.address(pc, self);
                let content = self.memory.get(address);
                self.p.set_z(content & self.a == 0);
                self.p.set_n(content & 0x80 != 0);
                self.p.set_v(content & 0x40 != 0);
            },
            BPL => { cycles += self.branch(self.memory.get(pc + 1), ! self.p.n()) },
            BMI => { cycles += self.branch(self.memory.get(pc + 1), self.p.n()) },
            BNE => { cycles += self.branch(self.memory.get(pc + 1), ! self.p.z()) },
            BEQ => { cycles += self.branch(self.memory.get(pc + 1), self.p.z()) },
            BCC => { cycles += self.branch(self.memory.get(pc + 1), ! self.p.c()) },
            BCS => { cycles += self.branch(self.memory.get(pc + 1), self.p.c()) },
            BVC => { cycles += self.branch(self.memory.get(pc + 1), ! self.p.v()) },
            BVS => { cycles += self.branch(self.memory.get(pc + 1), self.p.v()) },
            BRK => self.handle_interrupt(true, IRQ_VECTOR_H, IRQ_VECTOR_L),
            CMP_IMM => self.cmp(self.a, self.memory.get(pc + 1)),
            CMP_ZP| CMP_ZP_X| CMP_ABS| CMP_ABS_X| CMP_ABS_Y| CMP_IND_X| CMP_IND_Y => {
                self.cmp(self.a, self.memory.get(
                    addressing_type.address(pc, self)));
            },
            CPX_IMM => self.cmp(self.x, self.memory.get(pc + 1)),
            CPX_ZP | CPX_ABS => {
                self.cmp(self.x, self.memory.get(
                    addressing_type.address(pc, self)));
            },
            CPY_IMM => self.cmp(self.y, self.memory.get(pc + 1)),
            CPY_ZP | CPY_ABS => {
                self.cmp(self.y, self.memory.get(
                    addressing_type.address(pc, self)));
            },
            DEC_ZP| DEC_ZP_X| DEC_ABS| DEC_ABS_X => {
                let address = addressing_type.address(pc, self);
                let old_value = self.memory.get(address);
                let new_value = if old_value == 0 { 0xff } else { old_value - 1};
                self.memory.set(address, new_value);
                self.p.set_nz_flags(new_value);
            },
            EOR_IMM => {
                self.a = self.a ^ self.memory.get(pc + 1);
                self.p.set_nz_flags(self.a);
            },
            EOR_ZP| EOR_ZP_X| EOR_ABS| EOR_ABS_X| EOR_ABS_Y| EOR_IND_Y| EOR_IND_X => {
                let address = addressing_type.address(pc, self);
                self.a = self.a ^ self.memory.get(address);
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
                let content = self.memory.get(address);
                let word = self.memory.word(pc + 1);
                let new_value = if content == 0xff { 0 } else { content + 1 };
                self.memory.set(address, new_value);
                self.p.set_nz_flags(new_value);
            },
            JMP => self.pc = self.memory.word(pc + 1) as usize,
            JMP_IND => {
                self.pc = self.memory.word(addressing_type.address(pc, self))
                    as usize
            },
            JSR => {
                self.memory.push_word(pc as u16 + 2);
                self.pc = self.memory.word(pc + 1) as usize;
            },
            LDX_IMM => {
                self.x = self.memory.get(pc + 1);
                self.p.set_nz_flags(self.x);
            },
            LDA_IMM => {
                self.a = self.memory.get(pc + 1);
                self.p.set_nz_flags(self.a);
            },
            LDA_ZP| LDA_ZP_X| LDA_ABS| LDA_ABS_X| LDA_ABS_Y| LDA_IND_X| LDA_IND_Y => {
                let address = addressing_type.address(pc, self);
                self.a = self.memory.get(address);
                self.p.set_nz_flags(self.a);
            },
            LDX_IMM => {
                self.x = self.memory.get(pc + 1);
                self.p.set_nz_flags(self.x);
            },
            LDX_ZP | LDX_ZP_Y | LDX_ABS | LDX_ABS_Y => {
                let address = addressing_type.address(pc, self);
                let content = self.memory.get(address);
                self.x = content;
                self.p.set_nz_flags(self.x);
            },
            LDY_IMM => {
                self.y = self.memory.get(pc + 1);
                self.p.set_nz_flags(self.y);
            },
            LDY_ZP | LDY_ZP_X | LDY_ABS | LDY_ABS_X => {
                let address = addressing_type.address(pc, self);
                let content = self.memory.get(address);
                self.y = content;
                self.p.set_nz_flags(self.y);
            },
            LSR => self.a = self.lsr(self.a),
            LSR_ZP| LSR_ZP_X| LSR_ABS| LSR_ABS_X => {
                let address = addressing_type.address(pc, self);
                let new_value = self.lsr(self.memory.get(address));
                self.memory.set(address, new_value);
            },
            NOP => {},
            ORA_IMM => {
                self.a = self.a | self.memory.get(pc + 1);
                self.p.set_nz_flags(self.a);
            },
            ORA_ZP| ORA_ZP_X| ORA_ABS| ORA_ABS_X| ORA_ABS_Y| ORA_IND_X| ORA_IND_Y => {
                let address = addressing_type.address(pc, self);
                self.a = self.a | self.memory.get(address);
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
                if self.x == 0 { self.x = 0xff } else { self.x -= 1; }
                self.p.set_nz_flags(self.x);
            },
            INX => {
                if self.x == 0xff { self.x = 0 } else { self.x += 1; }
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
                if self.y == 0 { self.y = 0xff; } else { self.y -= 1; }
                self.p.set_nz_flags(self.y);
            },
            INY => {
                if self.y == 0xff { self.y = 0; } else { self.y += 1; }
                self.p.set_nz_flags(self.y);
            },
            ROL => {
                self.a = self.rol(self.a);
            },
            ROL_ZP | ROL_ZP_X | ROL_ABS | ROL_ABS_X => {
                let address = addressing_type.address(pc, self);
                let new_value = self.rol(self.memory.get(address));
                self.memory.set(address, new_value);
            },
            ROR => {
                self.a = self.ror(self.a);
            },
            ROR_ZP | ROR_ZP_X | ROR_ABS | ROR_ABS_X => {
                let address = addressing_type.address(pc, self);
                let new_value = self.ror(self.memory.get(address));
                self.memory.set(address, new_value);
            },
            RTI => {
                self.p.set_value(self.memory.pop_byte());
                self.pc = self.memory.pop_word();
            },
            RTS => {
                self.pc = self.memory.pop_word() + 1;
            },
            SBC_IMM => {
                self.sbc(self.memory.get(pc + 1));
            },
            SBC_ZP |  SBC_ZP_X | SBC_ABS | SBC_ABS_X | SBC_ABS_Y | SBC_IND_X | SBC_IND_Y =>{
                let address = addressing_type.address(pc, self);
                self.sbc(self.memory.get(address));
            },
            STA_ZP | STA_ZP_X | STA_ABS | STA_ABS_X | STA_ABS_Y | STA_IND_X | STA_IND_Y => {
                let address = addressing_type.address(pc, self);
                self.memory.set(address, self.a);
            },
            TXS => self.memory.stack_pointer = self.x as usize,
            TSX => {
                self.x = self.memory.stack_pointer as u8;
                self.p.set_nz_flags(self.x);
            },
            PHA => self.memory.push_byte(self.a),
            PLA => {
                self.a = self.memory.pop_byte();
                self.p.set_nz_flags(self.a);
            },
            PHP => {
                self.p.set_b(true);
                // self.p.set_reserved(true);
                self.memory.push_byte(self.p.value());
            },
            PLP => self.p.set_value(self.memory.pop_byte()),
            STX_ZP | STX_ZP_Y | STX_ABS => {
                let address = addressing_type.address(pc, self);
                self.memory.set(address, self.x);
            },
            STY_ZP | STY_ZP_X | STY_ABS => {
                let address = addressing_type.address(pc, self);
                self.memory.set(address, self.y);
            },
            _ => {
                panic!("Unknown opcode");
            }
        }
        if self.pc == DEBUG_PC {
            println!("BREAKPOINT DEBUG_PC");
        }
        if self.cycles == DEBUG_CYCLES {
            println!("BREAKPOINT CYCLES");
        }
        let close_to_breakpoint = self.pc > DEBUG_PC - 50 && self.pc < DEBUG_PC;
        if DEBUG_ASM || close_to_breakpoint || self.cycles > DEBUG_CYCLES {
            let (s, size) = self.memory.disassemble(pc);
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
        self.p.set_b(brk);
        self.memory.push_word((self.pc + 1) as u16);
        self.memory.push_byte(self.p.value());
        self.p.set_i(true);
        let memory = &self.memory;
        let new_pc = (self.memory.get(vector_high) as u16) << 8 | self.memory.get(vector_low) as u16;
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
        self.p.set_c(v & 0x80 != 0);
        let result: u8 = v << 1;
        self.p.set_nz_flags(result);
        return result;
    }

    fn adc(&mut self, v: u8) {
        if self.p.d() {
            let mut l = (self.a & 0x0f) + (v & 0x0f) + self.p.c() as u8;
            if l & 0xff > 9 { l += 6 }
            let mut h = (self.a >> 4) + (v >> 4) + if l > 15 { 1 } else { 0 };
            if h & 0xff > 9 { h += 6 };
            let result = (l & 0x0f | (h << 4)) & 0xff;

            self.p.set_c(h > 15);
            self.p.set_z(result == 0);
            self.p.set_v(false);  // BCD never sets overflow flag
            self.p.set_n((result & 0x80) != 0);  // N flag is valid on CMOS 6502/65816

            self.a = result;
        } else {
            self.add(v);
        }
    }

    fn add(&mut self, v: u8) {
        let result: u16 = self.a as u16 + v as u16 + self.p.c() as u16;
        // NOTE: Parentheses are important here! Remove them and carry6 is incorrectly calculated
        let carry6 = (self.a & 0x7f) + (v as u8 & 0x7f) + self.p.c() as u8;
        self.p.set_c(result & 0x100 != 0);
        self.p.set_v(self.p.c() ^ (carry6 & 0x80 != 0));
        let result2 = result as u8;
        self.p.set_nz_flags(result2);
        self.a = result2;
    }

    fn sbc(&mut self, v: u8) {
        if self.p.d() {
            let mut l: i16 = (self.a as i16 & 0x0f) - (v as i16 & 0x0f)
                - if self.p.c() { 0 } else { 1 };
            if (l & 0x10) != 0 { l -= 6 };
            let mut h: i16 = (self.a  as i16 >> 4) - (v as i16 >> 4)
                - if (l & 0x10) != 0 { 1 } else { 0 };
            if (h & 0x10) != 0 { h -= 6 }
            let result = (l & 0x0f | (h << 4)) & 0xff;

            self.p.set_c((h & 0xff) < 15);
            self.p.set_z(result == 0);
            self.p.set_v(false);  // BCD never sets overflow flag
            self.p.set_n((result & 0x80) != 0);

            self.a = result as u8 & 0xff;
        } else {
            self.add((v ^ 0xff) as u8);
        }
    }
}
