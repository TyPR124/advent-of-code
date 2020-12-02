use anyhow::{Context, Result};
const INPUT: &str = include_str!("input.txt");
fn main() -> Result<()> {
    let numbers = parse_input(INPUT)?;
    let (a, b) = find_two_numbers_that_sum_to_2020(&numbers)?;
    println!("1: {}", a as usize * b as usize);
    let (a, b, c) = find_three_numbers_that_sum_to_2020(&numbers)?;
    println!("2: {}", a as usize * b as usize * c as usize);

    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<u16>> {
    input.lines().try_fold(vec![], |mut vec, line| {
        if line.is_empty() {
            return Result::<_>::Ok(vec);
        }
        let n = line
            .parse()
            .with_context(|| format!("Could not parse input line '{}'", line))?;
        vec.push(n);
        Ok(vec)
    })
}

fn find_two_numbers_that_sum_to_2020(numbers: &[u16]) -> Result<(u16, u16)> {
    find_two_numbers_that_sum_to_x(numbers, 2020, None)
        .context("Couldn't find two numbers that sum to 2020")
}

fn find_three_numbers_that_sum_to_2020(numbers: &[u16]) -> Result<(u16, u16, u16)> {
    find_three_numbers_that_sum_to_x(numbers, 2020)
        .context("Couldn't find three numbers that sum to 2020")
}

fn find_two_numbers_that_sum_to_x(
    numbers: &[u16],
    sum: u16,
    skip_index: Option<usize>,
) -> Option<(u16, u16)> {
    let mut need = std::collections::HashSet::new();
    for (i, &n) in numbers.iter().enumerate() {
        if skip_index == Some(i) {
            continue;
        }
        let want_for_n = if let Some(want) = sum.checked_sub(n) {
            want
        } else {
            continue;
        };
        if need.contains(&want_for_n) {
            return Some((n, want_for_n));
        }
        need.insert(n);
    }
    None
}

fn find_three_numbers_that_sum_to_x(numbers: &[u16], sum: u16) -> Option<(u16, u16, u16)> {
    for (i, &n) in numbers.iter().enumerate() {
        let want = if let Some(want) = sum.checked_sub(n) {
            want
        } else {
            continue;
        };
        if let Some((a, b)) = find_two_numbers_that_sum_to_x(numbers, want, Some(i)) {
            return Some((a, b, n));
        }
    }
    None
}
