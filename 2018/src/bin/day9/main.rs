extern crate aoc_2018;
use aoc_2018::Result;

use std::cell::RefCell;

fn main() -> Result<()> {
    let score = part1(470, 72170)?;
    println!("Part1: Score: {}", score);
    let score2 = part1(470, 72170 * 100)?;
    println!("Part2: Score: {}", score2);
    Ok(())
}

fn part1(n_players: usize, last_marble_score: usize) -> Result<usize> {
    let mut players = vec![0; n_players];
    let marble_count = last_marble_score + 1;
    play_game(&mut players, marble_count);
    Ok(*players.iter().max()?)
}

#[test]
fn test_part1() {
    assert_eq!(32, part1(10, 25).unwrap());
    assert_eq!(8317, part1(10, 1618).unwrap());
    assert_eq!(146373, part1(13, 7999).unwrap());
    assert_eq!(2764, part1(17, 1104).unwrap());
    assert_eq!(54718, part1(21, 6111).unwrap());
    assert_eq!(37305, part1(30, 5807).unwrap());
}

fn play_game(scores: &mut Vec<usize>, marbles: usize) {
    let mut board = Board::new(0, marbles - (marbles / 26));
    let mut player = 0;
    // First marble already in board
    for v in 1..marbles {
        if v % 23 == 0 {
            // "First, the current player keeps the marble they would have placed, adding it to their score."
            scores[player] += v;
            // "the marble 7 marbles counter-clockwise from the current marble is removed from the circle ...
            // ... and also added to the current player's score."
            board.counter_clockwise(6);
            scores[player] += *board.remove_counter_clockwise();
            // "The marble located immediately clockwise of the marble that was removed becomes the new current marble."
            // This is already true
        } else {
            board.clockwise(1);
            board.current = board.insert_clockwise(v);
        }
        player += 1;
        if player == scores.len() { player = 0; }
    }
}
#[derive(Clone)]
struct Marble<T> {
    pub value: RefCell<T>,
    pub next: usize,
    pub prev: usize,
}
impl<T> Marble<T> {
    fn new(value: T, prev: usize, next: usize) -> Marble<T> {
        Marble {
            value: RefCell::new(value),
            next,
            prev
        }
    }
}
struct Board<T> {
    marbles: Vec<Marble<T>>,
    current: usize,
}
impl<T> Board<T> {
    pub fn new(first: T, count: usize) -> Board<T> {
        let mut marbles = Vec::with_capacity(count);
        marbles.push(Marble::new(first, 0, 0));
        Board {
            marbles,
            current: 0
        }
    }
    pub fn clockwise(&mut self, n: usize) {
        for _ in 0..n {
            self.current = self.marbles[self.current].next;
        }
    }
    pub fn counter_clockwise(&mut self, n: usize) {
        for _ in 0..n {
            self.current = self.marbles[self.current].prev;
        }
    }
    // Returns index of inserted marble
    fn insert_after(&mut self, new: T) -> usize {
        // New marble, prev is current, next is current's next
        let new_marble = Marble::new(new, self.current, self.marbles[self.current].next);
        let new_index = self.marbles.len();
        // Current marble must point to new marble
        let current_next = self.marbles[self.current].next; // Copy out for later
        self.marbles[self.current].next = new_index;
        // Next marble must point to new marble
        self.marbles[current_next].prev = new_index;
        // Finally, insert new
        self.marbles.push(new_marble);
        new_index
    }
    pub fn insert_clockwise(&mut self, new: T) -> usize {
        self.insert_after(new)
    }
    pub fn remove_counter_clockwise(&mut self) -> std::cell::RefMut<T> {
        let to_remove = self.marbles[self.current].prev;
        let removed_prev = self.marbles[to_remove].prev;
        let removed_next = self.marbles[to_remove].next;
        self.marbles[removed_prev].next = removed_next;
        self.marbles[removed_next].prev = removed_prev;
        self.marbles[to_remove].value.borrow_mut()
    }
}