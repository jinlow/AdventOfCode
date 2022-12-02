use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::hash::Hash;

fn main() -> Result<(), Box<dyn Error>> {
    // P1
    let file = fs::read_to_string("p1.txt")?;
    let matches = file
        .lines()
        .map(|x| x.split(" "))
        .map(|mut x| (x.next().unwrap(), x.next().unwrap()))
        .collect::<Vec<(&str, &str)>>();
    let mut scores: Vec<i64> = Vec::new();
    let match_map = HashMap::from([("X", "A"), ("Y", "B"), ("Z", "C")]);
    let win = HashSet::from([("A", "B"), ("B", "C"), ("C", "A")]);
    let values = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);
    for (o, raw_p) in &matches {
        let p = *match_map.get(raw_p).unwrap();
        let mut points = *values.get(p).unwrap();
        if win.contains(&(o, p)) {
            points += 6
        } else if *o == p {
            points += 3
        }
        scores.push(points);
    }
    let total_score = scores.iter().sum::<i64>();
    println!("{}", total_score);
    // P2
    let lose_map = win
        .iter()
        .map(|(o, p)| (*p, *o))
        .collect::<HashMap<&str, &str>>();
    let win_map = win.into_iter().collect::<HashMap<&str, &str>>();
    let mut scores: Vec<i64> = Vec::new();
    for (o, p) in matches {
        let points = match p {
            // Lose
            "X" => {
                let lose_play = *lose_map.get(o).unwrap();
                let points = *values.get(lose_play).unwrap();
                points
            },
            // Draw
            "Y" => {
                let points = *values.get(o).unwrap() + 3;
                points
            },
            // Win Rocky, Win!
            _ => {
                let win_play = *win_map.get(o).unwrap();
                let points = *values.get(win_play).unwrap() + 6;
                points
            },
        };
        scores.push(points)
    }
    let total_score = scores.iter().sum::<i64>();
    println!("{}", total_score);
    Ok(())
}
