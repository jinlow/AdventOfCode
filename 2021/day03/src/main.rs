use std::num::ParseIntError;
use std::fs;

fn main() -> Result<(), ParseIntError>{
    let file = fs::read_to_string("day03_input.txt").expect("Problem reading file");
    let input: Vec<String> = file.lines().map(|x| x.to_string()).collect();
    let g = gamma(&input)?;
    let g_d = isize::from_str_radix(&g, 2).unwrap();

    let e = epsilon(&input)?;
    let e_d = isize::from_str_radix(&e, 2).unwrap();
    println!("Gamma x Epsilon is {}", g_d * e_d);

    let o = bit_filter(&input, String::from("oxygen"));
    let o_d = isize::from_str_radix(&o, 2).unwrap();

    let c = bit_filter(&input, String::from("co2"));
    let c_d = isize::from_str_radix(&c, 2).unwrap();
    println!("Life Support Rating is {}", o_d * c_d);
    Ok(())
}

fn bit_filter(inpt: &Vec<String>, rating: String) -> String {
    if rating == "oxygen" {
        return oxygen_bit_filter(inpt, 0)
    } else {
        return co2_bit_filter(inpt, 0)
    }
}

fn oxygen_bit_filter(inpt: &Vec<String>, bit: usize) -> String{
    let (ones, zeros) = bit_count(&inpt, bit);
    let mut filt_int = Vec::new();
    let k = if ones >= zeros {
        '1'
    } else {
        '0'
    };
    for s in inpt {
        if s.chars().nth(bit).unwrap() == k {
            filt_int.push(s.to_string())
        }
    };
    if filt_int.len() == 1 {
        return filt_int[0].to_string()
    } else {
        return oxygen_bit_filter(&filt_int, bit + 1);
    }
}

fn co2_bit_filter(inpt: &Vec<String>, bit: usize) -> String{
    let (ones, zeros) = bit_count(&inpt, bit);
    let mut filt_int = Vec::new();
    let k = if ones < zeros {
        '1'
    } else {
        '0'
    };
    for s in inpt {
        if s.chars().nth(bit).unwrap() == k {
            filt_int.push(s.to_string())
        }
    };
    if filt_int.len() == 1 {
        return filt_int[0].to_string()
    } else {
        return co2_bit_filter(&filt_int, bit + 1);
    }
}

fn bit_count(inpt: &Vec<String>, bit: usize) -> (i32, i32) {
    let mut ones = 0;
    let mut zeros = 0;
    for s in inpt {
        if s.chars().nth(bit).unwrap() == '1' {
            ones += 1;
        } else {
            zeros += 1;
        }
    }
    (ones, zeros)
}

fn inc_count(inpt: &String, ones: &mut Vec<u64>, zeros: &mut Vec<u64>) -> Result<(), ParseIntError>{
    for (i, c) in inpt.chars().enumerate() {
        let v = c.to_string().parse::<u64>()?;
        if v == 1 {
            ones[i] += 1;
        } else if v == 0{
            zeros[i] += 1;
        };
    }
    Ok(())
}

fn calc_stats(b: &Vec<String>) -> Result<(Vec<u64>, Vec<u64>), ParseIntError> {
    let mut ones = vec![0; b[0].len()];
    let mut  zeros = vec![0; b[0].len()];
    for s in b {
        inc_count(s, &mut ones, &mut zeros)?;
    }
    Ok((ones, zeros))
}

fn gamma(b: &Vec<String>) -> Result<String, ParseIntError> {
    let (ones, zeros) = calc_stats(&b)?;
    let mut g = String::new();
    for (o, z) in ones.iter().zip(zeros) {
        if o > &z {
            g.push('1');
        } else {
            g.push('0');
        }
    }
    Ok(g)
}


fn epsilon(b: &Vec<String>) -> Result<String, ParseIntError> {
    let (ones, zeros) = calc_stats(&b)?;
    let mut g = String::new();
    for (o, z) in ones.iter().zip(zeros) {
        if o < &z {
            g.push('1');
        } else {
            g.push('0');
        }
    }
    Ok(g)
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_gamma_epsilon() {
        let inpt = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        ];
        let g = gamma(&inpt).unwrap();
        assert_eq!(g, String::from("10110"));
        let g_d = isize::from_str_radix(&g, 2).unwrap();
        assert_eq!(g_d, 22);

        let e = epsilon(&inpt).unwrap();
        assert_eq!(e, String::from("01001"));
        let e_d = isize::from_str_radix(&e, 2).unwrap();
        assert_eq!(e_d, 9);
    }

    #[test]
    fn test_bit_filter() {
        let inpt = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        ];
        let o = bit_filter(&inpt, String::from("oxygen"));
        assert_eq!(o, String::from("10111"));
        let o_d = isize::from_str_radix(&o, 2).unwrap();
        assert_eq!(o_d, 23);

        let c = bit_filter(&inpt, String::from("co2"));
        assert_eq!(c, String::from("01010"));
        let c_d = isize::from_str_radix(&c, 2).unwrap();
        assert_eq!(c_d, 10);
    }
}
