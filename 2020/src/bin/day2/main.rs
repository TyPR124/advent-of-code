use std::ops::RangeInclusive;

use anyhow::{Context, Result};

const INPUT: &str = include_str!("input.txt");
fn main() -> Result<()> {
    let password_records = INPUT
        .lines()
        .filter(|line| !line.is_empty())
        .try_fold::<_, _, Result<_>>(vec![], |mut vec, line| {
            let record = PasswordRecord::from_str(line)?;
            vec.push(record);
            Ok(vec)
        })?;

    let valid_sled_corp_passwords = password_records
        .iter()
        .filter(|record| record.is_valid_sled_corp_password())
        .count();
    println!("1: {}", valid_sled_corp_passwords);
    let valid_toboggan_corp_passwords = password_records
        .iter()
        .filter(|record| record.is_valid_toboggan_corp_password())
        .count();
    println!("2: {}", valid_toboggan_corp_passwords);

    Ok(())
}

pub struct Policy {
    required_char: char,
    count_range: RangeInclusive<usize>,
}

pub struct PasswordRecord<'a> {
    policy: Policy,
    password: &'a str,
}

impl<'a> PasswordRecord<'a> {
    fn from_str(s: &'a str) -> Result<Self> {
        let mut parts = s.split(&['-', ' ', ':'][..]);
        let min = parts
            .next()
            .and_then(|s| s.parse().ok())
            .context("Failed to parse min policy value")?;
        let max = parts
            .next()
            .and_then(|s| s.parse().ok())
            .context("Failed to parse max policy value")?;
        let count_range = min..=max;
        let required_char = parts
            .next()
            .and_then(|s| s.parse().ok())
            .context("Failed to prase policy required character")?;
        parts.next(); // skip empty part
        let password = parts
            .next()
            .context("Failed to parse password record, missing password")?;
        let policy = Policy {
            count_range,
            required_char,
        };
        Ok(Self { policy, password })
    }

    fn is_valid_sled_corp_password(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|c| *c == self.policy.required_char)
            .count();
        count >= *self.policy.count_range.start() && count <= *self.policy.count_range.end()
    }

    fn is_valid_toboggan_corp_password(&self) -> bool {
        let first_index = self.policy.count_range.start() - 1;
        let second_index = self.policy.count_range.end() - 1;
        let mut chars = self.password.chars().take(second_index + 1);
        let first_char = chars.nth(first_index);
        let second_char = chars.last();
        (first_char == Some(self.policy.required_char))
            ^ (second_char == Some(self.policy.required_char))
    }
}
