use std::collections::{HashMap};
use std::fs;
fn main() {
    // let file = fs::read_to_string("small_input.txt").expect("File not found");
    let file = fs::read_to_string("day14_input.txt").expect("File not found");
    let lines: Vec<&str> = file.lines().collect();
    let start_input = lines[0].chars().collect::<Vec<char>>();
    let char_map = lines[2..]
        .iter()
        .map(|s| {
            let sv = s.split(" -> ").collect::<Vec<&str>>();
            let cs = sv[0].chars().collect::<Vec<char>>();
            ((cs[0], cs[1]), sv[1].chars().next().unwrap())
        })
        .collect::<HashMap<(char, char), char>>();

    // Part 1
    let mut poly = start_input.to_vec();
    for _ in 0..10    {
        let mut next_vec = Vec::new();
        for i in 0..poly.len() {
            if i + 1 == poly.len() {
                next_vec.push(poly[i]);
                break;
            }
            next_vec.push(poly[i]);
            if let Some(v) = char_map.get(&(poly[i], poly[i + 1])) {
                next_vec.push(*v);
            }
        }
        poly = next_vec;
    }

    let mut char_cnt: HashMap<&char, u64> = HashMap::new();

    for c in &poly {
        let i = char_cnt.entry(c).or_insert(0);
        *i += 1;
    }
    let most = match char_cnt.iter().map(|(_, cnt)| *cnt).max() {
        Some(v) => v,
        None => 0,
    };
    let least = match char_cnt.iter().map(|(_, cnt)| *cnt).min() {
        Some(v) => v,
        None => 0,
    };

    println!("10 Rounds Most - Least: {}", most - least);

    // Part 2 - Another one where we can't brute force it.
    let mut pair_cnt: HashMap<(char, char), u64> = HashMap::new();
    for c in start_input.windows(2) {
        let i = pair_cnt.entry((c[0], c[1])).or_insert(0);
        *i += 1;
    }

    let mut char_cnt: HashMap<&char, u64> = HashMap::new();
    for c in &start_input {
        let i = char_cnt.entry(c).or_insert(0);
        *i += 1;
    }

    for _ in 0..40 {
        let mut add_map: HashMap<(char, char), u64> = HashMap::new();
        for ((c1, c2), cnt) in pair_cnt.iter() {
            if let Some(v) = char_map.get(&(*c1, *c2)) {
                let i = add_map.entry((*c1, *v)).or_insert(0);
                *i = *i + *cnt;

                let i = add_map.entry((*v, *c2)).or_insert(0);
                *i = *i + *cnt;

                let i = char_cnt.entry(v).or_insert(0);
                *i = *i + cnt;
            }
        }
        pair_cnt = add_map;
    }

    let most = match char_cnt.iter().map(|(_, cnt)| *cnt).max() {
        Some(v) => v,
        None => 0,
    };
    let least = match char_cnt.iter().map(|(_, cnt)| *cnt).min() {
        Some(v) => v,
        None => 0,
    };

    println!("40 Rounds Most - Least: {}", most - least);
}
