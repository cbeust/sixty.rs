#![allow(unused)]

const BRK: u8 = 0x00;

const ADC_IMM: u8 = 0x69;
const ADC_ZP: u8 = 0x65;
const ADC_ZP_X: u8 = 0x75;
const ADC_ABS: u8 = 0x6d;
const ADC_ABS_X: u8 = 0x7d;
const ADC_ABS_Y: u8 = 0x79;
const ADC_IND_X: u8 = 0x61;
const ADC_IND_Y: u8 = 0x71;

const AND_IMM: u8 = 0x29;
const AND_ZP: u8 = 0x25;
const AND_ZP_X: u8 = 0x35;
const AND_ABS: u8 = 0x2d;
const AND_ABS_X: u8 = 0x3d;
const AND_ABS_Y: u8 = 0x39;
const AND_IND_X: u8 = 0x21;
const AND_IND_Y: u8 = 0x31;

const ASL: u8 = 0xa;
const ASL_ZP: u8 = 0x06;
const ASL_ZP_X: u8 = 0x16;
const ASL_ABS: u8 = 0xe;
const ASL_ABS_X: u8 = 0x1e;

const BIT_ZP: u8 = 0x24;
const BIT_ABS: u8 = 0x2c;

const BPL: u8 = 0x10;
const BMI: u8 = 0x30;
const BVC: u8 = 0x50;
const BVS: u8 = 0x70;
const BCC: u8 = 0x90;
const BCS: u8 = 0xb0;
const BNE: u8 = 0xd0;
const BEQ: u8 = 0xf0;

const CPX_IMM: u8 = 0xe0;
const CPX_ZP: u8 = 0xe4;
const CPX_ABS: u8 = 0xec;

const CLC: u8 = 0x18;
const SEC: u8 = 0x38;
const CLI: u8 = 0x58;
const SEI: u8 = 0x78;
const CLV: u8 = 0xb8;
const CLD: u8 = 0xd8;
const SED: u8 = 0xf8;

const CMP_IMM: u8 = 0xc9;
const CMP_ZP: u8 = 0xc5;
const CMP_ZP_X: u8 = 0xd5;
const CMP_ABS: u8 = 0xcd;
const CMP_ABS_X: u8 = 0xdd;
const CMP_ABS_Y: u8 = 0xd9;
const CMP_IND_X: u8 = 0xc1;
const CMP_IND_Y: u8 = 0xd1;

const CPY_IMM: u8 = 0xc0;
const CPY_ZP: u8 = 0xc4;
const CPY_ABS: u8 = 0xcc;

const DEC_ZP: u8 = 0xc6;
const DEC_ZP_X: u8 = 0xd6;
const DEC_ABS: u8 = 0xce;
const DEC_ABS_X: u8 = 0xde;

const EOR_IMM: u8 = 0x49;
const EOR_ZP: u8 = 0x45;
const EOR_ZP_X: u8 = 0x55;
const EOR_ABS: u8 = 0x4d;
const EOR_ABS_X: u8 = 0x5d;
const EOR_ABS_Y: u8 = 0x59;
const EOR_IND_X: u8 = 0x41;
const EOR_IND_Y: u8 = 0x51;

const INC_ZP: u8 = 0xe6;
const INC_ZP_X: u8 = 0xf6;
const INC_ABS: u8 = 0xee;
const INC_ABS_X: u8 = 0xfe;

const JMP: u8 = 0x4c;
const JMP_IND: u8 = 0x6c;
const JSR: u8 = 0x20;

const LDA_IMM: u8 = 0xa9;
const LDA_ZP: u8 = 0xa5;
const LDA_ZP_X: u8 = 0xb5;
const LDA_ABS: u8 = 0xad;
const LDA_ABS_X: u8 = 0xbd;
const LDA_ABS_Y: u8 = 0xb9;
const LDA_IND_X: u8 = 0xa1;
const LDA_IND_Y: u8 = 0xb1;

const LDX_IMM: u8 = 0xa2;
const LDX_ZP: u8 = 0xa6;
const LDX_ZP_Y: u8 = 0xb6;
const LDX_ABS: u8 = 0xae;
const LDX_ABS_Y: u8 = 0xbe;

const LDY_IMM: u8 = 0xa0;
const LDY_ZP: u8 = 0xa4;
const LDY_ZP_X: u8 = 0xb4;
const LDY_ABS: u8 = 0xac;
const LDY_ABS_X: u8 = 0xbc;

