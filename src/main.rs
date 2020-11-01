mod constants;

fn main() {
    sixty();
}

use std::io::prelude::*;
use std::io;
use rand::Rng;
use std::fs::File;

use std::cmp::Ordering;

fn sixty() {
    let mut buffer: Vec<u8> = Vec::new();
    let mut f = File::open("6502_functional_test.bin").expect("Couldn't find the file");
    f.read_to_end(&mut buffer);
    let start = buffer[0x400];

    let mut i: u16 = 0x400;
    while i < 0x600 {
        let index = i as usize;
        let opcode = buffer[index] as usize;
        let name = constants::OPCODE_NAMES[opcode];
        let size = constants::SIZES[opcode];
        if size == 1 {
            println!("{:04X}: {}", i, name);
        } else if size == 2 {
            println!("{:04X}: {} {:02X}", i, name, buffer[index + 1]);
        } else {
            let word: u16 = buffer[index + 1] as u16 | ((buffer[index + 2] as u16) << 8);
            println!("{:04X}: {} {:X}", i, name, word);
        }
        i = i + size as u16;
    }
    println!("Read file");
}

fn game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
