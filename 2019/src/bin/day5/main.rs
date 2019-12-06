mod input;
use input::INPUT;

use aoc_2019::*;

fn main() {
    let (mut cpu, tx, rx) = Cpu::from_input(INPUT);
    tx.send(1).unwrap();
    cpu.run().unwrap();
    let mut out = 0;
    while let Ok(o) = rx.try_recv() {
        if out != 0 { panic!("Bad output in part 1") }
        out = o;
    }
    println!("1. {}", out);

    let (mut cpu, tx, rx) = Cpu::from_input(INPUT);
    tx.send(5).unwrap();
    cpu.run().unwrap();
    let mut out = 0;
    while let Ok(o) = rx.try_recv() {
        out = o;
    }
    println!("2. {}", out);
}