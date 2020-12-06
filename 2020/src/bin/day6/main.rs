fn main() {
    let mut sum_of_any_yes = 0;
    let mut sum_of_all_yes = 0;
    let mut persons_in_group = 0;
    let mut have = std::collections::HashMap::<char, usize>::new();
    for line in include_str!("input.txt").trim().lines() {
        if line.is_empty() {
            sum_of_any_yes += have.len();
            for (_, v) in have.drain() {
                if v == persons_in_group {
                    sum_of_all_yes += 1;
                }
            }
            persons_in_group = 0;
            continue;
        }
        persons_in_group += 1;
        for c in line.chars() { *have.entry(c).or_default() += 1usize };
    }
    sum_of_any_yes += have.len();
    for (_, v) in have {
        if v == persons_in_group { sum_of_all_yes += 1 }
    }
    println!("1: {}", sum_of_any_yes);
    println!("2: {}", sum_of_all_yes);
}
