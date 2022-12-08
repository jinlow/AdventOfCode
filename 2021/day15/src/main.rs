use std::collections::{HashMap, BinaryHeap, HashSet};
use std::error::Error;
use std::fs;
use std::cmp::Ordering;
use std::cmp::Reverse;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Debug)]
struct Node {
    risk: usize,
    coord: (usize, usize),
}

impl Node {
    fn new(risk: usize,
        coord: (usize, usize),) -> Self {
            Node {
                risk,
                coord,
            } 
        }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.risk.cmp(&other.risk)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("scratch.txt")?;
    let mut nodes = HashMap::new();
    let mut paths = BinaryHeap::new();
    let mut shortest_path = HashSet::new();
    // Collect nodes
    for (i, l) in file.lines().enumerate() {
        for (j, r) in l.chars().enumerate() {
            let risk = if (i, j) == (0, 0) {
                0
            } else {
                usize::MAX
            };
            paths.push(Reverse(Node::new(risk, (i, j))));
            nodes.insert(
                (i, j),
                r.to_digit(10).expect("Unable to convert to digit.") as usize,
            );
        }
    }

    while shortest_path.len() < paths.len() {
        
    }

    println!("{:?}", paths.peek());
    println!("{:?}", nodes.get(&(0, 2)));
    Ok(())
}
