# sixty.rs
A 6502 emulator in Rust

This is a straight port of the 6502 emulator I wrote for [https://github.com/cbeust/sixty](my Apple ][ emulator).

`cargo test` will run [Klaus' functional suite for the 6502](https://github.com/Klaus2m5/6502_65C02_functional_tests), which guarantees that the emulation is correct. Additionally, my emulator boots a few Apple ][ games that use precise cycle timing for their protection, so I'm reasonably confident the cycle counting is correct as well, including the handling of page crossing and "branch taken", but there are no tests for cycle counting.

This code is pretty rigid right now, it needs to add some kind of listener support for the memory reads and writes in order to be usable in an emulator, but this should be pretty trivial to add.
  
