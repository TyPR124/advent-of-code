mod input;
use input::INPUT;

fn main() {
    let mut layers = Vec::new();
    let mut fewest_zeroes = std::usize::MAX;
    let mut zero_count = std::usize::MAX;
    let mut best_layer = 0;
    for (i, x) in INPUT.trim().bytes().map(|x| x - b'0').enumerate() {
        let idx = i % (25 * 6);
        if idx == 0 {
            if zero_count < fewest_zeroes {
                fewest_zeroes = zero_count;
                best_layer = layers.len() - 1;
            }
            zero_count = 0;
            layers.push([0u8; 25 * 6]);
        }
        let len = layers.len();
        let layer = &mut layers[len-1];
        layer[idx] = x;

        if x == 0 { zero_count += 1 };
    }
    if zero_count < fewest_zeroes {
        // fewest_zeroes = zero_count;
        best_layer = layers.len() - 1;
    }
    let mut ones_count = 0;
    let mut twos_count = 0;
    layers[best_layer].iter().copied().for_each(|x| if x == 1 { ones_count += 1 } else if x == 2 { twos_count += 1 });
    println!("1. {}", ones_count * twos_count);

    let mut img = layers[0];
    img.iter_mut().enumerate().filter(|(_i, x)| **x == 2).for_each(|(i, x)| {
        *x = layers.iter().find(|layer| layer[i] != 2).unwrap()[i]
    });

    println!("2. Image:");

    for (i, x) in img.iter().copied().enumerate() {
        if i % 25 == 0 { println!() }
        if x == 0 { print!(" ") }
        if x == 1 { print!("#") }
    }

    println!();
}