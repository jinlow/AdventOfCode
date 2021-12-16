use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::hash::Hash;
use std::ops::Not;
fn main() {
    let input = fs::read_to_string("small_input.txt").expect("Unable to read file.");
    // let input = fs::read_to_string("day15_input.txt").expect("Unable to read file.");
    let mut map: Vec<Vec<u64>> = Vec::new();
    for l in input.lines() {
        let r: Vec<u64> = l
            .chars()
            .map(|i| i.to_digit(10).unwrap())
            .map(|x| u64::from(x))
            .collect();
        map.push(r);
    }
    // Part 1
    // Can we brute force it?
    // We cannot.
    let rows = map.len() - 1;
    let cols = map[0].len() - 1;

    println!("Matrix size: {}, {}", rows, cols);

    let mut dist: HashMap<(usize, usize), u64> = HashMap::new();
    let mut q = HashSet::new();
    let mut s: HashSet<(usize, usize)> = HashSet::new();
    dist.insert((0, 0), 0);
    q.insert((0, 0));
    for r in 1..rows {
        for c in 1..cols {
            dist.insert((r, c), u64::MAX);
            q.insert((r, c));
        }
    }

    while q.len() > 0 {
        let smallest = dist
            .iter()
            .filter(|(k, _)| s.contains(k).not())
            .min_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _)| k).unwrap();
        q.remove(smallest);
        s.insert(*smallest);
        // println!("{:?}", smallest);
    }

    // let mut p = CavePath::new();
    // loop {
    //     let (r, c) = p.point;
    //     if r < rows && c < cols {
    //         let mut p_c = p.clone();
    //         // Go down
    //         let t1 = p.total + map[r + 1][c];
    //         // Go right
    //         let t2 = p.total + map[r][c + 1];

    //         if t1 < t2 {
    //             p.total = t1;
    //             p.point = (r + 1, c);
    //         } else if t1 > t2 {
    //             p.total = t2;
    //             p.point = (r, c + 1);
    //         } else {

    //         }

    //     } else if r == rows && c < cols {
    //         p.total += map[r][c + 1];
    //         p.point = (r, c + 1);
    //     } else if r < rows && c == cols {
    //         p.total += map[r + 1][c];
    //         p.point = (r + 1, c);
    //     } else {
    //         break;
    //     }
    // }

    // let mut point_que = VecDeque::new();
    // let start_point = CavePath::new();
    // point_que.push_front(start_point);

    // let mut finished_paths = Vec::new();
    // while point_que.len() > 0 {
    //     if let Some(mut p) = point_que.pop_back() {
    //         let (r, c) = p.point;
    //         if r < rows && c < cols {
    //             let mut p_c = p.clone();

    //             p.total += map[r][c + 1];
    //             p.point = (r, c + 1);

    //             p_c.total += map[r + 1][c];
    //             p_c.point = (r + 1, c);

    //             point_que.push_front(p);
    //             point_que.push_front(p_c);
    //         } else if r == rows && c < cols {
    //             p.total += map[r][c + 1];
    //             p.point = (r, c + 1);
    //             point_que.push_front(p);
    //         } else if r < rows && c == cols {
    //             p.total += map[r + 1][c];
    //             p.point = (r + 1, c);
    //             point_que.push_front(p);
    //         } else {
    //             finished_paths.push(p.total);
    //         }
    //     };
    // }

    // finished_paths.sort();
    // println!("{}", finished_paths.len());
    // println!("{:?}", finished_paths[0..10].to_vec());
}

#[derive(Clone, Debug)]
struct CavePath {
    total: u64,
    point: (usize, usize),
}

impl CavePath {
    fn new() -> Self {
        CavePath {
            // data: Vec::new(),
            total: 0,
            point: (0, 0),
        }
    }
}