const LSR: u8 = 0x4a;
const LSR_ZP: u8 = 0x46;
const LSR_ZP_X: u8 = 0x56;
const LSR_ABS: u8 = 0x4e;
const LSR_ABS_X: u8 = 0x5e;

const ORA_IMM: u8 = 0x09;
const ORA_ZP: u8 = 0x05;
const ORA_ZP_X: u8 = 0x15;
const ORA_ABS: u8 = 0x0d;
const ORA_ABS_X: u8 = 0x1d;
const ORA_ABS_Y: u8 = 0x19;
const ORA_IND_X: u8 = 0x01;
const ORA_IND_Y: u8 = 0x11;

const NOP: u8 = 0xea;

const ROL: u8 = 0x2a;
const ROL_ZP: u8 = 0x26;
const ROL_ZP_X: u8 = 0x36;
const ROL_ABS: u8 = 0x2e;
const ROL_ABS_X: u8 = 0x3e;

const ROR: u8 = 0x6a;
const ROR_ZP: u8 = 0x66;
const ROR_ZP_X: u8 = 0x76;
const ROR_ABS: u8 = 0x6e;
const ROR_ABS_X: u8 = 0x7e;

const RTI: u8 = 0x40;

const STA_ZP: u8 = 0x85;
const STA_ZP_X: u8 = 0x95;
const STA_ABS: u8 = 0x8d;
const STA_ABS_X: u8 = 0x9d;
const STA_ABS_Y: u8 = 0x99;
const STA_IND_X: u8 = 0x81;
const STA_IND_Y: u8 = 0x91;

const STY_ZP: u8 = 0x84;
const STY_ZP_X: u8 = 0x94;
const STY_ABS: u8 = 0x8c;

const STX_ZP: u8 = 0x86;
const STX_ZP_Y: u8 = 0x96;
const STX_ABS: u8 = 0x8e;

const TXS: u8 = 0x9a;
const TSX: u8 = 0xba;
const PHA: u8 = 0x48;
const PLA: u8 = 0x68;
const PHP: u8 = 0x08;
const PLP: u8 = 0x28;

const SBC_IMM: u8 = 0xe9;
const SBC_ZP: u8 = 0xe5;
const SBC_ZP_X: u8 = 0xf5;
const SBC_ABS: u8 = 0xed;
const SBC_ABS_X: u8 = 0xfd;
const SBC_ABS_Y: u8 = 0xf9;
const SBC_IND_X: u8 = 0xe1;
const SBC_IND_Y: u8 = 0xf1;

const TAX: u8 = 0xaa;
const TXA: u8 = 0x8a;
const DEX: u8 = 0xca;
const INX: u8 = 0xe8;
const TAY: u8 = 0xa8;
const TYA: u8 = 0x98;
const DEY: u8 = 0x88;
const INY: u8 = 0xc8;

const RTS: u8 = 0x60;

pub const SIZES: [usize; 256] = [
    1, 2, 2, 1, 2, 2, 2, 2, 1, 2, 1, 1, 3, 3, 3, 3,  // 0x00-0x0f
    2, 2, 2, 1, 2, 2, 2, 2, 1, 3, 1, 1, 3, 3, 3, 3,  // 0x10-0x1f
    3, 2, 2, 1, 2, 2, 2, 2, 1, 2, 1, 1, 3, 3, 3, 3,  // 0x20-0x2f
    2, 2, 2, 1, 2, 2, 2, 2, 1, 3, 1, 1, 3, 3, 3, 3,  // 0x30-0x3f
    1, 2, 2, 1, 2, 2, 2, 2, 1, 2, 1, 1, 3, 3, 3, 3,  // 0x40-0x4f
    2, 2, 2, 1, 2, 2, 2, 2, 1, 3, 1, 1, 3, 3, 3, 3,  // 0x50-0x5f
    1, 2, 2, 1, 2, 2, 2, 2, 1, 2, 1, 1, 3, 3, 3, 3,  // 0x60-0x6f
    2, 2, 2, 1, 2, 2, 2, 2, 1, 3, 1, 1, 3, 3, 3, 3,  // 0x70-0x7f
    2, 2, 2, 1, 2, 2, 2, 2, 1, 2, 1, 1, 3, 3, 3, 3,  // 0x80-0x8f
    2, 2, 2, 1, 2, 2, 2, 2, 1, 3, 1, 1, 3, 3, 3, 3,  // 0x90-0x9f
    2, 2, 2, 1, 2, 2, 2, 2, 1, 2, 1, 1, 3, 3, 3, 3,  // 0xa0-0xaf
    2, 2, 2, 1, 2, 2, 2, 2, 1, 3, 1, 1, 3, 3, 3, 3,  // 0xb0-0xbf
    2, 2, 2, 1, 2, 2, 2, 2, 1, 2, 1, 1, 3, 3, 3, 3,  // 0xc0-0xcf
    2, 2, 2, 1, 2, 2, 2, 2, 1, 3, 1, 1, 3, 3, 3, 3,  // 0xd0-0xdf
    2, 2, 2, 1, 2, 2, 2, 2, 1, 2, 1, 1, 3, 3, 3, 3,  // 0xe0-0xef
    2, 2, 2, 1, 2, 2, 2, 2, 1, 3, 1, 1, 3, 3, 3, 3 // 0xf0-0xff
];

