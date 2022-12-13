use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::fs;

fn calc_weight(b: u8) -> usize {
    (b - b'a' + 1) as usize
}

// Return the indices of the neighbor nodes
fn get_neighbors(node: &(usize, usize), graph_max: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    // up
    if node.0 > 0 {
        neighbors.push((node.0 - 1, node.1));
    }
    // down
    if graph_max.0 > node.0 {
        neighbors.push((node.0 + 1, node.1));
    }
    // left
    if node.1 > 0 {
        neighbors.push((node.0, node.1 - 1));
    }
    // right
    if graph_max.1 > node.1 {
        neighbors.push((node.0, node.1 + 1));
    }
    neighbors
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
struct Node {
    key: (usize, usize),
    steps_to_node: usize,
}

impl Node {
    fn new(key: (usize, usize), steps_to_node: usize) -> Self {
        Node { key, steps_to_node }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.steps_to_node.cmp(&other.steps_to_node)
    }
}

fn dijkstra(
    start: (usize, usize),
    end: (usize, usize),
    elevation: &HashMap<(usize, usize), usize>,
) -> Option<usize> {
    let i_max = *elevation.keys().map(|(i, _)| i).max().unwrap();
    let j_max = *elevation.keys().map(|(_, j)| j).max().unwrap();

    let mut shortest_paths = HashMap::<(usize, usize), usize>::new();
    let mut heap = BinaryHeap::new();
    heap.push(Reverse(Node::new(start, 0)));
    shortest_paths.insert(start, 0);
    while !heap.is_empty() {
        let node = match heap.pop() {
            Some(Reverse(x)) => x,
            None => break,
        };
        let neighbors = get_neighbors(&node.key, &(i_max, j_max));

        for n in neighbors {
            // Is this a valid neighbor...
            let node_e = elevation.get(&node.key).unwrap();
            let n_e = elevation.get(&n).unwrap();
            if *n_e > (node_e + &1) {
                continue;
            }
            // This should exist...
            let new_shortest_path = shortest_paths.get(&node.key).unwrap() + &1;
            let neighbor_shortest_path = *shortest_paths.get(&n).unwrap_or(&usize::MAX);
            if new_shortest_path < neighbor_shortest_path {
                heap.push(Reverse(Node::new(n, neighbor_shortest_path)));
                shortest_paths.insert(n, new_shortest_path);
            }
        }
    }
    shortest_paths.get(&end).copied()
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("input.txt")?;
    let mut elevation = HashMap::<(usize, usize), usize>::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut graph_max = (0, 0);
    let mut a_nodes = Vec::new();
    for (i, l) in file.lines().enumerate() {
        graph_max.0 = std::cmp::max(graph_max.0, i);
        for (j, c) in l.bytes().enumerate() {
            graph_max.1 = std::cmp::max(graph_max.1, j);
            let v = match c {
                b'S' => {
                    start = (i, j);
                    a_nodes.push(start);
                    calc_weight(b'a')
                }
                b'E' => {
                    end = (i, j);
                    calc_weight(b'z')
                }
                b'a' => {
                    a_nodes.push((i, j));
                    calc_weight(c)
                }
                _ => calc_weight(c),
            };
            elevation.insert((i, j), v);
        }
    }
    // P1
    println!("{}", dijkstra(start, end, &elevation).unwrap());

    // P2
    let all_a = a_nodes
        .iter()
        .map(|s| dijkstra(*s, end, &elevation).unwrap_or(usize::MAX))
        .collect::<Vec<usize>>();

    println!("{}", all_a.iter().min().unwrap());
    Ok(())
}
