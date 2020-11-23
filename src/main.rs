use crate::constants::{LDA_IMM, RTS};
use crate::memory::Memory;
use crate::cpu::Cpu;

mod constants;
mod cpu;
mod memory;
mod test;

// use crate::cpu::{Cpu, CpuListener};
// use crate::memory::Memory;

fn main() {
    let vec = vec!(LDA_IMM, 0x42, RTS);
    let m = Memory::new_with_vec(vec, None);
    Cpu::new(m, None).run(0x0);
}


