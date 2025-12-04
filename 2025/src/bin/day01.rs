use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let i = fs::read_to_string("input/day01.txt")?;
    let turns: Vec<(char, i32)> = i
        .lines()
        .map(|s| (s.chars().next().unwrap(), s[1..].parse::<i32>().unwrap()))
        .collect();
    let mut cur = 50;
    let mut p1: i32 = 0; // Part 1
    let mut p2 = 0; // Part 2
    for (d, n) in turns.iter() {
        let nt: i32 = match d {
            &'L' => -1,
            &'R' => 1,
            _ => panic!(),
        };
        for _ in 0..*n {
            cur = (cur + nt).rem_euclid(100);
            p2 += (cur == 0) as i32;
        }
        p1 += (cur == 0) as i32;
    }
    println!("{},{}", p1, p2);
    Ok(())
}
