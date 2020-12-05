mod input;
use self::input::INPUT;

extern crate aoc_2018;

fn main() {
    let md_sum = part1(INPUT);
    println!("Part1: Metadata Sum: {}", md_sum);
    let value = part2(INPUT);
    println!("Part2: Summarized Node Value: {}", value);
}

fn part1(input: &str) -> usize {
    let data: Vec<u8> = input
        .trim()
        .split(' ')
        .map(|s| s.parse().expect("Found invalid input"))
        .collect();
    let iter = MetadataIterator::new(&data);
    iter.map(|x| x as usize).sum()
}

struct NodeSummary {
    pub value: usize,
    pub len: usize,
}

fn part2(input: &str) -> usize {
    let data: Vec<u8> = input
        .trim()
        .split(' ')
        .map(|s| s.parse().expect("Found invalid input"))
        .collect();
    summarize_node(&data).value
}

fn summarize_node(data: &[u8]) -> NodeSummary {
    //println!("Summarizing node");
    let header = Header::raw_ref(data);
    let mut len = std::mem::size_of::<Header>(); // Will be incremented as read from data
    let mut value = 0;
    if header.child_count == 0 {
        //println!(" Node has no children");
        for _ in 0..header.metadata_count {
            value += data[len] as usize;
            len += 1;
        }
        //println!(" Node sums to {}", value);
        NodeSummary { value, len }
    } else {
        //println!(" Need to first summarize {} children", header.child_count);
        let mut children = Vec::with_capacity(header.child_count as usize);
        for _ in 0..header.child_count {
            let child = summarize_node(&data[len..]);
            len += child.len;
            children.push(child);
            //println!("Child {} on stack with value {}", children.len()-1, children[children.len()-1].value);
        }
        //println!("Got {} children", children.len());
        for _ in 0..header.metadata_count {
            let m = data[len] as usize;
            len += 1;
            // "A metadata entry of 0 does not refer to any child node."
            if m <= children.len() && m > 0 {
                // "A metadata entry of 1 refers to the first child node"
                value += children[m - 1].value;
            }
        }
        NodeSummary { value, len }
    }
}

struct MetadataIterator<'n> {
    data: &'n [u8],     // Data that makes up node(s)
    header: &'n Header, // Will point to first two bytes of data
    ci: u8,             // Child Index
    mr: u8,             // Count of own (non-child) Metadata Returns
    child: Option<Box<MetadataIterator<'n>>>,
    i: usize, // Byte Index
}

impl<'n> MetadataIterator<'n> {
    pub fn new(data: &'n [u8]) -> MetadataIterator {
        use std::mem::size_of;
        let header = Header::raw_ref(data);
        let child = if header.child_count == 0 {
            None
        } else {
            Some(Box::new(MetadataIterator::new(
                &data[size_of::<Header>()..],
            )))
        };

        MetadataIterator {
            data,
            header,
            i: size_of::<Header>(), // Start after header
            ci: 0,
            mr: 0,
            child,
        }
    }
    fn next_recursive(&mut self) -> ChildResult<u8> {
        if let Some(child) = &mut self.child {
            match child.next_recursive() {
                ChildResult::Item(item) => ChildResult::Item(item),
                ChildResult::End(bytes) => {
                    // This child is done
                    self.i += bytes;
                    self.ci += 1;
                    if self.ci < self.header.child_count {
                        self.child = Some(Box::new(MetadataIterator::new(&self.data[self.i..])));
                        self.next_recursive() // Check the child we just made
                    } else {
                        self.child = None;
                        self.next_recursive()
                    } // This self.next_recursive will fall through to own metadata
                }
            }
        } else {
            // There is no child
            if self.mr == self.header.metadata_count {
                // No more metadata
                ChildResult::End(self.i)
            } else {
                // Return a metadata
                self.mr += 1;
                self.i += 1;
                ChildResult::Item(self.data[self.i - 1])
            }
        }
    }
}

enum ChildResult<T> {
    Item(T),
    End(usize),
}

impl<'n> Iterator for MetadataIterator<'n> {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        match self.next_recursive() {
            ChildResult::End(_) => None,
            ChildResult::Item(item) => Some(item),
        }
    }
}

#[repr(C)]
struct Header {
    pub child_count: u8,
    pub metadata_count: u8,
}

impl Header {
    pub fn raw_ref(data: &[u8]) -> &Header {
        use std::mem::size_of;
        if data.len() < size_of::<Header>() {
            panic!("Data len too small for header!")
        } else {
            unsafe { &*(data.as_ptr() as *const Header) }
        }
    }
}
