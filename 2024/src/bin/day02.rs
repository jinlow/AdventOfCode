use std::error::Error;
use std::fs;

fn is_safe(v: &[i64]) -> bool {
    let mut sign = (v[0] - v[1]).signum();
    for x in v.windows(2) {
        let diff = x[0] - x[1];
        let nsign = diff.signum();
        if (sign != nsign) || (diff.abs() > 3) || (diff.abs() == 0) {
            return false;
        }
        sign = nsign;
    }
    true
}

fn is_safe_tollerant(v: &[i64]) -> bool {
    if is_safe(v) {
        return true
    } else {
        for i in 0..v.len() {
            let mut vc = v.to_vec();
            vc.remove(i);
            if is_safe(&vc) {
                return true
            }
        }
    }
    false
}


fn main() -> Result<(), Box<dyn Error>> {
    let i = fs::read_to_string("input/day02.txt")?
        .lines()
        .map(|l| l.split(" ").map(|n| n.parse().unwrap()).collect())
        .collect::<Vec<Vec<i64>>>();
    let nsafe: i64 = i.iter().map(|v| is_safe(v) as i64).sum();
    println!("{}", nsafe);

    let tol_nsafe: i64 = i.iter().map(|v| is_safe_tollerant(v) as i64).sum();
    println!("{}", tol_nsafe);
    Ok(())
}
