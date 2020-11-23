#[cfg(test)]
mod tests {
    use crate::memory::Memory;
    use crate::cpu::{Cpu, CpuListener, RunStatus};

    struct Listener {
        previous_pc: usize
    }

    impl CpuListener for Listener {
        fn on_pc_changed(&mut self, cpu: &Cpu) -> RunStatus {
            let result =
            if cpu.pc == 0x346c || cpu.pc == 0x3469 {
                RunStatus::Stop(true, String::from("All tests passed"))
            } else {
                if self.previous_pc != 0 && self.previous_pc == cpu.pc {
                    RunStatus::Stop(false,
                                    format!("Infinite loop at PC={:2X} cycles={:04X} {}",
                                            cpu.pc, cpu.cycles, cpu))
                } else {
                    self.previous_pc = cpu.pc;
                    RunStatus::Continue
                }
            };
            result
        }
    }

    #[test]
    fn functional_tests() {
        let m = Memory::new_with_file("6502_functional_test.bin", None);
        let status = Cpu::new(m, Some(Box::new(Listener{ previous_pc: 0}))).run(0x400);
        match status {
            RunStatus::Stop(success, reason) => {
                if success {
                    println!("SUCCESS: {}", reason);
                } else {
                    assert!(success, reason)
                }
            },
            _ => { unimplemented!("Should never happen"); }
        }
    }
}