pub const OPCODE_NAMES: [&str; 256] = [
    "BRK", "ORA", "NOP", "NOP", "TSB", "ORA", "ASL", "RMB0",  // 0x00-0x07
    "PHP", "ORA", "ASL", "NOP", "TSB", "ORA", "ASL", "BBR0",  // 0x08-0x0f
    "BPL", "ORA", "ORA", "NOP", "TRB", "ORA", "ASL", "RMB1",  // 0x10-0x17
    "CLC", "ORA", "INC", "NOP", "TRB", "ORA", "ASL", "BBR1",  // 0x18-0x1f
    "JSR", "AND", "NOP", "NOP", "BIT", "AND", "ROL", "RMB2",  // 0x20-0x27
    "PLP", "AND", "ROL", "NOP", "BIT", "AND", "ROL", "BBR2",  // 0x28-0x2f
    "BMI", "AND", "AND", "NOP", "BIT", "AND", "ROL", "RMB3",  // 0x30-0x37
    "SEC", "AND", "DEC", "NOP", "BIT", "AND", "ROL", "BBR3",  // 0x38-0x3f
    "RTI", "EOR", "NOP", "NOP", "NOP", "EOR", "LSR", "RMB4",  // 0x40-0x47
    "PHA", "EOR", "LSR", "NOP", "JMP", "EOR", "LSR", "BBR4",  // 0x48-0x4f
    "BVC", "EOR", "EOR", "NOP", "NOP", "EOR", "LSR", "RMB5",  // 0x50-0x57
    "CLI", "EOR", "PHY", "NOP", "NOP", "EOR", "LSR", "BBR5",  // 0x58-0x5f
    "RTS", "ADC", "NOP", "NOP", "STZ", "ADC", "ROR", "RMB6",  // 0x60-0x67
    "PLA", "ADC", "ROR", "NOP", "JMP", "ADC", "ROR", "BBR6",  // 0x68-0x6f
    "BVS", "ADC", "ADC", "NOP", "STZ", "ADC", "ROR", "RMB7",  // 0x70-0x77
    "SEI", "ADC", "PLY", "NOP", "JMP", "ADC", "ROR", "BBR7",  // 0x78-0x7f
    "BRA", "STA", "NOP", "NOP", "STY", "STA", "STX", "SMB0",  // 0x80-0x87
    "DEY", "BIT", "TXA", "NOP", "STY", "STA", "STX", "BBS0",  // 0x88-0x8f
    "BCC", "STA", "STA", "NOP", "STY", "STA", "STX", "SMB1",  // 0x90-0x97
    "TYA", "STA", "TXS", "NOP", "STZ", "STA", "STZ", "BBS1",  // 0x98-0x9f
    "LDY", "LDA", "LDX", "NOP", "LDY", "LDA", "LDX", "SMB2",  // 0xa0-0xa7
    "TAY", "LDA", "TAX", "NOP", "LDY", "LDA", "LDX", "BBS2",  // 0xa8-0xaf
    "BCS", "LDA", "LDA", "NOP", "LDY", "LDA", "LDX", "SMB3",  // 0xb0-0xb7
    "CLV", "LDA", "TSX", "NOP", "LDY", "LDA", "LDX", "BBS3",  // 0xb8-0xbf
    "CPY", "CMP", "NOP", "NOP", "CPY", "CMP", "DEC", "SMB4",  // 0xc0-0xc7
    "INY", "CMP", "DEX", "NOP", "CPY", "CMP", "DEC", "BBS4",  // 0xc8-0xcf
    "BNE", "CMP", "CMP", "NOP", "NOP", "CMP", "DEC", "SMB5",  // 0xd0-0xd7
    "CLD", "CMP", "PHX", "NOP", "NOP", "CMP", "DEC", "BBS5",  // 0xd8-0xdf
    "CPX", "SBC", "NOP", "NOP", "CPX", "SBC", "INC", "SMB6",  // 0xe0-0xe7
    "INX", "SBC", "NOP", "NOP", "CPX", "SBC", "INC", "BBS6",  // 0xe8-0xef
    "BEQ", "SBC", "SBC", "NOP", "NOP", "SBC", "INC", "SMB7",  // 0xf0-0xf7
    "SED", "SBC", "PLX", "NOP", "NOP", "SBC", "INC", "BBS7" // 0xf8-0xff
];

