fn main() {
    let mut occupied = [false; 1024];
    let input = include_str!("input.txt").trim();
    let max_occupied = input
        .lines()
        .map(|line| {
            let sid = line
                .bytes()
                .fold(0, |sid, b| b"BR".contains(&b) as u16 + (sid << 1));
            occupied[sid as usize] = true;
            sid
        })
        .max()
        .expect("Missing seats");
    println!("1: {}", max_occupied);
    let my_sid = (0..max_occupied)
        .rev()
        .find(|&sid| !occupied[sid as usize])
        .expect("Can't find my seat");
    println!("2: {}", my_sid);
}
