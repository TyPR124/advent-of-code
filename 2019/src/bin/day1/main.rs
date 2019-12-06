mod input;
use input::INPUT;

use std::str::FromStr;

fn main() {
    let sum: u64 = INPUT.lines()
        .flat_map(u64::from_str)
        .map(|x| x/3 - 2).sum();
    println!("1. Sum of modules is {}", sum);
    
    let sum2: u64 = INPUT.lines()
        .flat_map(u64::from_str)
        .map(|x| {
            let mut last = x/3 - 2;
            let mut module_sum = last;

            while last > 8 {
                last = last / 3 - 2;
                module_sum += last;
            }

            module_sum
        })
        .sum();
    println!("2. Sum with additional fuel is {}", sum2);
}

