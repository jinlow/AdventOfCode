use std::error::Error;
use std::fs;

fn left_contains_right(p1: &(i64, i64), p2: &(i64, i64)) -> bool {
    (p1.0 <= p2.0) && (p1.1 >= p2.1)
}

fn left_overlaps_right(p1: &(i64, i64), p2: &(i64, i64)) -> bool {
    ((p1.0 <= p2.0) && (p1.1 >= p2.0))
        || ((p1.0 >= p2.0) && (p1.1 <= p2.1))
        || ((p1.0 <= p2.0) && (p1.1 >= p2.1))
}

fn main() -> Result<(), Box<dyn Error>> {
    // P1
    let pairs = fs::read_to_string("input.txt")?
        .lines()
        .map(|i| {
            let mut res = i.split(",").into_iter().map(|j| {
                let mut p = j.split("-").into_iter().map(|x| x.parse::<i64>().unwrap());
                (p.next().unwrap(), p.next().unwrap())
            });
            (res.next().unwrap(), res.next().unwrap())
        })
        .collect::<Vec<((i64, i64), (i64, i64))>>();
    let contains = pairs
        .iter()
        .map(|(p1, p2)| left_contains_right(p1, p2) || left_contains_right(p2, p1))
        .map(|i| i as i64)
        .collect::<Vec<i64>>();
    let n_contains = contains.iter().sum::<i64>();
    println!("{}", n_contains);

    // P2
    let overlaps = pairs
    .iter()
    .map(|(p1, p2)| left_overlaps_right(p1, p2) || left_overlaps_right(p2, p1))
    .map(|i| i as i64)
    .collect::<Vec<i64>>();
    let n_overlaps = overlaps.iter().sum::<i64>();
    println!("{}", n_overlaps);
    Ok(())
}
