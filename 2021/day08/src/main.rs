use std::collections::{HashMap, HashSet};
use std::fs;

type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;
fn main() -> BoxResult<()> {
    // let input = fs::read_to_string("small_input.txt").expect("unable to read file");
    let input = fs::read_to_string("day08_input.txt").expect("unable to read file");
    let outputs = input
        .lines()
        .map(|s| s.split(" | ").collect::<Vec<&str>>()[1])
        .map(|s| s.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let lengths = vec![2, 3, 4, 7];
    let mut total = 0;
    for o in &outputs {
        // println!("{:?}", o);
        let sum = o
            .iter()
            .map(|s| lengths.contains(&s.chars().count()) as usize)
            .sum::<usize>();
        // println!("Sum: {}", sum);
        total += sum;
    }
    println!("{}", total);

    // Part 2
    let mut n_list = Vec::new();
    for inpt in input.lines() {
        // let inpt =
        //     "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let split_inpts = inpt.split(" | ").collect::<Vec<&str>>();
        let nsets = split_inpts[0]
            .split(" ")
            .map(|s| HashSet::from_iter(s.chars()))
            .collect::<Vec<HashSet<char>>>();
        let output = split_inpts[1]
            .split(" ")
            .map(|s| HashSet::from_iter(s.chars()))
            .collect::<Vec<HashSet<char>>>();

        let mut nums: HashMap<u8, HashSet<char>> = HashMap::new();
        gen_known_numbers(&nsets, &mut nums);

        // Find what number is 6
        let len_six = nsets
            .iter()
            .filter(|s| s.len() == 6)
            .map(|s| s.clone())
            .collect::<Vec<HashSet<char>>>();

        let one = nums.entry(1).or_default().clone();
        set_six(&len_six, &one, &mut nums);

        // Get the letter C
        let six = nums.entry(6).or_default().clone();
        let c = &one - &six;

        let zero_and_nine = len_six
            .iter()
            .filter(|s| *s != &six)
            .map(|s| s.clone())
            .collect::<Vec<HashSet<char>>>();

        let four = nums.entry(4).or_default().clone();
        // println!("{:?}", zero_and_nine);
        set_zero_and_nine(&zero_and_nine, &four, &mut nums);

        let len_five = nsets
            .iter()
            .filter(|s| s.len() == 5)
            .map(|s| s.clone())
            .collect::<Vec<HashSet<char>>>();

        set_three(&len_five, &one, &mut nums);

        let three = nums.entry(3).or_default().clone();
        let two_and_five = len_five
            .iter()
            .filter(|s| *s != &three)
            .map(|s| s.clone())
            .collect::<Vec<HashSet<char>>>();

        set_two_five(&two_and_five, &c, &mut nums);

        let mut num_value = String::new();
        for n in output {
            num_value.push_str(get_num(&n, &nums).as_str());
        }

        let n_val = num_value.parse::<u64>()?;
        n_list.push(n_val);
    }
    let total: u64 = n_list.iter().sum();
    println!("Total output {}", total);
    Ok(())
}

// def two_five(two_and_five, c) -> Tuple[Set[str], Set[str]]:
// two, five = {}, {}
// for s in two_and_five:
//     if len(c - s) == 0:
//         two = s
//     else:
//         five = s
// return two, five

fn get_num(num: &HashSet<char>, nums: &HashMap<u8, HashSet<char>>) -> String {
    for (k, v) in nums {
        // println!("{:?}", num);
        if v == num {
            return k.to_string();
        }
    }
    "0".to_string()
}

fn set_two_five(
    two_and_five: &Vec<HashSet<char>>,
    c: &HashSet<char>,
    nums: &mut HashMap<u8, HashSet<char>>,
) {
    for s in two_and_five {
        if (c - s).len() == 0 {
            nums.insert(2, s.clone());
        } else {
            nums.insert(5, s.clone());
        }
    }
}

fn set_three(
    len_five: &Vec<HashSet<char>>,
    one: &HashSet<char>,
    nums: &mut HashMap<u8, HashSet<char>>,
) {
    for s in len_five {
        if (one - s).len() == 0 {
            nums.insert(3, s.clone());
        }
    }
}

fn set_zero_and_nine(
    zero_and_nine: &Vec<HashSet<char>>,
    four: &HashSet<char>,
    nums: &mut HashMap<u8, HashSet<char>>,
) {
    for s in zero_and_nine {
        if (s - four).len() == 3 {
            nums.insert(0, s.clone());
        } else {
            nums.insert(9, s.clone());
        }
    }
}

fn set_six(
    len_six: &Vec<HashSet<char>>,
    one: &HashSet<char>,
    nums: &mut HashMap<u8, HashSet<char>>,
) {
    for s in len_six {
        if (s - one).len() == 5 {
            nums.insert(6, s.clone());
        }
    }
}

fn gen_known_numbers(nsets: &Vec<HashSet<char>>, nums: &mut HashMap<u8, HashSet<char>>) {
    for s in nsets {
        if s.len() == 2 {
            nums.insert(1, s.clone());
        } else if s.len() == 4 {
            nums.insert(4, s.clone());
        } else if s.len() == 3 {
            nums.insert(7, s.clone());
        } else if s.len() == 7 {
            nums.insert(8, s.clone());
        }
    }
}