pub enum AddressingType {
    IMMEDIATE, ZP, ZP_X, ZP_Y, ABSOLUTE, ABSOLUTE_X, ABSOLUTE_Y, INDIRECT_X, INDIRECT_Y, REGISTER_A,
    INDIRECT, RELATIVE, ZPI, AIX, NONE,
}

fn h(v: u8) -> String {
    return format!("{:02X}", v);
}

fn hh(v: u16) -> String {
    return format!("{:X}", v);
}

impl AddressingType {
    pub fn to_string(&self, pc: usize, byte: u8, word: u16) -> String {
        match self {
            AddressingType::IMMEDIATE => format!("#${}", h(byte)),
            AddressingType::ZP => format!("${}", h(byte)),
            AddressingType::ZP_X => format!("${},X", h(byte)),
            AddressingType::ZP_Y => format!("${},Y", h(byte)),
            AddressingType::ABSOLUTE => format!("${}", hh(word)),
            AddressingType::ABSOLUTE_X => format!("${},X", hh(word)),
            AddressingType::ABSOLUTE_Y => format!("${},Y", hh(word)),
            AddressingType::INDIRECT_X => format!("(${},X)", hh(word)),
            AddressingType::INDIRECT_Y => format!("(${}),Y", hh(word)),
            AddressingType::INDIRECT => format!("(${})", hh(word)),
            AddressingType::RELATIVE => {
                let p = pc as u16;
                let n = if byte >= 0x7f { p - 0x100 + (byte as u16) } else { p + byte as u16 };
                format!("${}", hh(n + 2))
            },
            _ => "".to_string()
        }
    }
}

use AddressingType::*;

