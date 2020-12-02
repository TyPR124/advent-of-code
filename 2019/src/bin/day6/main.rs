mod input;
use input::INPUT;

struct Node {
    id: &'static str,
    // index: usize,
    orbiting: usize,
    count: Option<usize>,
}

impl Node {
    fn new(id: &'static str) -> Self {
        Node {
            id,
            orbiting: 0,
            count: None,
        }
    }
}

fn main() {
    use std::collections::HashMap;
    let mut found = HashMap::new(); // <&'static str, usize>
    let mut nodes = Vec::new();
    nodes.push(Node::new("COM"));
    nodes[0].count = Some(0);
    found.insert("COM", 0);
    for line in INPUT.trim().lines() {
        let (orbiting, orbiter) = line.split_at(3);
        let orbiter = &orbiter[1..];
        // Insert
        let orbiting_index;
        let orbit_count;
        if let Some(idx) = found.get(orbiting).copied() {
            orbiting_index = idx;
            orbit_count = nodes[idx].count.map(|c| c + 1);
        } else {
            orbiting_index = nodes.len();
            orbit_count = None;
            nodes.push(Node::new(orbiting));
            found.insert(orbiting, orbiting_index);
        }

        if let Some(idx) = found.get(orbiter).copied() {
            let node = &mut nodes[idx];
            node.orbiting = orbiting_index;
            node.count = orbit_count;
        } else {
            let orbiter_index = nodes.len();
            let mut node = Node::new(orbiter);
            node.orbiting = orbiting_index;
            node.count = orbit_count;
            nodes.push(node);
            found.insert(orbiter, orbiter_index);
        }
    }

    let mut chain = Vec::new();
    for i in 1..nodes.len() {
        let mut j = i;
        while nodes[j].count.is_none() {
            chain.push(j);
            j = nodes[j].orbiting;
        }

        let mut count = nodes[j].count.unwrap();
        while let Some(j) = chain.pop() {
            count += 1;
            nodes[j].count = Some(count);
        }
    }

    let total_orbits: usize = nodes.iter().map(|n| n.count.unwrap()).sum();
    println!("1. {}", total_orbits);

    let mut transfers = HashMap::new(); // <&'static str, usize>
    let mut count = 0;
    let mut i = found.get("YOU").copied().unwrap();
    loop {
        i = nodes[i].orbiting;
        transfers.insert(nodes[i].id, count);
        if i == 0 {
            break;
        }
        count += 1;
    }
    let mut count = 0;
    let mut i = found.get("SAN").copied().unwrap();
    loop {
        i = nodes[i].orbiting;
        if let Some(you_count) = transfers.get(nodes[i].id).copied() {
            println!("2. {}", you_count + count);
            return;
        }
        count += 1;
    }
}
