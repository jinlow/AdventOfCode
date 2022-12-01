use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // P1
    let file = fs::read_to_string("p1.txt")?;
    let mut cals: Vec<i64> = Vec::new();
    cals.push(0);
    for l in file.lines() {
        if l == "" {
            cals.push(0);
        } else {
            let n = cals.len() - 1;
            cals[n] += l.parse::<i64>()?
        }
    }
    let max_cals = cals.iter().max().unwrap_or(&i64::MIN);
    println!("{:?}", max_cals);
    
    // P2
    cals.sort();
    cals.reverse();
    let top_3: i64 = cals[0..3].iter().sum();
    println!("{:?}", top_3);

    Ok(())


}
