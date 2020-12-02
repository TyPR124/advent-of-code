mod input;
use input::INPUT;

use aoc_2019::*;

fn main() {
    let (mut cpu, tx, rx) = Cpu::from_input(INPUT);
    tx.send(1).unwrap();
    cpu.run().unwrap();
    println!("1. {}", rx.recv().unwrap());

    let (mut cpu, tx, rx) = Cpu::from_input(INPUT);
    tx.send(2).unwrap();
    cpu.run().unwrap();
    println!("2. {}", rx.recv().unwrap());
}
