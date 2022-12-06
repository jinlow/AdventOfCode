use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // P1
    let file = fs::read_to_string("input.txt")?;
    let w = 4;
    let pos = file
        .as_bytes()
        .to_vec()
        .windows(w)
        .position(|x| x.iter().collect::<HashSet<&u8>>().len() == w)
        .unwrap()
        + w;
    println!("{}", pos);

    // P2
    let w = 14;
    let pos = file
        .as_bytes()
        .to_vec()
        .windows(w)
        .position(|x| x.iter().collect::<HashSet<&u8>>().len() == w)
        .unwrap()
        + w;
    println!("{}", pos);
    Ok(())
}
