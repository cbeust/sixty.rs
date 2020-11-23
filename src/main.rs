mod constants;
mod cpu;
mod memory;
mod test;

use crate::cpu::{Cpu, CpuListener};
use crate::memory::Memory;


fn main() {
    // let m = Memory::new("6502_functional_test.bin");
    // Cpu::new(m, Some(Box::new(Listener{ previous_pc: 0}))).run(0x400);
}


