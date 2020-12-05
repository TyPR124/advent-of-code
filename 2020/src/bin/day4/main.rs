fn main() {
    both_parts(include_str!("input.txt"));
}

fn split_once(s: &str, ch: char) -> Option<(&str, &str)> {
    s.find(ch).map(|n| (&s[..n], &s[n + ch.len_utf8()..]))
}

fn passports_iter(
    input: &str,
) -> impl Iterator<Item = impl Iterator<Item = (&str, &str)>> {
    let mut nl_count = 0;
    input
        .split(move |ch| match ch {
            '\n' => {
                nl_count += 1;
                if nl_count == 2 {
                    nl_count = 0;
                    true
                } else {
                    false
                }
            }
            '\r' => false,
            _ => {
                nl_count = 0;
                false
            }
        })
        .map(|entry| {
            // println!("\nentry:{}", entry);
            entry
                .trim()
                .split_whitespace()
                .map(|kv| split_once(kv, ':').expect("Invalid passport"))
        })
}

fn both_parts(input: &str) {
    let mut map = std::collections::HashMap::new();
    map.insert("byr", None);
    map.insert("iyr", None);
    map.insert("eyr", None);
    map.insert("hgt", None);
    map.insert("hcl", None);
    map.insert("ecl", None);
    map.insert("pid", None);

    let mut p1_valid_count = 0;
    let mut p2_valid_count = 0;

    for passport in passports_iter(input.trim()) {
        // println!();
        // println!("In new passport");
        for (k, v) in passport {
            // println!("{} = {}", k, v);
            map.entry(k).and_modify(|have| *have = Some(v));
        }
        let mut is_p1_valid = true;
        let mut is_p2_valid = true;
        for (&k, v) in &mut map {
            is_p1_valid &= v.is_some();
            let v = if let Some(v) = v.take() { v } else { continue };
            is_p2_valid &= match k {
                "byr" => v
                    .parse::<u16>()
                    .map_or(false, |byr| (1920..=2002).contains(&byr)),
                "iyr" => v
                    .parse::<u16>()
                    .map_or(false, |iyr| (2010..=2020).contains(&iyr)),
                "eyr" => v
                    .parse::<u16>()
                    .map_or(false, |eyr| (2020..=2030).contains(&eyr)),
                "hgt" => validate_hgt(v),
                "hcl" => {
                    v.starts_with('#')
                        && v.len() == 7
                        && v[1..]
                            .chars()
                            .all(|ch| ('0'..='9').contains(&ch) || ('a'..='f').contains(&ch))
                }
                "ecl" => matches!(v, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
                "pid" => v.len() == 9 && v.chars().all(|ch| ('0'..='9').contains(&ch)),
                _ => true,
            }
        }
        if is_p1_valid {
            p1_valid_count += 1
        }
        if is_p1_valid && is_p2_valid {
            p2_valid_count += 1
        }
    }
    println!("1: {}\n2: {}", p1_valid_count, p2_valid_count);
}

fn validate_hgt(s: &str) -> bool {
    if !s.is_char_boundary(s.len() - 2) {
        return false;
    }
    let (n, u) = s.split_at(s.len() - 2);
    n.parse::<u8>().map_or(false, |n| match u {
        "cm" => (150..=193).contains(&n),
        "in" => (59..=76).contains(&n),
        _ => false,
    })
}
