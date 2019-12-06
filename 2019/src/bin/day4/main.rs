const LO: u64 = 273025;
const HI: u64 = 767253;

fn main() {
    let count1 = (LO..=HI).filter(is_valid1).count();
    println!("1. {}", count1);
    let count2 = (LO..=HI).filter(is_valid2).count();
    println!("2. {}", count2);
}

fn is_valid1(x: &u64) -> bool {
    let x = *x;
    if x < LO || x > HI { return false }
    let mut have_pair = false;
    let mut last_digit = 0;

    for i in (0..=5).rev() {
        let digit = (x / (10u64.pow(i))) % 10;
        if digit == last_digit { have_pair = true }
        else if digit < last_digit { return false }
        else { last_digit = digit }
    }
    have_pair
}

fn is_valid2(x: &u64) -> bool {
    let x = *x;
    if x < LO || x > HI { return false }

    let mut counts = [0; 10];
    let mut last_digit = 0;

    for i in (0..=5).rev() {
        let digit = (x / (10u64.pow(i))) % 10;
        if digit < last_digit { return false }
        counts[digit as usize] += 1;
        last_digit = digit;
    }

    for i in 0..=9 {
        if counts[i] == 2 { return true }
    }
    false
}