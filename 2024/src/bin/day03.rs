use regex::Regex;
use std::error::Error;
use std::fs;

fn fix_instructions(v: &str) -> i64 {
    let pat = Regex::new(r"mul\((\d+)\,(\d+)\)").unwrap();
    let instructions: Vec<(i64, i64)> = pat
        .captures_iter(&v)
        .map(|c| c.extract())
        .map(|(_, [d1, d2])| (d1.parse::<i64>().unwrap(), d2.parse::<i64>().unwrap()))
        .collect::<Vec<(i64, i64)>>();
    instructions.iter().map(|(x, y)| x * y).sum::<i64>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let inpt = fs::read_to_string("input/day03.txt")?;
    let p1 = fix_instructions(&inpt);
    println!("{}", p1);
    let mut p2 = 0;
    for chunk in inpt.split("do()") {
        p2 += match chunk.split_once("don't()") {
            Some((m, _)) => fix_instructions(&m),
            None => fix_instructions(chunk),
        }
    }
    println!("{}", p2);
    Ok(())
}
