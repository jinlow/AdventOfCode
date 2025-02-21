use std::collections::HashMap;
use std::error::Error;
use std::{fs, vec};
fn xmas_search<F: Fn(i64, i64, i64) -> (i64, i64)>(
    start: &(i64, i64),
    data: &HashMap<(i64, i64), char>,
    mv: F,
) -> i64 {
    let mut total = 0;
    let xmas = vec!['X', 'M', 'A', 'S'];
    for i in 0..4 {
        if let Some(v) = data.get(&mv(start.0, start.1, i as i64)) {
            if *v != xmas[i] {
                break;
            }
            if *v == 'S' {
                total += 1
            }
            continue;
        }
        break;
    }
    total
}

fn mas_search(start: &(i64, i64), data: &HashMap<(i64, i64), char>) -> bool {
    // Forward slash
    let mut fws: Vec<char> = vec![];
    
    if let Some(v) = data.get(&(start.0 - 1, start.1 - 1)) {
        fws.push(*v);
    }
    if let Some(v) = data.get(&(start.0 + 1, start.1 + 1)) {
        fws.push(*v);
    }
    // Backslash
    let mut bks: Vec<char> = vec![];
    if let Some(v) = data.get(&(start.0 + 1, start.1 - 1)) {
        bks.push(*v);
    }
    if let Some(v) = data.get(&(start.0 - 1, start.1 + 1)) {
        bks.push(*v);
    }
    fws.sort();
    bks.sort();
    return (fws == vec!['M', 'S']) && (bks == vec!['M', 'S']);
}

fn main() -> Result<(), Box<dyn Error>> {
    let inpt = fs::read_to_string("input/day04.txt")?;
    let mut data = HashMap::new();
    let mut xstarts = Vec::new();
    let mut astarts = Vec::new();
    for (i, l) in inpt.lines().enumerate() {
        for (j, char) in l.chars().enumerate() {
            let pos = (i as i64, j as i64);
            data.insert(pos.clone(), char);
            if char == 'X' {
                xstarts.push(pos);
            } 
            if char == 'A' {
                astarts.push(pos)
            }
        }
    }
    let mut xmas_total = 0;
    let mv_fn: [fn(i64, i64, i64) -> (i64, i64); 8] = [
        |i, j, n| (i, j + n),     // up
        |i, j, n| (i, j - n),     // down
        |i, j, n| (i - n, j),     // left
        |i, j, n| (i + n, j),     // right
        |i, j, n| (i - n, j + n), // up-left
        |i, j, n| (i + n, j + n), // up-right
        |i, j, n| (i + n, j - n), // down-right
        |i, j, n| (i - n, j - n), // down-left
    ];
    for start in xstarts {
        for mv in mv_fn {
            xmas_total += xmas_search(&start, &data, mv);
        }
    }

    println!("{}", xmas_total);

    let mut mas_total = 0;
    println!("{}", astarts.len());
    for start in astarts {
        mas_total += mas_search(&start, &data) as i64;
    }

    println!("{}", mas_total);
    Ok(())
}
