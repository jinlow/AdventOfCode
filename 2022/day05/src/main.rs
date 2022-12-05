use std::error::Error;
use std::fs;
use std::num::ParseIntError;

fn parse_crate(x: &str) -> Option<char> {
    let mut chars = x.chars();
    let first = match chars.next() {
        Some(c) => c,
        None => ' ',
    };
    if first == '[' {
        chars.next()
    } else {
        None
    }
}

fn parse_move(x: &str) -> Result<(usize, usize, usize), ParseIntError> {
    let m = x.split(" ").collect::<Vec<&str>>();
    Ok((
        m[1].parse::<usize>()?,
        m[3].parse::<usize>()? - 1,
        m[5].parse::<usize>()? - 1,
    ))
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("input.txt")?;
    let mut stacks: Vec<Vec<char>> = Vec::new();

    let n_stacks = (file.lines().next().unwrap().len() / 4) + 1;

    // Create the different stacks of crates.
    for _ in 0..n_stacks {
        stacks.push(Vec::new());
    }

    let mut file_iter = file.lines();
    for l in file_iter.by_ref() {
        // A line of all spaces, we know we have reached the end.
        if l.chars().all(|x| x.is_ascii_whitespace()) {
            break;
        }

        let mut row = l.chars().peekable();
        // while row.peek().is_some() {
        for s in 0..n_stacks {
            let chunk: String = row.by_ref().take(4).collect();
            if let Some(c) = parse_crate(chunk.as_str()) {
                stacks[s].push(c)
            }
        }
    }
    for s in stacks.iter_mut() {
        s.reverse();
    }
    let mut p1_stacks = stacks.clone();

    // Next process the moves
    let mut moves = Vec::new();
    for l in file_iter {
        moves.push(parse_move(l)?);
    }
    for m in moves.iter() {
        for _ in 0..(m.0) {
            let c = p1_stacks[m.1].pop().unwrap();
            p1_stacks[m.2].push(c);
        }
    }

    let message = p1_stacks
        .iter()
        .map(|s| s.to_owned().pop().unwrap())
        .collect::<String>();
    println!("{}", message);

    // P2
    let mut p2_stacks = stacks.clone();
    for m in moves.iter() {
        let n = p2_stacks[m.1].len();
        let cs = p2_stacks[m.1].split_off(n - m.0);
        p2_stacks[m.2].extend(cs);
    }

    let message = p2_stacks
        .iter()
        .map(|s| s.to_owned().pop().unwrap())
        .collect::<String>();
    println!("{}", message);

    Ok(())
}
