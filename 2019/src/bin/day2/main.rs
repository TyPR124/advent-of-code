mod input;
use input::INPUT;

use std::str::FromStr;

fn main() {
    let data0: Vec<usize> = INPUT.split(",")
        .flat_map(usize::from_str)
        .collect();

    let mut i = 0;
    let mut data = data0.clone();
    data[1] = 12;
    data[2] = 2;
    run_program(&mut data);
    println!("1. Value at position 0 is {}", data[0]);

    const GOAL: usize = 19690720;
    'outer: for a in 0..=99 {
        for b in 0..=99 {
            let mut data = data0.clone();
            data[1] = a;
            data[2] = b;
            run_program(&mut data);
            if data[0] == GOAL {
                println!("2. {}", 100*a + b);
                break 'outer;
            }
        }
    }

    main2();
}

fn run_program(data: &mut [usize]) {
    let mut i = 0;
    loop {
        let op = data[i];
        if op == 99 { break }
        let v1 = data[data[i+1]];
        let v2 = data[data[i+2]];
        let out_index = data[i+3];
        data[out_index] = if op == 1 { v1 + v2 }
                          else { v1 * v2 };
        i += 4;
    }
}

use aoc_2019::*;
fn main2() {
    let data0: Vec<isize> = INPUT.split(",")
        .flat_map(isize::from_str)
        .collect();
    let mut data = data0.clone();
    data[1] = 12;
    data[2] = 2;
    let (mut cpu, ..) = Cpu::new(data);
    cpu.run().unwrap();
    println!("1. {}", cpu.mem()[0]);

    const GOAL: isize = 19690720;
    'outer: for a in 0..=99 {
        for b in 0..=99 {
            let mut data = data0.clone();
            data[1] = a;
            data[2] = b;
            let (mut cpu, ..) = Cpu::new(data);
            cpu.run().unwrap();
            if cpu.mem()[0] == GOAL {
                println!("2. {}", 100*a + b);
                break 'outer;
            }
        }
    }
}