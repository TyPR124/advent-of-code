#![feature(step_trait)]
mod input;
use input::INPUT;

use aoc_2019::*;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut remaining = vec![0, 1, 2, 3, 4];
    let mut phases = Vec::with_capacity(5);
    let mut best = std::i64::MIN;
    for i1 in 0..5 {
        phases.push(remaining.swap_remove(i1));
        for i2 in 0..4 {
            phases.push(remaining.swap_remove(i2));
            for i3 in 0..3 {
                phases.push(remaining.swap_remove(i3));
                for i4 in 0..2 {
                    phases.push(remaining.swap_remove(i4));
                    for i5 in 0..1 {
                        phases.push(remaining.swap_remove(i5));

                        let v = run_with(&phases);
                        if v > best {
                            best = v
                        }

                        remaining.push(phases.pop().unwrap());
                        let end = remaining.len() - 1;
                        remaining.swap(i5, end);
                    } // i5
                    remaining.push(phases.pop().unwrap());
                    let end = remaining.len() - 1;
                    remaining.swap(i4, end);
                } // i4
                remaining.push(phases.pop().unwrap());
                let end = remaining.len() - 1;
                remaining.swap(i3, end);
            } // i3
            remaining.push(phases.pop().unwrap());
            let end = remaining.len() - 1;
            remaining.swap(i2, end);
        } // i2
        remaining.push(phases.pop().unwrap());
        let end = remaining.len() - 1;
        remaining.swap(i1, end);
    } // i1

    println!("1. {}", best);
}

fn run_with(phases: &[i64]) -> i64 {
    assert_eq!(5, phases.len());

    let mut signal = 0;

    phases.iter().copied().for_each(|p| {
        let (cpu, tx, rx) = Cpu::from_input(INPUT);
        let handle = cpu.run_parallel();
        tx.send(p).unwrap();
        tx.send(signal).unwrap();
        signal = rx.recv().unwrap();
        assert!(handle.join().map(|r| r.is_ok()).unwrap_or(false));
    });

    signal
}

fn part2() {
    let mut remaining = vec![5, 6, 7, 8, 9];
    let mut phases = Vec::with_capacity(5);
    let mut best = std::i64::MIN;
    for i1 in 0..5 {
        phases.push(remaining.swap_remove(i1));
        for i2 in 0..4 {
            phases.push(remaining.swap_remove(i2));
            for i3 in 0..3 {
                phases.push(remaining.swap_remove(i3));
                for i4 in 0..2 {
                    phases.push(remaining.swap_remove(i4));
                    for i5 in 0..1 {
                        phases.push(remaining.swap_remove(i5));

                        let v = run_with_feedback(&phases);
                        if v > best {
                            best = v
                        }

                        remaining.push(phases.pop().unwrap());
                        let end = remaining.len() - 1;
                        remaining.swap(i5, end);
                    } // i5
                    remaining.push(phases.pop().unwrap());
                    let end = remaining.len() - 1;
                    remaining.swap(i4, end);
                } // i4
                remaining.push(phases.pop().unwrap());
                let end = remaining.len() - 1;
                remaining.swap(i3, end);
            } // i3
            remaining.push(phases.pop().unwrap());
            let end = remaining.len() - 1;
            remaining.swap(i2, end);
        } // i2
        remaining.push(phases.pop().unwrap());
        let end = remaining.len() - 1;
        remaining.swap(i1, end);
    } // i1

    println!("2. {}", best);
}

fn run_with_feedback(phases: &[i64]) -> i64 {
    // println!("Feedback with {:?}", phases);
    #![allow(clippy::clippy::many_single_char_names)]
    let (a, a_tx, a_rx) = Cpu::from_input(INPUT);
    let (b, b_tx, b_rx) = Cpu::from_input(INPUT);
    let (c, c_tx, c_rx) = Cpu::from_input(INPUT);
    let (d, d_tx, d_rx) = Cpu::from_input(INPUT);
    let (e, e_tx, e_rx) = Cpu::from_input(INPUT);

    let ah = a.run_parallel_and_drop();
    let bh = b.run_parallel_and_drop();
    let ch = c.run_parallel_and_drop();
    let dh = d.run_parallel_and_drop();
    let eh = e.run_parallel_and_drop();

    a_tx.send(phases[0]).unwrap();
    b_tx.send(phases[1]).unwrap();
    c_tx.send(phases[2]).unwrap();
    d_tx.send(phases[3]).unwrap();
    e_tx.send(phases[4]).unwrap();

    let mut last_signal = 0;

    loop {
        if a_tx.send(last_signal).is_err() {
            break;
        }
        // println!("Sent to a");
        b_tx.send(a_rx.recv().unwrap()).unwrap();
        // println!("Sent to b");
        c_tx.send(b_rx.recv().unwrap()).unwrap();
        d_tx.send(c_rx.recv().unwrap()).unwrap();
        e_tx.send(d_rx.recv().unwrap()).unwrap();
        last_signal = e_rx.recv().unwrap();
    }

    assert!(ah.join().unwrap().is_ok());
    assert!(bh.join().unwrap().is_ok());
    assert!(ch.join().unwrap().is_ok());
    assert!(dh.join().unwrap().is_ok());
    assert!(eh.join().unwrap().is_ok());

    last_signal
}
