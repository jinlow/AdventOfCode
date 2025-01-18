use std::collections::HashMap;
use std::error::Error;
use std::fs;
fn main() -> Result<(), Box<dyn Error>> {
    let i = fs::read_to_string("input/day01.txt")?;
    let (mut l1, mut l2): (Vec<i64>, Vec<i64>) = i
        .lines()
        .map(|l| {
            // let (x, y) = l.split_once(" ").unwrap();
            // (x.trim().parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
            let x: Vec<i64> = l
                .split(" ")
                .map(|el| el.trim())
                .filter(|x| x.len() > 0)
                .map(|x| x.parse().unwrap())
                .collect();
            (x[0], x[1])
        })
        .unzip();
    l1.sort();
    l2.sort();
    let diff: i64 = l1.iter().zip(l2.iter()).map(|(x, y)| (x - y).abs()).sum();
    println!("The difference {}", diff);

    // The similarity score map...
    let mut scmap = HashMap::new();
    for i in l2 {
        let n = scmap.entry(i).or_insert_with(|| 0);
        *n += 1;
    }
    let score: i64 = l1.iter().map(|x| {
        let n = match scmap.get(x) {
            None => 0,
            Some(v) => *v,
        };
        x * n}
    ).sum();
    println!("Similarity score {}", score);
    Ok(())
}
