use std::fs;

fn main() -> Result<(), std::io::Error> {
    let mut input: Vec<i64> = Vec::new();
    let file = fs::read_to_string("day1_input.txt")?;
    file.lines()
        .map(|x| x.parse::<i64>().unwrap())
        .for_each(|x| input.push(x));
    let n = count_greater(&input);
    println!("The input data is {} records", input.len());
    println!("There are {} greater instances.", n);
    let w_n = count_greater_windowed(&input, 3);
    println!("There are {} greater instances with a window of 3.", w_n);

    Ok(())
}

fn count_greater(x: &[i64]) -> i64 {
    let mut current = x[0];
    let mut n_greater = 0;
    for i in &x[1..] {
        if i > &current {
            n_greater += 1;
        }
        current = *i;
    }
    return n_greater;
}

fn count_greater_windowed(x: &[i64], window: usize) -> i64 {
    let mut current = x[0..window].iter().sum::<i64>();
    let mut n_greater = 0;
    let mut new: i64;
    for i in 1..x.len() {
        if (i + window) == x.len() {
            new = x[i..].iter().sum::<i64>();
            if new > current {
                n_greater += 1;
            }
            break;
        } else if (i + window) > x.len() {
            break;
        }
        new = x[i..(i + window)].iter().sum::<i64>();

        if new > current {
            n_greater += 1;
        }

        current = new;
    }

    return n_greater;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_greater() {
        let x = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let n = count_greater(&x);
        assert_eq!(n, 7);
    }

    #[test]
    fn test_count_greater_windowed() {
        let x = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let n = count_greater_windowed(&x, 3);
        assert_eq!(n, 5);
    }
}
