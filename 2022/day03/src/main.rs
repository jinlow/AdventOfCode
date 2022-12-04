use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn order(x: u8) -> u8 {
    // Convert bytes into the values expected
    if x < 97 {
        x - b'A' + 27
    } else {
        x - b'a' + 1
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // P1
    let file = fs::read_to_string("input.txt")?;
    let p = file
        .lines()
        .map(|x| x.as_bytes())
        .map(|x| {
            let n = x.len() / 2;
            let c1 = x[..n].iter().map(|x| *x).collect::<HashSet<u8>>();
            let c2 = x[n..].iter().map(|x| *x).collect::<HashSet<u8>>();
            order(*c1.intersection(&c2).next().unwrap()) as i64
        })
        .collect::<Vec<i64>>();
    let psum = p.iter().sum::<i64>();
    println!("{}", psum);

    // P2
    let groups = file.lines().map(|x| x.as_bytes()).collect::<Vec<&[u8]>>();
    let p = groups.chunks(3).map(|x| {
        // Create a list of sets
        let mut sets = x
            .iter()
            .map(|x| x.iter().copied().collect::<HashSet<u8>>())
            .collect::<Vec<HashSet<u8>>>();
        // Grab the first set.
        let mut res = sets.pop().unwrap();
        // Only retain items in both.
        res.retain(|i| sets.iter().all(|set| set.contains(i)));
        order(res.into_iter().next().unwrap()) as i64
    }).collect::<Vec<i64>>();
    let psum = p.iter().sum::<i64>();
    println!("{}", psum);
    Ok(())
}