pub const ADDRESSING_TYPES: [AddressingType; 256] = [
    NONE, INDIRECT_X, NONE, NONE,  // 0x00-0x03
    ZP, ZP, ZP, ZP,  // 0x04-0x07
    NONE, IMMEDIATE, REGISTER_A, NONE,  // 0x08-0x0b
    ABSOLUTE, ABSOLUTE, ABSOLUTE, RELATIVE,  // 0x0c-0x0f
    RELATIVE, INDIRECT_Y, ZPI, NONE,  // 0x10-0x13
    ZP, ZP_X, ZP_X, ZP,  // 0x14-0x17
    NONE, ABSOLUTE_Y, NONE, NONE,  // 0x18-0x1b
    ABSOLUTE, ABSOLUTE_X, ABSOLUTE_X, RELATIVE,  // 0x1c-0x1f
    ABSOLUTE, INDIRECT_X, NONE, NONE,  // 0x20-0x23
    ZP, ZP, ZP, ZP,  // 0x24-0x27
    NONE, IMMEDIATE, REGISTER_A, NONE,  // 0x28-0x2b
    ABSOLUTE, ABSOLUTE, ABSOLUTE, RELATIVE,  // 0x2c-0x2f
    RELATIVE, INDIRECT_Y, ZPI, NONE,  // 0x30-0x33
    ZP_X, ZP_X, ZP_X, ZP,  // 0x34-0x37
    NONE, ABSOLUTE_Y, NONE, NONE,  // 0x38-0x3b
    NONE, ABSOLUTE_X, ABSOLUTE_X, RELATIVE,  // 0x3c-0x3f
    NONE, INDIRECT_X, NONE, NONE,  // 0x40-0x43
    NONE, ZP, ZP, ZP,  // 0x44-0x47
    NONE, IMMEDIATE, REGISTER_A, NONE,  // 0x48-0x4b
    ABSOLUTE, ABSOLUTE, ABSOLUTE, RELATIVE,  // 0x4c-0x4f
    RELATIVE, INDIRECT_Y, ZPI, NONE,  // 0x50-0x53
    NONE, ZP_X, ZP_X, ZP,  // 0x54-0x57
    NONE, ABSOLUTE_Y, NONE, NONE,  // 0x58-0x5b
    NONE, ABSOLUTE_X, ABSOLUTE_X, RELATIVE,  // 0x5c-0x5f
    NONE, INDIRECT_X, NONE, NONE,  // 0x60-0x63
    ZP, ZP, ZP, ZP,  // 0x64-0x67
    NONE, IMMEDIATE, REGISTER_A, NONE,  // 0x68-0x6b
    INDIRECT, ABSOLUTE, ABSOLUTE, RELATIVE,  // 0x6c-0x6f
    RELATIVE, INDIRECT_Y, ZPI, NONE,  // 0x70-0x73
    ZP_X, ZP_X, ZP_X, ZP,  // 0x74-0x77
    NONE, ABSOLUTE_Y, NONE, NONE,  // 0x78-0x7b
    AIX, ABSOLUTE_X, ABSOLUTE_X, RELATIVE,  // 0x7c-0x7f
    RELATIVE, INDIRECT_X, NONE, NONE,  // 0x80-0x83
    ZP, ZP, ZP, ZP,  // 0x84-0x87
    NONE, NONE, NONE, NONE,  // 0x88-0x8b
    ABSOLUTE, ABSOLUTE, ABSOLUTE, RELATIVE,  // 0x8c-0x8f
    RELATIVE, INDIRECT_Y, ZPI, NONE,  // 0x90-0x93
    ZP_X, ZP_X, ZP_Y, ZP,  // 0x94-0x97
    NONE, ABSOLUTE_Y, NONE, NONE,  // 0x98-0x9b
    ABSOLUTE, ABSOLUTE_X, ABSOLUTE_X, RELATIVE,  // 0x9c-0x9f
    IMMEDIATE, INDIRECT_X, IMMEDIATE, NONE,  // 0xa0-0xa3
    ZP, ZP, ZP, ZP,  // 0xa4-0xa7
    NONE, IMMEDIATE, NONE, NONE,  // 0xa8-0xab
    ABSOLUTE, ABSOLUTE, ABSOLUTE, RELATIVE,  // 0xac-0xaf
    RELATIVE, INDIRECT_Y, ZPI, NONE,  // 0xb0-0xb3
    ZP_X, ZP_X, ZP_Y, ZP,  // 0xb4-0xb7
    NONE, ABSOLUTE_Y, NONE, NONE,  // 0xb8-0xbb
    ABSOLUTE_X, ABSOLUTE_X, ABSOLUTE_Y, RELATIVE,  // 0xbc-0xbf
    IMMEDIATE, INDIRECT_X, NONE, NONE,  // 0xc0-0xc3
    ZP, ZP, ZP, ZP,  // 0xc4-0xc7
    NONE, IMMEDIATE, NONE, NONE,  // 0xc8-0xcb
    ABSOLUTE, ABSOLUTE, ABSOLUTE, RELATIVE,  // 0xcc-0xcf
    RELATIVE, INDIRECT_Y, ZPI, NONE,  // 0xd0-0xd3
    NONE, ZP_X, ZP_X, ZP,  // 0xd4-0xd7
    NONE, ABSOLUTE_Y, NONE, NONE,  // 0xd8-0xdb
    NONE, ABSOLUTE_X, ABSOLUTE_X, RELATIVE,  // 0xdc-0xdf
    IMMEDIATE, INDIRECT_X, NONE, NONE,  // 0xe0-0xe3
    ZP, ZP, ZP, ZP,  // 0xe4-0xe7
    NONE, IMMEDIATE, NONE, NONE,  // 0xe8-0xeb
    ABSOLUTE, ABSOLUTE, ABSOLUTE, RELATIVE,  // 0xec-0xef
    RELATIVE, INDIRECT_Y, ZPI, NONE,  // 0xf0-0xf3
    NONE, ZP_X, ZP_X, ZP,  // 0xf4-0xf7
    NONE, ABSOLUTE_Y, NONE, NONE,  // 0xf8-0xfb
    NONE, ABSOLUTE_X, ABSOLUTE_X, RELATIVE // 0xfc-0xff
];
