use std::{fs, num::ParseIntError};

fn main() -> BoxResult<()> {
    // let file = fs::read_to_string("small_input.txt").expect("File not found.");
    let file = fs::read_to_string("day07_input.txt").expect("File not found.");
    let pos: Vec<i64> = file
        .split(",")
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<i64>, ParseIntError>>()?;
    let mut points = Vec::new();
    let mut fuel = Vec::new();
    let mut best_idx = 0;
    let min = pos.iter().min().unwrap();
    let max = pos.iter().max().unwrap();
    let mut init_fuel = pos.iter().map(|f| (f - min).abs()).sum::<i64>();
    println!("init_fuel: {}", init_fuel);
    for (i, p) in (*min..=*max).enumerate() {
        // Don't double count
        let f_use = if i == 0 {
            init_fuel} else {
                pos.iter().map(|f| (f - p).abs()).sum::<i64>()
            };
        fuel.push(f_use);
        points.push(p);
        if f_use < init_fuel {
            best_idx = i;
            init_fuel = f_use;
        }
    }
    println!("Best Idx {}", best_idx);
    println!("The lowest fuel is {}", fuel[best_idx]);
    println!("The best point is {}", points[best_idx]);

    // Part 2
    let mut points = Vec::new();
    let mut fuel = Vec::new();
    let mut best_idx = 0;
    let min = pos.iter().min().unwrap();
    let max = pos.iter().max().unwrap();
    let mut init_fuel = pos.iter().map(|f| fuel_cost((f - min).abs())).sum::<i64>();
    println!("init_fuel: {}", init_fuel);
    for (i, p) in (*min..=*max).enumerate() {
        let f_use = if i == 0 {
            init_fuel} else {
                pos.iter().map(|f| fuel_cost((f - p).abs())).sum::<i64>()
            };
        fuel.push(f_use);
        points.push(p);
        // println!("{}, {}", f_use, p);
        if f_use < init_fuel {
            best_idx = i;
            init_fuel = f_use;
        }
    }
    println!("Best Idx {}", best_idx);
    println!("The lowest fuel is {}", fuel[best_idx]);
    println!("The best point is {}", points[best_idx]);

    Ok(())
}

fn fuel_cost(d: i64) -> i64 {
    let mut p_add = 0;
    let mut total_fuel = 0;
    for _ in 0..d {
        total_fuel += (1 + p_add);
        p_add += 1;
    }
    total_fuel
}

type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;
